use crate::{
    add_to_compiled, compile, init, set_stack, Control, Env, ProgramParser, Stdout, Stdin, Value,
    HEAP_SIZE, RETURN, STACK_PTR, STACK_SIZE,
};
use comment::rust::strip;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

lazy_static! {
    /// This is used to manage variable definitions within scopes.
    /// When a function is called, a scope is pushed on the scope stack,
    /// and all new definitions are contained in the new scope.
    /// When the function call finishes, the scope is popped and freed.
    pub static ref SCOPE_STACK: Mutex<Vec<Env>> = Mutex::new(vec![Env::new()]);

    /// If the user enables brainfuck compatibility mode, this flag is set
    static ref ENABLE_BRAINFUCK: Mutex<bool> = Mutex::new(false);
    /// If the user enables size warnings, this flag is set
    static ref ENABLE_SIZE_WARN: Mutex<bool> = Mutex::new(false);

    /// This hashmap contains all the user defined functions for the program
    static ref FN_DEFS: Mutex<HashMap<String, UserFn>> = Mutex::new(HashMap::new());
    /// This hashmap contains all the compiler defined functions for the program
    static ref FOREIGN_FN_DEFS: Mutex<HashMap<String, ForeignFn>> = Mutex::new(HashMap::new());
}

/// Generate a random string, used for naming temporary variables
fn rand_str() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}

/// This object manages compiling the program, and setting the enabled flags.
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
    /// Create new Program object
    pub fn new(flags: Vec<Flag>, funs: Vec<UserFn>) -> Self {
        // Set the flags for the compiler
        for flag in &flags {
            match flag {
                Flag::EnableBrainFuck => *ENABLE_BRAINFUCK.lock().unwrap() = true,
                Flag::EnableSizeWarn => *ENABLE_SIZE_WARN.lock().unwrap() = true,
            }
        }
        // Return self
        Self(flags, funs)
    }

    /// Instantiate compiler functions
    fn prelude() {
        init();

        // Add lhs and rhs
        deforfun("add", &["a", "b"], || {
            get("a")?.plus_eq(get("b")?);
            set_return(get("a")?)?;
            Ok(())
        });

        // Subtract rhs from lhs
        deforfun("sub", &["a", "b"], || {
            get("a")?.minus_eq(get("b")?);
            set_return(get("a")?)?;
            Ok(())
        });

        // Print function
        deforfun("print", &["a"], || {
            Stdout::print(get("a")?);
            Ok(())
        });

        // Println function
        deforfun("println", &["a"], || {
            Stdout::print(get("a")?);
            Stdout::print(Eval::Literal(Literal::character('\n')).lower()?);
            Ok(())
        });
        
        // Print function
        deforfun("cprint", &["ptr"], || {
            Stdout::print_cstr(get("ptr")?)?;
            Ok(())
        });

        // Println function
        deforfun("cprintln", &["ptr"], || {
            Stdout::print_cstr(get("ptr")?)?;
            Stdout::print(Eval::Literal(Literal::character('\n')).lower()?);
            Ok(())
        });
        
        // Allocate `size` number of bytes
        deforfun("alloc", &["size"], || {
            define("ptr", Eval::Value(Value::variable_alloc(get("size")?)?))?;
            set_return(get("ptr")?)?;
            Ok(())
        });

        // Free a byte at ptr
        deforfun("free_byte", &["ptr"], || {
            get("ptr")?.deref()?.free();
            Ok(())
        });

        // Free a byte at ptr
        deforfun("getch", &[], || {
            define("ch", Eval::Value(Value::character('\0')?))?;
            Stdin::getch(get("ch")?);
            set_return(get("ch")?)?;
            Ok(())
        });
    }

    /// Is brainfuck compatibility mode enabled?
    pub fn brainfuck_enabled() -> bool {
        *ENABLE_BRAINFUCK.lock().unwrap()
    }

    /// Are size warnings enabled?
    pub fn size_warn_enabled() -> bool {
        *ENABLE_SIZE_WARN.lock().unwrap()
    }

    /// Get the stack size
    pub fn stack_size() -> u32 {
        *STACK_SIZE.lock().unwrap()
    }

    /// Get the heap size
    pub fn heap_size() -> u32 {
        *HEAP_SIZE.lock().unwrap()
    }

    /// Get the tape size
    pub fn tape_size() -> u32 {
        Self::stack_size() + Self::heap_size()
    }

    /// Compile the code
    pub fn compile(self) -> Result<String, Error> {
        // Add the compiler functions
        Self::prelude();

        // Get function definitions
        let Program(_, funs) = self;

        // Compile
        for fun in funs {
            fun.compile();
        }

        call("start", &vec![])?;

        // Return compiled code
        Ok(compile())
    }
}

/// The possible flags to be returned by the parser
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Flag {
    EnableBrainFuck,
    EnableSizeWarn,
}

/// The possible compiler errors
#[derive(Clone, Debug)]
pub enum Error {
    StackOverflow,
    MustReturnSingleByte,
    CannotReferenceAReference,
    CannotUsePointersInBrainFuckMode,
    CannotUseUnsignedShortsInBrainFuckMode,
    CannotAssignLargerValueToSmallerValueInBrainFuckMode,
    FunctionNotDefined(String),
    VariableNotDefined(String, Env),
}

