use crate::{Control, Env, ProgramParser, Value, RETURN, STACK_PTR, add_to_compiled};
use comment::rust::strip;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};


lazy_static! {
    pub static ref SCOPE_STACK: Mutex<Vec<Env>> = Mutex::new(vec![Env::new()]);
    static ref ENABLE_BRAINFUCK: Mutex<bool> = Mutex::new(false);
    static ref ENABLE_SIZE_WARN: Mutex<bool> = Mutex::new(false);
    static ref FN_DEFS: Mutex<HashMap<String, UserFn>> = Mutex::new(HashMap::new());
    static ref FOREIGN_FN_DEFS: Mutex<HashMap<String, ForeignFn>> = Mutex::new(HashMap::new());
}

fn rand_str() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect()
} 


#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Program(Vec<Flag>, Vec<UserFn>);


impl<T: ToString> From<T> for Program {
    fn from(t: T) -> Self {
        match ProgramParser::new().parse(&strip(t.to_string()).unwrap()) {
            Ok(val) => val,
            Err(e) => panic!("{:#?}", e),
        }
    }
}

impl Program {
    pub fn new(flags: Vec<Flag>, funs: Vec<UserFn>) -> Self {
        for flag in &flags {
            match flag {
                Flag::EnableBrainFuck => *ENABLE_BRAINFUCK.lock().unwrap() = true,
                Flag::EnableSizeWarn => *ENABLE_SIZE_WARN.lock().unwrap() = true
            }
        }
        Self(flags, funs)
    }

    pub fn brainfuck_enabled() -> bool {
        *ENABLE_BRAINFUCK.lock().unwrap()
    }

    pub fn size_warn_enabled() -> bool {
        *ENABLE_SIZE_WARN.lock().unwrap()
    }

