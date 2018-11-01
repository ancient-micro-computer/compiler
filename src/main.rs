fn compile(input: &str) -> String {
    let mut code = input;

    let mut code = & mut code;

    eval(& mut code)
}

fn eval(code: & mut &str) -> String {
    // FIXME
    let mut lines = String::new();

    let chars = code.chars();

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
                lines.push_str(&format!("mov r0, {}\n", num));
                lines.push_str("push r0");
            }
        }
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
    // FIXME
}

#[test]
fn test_compiler_compile() {
    {
        let compiler = Compiler::new("1".to_string());

        assert_eq!(compiler.compile(), r"mov r0, 1
push r0");
    }
}
