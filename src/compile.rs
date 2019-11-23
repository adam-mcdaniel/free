use crate::{Control, Env, Value, RETURN, STACK_PTR};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Error {
    CannotReferenceAReference,
}

pub trait Lower {
    fn lower(&self) -> Result<Value, Error>;
}

impl Lower for Value {
    fn lower(&self) -> Result<Value, Error> {
        Ok(*self)
    }
}

pub trait Compile {
    fn compile(&self) -> Result<(), Error>;
}

lazy_static! {
    pub static ref SCOPE_STACK: Mutex<Vec<Env>> = Mutex::new(vec![Env::new()]);
    static ref FN_DEFS: Mutex<HashMap<String, UserFn>> = Mutex::new(HashMap::new());
    static ref FOREIGN_FN_DEFS: Mutex<HashMap<String, ForeignFn>> = Mutex::new(HashMap::new());
}

fn push_scope(env: Env) {
    SCOPE_STACK.lock().unwrap().push(env);
}

fn pop_scope() -> Env {
    SCOPE_STACK.lock().unwrap().pop().unwrap()
}

pub fn set_return(val: Value) {
    RETURN.assign(val);
}

pub fn get_return() -> Result<Value, Error> {
    define("%TEMP_RETURN%", Box::new(*RETURN))?;
    get("%TEMP_RETURN%")
}

pub enum Eval {
    Load(Load),
    Literal(Literal),
    Call(Call),
    Deref(Deref),
}

impl Lower for Eval {
    fn lower(&self) -> Result<Value, Error> {
        match self {
            Self::Load(l) => l.lower(),
            Self::Literal(l) => l.lower(),
            Self::Deref(r) => r.lower(),
            Self::Call(c) => c.lower(),
        }
    }
}

pub enum Expr {
    If(If),
    Eval(Eval),
    Define(Define),
    Assign(Assign),
    Return(Return),
}

impl Compile for Expr {
    fn compile(&self) -> Result<(), Error> {
        match self {
            Self::If(l) => l.compile()?,
            Self::Eval(e) => { e.lower()?; },
            Self::Define(def) => def.compile()?,
            Self::Assign(a) => a.compile()?,
            Self::Return(r) => r.compile()?,
        }
        Ok(())
    }
}

pub struct Return(Box<dyn Lower>);

impl Compile for Return {
    fn compile(&self) -> Result<(), Error> {
        let Return(val) = self;
        set_return(val.lower()?);
        Ok(())
    }
}

pub struct Call(String, Vec<Box<dyn Lower>>);

impl Call {
    pub fn new(name: impl ToString, args: Vec<Box<dyn Lower>>) -> Self {
        Self(name.to_string(), args)
    }
}

impl Lower for Call {
    fn lower(&self) -> Result<Value, Error> {
        let Call(name, args) = self;
        println!("CALLING {}", name);
        call(name, args)?;
        println!("DONE");
        get_return()
    }
}

pub struct Deref(Box<dyn Lower>);

impl Deref {
    pub fn new(refer: Box<dyn Lower>) -> Self {
        Self(refer)
    }
}

impl Lower for Deref {
    fn lower(&self) -> Result<Value, Error> {
        let Deref(refer) = self;
        Ok(refer.lower()?.deref())
    }
}

pub struct Load(String);

impl Load {
    pub fn new(s: impl ToString) -> Self {
        Self(s.to_string())
    }
}

impl Lower for Load {
    fn lower(&self) -> Result<Value, Error> {
        let Load(name) = self;
        get(name)
    }
}

pub enum Literal {
    String(String),
    Character(char),
}

impl Literal {
    pub fn string(s: impl ToString) -> Self {
        Self::String(s.to_string())
    }

    pub fn character(ch: char) -> Self {
        Self::Character(ch)
    }
}

impl Lower for Literal {
    fn lower(&self) -> Result<Value, Error> {
        let name;
        match self {
            Self::String(s) => {
                unsafe { name = format!("%TEMP_STR_LITERAL{}%", STACK_PTR) }
                define(&name, Box::new(Value::string(s))).unwrap();
            }
            Self::Character(ch) => {
                unsafe { name = format!("%TEMP_CHAR_LITERAL{}%", STACK_PTR) }
                define(&name, Box::new(Value::character(*ch))).unwrap();
            }
        }
        get(name)
    }
}