    pub fn compile(self) -> Result<(), Error> {
        let Program(_flags, funs) = self;
        for fun in funs {
            fun.compile();
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Flag {
    EnableBrainFuck,
    EnableSizeWarn,
}

#[derive(Clone, Debug)]
pub enum Error {
    CannotReferenceAReference,
    CannotUsePointersInBrainFuckMode,
    CannotUse4ByteUnsignedIntsInBrainFuckMode,
    CannotAssignLargerValueToSmallerValueInBrainFuckMode,
    FunctionNotDefined(String),
    VariableNotDefined(String, Env),
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

fn push_scope(env: Env) {
    SCOPE_STACK.lock().unwrap().push(env);
}

fn pop_scope() -> Env {
    SCOPE_STACK.lock().unwrap().pop().unwrap()
}

pub fn set_return(val: Value) -> Result<(), Error> {
    RETURN.zero();
    RETURN.assign(val)?;
    Ok(())
}

pub fn get_return() -> Result<Value, Error> {
    // let val = Eval::Value(*RETURN);
    // let name = format!("%TEMP_RETURN{}%", *STACK_PTR.lock().unwrap());
    // define(&name, val)?;
    // get(name)
    Ok(*RETURN)
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Eval {
    Load(Load),
    Literal(Literal),
    Call(Call),
    Deref(Deref),
    Refer(Refer),
    Value(Value),
}

impl Lower for Eval {
    fn lower(&self) -> Result<Value, Error> {
        match self {
            Self::Load(l) => l.lower(),
            Self::Literal(l) => l.lower(),
            Self::Deref(r) => r.lower(),
            Self::Call(c) => c.lower(),
            Self::Refer(v) => v.lower(),
            Self::Value(v) => v.lower(),
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Expr {
    If(If),
    While(While),
    Eval(Eval),
    Define(Define),
    Assign(Assign),
    Return(Return),
}

impl Compile for Expr {
    fn compile(&self) -> Result<(), Error> {
        match self {
            Self::If(l) => l.compile()?,
            Self::Eval(e) => {
                e.lower()?;
            }
            Self::Define(def) => def.compile()?,
            Self::Assign(a) => a.compile()?,
            Self::While(w) => w.compile()?,
            Self::Return(r) => r.compile()?,
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Return(Eval);

impl Return {
    pub fn new(val: Eval) -> Self {
        Return(val)
    }
}

impl Compile for Return {
    fn compile(&self) -> Result<(), Error> {
        let Return(val) = self;
        set_return(val.lower()?)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Call(String, Vec<Eval>);

impl Call {
    pub fn new(name: impl ToString, args: Vec<Eval>) -> Self {
        Self(name.to_string(), args)
    }
}

impl Lower for Call {
    fn lower(&self) -> Result<Value, Error> {
        let Call(name, args) = self;
        add_to_compiled(format!("CALLING {}", name));
        call(name, args)?;
        add_to_compiled("DONE");
        get_return()
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Deref(Arc<Eval>);

impl Deref {
    pub fn new(refer: Eval) -> Self {
        Self(Arc::new(refer))
    }
}

impl Lower for Deref {
    fn lower(&self) -> Result<Value, Error> {
        let Deref(refer) = self;
        Ok(refer.lower()?.deref()?)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Refer(Arc<Eval>);

impl Refer {
    pub fn new(var: Eval) -> Self {
        Self(Arc::new(var))
    }
}

impl Lower for Refer {
    fn lower(&self) -> Result<Value, Error> {
        let Refer(var) = self;
        Ok(var.lower()?.refer()?)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
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

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Literal {
    String(String),
    Character(char),
    ByteInt(u8),
    Unsigned4ByteInt(u32),
}

impl Literal {
    pub fn string(s: impl ToString) -> Self {
        Self::String(s.to_string())
    }

    pub fn character(ch: char) -> Self {
        Self::Character(ch)
    }

    pub fn byte_int(b: u8) -> Self {
        Self::ByteInt(b)
    }

    pub fn unsigned_4byte_int(ui: u32) -> Self {
        Self::Unsigned4ByteInt(ui)
    }
}

impl Lower for Literal {
    fn lower(&self) -> Result<Value, Error> {
        let name;
        match self {
            Self::String(s) => {
                name = format!("%TEMP_STR_LITERAL_{}%", rand_str());
                define_no_cp(&name, Eval::Value(Value::string(s)))?;
            }
            Self::Character(ch) => {
                name = format!("%TEMP_CHAR_LITERAL_{}%", rand_str());
                define_no_cp(&name, Eval::Value(Value::character(*ch)))?;
            }
            Self::ByteInt(byte) => {
                name = format!("%TEMP_BYTE_LITERAL_{}%", rand_str());
                define_no_cp(&name, Eval::Value(Value::byte_int(*byte)))?;
            }
            Self::Unsigned4ByteInt(ui) => {
                name = format!("%TEMP_U32_LITERAL_{}%", rand_str());
                define_no_cp(&name, Eval::Value(Value::unsigned_4byte_int(*ui)?))?;
            }
        }
        get(name)
        // match self {
        //     Self::String(s) => {
        //         Value::string(s).copy()
        //     }
        //     Self::Character(ch) => {
        //         Value::character(*ch).copy()
        //     }
        //     Self::ByteInt(byte) => {
        //         Value::byte_int(*byte).copy()
        //     }
        //     Self::Unsigned4ByteInt(ui) => {
        //         Value::unsigned_4byte_int(*ui)?.copy()
        //     }
        // }
    }
}

pub fn deforfun(name: impl ToString, args: &[&'static str], fun: fn() -> Result<(), Error>) {
    FOREIGN_FN_DEFS
        .lock()
        .unwrap()
        .insert(name.to_string(), ForeignFn::new(args.to_vec(), fun));
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct UserFn {
    name: String,
    parameters: Vec<String>,
    body: Vec<Expr>,
}

impl UserFn {
    pub fn new(name: impl ToString, parameters: Vec<String>, body: Vec<Expr>) -> Self {
        Self {
            name: name.to_string(),
            parameters: parameters.iter().map(ToString::to_string).collect(),
            body,
        }
    }

    pub fn compile(self) {
        FN_DEFS.lock().unwrap().insert(self.name.clone(), self);
    }

    pub fn call(&self, args: &Vec<Eval>) -> Result<(), Error> {
        let stack_frame;
        stack_frame = *STACK_PTR.lock().unwrap();

        let mut env = Env::new();

        for (i, p) in self.parameters.iter().enumerate() {
            env.define(p.to_string(), args[i].lower()?)?; //.copy());
        }

        push_scope(env);

        for instruction in &self.body {
            instruction.compile()?;
        }

        pop_scope().free();
        *STACK_PTR.lock().unwrap() = stack_frame;// + RETURN.size();

        Ok(())
    }
}

pub fn call(name: impl ToString, args: &Vec<Eval>) -> Result<(), Error> {
    let table = FN_DEFS.lock().unwrap();
    if let Some(f_ref) = table.get(&name.to_string()) {
        let fun = f_ref.clone();
        drop(table);
        fun.call(args)?;
        return Ok(());
    } else {
        drop(table)
    }

    let table = FOREIGN_FN_DEFS.lock().unwrap();
    if let Some(f_ref) = table.get(&name.to_string()) {
        let fun = f_ref.clone();
        drop(table);
        fun.call(args)?;
        return Ok(());
    }

    Err(Error::FunctionNotDefined(name.to_string()))
}

pub fn define(name: impl ToString, val: Eval) -> Result<(), Error> {
    let temp_name = format!("%TEMP_DEFINE_{}%", rand_str());
    Define::new(&temp_name, val).compile()?;
    Define::new(name, Eval::Load(Load::new(temp_name))).compile()?;
    Ok(())
}

pub fn define_no_cp(final_name: impl ToString, value: Eval) -> Result<(), Error> {
    // Define::new("%TEMP_DEFINE%", val).compile()?;
    // Define::new(name, Eval::Load(Load::new("%TEMP_DEFINE%"))).compile()?;
    // Ok(())
    let name = format!("%TEMP_DEFINE_{}%", rand_str());

    let val = value.lower()?;
    let mut scope_stack = SCOPE_STACK.lock().unwrap();
    let scope = scope_stack.last_mut().unwrap();
    scope.define_no_cp(&name, val);
    drop(scope_stack);

    let val = get(name)?;
    let mut scope_stack = SCOPE_STACK.lock().unwrap();
    let scope = scope_stack.last_mut().unwrap();
    scope.define_no_cp(final_name, val);
    drop(scope_stack);

    Ok(())
}

pub fn get(name: impl ToString) -> Result<Value, Error> {
    let mut scope_stack = SCOPE_STACK.lock().unwrap();
    let scope = scope_stack.last_mut().unwrap();
    scope.get(name.to_string())
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Define(String, Eval);

impl Define {
    pub fn new(var: impl ToString, value: Eval) -> Self {
        Self(var.to_string(), value)
    }
}

impl Compile for Define {
    fn compile(&self) -> Result<(), Error> {
        let Define(name, value) = self;
        add_to_compiled(format!("DEFINING {}", name));

        let val = value.lower()?;
        let mut scope_stack = SCOPE_STACK.lock().unwrap();
        let scope = scope_stack.last_mut().unwrap();
        scope.define(&name, val)?;
        drop(scope_stack);

        add_to_compiled("DONE");

        Ok(())
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Assign(Eval, Eval);

impl Assign {
    pub fn new(lhs: Eval, rhs: Eval) -> Self {
        Self(lhs, rhs)
    }
}

impl Compile for Assign {
    fn compile(&self) -> Result<(), Error> {
        let Assign(lhs, rhs) = self;
        lhs.lower()?.assign(rhs.lower()?)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct If(Eval, Vec<Expr>, Vec<Expr>);

impl If {
    pub fn new(condition: Eval, then: Vec<Expr>, _otherwise: Vec<Expr>) -> Self {
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
        Control::if_end()?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct While(Eval, Vec<Expr>);

impl While {
    pub fn new(condition: Eval, then: Vec<Expr>) -> Self {
        Self(condition, then)
    }
}

impl Compile for While {
    fn compile(&self) -> Result<(), Error> {
        let While(condition, then) = self;
        Control::while_begin(condition.lower()?);
        {
            for exp in then {
                exp.compile()?;
            }
        }
        Control::while_end();
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

    pub fn call(&self, args: &Vec<Eval>) -> Result<(), Error> {
        let stack_frame;
        stack_frame = *STACK_PTR.lock().unwrap();

        let mut env = Env::new();

        for (i, p) in self.parameters.iter().enumerate() {
            env.define(p.to_string(), args[i].lower()?)?; //.copy());
        }

        push_scope(env);

        (self.body)()?;

        // pop_scope();
        pop_scope().free();
        *STACK_PTR.lock().unwrap() = stack_frame; // + RETURN.size();

        Ok(())
    }
}
