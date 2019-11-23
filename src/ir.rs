use crate::Error;
use core::fmt;

static mut COMPILED: String = String::new();

#[allow(unused_must_use)]
pub fn init() {
    let f = |_: &Value| {};
    f(&RETURN);
    f(&TEMP0);
    f(&TEMP1);
    f(&TEMP2);
    f(&TEMP3);
    f(&TEMP4);
    f(&TEMP5);
    f(&TEMP6);
}

pub fn compile() -> String {
    RETURN.free();
    TEMP0.free();
    TEMP1.free();
    TEMP2.free();
    TEMP3.free();
    TEMP4.free();
    TEMP5.free();
    TEMP6.free();
    unsafe {
        COMPILED += &format!("\nFINAL STACK PTR POS: {}", STACK_PTR);
        return COMPILED.clone();
    }
}

static mut CONTROL_REGISTERS: Vec<Value> = Vec::new();
pub struct Control;
impl Control {
    pub fn if_begin(var: Value) {
        unsafe {
            COMPILED += "\nIF BEGIN\n";
            TEMP0.zero();
            CONTROL_REGISTERS.push(var);
            COMPILED += &(var.to() + "[" + &var.from());
            COMPILED += "\nCODE BEGIN\n";
        }
    }

    pub fn if_end() {
        unsafe {
            COMPILED += "\nCODE END\n";
            TEMP0.zero();
            let var = CONTROL_REGISTERS.pop().unwrap();
            TEMP1.assign(var);
            var.zero();
            COMPILED += &(var.to() + "]" + &var.from());
            var.assign(*TEMP1);
            TEMP1.zero();
            COMPILED += "\nIF END\n";
        }
    }

    pub fn while_begin(var: Value) {
        unsafe {
            COMPILED += "\nWHILE BEGIN\n";
            CONTROL_REGISTERS.push(var);
            COMPILED += &(var.to() + "[" + &var.from());
            COMPILED += "\nCODE BEGIN\n";
        }
    }

    pub fn while_end() {
        unsafe {
            COMPILED += "\nCODE END\n";
            let var = CONTROL_REGISTERS.pop().unwrap();
            COMPILED += &(var.to() + "]" + &var.from());
            COMPILED += "\nWHILE END\n";
        }
    }
}

pub struct Stdout;
impl Stdout {
    pub fn print(var: Value) {
        let mut result = String::new();

        result += &var.to();
        for _ in 0..var.size() {
            result += ".>";
        }
        for _ in 0..var.size() {
            result += "<";
        }
        result += &var.from();

        unsafe {
            COMPILED += "\nPRINT CELL\n";
            COMPILED += &result;
            COMPILED += "\nDONE\n";
        }
    }

    pub fn print_cstr(var: Value) {
        let mut result = String::new();

        result += &var.to();
        result += "*[.>]&";
        result += &var.from();

        unsafe {
            COMPILED += "\nPRINT CELL\n";
            COMPILED += &result;
            COMPILED += "\nDONE\n";
        }
    }
}

lazy_static! {
    pub static ref RETURN: Value = Value::new(1);
    pub static ref TEMP0: Value = Value::new(1);
    pub static ref TEMP1: Value = Value::new(1);
    pub static ref TEMP2: Value = Value::new(1);
    pub static ref TEMP3: Value = Value::new(1);
    pub static ref TEMP4: Value = Value::new(1);
    pub static ref TEMP5: Value = Value::new(1);
    pub static ref TEMP6: Value = Value::new(1);
}

/// This variable is responsible for keeping track of each statically allocated variable
/// For example, if a variable `test` is allocated statically with size 4, the STACK_PTR
/// will be allocated by 4, and the next variable will be allocated at the STACK_PTR
pub static mut STACK_PTR: u32 = 0;

// a = alloc(8);
// stdout.print(*a);
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Value {
    pub offset: u32,
    pub reference_depth: u32,
    pub number_cells: u32,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "(to `{}` | from `{}` | {} wide)",
            self.to(),
            self.from(),
            self.size()
        )
    }
}

impl Value {
    pub fn new(size: u32) -> Self {
        unsafe {
            let result = Self {
                offset: STACK_PTR,
                reference_depth: 0,
                number_cells: size,
            };
            STACK_PTR += size;
            return result;
        }
    }

    pub fn alloc(size: u32) -> Self {
        let mut result = Self::new(1);
        result.number_cells = size;

        unsafe {
            COMPILED += &format!("\nALLOCATING {} CELLS\n", size);
            COMPILED += &result.to();
            COMPILED += &"+".repeat(size as usize);
            COMPILED += "?*";
            COMPILED += &"+>".repeat(size as usize);
            COMPILED += "&";
            COMPILED += &result.from();
            COMPILED += "\nDONE\n";
        }
        result
    }

    pub fn variable_alloc(size: Self) -> Self {
        let result = Self::new(1);

        result.assign(size);

        unsafe {
            COMPILED += "\nALLOCATING CELLS\n";
            COMPILED += &result.to();
            COMPILED += "?";
            COMPILED += &result.from();
            COMPILED += "\nDONE\n";
        }

        result
    }

    pub fn copy(&self) -> Self {
        let val = Value::new(self.number_cells);
        val.assign(*self);
        val
    }

    pub fn is_ref(&self) -> bool {
        self.reference_depth > 0
    }

    pub fn zero(&self) {
        unsafe {
            COMPILED += &self.to();
            COMPILED += "[-]";
            COMPILED += &self.from();
        }
    }

    pub fn free(&self) {
        unsafe {
            COMPILED += &format!(
                "\nFREEING CELLS {}~{}\n",
                self.offset,
                self.offset + self.size()
            );
            COMPILED += &self.to();

            for _ in 0..self.size() {
                COMPILED += "[-]>";
            }

            for _ in 0..self.size() {
                COMPILED += "<";
            }

            COMPILED += &self.from();

            COMPILED += "\nDONE\n";
        }
    }

