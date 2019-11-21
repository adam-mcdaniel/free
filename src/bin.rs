extern crate smpl_typchk;
use smpl_typchk::{compile, init, Control, Stdout, Value, If};



fn main() {
    init();

    let a = Value::string("Hello, world!\n");
    let b = Value::character(0 as char);
    If::new(&b, Box::new(|| {
        Stdout::print(&a)
    }), Box::new(|| {
        
    }));
    Stdout::print(&a);

    println!("{}", compile());
}
