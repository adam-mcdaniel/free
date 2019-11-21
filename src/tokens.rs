use crate::{Value, Control};


pub enum Error {}

pub trait Compile {
    fn compile(self) -> Result<(), Error>;
}



pub struct Assign<'a>(&'a Value, &'a Value);

impl<'a> Compile for Assign<'a> {
    fn compile(self) -> Result<(), Error> {
        let Assign(a, b) = self;
        a.assign(&b);
        Ok(())
    }
}

pub struct If<'a>(&'a Value, Box<dyn Fn() -> ()>, Box<dyn Fn() -> ()>);

impl<'a> If<'a> {
    pub fn new(condition: &'a Value, then: Box<dyn Fn() -> ()>, _otherwise: Box<dyn Fn() -> ()>) -> Self {
        Self(condition, then, _otherwise)
    }
}

impl<'a> Compile for If<'a> {
    fn compile(self) -> Result<(), Error> {
        let If(condition, then, _otherwise) = self;
        Control::if_begin(&condition);
        {
            then();
        }
        Control::if_end();
        Ok(())
    }
}