    pub fn set(&self, val: impl Into<usize>) {
        unsafe {
            COMPILED += &self.to();
            COMPILED += "[-]";
            COMPILED += &"+".repeat(val.into());
            COMPILED += &self.from();
        }
    }

    pub fn assign(&self, val: Self) {
        TEMP0.zero();

        unsafe {
            for cell in 0..val.size() {
                let cell_to = val.to() + &">".repeat(cell as usize);
                let cell_from = "<".repeat(cell as usize) + &val.from();
                let this_to = self.to() + &">".repeat(cell as usize);
                let this_from = "<".repeat(cell as usize) + &self.from();
                COMPILED += &(this_to.clone() + "[-]" + &this_from);
                COMPILED += &(cell_to.clone()
                    + "["
                    + &cell_from
                    + &this_to
                    + "+"
                    + &this_from
                    + &TEMP0.to()
                    + "+"
                    + &TEMP0.from()
                    + &cell_to
                    + "-"
                    + &cell_from
                    + &cell_to
                    + "]"
                    + &cell_from);
                COMPILED += &(TEMP0.to()
                    + "["
                    + &TEMP0.from()
                    + &cell_to
                    + "+"
                    + &cell_from
                    + &TEMP0.to()
                    + "-"
                    + &TEMP0.from()
                    + &TEMP0.to()
                    + "]"
                    + &TEMP0.from());
            }
        }
    }

    pub fn plus_eq(&self, val: Self) {
        TEMP0.zero();

        unsafe {
            for cell in 0..val.size() {
                let cell_to = val.to() + &">".repeat(cell as usize);
                let cell_from = "<".repeat(cell as usize) + &val.from();
                let this_to = self.to() + &">".repeat(cell as usize);
                let this_from = "<".repeat(cell as usize) + &self.from();
                COMPILED += &(cell_to.clone()
                    + "["
                    + &cell_from
                    + &this_to
                    + "+"
                    + &this_from
                    + &TEMP0.to()
                    + "+"
                    + &TEMP0.from()
                    + &cell_to
                    + "-"
                    + &cell_from
                    + &cell_to
                    + "]"
                    + &cell_from);
                COMPILED += &(TEMP0.to()
                    + "["
                    + &TEMP0.from()
                    + &cell_to
                    + "+"
                    + &cell_from
                    + &TEMP0.to()
                    + "-"
                    + &TEMP0.from()
                    + &TEMP0.to()
                    + "]"
                    + &TEMP0.from());
            }
        }
    }

    pub fn minus_eq(&self, val: Self) {
        TEMP0.zero();

        unsafe {
            for cell in 0..val.size() {
                let cell_to = val.to() + &">".repeat(cell as usize);
                let cell_from = "<".repeat(cell as usize) + &val.from();
                let this_to = self.to() + &">".repeat(cell as usize);
                let this_from = "<".repeat(cell as usize) + &self.from();
                COMPILED += &(cell_to.clone()
                    + "["
                    + &cell_from
                    + &this_to
                    + "-"
                    + &this_from
                    + &TEMP0.to()
                    + "+"
                    + &TEMP0.from()
                    + &cell_to
                    + "-"
                    + &cell_from
                    + &cell_to
                    + "]"
                    + &cell_from);
                COMPILED += &(TEMP0.to()
                    + "["
                    + &TEMP0.from()
                    + &cell_to
                    + "+"
                    + &cell_from
                    + &TEMP0.to()
                    + "-"
                    + &TEMP0.from()
                    + &TEMP0.to()
                    + "]"
                    + &TEMP0.from());
            }
        }
    }

    pub fn byte_int(value: u8) -> Self {
        let result = Self::new(1);
        unsafe {
            COMPILED += &result.to();
            COMPILED += &"+".repeat(value as usize);
            COMPILED += &result.from();
        }
        result
    }

    pub fn unsigned_4byte_int(value: u32) -> Self {
        let result = Self::new(1);
        unsafe {
            COMPILED += &result.to();
            COMPILED += &"+".repeat(value as usize);
            COMPILED += &result.from();
        }
        result
    }

    pub fn character(value: char) -> Self {
        let result = Self::new(1);
        unsafe {
            COMPILED += &result.to();
            COMPILED += &"+".repeat(value as usize);
            COMPILED += &result.from();
        }
        result
    }

    pub fn string(value: impl ToString) -> Self {
        let result = Self::new(value.to_string().len() as u32);

        unsafe {
            COMPILED += &result.to();
            for ch in value.to_string().chars() {
                COMPILED += &("+".repeat(ch as usize) + ">");
            }
            for _ in value.to_string().chars() {
                COMPILED += "<";
            }
            COMPILED += &result.from();
        }

        result
    }

    pub fn to(&self) -> String {
        ">".repeat(self.offset as usize).to_string() + &"*".repeat(self.reference_depth as usize)
    }

    pub fn from(&self) -> String {
        "&".repeat(self.reference_depth as usize) + &"<".repeat(self.offset as usize).to_string()
    }

    pub fn size(&self) -> u32 {
        self.number_cells
    }

    pub fn deref(&self) -> Self {
        let mut result = Self::new(1);
        // result.number_cells = self.size();

        result.reference_depth = self.reference_depth + 1;
        result.offset = self.offset;

        result
    }

    pub fn refer(&self) -> Result<Self, Error> {
        let result = Self::new(1);
        
        unsafe {
            COMPILED += &result.to();
            COMPILED += &"+".repeat(self.offset as usize);
            COMPILED += &result.from();
        }

        Ok(result)
    }
}
