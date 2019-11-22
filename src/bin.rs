extern crate smpl_typchk;
use smpl_typchk::{compile, compile::*, init, Error, Stdout, Value};

fn main() -> Result<(), Error> {
    init();

    defun("print", &["a"], || {
        Stdout::print(get("a"));
        Ok(())
    });

    defun("free", &["ptr", "size"], || {
        get("ptr").deref().free();
        Ok(())
    });

    defun("alloc", &["size"], || {
        define("ptr", Value::variable_alloc(get("size")))?;
        set_return(get("ptr"));
        Ok(())
    });

    defun("start", &[], || {
        println!("START BEGIN");

        define("a", string("Hello, cruel, cruel world!"))?;
        call("print", &[get("a")])?;

        println!("START END");
        Ok(())
    });

    call("start", &[])?;
    println!("{}", compile());

    Ok(())
}
