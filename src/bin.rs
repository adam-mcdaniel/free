extern crate smpl_typchk;
use smpl_typchk::{compile, compile::*, init, Error, Program, Stdout, Value};

fn main() -> Result<(), Error> {
    init();

    deforfun("prn", &["a"], || {
        Stdout::print(get("a")?);
        Ok(())
    });

    deforfun("print_cstr", &["a"], || {
        Stdout::print_cstr(get("a")?);
        Ok(())
    });

    deforfun("add", &["a", "b"], || {
        define("c", Eval::Value(get("a")?))?;
        define("d", Eval::Value(get("b")?))?;
        get("c")?.plus_eq(get("d")?);
        set_return(get("c")?)?;
        Ok(())
    });

    deforfun("sub", &["a", "b"], || {
        define("c", Eval::Value(get("a")?))?;
        define("d", Eval::Value(get("b")?))?;
        get("c")?.minus_eq(get("d")?);
        set_return(get("c")?)?;
        Ok(())
    });

    deforfun("alloc", &["size"], || {
        define("ptr", Eval::Value(Value::variable_alloc(get("size")?)?))?;
        set_return(get("ptr")?)?;
        Ok(())
    });

    deforfun("free_byte", &["ptr"], || {
        get("ptr")?.deref()?.free();
        Ok(())
    });

    let prog = Program::from(
        r#"


        
#[enable(brainfuck)]

fn cprn(cstr) {
    print_cstr(cstr);

    return 0;
}

fn cprnln(cstr) {
    cprn(cstr);
    prn('\n');

    return 0;
}

fn prnln(str) {
    prn(str);
    prn('\n');

    return 0;
}

fn free(ptr, size) {
    while size {
        size = sub(size, 1);
        free_byte(add(ptr, size));
    }

    return 0;
}

fn cstr(s, size) {
    def ptr = alloc(size);
    *ptr = s;

    return ptr;
}

fn mul(a, b) {
    def n = 0;
    while b {
        b = sub(b, 1);
        n = add(n, a);
    }

    return n;
}

fn beep() {
    prn(7);    
}

fn start() {
    return 0;
}

"#,
    );
    // println!("{:#?}", prog);
    prog.compile()?;

    call("start", &vec![])?;
    println!("{}", compile());
    eprintln!("Done!");
    Ok(())
}
