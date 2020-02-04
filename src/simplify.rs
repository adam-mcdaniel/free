use crate::Program;

pub trait Simplify {
    fn prelude() -> String;
    fn postlude() -> String;
    fn simplify(s: impl ToString) -> String;
}

pub struct C;
impl C {
    pub fn new() -> Self {
        Self
    }
}

impl Simplify for C {
    fn prelude() -> String {
        format!(
            r#"#include <stdio.h>
const int TAPE_SIZE = {TAPE_SIZE};
const int REF_TAPE_SIZE = {REF_TAPE_SIZE};

unsigned short tape[{TAPE_SIZE}];
unsigned int ref_tape[{TAPE_SIZE}];
unsigned int ptr = 0;
unsigned int ref_ptr = 0;

unsigned int allocate() {{
    unsigned int size = tape[ptr]; 
    int cons_empty_spaces = 0; 
    for (int i=TAPE_SIZE-1; i>0; i--) {{
        if (tape[i] == 0) {{ cons_empty_spaces++; }}
        else {{ cons_empty_spaces = 0; }}
        if (cons_empty_spaces == size) {{ return i; }}
    }}
    return 0;
}}


void plus(int n) {{
    tape[ptr] += n;
}}

void minus(int n) {{
    tape[ptr] -= n;
}}

void set(int n) {{
    tape[ptr] = n;
}}

void left(int n) {{
    ptr -= n;
}}

void right(int n) {{
    ptr += n;
}}

void deref() {{
    ref_tape[ref_ptr++ % REF_TAPE_SIZE] = ptr;
    ptr = tape[ptr];
}}

void refer() {{
    ptr = ref_tape[--ref_ptr % REF_TAPE_SIZE];
}}

int main() {{
"#,
            TAPE_SIZE = Program::tape_size(),
            REF_TAPE_SIZE = 256
        )
    }

    fn postlude() -> String {
        String::from("}")
    }

    fn simplify(s: impl ToString) -> String {
        let mut result = Self::prelude();
        let mut repeated = 0;
        let mut last = '\0';

        let mut filtered = s
            .to_string()
            .chars()
            .filter(|c| ['>', '<', '+', '-', '*', '&', '?', '[', ']', '.', ','].contains(c))
            .collect::<String>();
        filtered = filtered.replace("[-]", "0");

        for ch in filtered.chars() {
            if ch == last {
                repeated += 1;
            } else {
                let line = match last {
                    '>' => format!("right({});", repeated),
                    '<' => format!("left({});", repeated),
                    '+' => format!("plus({});", repeated),
                    '-' => format!("minus({});", repeated),
                    '0' => "set(0);".repeat(repeated),
                    '*' => "deref();".repeat(repeated),
                    '&' => "refer();".repeat(repeated),
                    '?' => "tape[ptr] = allocate();".repeat(repeated),
                    '[' => "while (tape[ptr]) {".repeat(repeated),
                    ']' => "}\n".repeat(repeated),
                    '.' => "printf(\"%c\",(char)(tape[ptr]%256));".repeat(repeated),
                    ',' => "scanf(\"%c\", (char*)&tape[ptr]);".repeat(repeated),
                    _ => String::new(),
                };
                result += &(line + "\n");
                repeated = 1;
                last = ch;
            }
        }

        result + &Self::postlude()
    }
}
