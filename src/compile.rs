use crate::{Control, Env, Value, RETURN, STACK_PTR};
use std::{collections::HashMap, sync::Mutex};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Error {}
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

pub trait Compile {
    fn compile(&self) -> Result<(), Error>;
}

lazy_static! {
    static ref SCOPE_STACK: Mutex<Vec<Env>> = Mutex::new(vec![Env::new()]);
    static ref FUNCTION_DEFS: Mutex<HashMap<String, Func>> = Mutex::new(HashMap::new());
}

fn push_scope() {
    SCOPE_STACK.lock().unwrap().push(Env::new());
}

fn pop_scope() -> Env {
    SCOPE_STACK.lock().unwrap().pop().unwrap()
}

pub fn set_return(val: Value) {
    RETURN.assign(val);
}

pub fn get_return() -> Value {
    RETURN.clone()
}

pub fn string(s: impl ToString) -> Value {
    let name;
    unsafe { name = format!("%TEMP_STR_LITERAL{}%", STACK_PTR) }
    define(&name, Value::string(s)).unwrap();
    get(name)
}

pub fn character(ch: char) -> Value {
    let name;
    unsafe { name = format!("%TEMP_CHAR_LITERAL{}%", STACK_PTR) }
    define(&name, Value::character(ch)).unwrap();
    get(name)
}

pub fn defun(name: impl ToString, args: &[&'static str], fun: fn() -> Result<(), Error>) {
    FUNCTION_DEFS
        .lock()
        .unwrap()
        .insert(name.to_string(), Func::new(args.to_vec(), fun));
}

pub fn call(name: impl ToString, args: &[Value]) -> Result<(), Error> {
    println!("CALLING {}", name.to_string());
    let table = FUNCTION_DEFS.lock().unwrap();
    let fun = table.get(&name.to_string()).unwrap().clone();
    drop(table);
    fun.call(args.to_vec())?;
    println!("DONE");
    Ok(())
}

pub fn define(name: impl ToString, val: Value) -> Result<(), Error> {
    println!("DEFINING {}", name.to_string());
    Define::new(name, val).compile()?;
    println!("DONE");
    Ok(())
}

pub fn get(name: impl ToString) -> Value {
    return SCOPE_STACK
        .lock()
        .unwrap()
        .last_mut()
        .unwrap()
        .get(name.to_string());
}

pub enum Expr {
    Define(Define),
    Assign(Assign),
    If(If),
}

pub struct Define(String, Value);

impl Define {
    pub fn new(var: impl ToString, value: Value) -> Self {
        Self(var.to_string(), value)
    }
}

impl Compile for Define {
    fn compile(&self) -> Result<(), Error> {
        let Define(name, value) = self;
        SCOPE_STACK
            .lock()
            .unwrap()
            .last_mut()
            .unwrap()
            .define(name, *value);
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Assign(Value, Value);

impl Compile for Assign {
    fn compile(&self) -> Result<(), Error> {
        let Assign(lhs, rhs) = self;
        lhs.assign(*rhs);
        Ok(())
    }
}

pub struct If(Value, Box<dyn Fn() -> ()>, Box<dyn Fn() -> ()>);

impl If {
    pub fn new(
        condition: Value,
        then: Box<dyn Fn() -> ()>,
        _otherwise: Box<dyn Fn() -> ()>,
    ) -> Self {
        Self(condition, then, _otherwise)
    }
}

impl Compile for If {
    fn compile(&self) -> Result<(), Error> {
        let If(condition, then, _otherwise) = self;
        Control::if_begin(*condition);
        {
            then();
        }
        Control::if_end();
        Ok(())
    }
}

#[derive(Clone)]
pub struct Func {
    parameters: Vec<String>,
    body: fn() -> Result<(), Error>,
}

impl Func {
    pub fn new(parameters: Vec<impl ToString>, body: fn() -> Result<(), Error>) -> Self {
        Self {
            parameters: parameters.iter().map(ToString::to_string).collect(),
            body,
        }
    }

    pub fn call(&self, args: Vec<Value>) -> Result<(), Error> {
        let stack_frame;
        unsafe {
            stack_frame = STACK_PTR;
        }

        push_scope();

        for (i, p) in self.parameters.iter().enumerate() {
            define(p.to_string(), args[i].clone())?;
            // println!("OLD ARG ADDRESS to: {} from: {}", args[i].to(), args[i].from());
            // println!("NEW CLONED ARG ADDRESS to: {} from: {}", val.to(), val.from());
        }

        (self.body)()?;

        unsafe {
            pop_scope().free();
            STACK_PTR = stack_frame;
        }

        Ok(())
    }
}

// pub struct Call(Func, Vec<Value>);

// impl Call {
//     pub fn new(func: &Func, args: Vec<Value>) -> Self {
//         Self(func.clone(), args)
//     }
// }

// impl Compile for Call {
//     fn compile(&self) -> Result<(), Error> {
//         (self.0).call((self.1).clone())
//     }
// }
