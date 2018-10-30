
struct Compiler {
    program: String,
}

impl Compiler {
    fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
        }
    }

    fn compile(&self) -> &str {
        // FIXME
        let chars = self.program.chars();

        for c in chars {
            match c {
                ' ' => {
                    // do nothing
                },
                '+' | '-' | '*' | '/' => {
                    println!("{}", c);
                },
                '0'...'9' => {
                    println!("{}", c);
                },
                x => {
                    panic!("Invalid token: {:?}", x);
                },
            }
        }

        r"push r0
mov r0, 1
add r0, 2
pop r0"
    }
}

fn main() {
    let compiler = Compiler::new("1 2 +");

    compiler.compile();
}

#[test]
fn test_compiler_compile_add() {
    let compiler = Compiler::new("1 2 +");

    assert_eq!(compiler.compile(), r"push r0
mov r0, 1
add r0, 2
pop r0");
}

#[test]
fn test_compiler_compile_function() {
    let compiler = Compiler {
        program: "A[a + b] A(1 2)".to_string(),
    };

    assert_eq!(compiler.compile(), r"mov r1, 1
mov r2, 2
call A
A:
push r1
push r2
add r1, r2
move r0, r1
pop r2
pop r1
ret");
}