/// This trait describes objects that are lowered
/// into values rather than expressions, such as function calls
/// literals, variables, etc..
pub trait Lower {
    fn lower(&self) -> Result<Value, Error>;
}

/// A value can be `lowered` into a value
impl Lower for Value {
    fn lower(&self) -> Result<Value, Error> {
        Ok(*self)
    }
}

/// This trait is for objects that compile into expressions
/// rather than values. This is applicable for definitions,
/// assignments, return expressions, while loops, etc..
pub trait Compile {
    fn compile(&self) -> Result<(), Error>;
}

/// This function creates a new scope on the scope stack.
/// THIS IS ONLY TO BE USED BY FUNCTION DEFINITIONS
fn push_scope(env: Env) {
    SCOPE_STACK.lock().unwrap().push(env);
}

/// This function destroys a scope on the scope stack.
/// THIS IS ONLY TO BE USED BY FUNCTION DEFINITIONS
fn pop_scope() -> Env {
    SCOPE_STACK.lock().unwrap().pop().unwrap()
}

/// This sets the RETURN value object to a specific value
pub fn set_return(val: Value) -> Result<(), Error> {
    if val.size() > 1 {
        Err(Error::MustReturnSingleByte)
    } else {
        RETURN.assign(val)?;
        Ok(())
    }
}

/// This retreives the last value returned by a function
pub fn get_return() -> Result<Value, Error> {
    // let val = Eval::Value(*RETURN);
    // let name = format!("%TEMP_RETURN{}%", *STACK_PTR.lock().unwrap());
    // define(&name, val)?;
    // get(name)
    Ok(*RETURN)
}

/// This represents a value that can be evaluated. Variables,
/// literals, function calls, value dereferences, variable references,
/// and values (this is a cheat for letting the compiler easily use
/// values in place of long Evals) can all be evaluated into a Value
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Eval {
    Load(Load),
    Literal(Literal),
    Call(Call),
    Deref(Deref),
    Refer(Refer),
    Value(Value),
}

/// An Eval expression is evaluated by the expression it contains
impl Lower for Eval {
    fn lower(&self) -> Result<Value, Error> {
        // Lower the contained expression
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

/// This represents a statement as opposed to a value.
/// A value can also be a statement, though.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Expr {
    If(If),
    While(While),
    Eval(Eval),
    Define(Define),
    Assign(Assign),
    Return(Return),
}

/// An Expression is evaluated by the expression it contains
impl Compile for Expr {
    fn compile(&self) -> Result<(), Error> {
        match self {
            Self::If(l) => l.compile()?,
            Self::Eval(e) => {
                e.lower()?; // Dont return value from lower
            }
            Self::Define(def) => def.compile()?,
            Self::Assign(a) => a.compile()?,
            Self::While(w) => w.compile()?,
            Self::Return(r) => r.compile()?,
        }
        Ok(())
    }
}

/// This sets the RETURN register to an Eval
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
    UnsignedShort(u16),
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

    pub fn unsigned_short(ui: u16) -> Self {
        Self::UnsignedShort(ui)
    }
}

impl Lower for Literal {
    fn lower(&self) -> Result<Value, Error> {
        let name;
        match self {
            Self::String(s) => {
                name = format!("%TEMP_STR_LITERAL_{}%", rand_str());
                define_no_cp(&name, Eval::Value(Value::string(s)?))?;
            }
            Self::Character(ch) => {
                name = format!("%TEMP_CHAR_LITERAL_{}%", rand_str());
                define_no_cp(&name, Eval::Value(Value::character(*ch)?))?;
            }
            Self::ByteInt(byte) => {
                name = format!("%TEMP_BYTE_LITERAL_{}%", rand_str());
                define_no_cp(&name, Eval::Value(Value::byte_int(*byte)?))?;
            }
            Self::UnsignedShort(ui) => {
                name = format!("%TEMP_U16_LITERAL_{}%", rand_str());
                define_no_cp(&name, Eval::Value(Value::unsigned_short(*ui)?))?;
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
            env.define(p.to_string(), args[i].lower()?)?;
        }

        push_scope(env);

        for instruction in &self.body {
            instruction.compile()?;
        }

        pop_scope().free();
        set_stack(stack_frame)?;

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
    pub fn new(condition: Eval, then: Vec<Expr>, otherwise: Vec<Expr>) -> Self {
        Self(condition, then, otherwise)
    }
}

impl Compile for If {
    fn compile(&self) -> Result<(), Error> {
        let If(condition, then, otherwise) = self;
        Control::if_begin(condition.lower()?)?;
        for exp in then {
            exp.compile()?;
        }
        Control::else_begin()?;
        for exp in otherwise {
            exp.compile()?;
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
            env.define(p.to_string(), args[i].lower()?)?;
        }

        push_scope(env);

        (self.body)()?;

        pop_scope().free();
        set_stack(stack_frame)?;

        Ok(())
    }
}
