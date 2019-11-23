extern crate smpl_typchk;
use smpl_typchk::{Program, compile, compile::*, init, Error, Stdout, Value};

fn main() -> Result<(), Error> {
    init();

    deforfun("print", &["a"], || {
        Stdout::print(get("a")?);
        Ok(())
    });

    deforfun("print_cstr", &["a"], || {
        Stdout::print_cstr(get("a")?);
        Ok(())
    });

    deforfun("add", &["a", "b"], || {
        get("a")?.plus_eq(get("b")?);
        set_return(get("a")?);
        Ok(())
    });

    deforfun("sub", &["a", "b"], || {
        get("a")?.minus_eq(get("b")?);
        set_return(get("a")?);
        Ok(())
    });

    deforfun("alloc", &["size"], || {
        define("ptr", Eval::Value(Value::variable_alloc(get("size")?)))?;
        set_return(get("ptr")?);
        Ok(())
    });

    deforfun("free_byte", &["ptr"], || {
        get("ptr")?.deref().free();
        Ok(())
    });

    let prog = Program::from(r#"

fn println(value) {
    print(value);
    print('\n');

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

fn cstr1(s, size) {
    def ptr = alloc(size);
    *ptr = s;

    return ptr;
}

fn cstr2(s, size) {
    def ptr = alloc(size);
    *ptr = s;

    return ptr;
}
fn cstr3(s, size) {
    def ptr = alloc(size);
    *ptr = s;

    return ptr;
}

fn start() {
    def a = cstr("hello world!", 20);

    print_cstr(a);
    free(a, 20);

    return 0;
}

"#);
    // println!("{:#?}", prog);
    prog.compile()?;

    call("start", &vec![])?;
    println!("{}", compile());

    Ok(())
}
