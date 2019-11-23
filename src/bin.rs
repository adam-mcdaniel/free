extern crate smpl_typchk;
use smpl_typchk::{compile, compile::*, init, ir::*, Error, Stdout, Value, SCOPE_STACK};

fn main() -> Result<(), Error> {
    init();

    deforfun("print", &["a"], || {
        Stdout::print(get("a")?);
        Stdout::print(Eval::Literal(Literal::character('\n')).lower()?);
        Ok(())
    });

    deforfun("add", &["a", "b"], || {
        get("a")?.plus_eq(get("b")?);
        set_return(get("a")?);
        Ok(())
    });

    UserFn::define(
        "test",
        vec![],
        vec![
            Box::new(Expr::Define(Define::new(
                "a",
                Box::new(Eval::Literal(Literal::character(65 as char))),
            ))),
            Box::new(Expr::Define(Define::new(
                "b",
                Box::new(Eval::Literal(Literal::character(1 as char))),
            ))),
            Box::new(Expr::Eval(Eval::Call(Call::new(
                String::from("print"),
                vec![Box::new(Eval::Call(Call::new(
                    String::from("add"),
                    vec![
                        Box::new(Eval::Load(Load::new("a"))),
                        Box::new(Eval::Load(Load::new("b")))
                    ],
                )))],
            )))),
            Box::new(Expr::Eval(Eval::Call(Call::new(
                String::from("print"),
                vec![Box::new(Eval::Load(Load::new("a")))]
            ))))
        ],
    );

    UserFn::define(
        "start",
        vec![],
        vec![
            Box::new(Expr::Eval(Eval::Call(Call::new(
                String::from("test"),
                vec![],
            )))),
        ],
    );
    // Box::new(Expr::Define(Define::new(
    //     "a",
    //     Box::new(Eval::Literal(Literal::string("Hello world!"))),
    // ))).compile()?;
    // Box::new(Expr::Eval(Eval::Call(Call::new(
    //     String::from("print"),
    //     vec![Box::new(Eval::Load(Load::new(String::from("a"))))],
    // )))).compile()?;

    // Expr::Define(Define::new("a", Box::new(Value::string("test")))).compile()?;
    call("start", &vec![])?;
    println!("{}", compile());

    Ok(())
}