pub fn deforfun(name: impl ToString, args: &[&'static str], fun: fn() -> Result<(), Error>) {
    FOREIGN_FN_DEFS
        .lock()
        .unwrap()
        .insert(name.to_string(), ForeignFn::new(args.to_vec(), fun));
}

pub struct UserFn {
    parameters: Vec<String>,
    body: Vec<Box<dyn Compile>>,
}

unsafe impl Send for UserFn {}

impl UserFn {
    pub fn new(parameters: Vec<String>, body: Vec<Box<dyn Compile>>) -> Self {
        Self {
            parameters: parameters.iter().map(ToString::to_string).collect(),
            body,
        }
    }

    pub fn define(name: impl ToString, args: Vec<String>, body: Vec<Box<dyn Compile>>) {
        FN_DEFS.lock().unwrap().insert(
            name.to_string(),
            Self::new(args.iter().map(ToString::to_string).collect(), body),
        );
    }

    pub fn call(&self, args: &Vec<Box<dyn Lower>>) -> Result<(), Error> {
        let stack_frame;
        unsafe {
            stack_frame = STACK_PTR;
        }

        let mut env = Env::new();

        for (i, p) in self.parameters.iter().enumerate() {
            env.define(p.to_string(), args[i].lower()?.clone());
        }

        push_scope(env);

        for instruction in &self.body {
            instruction.compile()?;
        }

        unsafe {
            pop_scope().free();
            STACK_PTR = stack_frame;
        }

        Ok(())
    }
}

pub fn call(name: impl ToString, args: &Vec<Box<dyn Lower>>) -> Result<(), Error> {
    let table = FN_DEFS.lock().unwrap();
    if let Some(f_ref) = table.get(&name.to_string()) {
        let fun = f_ref as *const UserFn;
        drop(table);
        unsafe {
            (*fun).call(args)?;
        }
        return Ok(());
    } else {
        drop(table)
    }

    let table = FOREIGN_FN_DEFS.lock().unwrap();
    if let Some(f_ref) = table.get(&name.to_string()) {
        let fun = f_ref as *const ForeignFn;
        drop(table);
        unsafe {
            (*fun).call(args)?;
        }
        return Ok(());
    } else {
        drop(table)
    }

    Ok(())
}

pub fn define(name: impl ToString, val: Box<dyn Lower>) -> Result<(), Error> {
    Define::new("%TEMP_DEFINE%", Box::new(val.lower()?)).compile()?;
    Define::new(name, Box::new(get("%TEMP_DEFINE%")?)).compile()?;
    Ok(())
}

pub fn get(name: impl ToString) -> Result<Value, Error> {
    let mut scope_stack = SCOPE_STACK.lock().unwrap();
    let scope = scope_stack.last_mut().unwrap();
    Ok(scope.get(name.to_string()))
}

pub struct Define(String, Box<dyn Lower>);

impl Define {
    pub fn new(var: impl ToString, value: Box<dyn Lower>) -> Self {
        Self(var.to_string(), value)
    }
}

impl Compile for Define {
    fn compile(&self) -> Result<(), Error> {
        let Define(name, value) = self;
        println!("DEFINING {}", name);

        let mut scope_stack = SCOPE_STACK.lock().unwrap();
        let scope = scope_stack.last_mut().unwrap() as *mut Env;
        drop(scope_stack);
        let val = value.lower()?;
        unsafe {
            (*scope).define(name, val);
        }
        println!("DONE");

        Ok(())
    }
}

pub struct Assign(Box<dyn Lower>, Box<dyn Lower>);

impl Compile for Assign {
    fn compile(&self) -> Result<(), Error> {
        let Assign(lhs, rhs) = self;
        lhs.lower()?.assign(rhs.lower()?);
        Ok(())
    }
}

pub struct If(Box<dyn Lower>, Vec<Expr>, Vec<Expr>);

impl If {
    pub fn new(condition: Box<dyn Lower>, then: Vec<Expr>, _otherwise: Vec<Expr>) -> Self {
        Self(condition, then, _otherwise)
    }
}

impl Compile for If {
    fn compile(&self) -> Result<(), Error> {
        let If(condition, then, _otherwise) = self;
        Control::if_begin(condition.lower()?);
        {
            for exp in then {
                exp.compile()?;
            }
        }
        Control::if_end();
        Ok(())
    }
}

/// This class is only used for foreign functions. Do not use for regular functions.
#[derive(Clone)]
pub struct ForeignFn {
    parameters: Vec<String>,
    body: fn() -> Result<(), Error>,
}

impl ForeignFn {
    pub fn new(parameters: Vec<impl ToString>, body: fn() -> Result<(), Error>) -> Self {
        Self {
            parameters: parameters.iter().map(ToString::to_string).collect(),
            body,
        }
    }

    pub fn define(name: impl ToString, args: Vec<impl ToString>, fun: fn() -> Result<(), Error>) {
        FOREIGN_FN_DEFS.lock().unwrap().insert(
            name.to_string(),
            Self::new(args.iter().map(ToString::to_string).collect(), fun),
        );
    }

    pub fn call(&self, args: &Vec<Box<dyn Lower>>) -> Result<(), Error> {
        let stack_frame;
        unsafe {
            stack_frame = STACK_PTR;
        }

        let mut env = Env::new();

        for (i, p) in self.parameters.iter().enumerate() {
            env.define(p.to_string(), args[i].lower()?.clone());
        }

        push_scope(env);

        (self.body)()?;

        // pop_scope();
        unsafe {
            pop_scope().free();
            STACK_PTR = stack_frame;
        }

        Ok(())
    }
}
