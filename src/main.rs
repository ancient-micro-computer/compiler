use std::env;

fn compile(input: &str) -> String {

    let mut assembly = String::new();
    assembly.push_str(r".ORG     $0000	// IPLエリア
         call	_MAIN
         halt

.ORG     $1000	// ユーザエリア
_MAIN:
");
    assembly.push_str(&eval(input));
    assembly.push_str("    ret\n");
    assembly
}
fn eval(input: &str) -> String {
    let mut code = input;
    let mut code = & mut code;
    eval_r(&mut code)
}

fn eval_r(code: & mut &str) -> String {
    if code.len() < 1 {
        return String::new();
    }
    // FIXME
    let mut lines = String::new();

    let mut chars = code.chars();

    let mut pos: usize = 0;

    let first_char = chars.next().unwrap();

    pos += 1;

    let mut num = 0;

    if first_char.is_digit(10) {
        num = first_char.to_digit(10).unwrap();

        for (i, c) in chars.enumerate() {
            if let Some(digit) = c.to_digit(10) {
                pos += 1;
                num = digit + num * 10;
            } else {
                break;
            }            
        }
        lines.push_str(&format!("    mov r0, {}\n", num));
        lines.push_str("    push r0\n");
        *code = &code[pos..];
        lines.push_str(&eval_r(code))
    } else {
        match first_char {
            ' ' => {
                //do nothing
            }
            '+' => {
                lines.push_str(r"    pop r1
    pop r0
    add r0, r1
    push r0
");
            },
            x => {
               panic!("Invalid token: {:?}", x); 
            }
        }
        *code = &code[1..];
        lines.push_str(&eval_r(code))
    }

    // for c in chars {
    //     match c {
    //         ' ' => {
    //             // do nothing
    //         },
    //         '+' | '-' | '*' | '/' => {
    //             println!("{}", c);
    //         },
    //         '0'...'9' => {
    //             lines.push("mov r0, {}");
    //             lines.push("push r0");
    //         },
    //         x => {
    //             panic!("Invalid token: {:?}", x);
    //         },
    //     }
    // }

    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("at least one arg is required")
    }
    println!("{}",compile(args.get(1).unwrap()));
}

#[test]
fn test_compiler_compile() {
    {
        assert_eq!(eval("1"), r"    mov r0, 1
    push r0
");
        assert_eq!(eval("12"), r"    mov r0, 12
    push r0
");
        assert_eq!(eval("1 2 +"), r"    mov r0, 1
    push r0
    mov r0, 2
    push r0
    pop r1
    pop r0
    add r0, r1
    push r0
");
    }
}
