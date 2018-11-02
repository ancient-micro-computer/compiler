#[macro_use]
extern crate tera;

use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use std::io::prelude::*;
use tera::Context;

fn compile(input: &str) -> String {
    let tera = compile_templates!("templates/*");

    let mut context = Context::new();
    context.insert("main", &eval(input));
    tera.render("main.asm", &context).unwrap()
}

fn eval(input: &str) -> String {
    let mut code = input;
    let mut code = & mut code;
    let mut funcs: HashMap<char, u32> = HashMap::new();
    eval_r(&mut code, &mut funcs, ' ')
}

fn eval_r(code: & mut &str, funcs: &mut HashMap<char, u32>, func: char) -> String {
    if code.len() < 1 {
        return String::new();
    }
    let mut func = func;
    let mut lines = String::new();
    let mut chars = code.chars();
    let mut pos: usize = 0;
    let first_char = chars.next().unwrap();
    pos += 1;

    let mut num;
    let tera = compile_templates!("templates/*");

    let mut context = Context::new();
    match first_char {
        '0'...'9' => {
            num = first_char.to_digit(10).unwrap();

            for c in chars {
                if let Some(digit) = c.to_digit(10) {
                    pos += 1;
                    num = digit + num * 10;
                } else {
                    break;
                }
            }
            lines.push_str(&format!("    mov r0, {}\n", num));
            lines.push_str("    push r0\n");
        },
        ' ' | '\n' => {
            //do nothing
        },
        '+' => {
            context.insert("ope", "add");
            lines.push_str(&tera.render("operator.asm", &context).unwrap());
        },
        '-' => {
            context.insert("ope", "sub");
            lines.push_str(&tera.render("operator.asm", &context).unwrap());
        },
        '*' => {
            context.insert("ope", "mul");
            lines.push_str(&tera.render("operator.asm", &context).unwrap());
        },
        '/' => {
            context.insert("ope", "div");
            lines.push_str(&tera.render("operator.asm", &context).unwrap());
        },
        'A'...'O' | 'Q'...'Z' => {
            let c = chars.next().unwrap();
            match c {
                '[' => {
                    func = first_char;
                    funcs.insert(first_char.clone(), 0);
                    lines.push_str(&first_char.to_string());
                    lines.push_str(":\n");
                    pos += 1;
                },
                _ => {
                    let argc = funcs.get(&first_char).unwrap();
                    if argc >= &2 {
                        lines.push_str("    pop r9\n");
                    }
                    lines.push_str("    pop r8\n");
                    lines.push_str("    call ");
                    lines.push_str(&first_char.to_string());
                    lines.push_str("\n");
                }
            }
        },
        '.' | 'a' => {
            let argc = funcs.remove(&func).unwrap();
            if argc < 1 {
                funcs.insert(func.clone(), 1);
            } else {
                funcs.insert(func.clone(), argc);
            }
            lines.push_str("    push r8\n");
        },
        'b' => {
            let argc = funcs.remove(&func).unwrap();
            if argc < 2 {
                funcs.insert(func.clone(), 2);
            } else {
                funcs.insert(func.clone(), argc);
            }
            lines.push_str("    push r9\n");
        },
        'P' => {
            lines.push_str("    call P\n");
        },
        ']' => {
            func = ' ';
            lines.push_str("    pop r14\n");
            lines.push_str("    pop r15\n");
            lines.push_str("    push r14\n");
            lines.push_str("    push r15\n");
            lines.push_str("    ret\n");
        },
        x => {
            panic!("Invalid token: {:?}", x);
        },
    }
    if pos < code.len() {
        *code = &code[pos..];
        lines.push_str(&eval_r(code, funcs, func));
    } else {
        //do nothing (it means end of string)
    }
    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("at least one arg is required")
    }

    // read from srcfile
    let srcfile = args.get(1).unwrap();
    let mut file = fs::File::open(srcfile).unwrap();
    let mut src = String::new();
    file.read_to_string(&mut src).unwrap();

    let assembly = compile(&src);

    let mut f = fs::File::create("OUT.asm").unwrap();

    f.write_all(assembly.as_bytes()).unwrap();

    // println!("{}", assembly);
}

#[test]
fn test_compiler_compile() {
    {
        assert_eq!(eval("M[1]"), r"M:
    mov r0, 1
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
");
        assert_eq!(eval("M[12]"), r"M:
    mov r0, 12
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
");
        assert_eq!(eval("M[1 2 +]"), r"M:
    mov r0, 1
    push r0
    mov r0, 2
    push r0
    pop r1
    pop r0
    add r0, r1
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
");
        assert_eq!(eval("M[1 2 -]"), r"M:
    mov r0, 1
    push r0
    mov r0, 2
    push r0
    pop r1
    pop r0
    sub r0, r1
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
");
        assert_eq!(eval("M[1 2 *]"), r"M:
    mov r0, 1
    push r0
    mov r0, 2
    push r0
    pop r1
    pop r0
    mul r0, r1
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
");
        assert_eq!(eval("M[1 2 /]"), r"M:
    mov r0, 1
    push r0
    mov r0, 2
    push r0
    pop r1
    pop r0
    div r0, r1
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
");

        assert_eq!(eval("M[1 2 / P]"), r"M:
    mov r0, 1
    push r0
    mov r0, 2
    push r0
    pop r1
    pop r0
    div r0, r1
    push r0
    call P
    pop r14
    pop r15
    push r14
    push r15
    ret
");
         assert_eq!(eval("Q[. . +] M[1 Q]"),r"Q:
    push r8
    push r8
    pop r1
    pop r0
    add r0, r1
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
M:
    mov r0, 1
    push r0
    pop r8
    call Q
    pop r14
    pop r15
    push r14
    push r15
    ret
");
         assert_eq!(eval("Q[. . +]\nM[1 Q]"),r"Q:
    push r8
    push r8
    pop r1
    pop r0
    add r0, r1
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
M:
    mov r0, 1
    push r0
    pop r8
    call Q
    pop r14
    pop r15
    push r14
    push r15
    ret
");
         assert_eq!(eval("Q[2 . +] M[1 Q]"),r"Q:
    mov r0, 2
    push r0
    push r8
    pop r1
    pop r0
    add r0, r1
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
M:
    mov r0, 1
    push r0
    pop r8
    call Q
    pop r14
    pop r15
    push r14
    push r15
    ret
");
         assert_eq!(eval("F[a b +] M[1 2 F]"),r"F:
    push r8
    push r9
    pop r1
    pop r0
    add r0, r1
    push r0
    pop r14
    pop r15
    push r14
    push r15
    ret
M:
    mov r0, 1
    push r0
    mov r0, 2
    push r0
    pop r9
    pop r8
    call F
    pop r14
    pop r15
    push r14
    push r15
    ret
");
    }
}
