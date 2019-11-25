extern crate smpl_typchk;
use smpl_typchk::{Simplify, C, Error, Program};
// use regex::Regex;


pub fn optimize(s: impl ToString, level: usize) -> String {
    let mut compiled = s.to_string().chars().filter(|ch| ['>', '<', ',', '.', '[', ']', '+', '-', '*', '?', '&'].contains(ch)).collect::<String>();
    let original_len = compiled.len();

    for n in 1..level+1 {
        let to = ">".repeat(n);
        let back = "<".repeat(n);

        let move1 = to.clone() + &back;
        let move2 = back + &to;
        compiled = compiled.replace(&move1, "").replace(&move2, "");
    }

    println!("// OPTIMIZED {} INSTRUCTIONS", original_len - compiled.len());
    compiled
}


fn main() -> Result<(), Error> {
    let prog = Program::from(
        r#"
// #[enable(brainfuck)]

// fn cprn(cstr) {
//     print_cstr(cstr);

//     return 0;
// }

// fn cprnln(cstr) {
//     cprn(cstr);
//     prn('\n');

//     return 0;
// }

// fn prnln(str) {
//     prn(str);
//     prn('\n');

//     return 0;
// }

fn free(ptr, size) {
    while size {
        size = sub(size, 1);
        free_byte(add(ptr, size));
    }

    return 0;
}

// fn cstr(s, size) {
//     def ptr = alloc(size);
//     *ptr = s;

//     return ptr;
// }

// fn mul(a, b) {
//     def n = 0;
//     while b {
//         b = sub(b, 1);
//         n = add(n, a);
//     }

//     return n;
// }

// fn beep() {
//     prn(7);    
// }

fn start() {
    fib(7);
    def a = "testing";
    cprintln(&a);
    def t = cstr("wow", 10);
    cprintln(t);
    free(t, 10);

    return 0;
}

fn cstr(s, len) {
    def a = alloc(len);
    *a = s;

    def counter = sub(len, 1);
    while counter {
        def ch = *add(a, counter);
        if ch {}
        else {
            *add(a, counter) = 1;
        }
        counter = sub(counter, 1);
    }

    return a;
}


fn fib(n) {
    def a = 0;
    def b = 1;
    def c = 1;

    while n {
        c = a;
        println(digit(a));
        a = b;
        b = add(b, c);
        n = sub(n, 1);
    }

    return 0;
}
    
fn not(a) {
    if a { return 0; }
    else { return 1; }
}

fn digit(n) {
    return add(n, 48);
}
"#,
    );
    // println!("{:#?}", prog);
    let level = 5;
    println!("{}", C::simplify(optimize(prog.compile()?, level)));
    // println!("{}", prog.compile()?);
    Ok(())
}
