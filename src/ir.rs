use crate::{Error, Eval, Literal, Lower, Program};
use core::fmt;
use std::sync::Mutex;

lazy_static! {
    static ref COMPILED: Mutex<String> = Mutex::new(String::new());
    static ref CONTROL_REGISTERS: Mutex<Vec<Value>> = Mutex::new(Vec::new());

    /// This variable is responsible for keeping track of each statically allocated variable
    /// For example, if a variable `test` is allocated statically with size 4, the STACK_PTR
    /// will be allocated by 4, and the next variable will be allocated at the STACK_PTR
    pub static ref STACK_PTR: Mutex<u32> = Mutex::new(0);
    pub static ref RETURN: Value = Value::new(1).unwrap();
    pub static ref TEMP0: Value = Value::new(1).unwrap();
    pub static ref TEMP1: Value = Value::new(1).unwrap();
    pub static ref TEMP2: Value = Value::new(1).unwrap();
    pub static ref TEMP3: Value = Value::new(1).unwrap();
    pub static ref TEMP4: Value = Value::new(1).unwrap();
    pub static ref TEMP5: Value = Value::new(1).unwrap();
    pub static ref TEMP6: Value = Value::new(1).unwrap();

    pub static ref STACK_SIZE: Mutex<u32> = Mutex::new(16384);
    pub static ref HEAP_SIZE: Mutex<u32> = Mutex::new(8192);
}

pub fn increment_stack(allocation_size: u32) -> Result<(), Error> {
    let mut stack_ptr = STACK_PTR.lock().unwrap();
    *stack_ptr += allocation_size;
    if *stack_ptr > *STACK_SIZE.lock().unwrap() {
        Err(Error::StackOverflow)
    } else {
        Ok(())
    }
}

pub fn set_stack(stack_size: u32) -> Result<(), Error> {
    let mut stack_ptr = STACK_PTR.lock().unwrap();
    *stack_ptr = stack_size;
    if *stack_ptr > *STACK_SIZE.lock().unwrap() {
        Err(Error::StackOverflow)
    } else {
        Ok(())
    }
}

pub fn add_to_compiled(s: impl ToString) {
    let mut c = COMPILED.lock().unwrap();
    (*c) += &s.to_string();
}

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

    add_to_compiled(format!(
        "STARTING STACK PTR IS {} ",
        *STACK_PTR.lock().unwrap()
    ));
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

    add_to_compiled(format!(
        "FINAL STACK PTR IS {} ",
        *STACK_PTR.lock().unwrap()
    ));

    COMPILED.lock().unwrap().clone()
}

pub struct Control;
impl Control {
    pub fn if_begin(var: Value) -> Result<(), Error> {
        add_to_compiled("\nIF BEGIN\n");
        TEMP5.assign(var)?;
        TEMP6.assign(Eval::Literal(Literal::byte_int(1)).lower()?)?;

        // CONTROL_REGISTERS.lock().unwrap().push(var);

        Self::while_begin(*TEMP5);
        add_to_compiled("\nTHEN CODE BEGIN\n");
        Ok(())
    }

    pub fn else_begin() -> Result<(), Error> {
        add_to_compiled("\nTHEN CODE END\n");
        TEMP5.zero();
        TEMP6.zero();
        Self::while_end();
        Self::while_begin(*TEMP6);
        add_to_compiled("\nELSE CODE BEGIN\n");
        // TEMP0.zero();
        // let var = CONTROL_REGISTERS.lock().unwrap().pop().unwrap();
        // TEMP1.assign(var)?;
        // var.zero();
        // add_to_compiled(var.to() + "]" + &var.from());
        // var.assign(*TEMP1)?;
        // TEMP1.zero();
        Ok(())
    }

    pub fn if_end() -> Result<(), Error> {
        add_to_compiled("\nELSE CODE END\n");
        TEMP6.zero();
        Self::while_end();
        // TEMP0.zero();
        // let var = CONTROL_REGISTERS.lock().unwrap().pop().unwrap();
        // TEMP1.assign(var)?;
        // var.zero();
        // add_to_compiled(var.to() + "]" + &var.from());
        // var.assign(*TEMP1)?;
        // TEMP1.zero();
        add_to_compiled("\nIF END\n");
        Ok(())
    }

