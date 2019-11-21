use core::fmt::{Debug, Error, Formatter};

static mut COMPILED: String = String::new();

#[allow(unused_must_use)]
pub fn init() {
    TEMP0.clone();
    TEMP1.clone();
    TEMP2.clone();
    TEMP3.clone();
    TEMP4.clone();
    TEMP5.clone();
    TEMP6.clone();
}

pub fn compile() -> String {
    unsafe {
        return COMPILED.clone();
    }
}

static mut CONTROL_REGISTERS: Vec<Value> = Vec::new();
pub struct Control;
impl Control {
    pub fn if_begin(var: &Value) {
        unsafe {
            COMPILED += "\nIF BEGIN\n";
            TEMP0.zero();
            CONTROL_REGISTERS.push(var.clone());
            COMPILED += &(var.to() + "[" + &var.from());
            COMPILED += "\nCODE BEGIN\n";
        }
    }

    pub fn if_end() {
        unsafe {
            COMPILED += "\nCODE END\n";
            TEMP0.zero();
            let var = CONTROL_REGISTERS.pop().unwrap();
            TEMP1.assign(&var);
            var.zero();
            COMPILED += &(var.to() + "]" + &var.from());
            var.assign(&TEMP1);
            TEMP1.zero();
            COMPILED += "\nIF END\n";
        }
    }
}

pub struct Stdout;
impl Stdout {
    pub fn print(var: &Value) {
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
}

lazy_static! {
    static ref TEMP0: Value = Value::new(1);
    static ref TEMP1: Value = Value::new(1);
    static ref TEMP2: Value = Value::new(1);
    static ref TEMP3: Value = Value::new(1);
    static ref TEMP4: Value = Value::new(1);
    static ref TEMP5: Value = Value::new(1);
    static ref TEMP6: Value = Value::new(1);
}

/// This variable is responsible for keeping track of each statically allocated variable
/// For example, if a variable `test` is allocated statically with size 4, the STACK_PTR
/// will be allocated by 4, and the next variable will be allocated at the STACK_PTR
static mut STACK_PTR: u32 = 0;

// a = alloc(8);
// stdout.print(*a);
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Value {
    to_instructions: String,
    from_instructions: String,
    number_cells: u32,
    address: Option<u32>,
    is_reference: bool,
}


impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.address {
            Some(addr) => write!(f, "(to `{}` | from `{}` | {} wide | @{})", self.to(), self.from(), self.size(), addr),
            None => write!(f, "(to `{}` | from `{}` | {} wide)", self.to(), self.from(), self.size())
        }
    }
}


impl Value {
    pub fn new(size: u32) -> Self {
        unsafe {
            let result = Self {
                to_instructions: ">".repeat(STACK_PTR as usize),
                from_instructions: "<".repeat(STACK_PTR as usize),
                number_cells: size,
                address: Some(STACK_PTR),
                is_reference: false,
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

    pub fn zero(&self) {
        unsafe {
            COMPILED += &self.to();
            COMPILED += "[-]";
            COMPILED += &self.from();
        }
    }

    pub fn free(&self) {
        unsafe {
            if let Some(addr) = self.address {
                COMPILED += &format!("\nFREEING CELLS {}~{}\n", addr, addr + self.size());
            } else {
                COMPILED += "FREEING CELLS";
            }
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

    pub fn assign(&self, val: &Self) {
        if val.size() > self.size() {
            panic!("Cannot assign larger variable to smaller variable");
        }

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

    pub fn character(value: char) -> Self {
        let result = Self::new(1);
        unsafe {
            COMPILED += &result.to();
            COMPILED += &"+".repeat(value as usize);
            COMPILED += &result.from();
        }
        return result;
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

        return result;
    }

    pub fn to(&self) -> String {
        self.to_instructions.to_string()
    }

    pub fn from(&self) -> String {
        self.from_instructions.to_string()
    }

    pub fn size(&self) -> u32 {
        self.number_cells
    }

    pub fn deref(&self) -> Self {
        let mut result = Self::new(1);
        result.is_reference = true;
        result.number_cells = self.size();

        result.to_instructions = self.to_instructions.clone();
        result.from_instructions = self.from_instructions.clone();
        result.to_instructions += "*";
        result.from_instructions = String::from("&") + &result.from_instructions;
        result
    }

    pub fn refer(&self) -> Self {
        let mut result = Self::new(1);
        result.number_cells = self.size();
        match self.address {
            Some(addr) => unsafe {
                COMPILED += &result.to();
                COMPILED += &"+".repeat(addr as usize);
                COMPILED += &result.from();
            },
            _ => {}
        }
        result
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        // println!("FREEING {:#?}", self);
        if !self.is_reference {
            self.free();
        }
    }
}