    pub fn while_begin(var: Value) {
        add_to_compiled("\nWHILE BEGIN\n");
        // TEMP0.zero();
        CONTROL_REGISTERS.lock().unwrap().push(var);
        add_to_compiled(var.to() + "[" + &var.from());
        add_to_compiled("\nCODE BEGIN\n");
    }

    pub fn while_end() {
        add_to_compiled("\nCODE END\n");
        let var = CONTROL_REGISTERS.lock().unwrap().pop().unwrap();
        add_to_compiled(var.to() + "]" + &var.from());
        add_to_compiled("\nWHILE END\n");
    }
}

pub struct Stdin;
impl Stdin {
    pub fn getch(var: Value) {
        let mut result = String::new();

        result += &var.to();
        result += ",";
        result += &var.from();

        add_to_compiled("\nPRINT CELL\n");
        add_to_compiled(&result);
        add_to_compiled("\nDONE\n");
    }
}

pub struct Stdout;
impl Stdout {
    /// This prints a Value according to its size.
    /// Basically, this prints each cell that the Value owns.
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

        add_to_compiled("\nPRINT CELL\n");
        add_to_compiled(&result);
        add_to_compiled("\nDONE\n");
    }
}

/// This object represents a value stored on the tape.
/// Objects can be stored indirectly or directly.
/// In brainfuck compatibility mode, dereferencing
/// or referencing an object is illegal.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Value {
    pub offset: u32,
    pub reference_depth: u32,
    pub number_cells: u32,
}

/// This is for debugging.
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
    pub fn new(size: u32) -> Result<Self, Error> {
        let result = Self {
            offset: *STACK_PTR.lock().unwrap(),
            reference_depth: 0,
            number_cells: size,
        };

        result.zero();

        increment_stack(size)?;
        Ok(result)
    }

    pub fn alloc(size: u32) -> Result<Self, Error> {
        let mut result = Self::new(1)?;
        result.number_cells = size;

        add_to_compiled(format!("\nALLOCATING {} CELLS\n", size));
        add_to_compiled(result.to());
        add_to_compiled("+".repeat(size as usize));
        add_to_compiled("?*");
        add_to_compiled("+>".repeat(size as usize));
        add_to_compiled("&");
        add_to_compiled(result.from());
        add_to_compiled("\nDONE\n");

        if Program::brainfuck_enabled() {
            Err(Error::CannotUsePointersInBrainFuckMode)
        } else {
            Ok(result)
        }
    }

    pub fn variable_alloc(size: Self) -> Result<Self, Error> {
        let result = Self::new(1)?;

        result.assign(size)?;

        add_to_compiled("\nALLOCATING CELLS\n");
        add_to_compiled(&result.to());
        add_to_compiled("?");
        add_to_compiled(&result.from());
        add_to_compiled("\nDONE\n");

        if Program::brainfuck_enabled() {
            Err(Error::CannotUsePointersInBrainFuckMode)
        } else {
            Ok(result)
        }
    }

    pub fn copy(&self) -> Result<Self, Error> {
        let val = Self::new(self.number_cells)?;
        val.assign(*self)?;
        Ok(val)
    }

    pub fn is_ref(&self) -> bool {
        self.reference_depth > 0
    }

    pub fn zero(&self) {
        add_to_compiled(self.to());
        for _ in 0..self.number_cells {
            add_to_compiled("[-]>");
        }
        for _ in 0..self.number_cells {
            add_to_compiled("<");
        }
        add_to_compiled(self.from());
    }

    pub fn free(&self) {
        add_to_compiled(&format!(
            "\nFREEING CELLS {}~{}\n",
            self.offset,
            self.offset + self.size()
        ));
        add_to_compiled(&self.to());

        for _ in 0..self.size() {
            add_to_compiled("[-]>");
        }

        for _ in 0..self.size() {
            add_to_compiled("<");
        }

        add_to_compiled(self.from());

        add_to_compiled("\nDONE\n");
    }

    pub fn set(&self, val: impl Into<usize>) {
        add_to_compiled(&self.to());
        add_to_compiled("[-]");
        add_to_compiled("+".repeat(val.into()));
        add_to_compiled(self.from());
    }

    pub fn assign(&self, val: Self) -> Result<(), Error> {
        if val == *self {
            return Ok(());
        }

        if Program::brainfuck_enabled() && val.size() > self.size() {
            return Err(Error::CannotAssignLargerValueToSmallerValueInBrainFuckMode);
        } else if Program::size_warn_enabled() && val.size() > self.size() {
            eprintln!("Warning: assigning larger value to smaller value");
        }

        TEMP0.zero();

        for cell in 0..val.size() {
            let cell_to = val.to() + &">".repeat(cell as usize);
            let cell_from = "<".repeat(cell as usize) + &val.from();
            let this_to = self.to() + &">".repeat(cell as usize);
            let this_from = "<".repeat(cell as usize) + &self.from();
            add_to_compiled(this_to.clone() + "[-]" + &this_from);
            add_to_compiled(
                cell_to.clone()
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
                    + &cell_from,
            );
            add_to_compiled(
                TEMP0.to()
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
                    + &TEMP0.from(),
            );
        }
        Ok(())
    }

    pub fn plus_eq(&self, val: Self) {
        TEMP0.zero();

        for cell in 0..val.size() {
            let cell_to = val.to() + &">".repeat(cell as usize);
            let cell_from = "<".repeat(cell as usize) + &val.from();
            let this_to = self.to() + &">".repeat(cell as usize);
            let this_from = "<".repeat(cell as usize) + &self.from();
            add_to_compiled(
                cell_to.clone()
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
                    + &cell_from,
            );
            add_to_compiled(
                TEMP0.to()
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
                    + &TEMP0.from(),
            );
        }
    }

    pub fn minus_eq(&self, val: Self) {
        TEMP0.zero();

        for cell in 0..val.size() {
            let cell_to = val.to() + &">".repeat(cell as usize);
            let cell_from = "<".repeat(cell as usize) + &val.from();
            let this_to = self.to() + &">".repeat(cell as usize);
            let this_from = "<".repeat(cell as usize) + &self.from();
            add_to_compiled(
                cell_to.clone()
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
                    + &cell_from,
            );
            add_to_compiled(
                TEMP0.to()
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
                    + &TEMP0.from(),
            );
        }
    }

    pub fn byte_int(value: u8) -> Result<Self, Error> {
        let result = Self::new(1)?;
        add_to_compiled(result.to());
        add_to_compiled("+".repeat(value as usize));
        add_to_compiled(result.from());
        Ok(result)
    }

    pub fn unsigned_short(value: u16) -> Result<Self, Error> {
        let result = Self::new(1)?;
        add_to_compiled(result.to());
        add_to_compiled("+".repeat(value as usize));
        add_to_compiled(result.from());

        if Program::brainfuck_enabled() {
            Err(Error::CannotUseUnsignedShortsInBrainFuckMode)
        } else {
            Ok(result)
        }
    }

    pub fn character(value: char) -> Result<Self, Error> {
        let result = Self::new(1)?;
        add_to_compiled(result.to());
        add_to_compiled("+".repeat(value as usize));
        add_to_compiled(result.from());
        Ok(result)
    }

    pub fn string(value: impl ToString) -> Result<Self, Error> {
        let result = Self::new((value.to_string().len()) as u32)?;

        add_to_compiled(result.to());
        for ch in value.to_string().chars() {
            add_to_compiled("+".repeat(ch as usize) + ">");
        }

        for _ in value.to_string().chars() {
            add_to_compiled("<");
        }
        add_to_compiled(result.from());

        Ok(result)
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

    pub fn deref(&self) -> Result<Self, Error> {
        let mut result = Self::new(1)?;

        result.reference_depth = self.reference_depth + 1;
        result.offset = self.offset;

        if Program::brainfuck_enabled() {
            Err(Error::CannotUsePointersInBrainFuckMode)
        } else {
            Ok(result)
        }
    }

    pub fn refer(&self) -> Result<Self, Error> {
        let result = Self::new(1)?;

        add_to_compiled(result.to());
        add_to_compiled("+".repeat(self.offset as usize));
        add_to_compiled(result.from());

        if Program::brainfuck_enabled() {
            Err(Error::CannotUsePointersInBrainFuckMode)
        } else {
            Ok(result)
        }
    }
}
