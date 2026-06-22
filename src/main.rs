#[derive(Debug, Clone, Copy)]
enum Operation {
    Push(i64),
    Pop,
    Dup,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Load(u8),
    Store(u8),
    Print,
    Halt,
}

// Converts an Operation into its assembly text form (e.g. "PUSH 7").
// Used by trace output now, and will be reused by the disassembler later.
fn op_to_string(op: &Operation) -> String {
    match op {
        Operation::Push(n) => format!("PUSH {}", n),
        Operation::Pop => "POP".to_string(),
        Operation::Dup => "DUP".to_string(),
        Operation::Swap => "SWAP".to_string(),
        Operation::Add => "ADD".to_string(),
        Operation::Sub => "SUB".to_string(),
        Operation::Mul => "MUL".to_string(),
        Operation::Div => "DIV".to_string(),
        Operation::Mod => "MOD".to_string(),
        Operation::Neg => "NEG".to_string(),
        Operation::Load(s) => format!("LOAD {}", s),
        Operation::Store(s) => format!("STORE {}", s),
        Operation::Print => "PRINT".to_string(),
        Operation::Halt => "HALT".to_string(),
    }
}

fn trap_err(ip: usize, msg: &str) -> Result<(), String> {
    Err(format!("trap at ip=0x{:04X}: {}", ip, msg))
}

fn run(program: Vec<Operation>, trace: bool) -> Result<(), String> {
    let mut stack: Vec<i64> = Vec::new();
    let mut ip: usize = 0;
    let mut globals: [i64; 256] = [0; 256];

    while ip < program.len() {
        let instr = program[ip];
        let current_ip = ip;

        if trace {
            println!(
                "ip=0x{:04X}  {}  stack={:?}",
                current_ip,
                op_to_string(&instr),
                stack
            );
        }

        ip += 1;

        match instr {
            Operation::Push(n) => {
                if stack.len() >= 1024 {
                    return trap_err(current_ip, "stack overflow (Push on full stack)");
                }
                stack.push(n);
            }

            Operation::Pop => {
                if stack.is_empty() {
                    return trap_err(current_ip, "stack underflow (Pop on empty stack)");
                }
                stack.pop();
            }

            Operation::Dup => {
                if stack.is_empty() {
                    return trap_err(current_ip, "stack underflow (Dup on empty stack)");
                }
                let top = *stack.last().unwrap();
                stack.push(top);
            }

            Operation::Swap => {
                if stack.len() < 2 {
                    return trap_err(current_ip, "stack underflow (Swap needs 2 values)");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(b);
                stack.push(a);
            }

            Operation::Add => {
                if stack.len() < 2 {
                    return trap_err(current_ip, "stack underflow (Add needs 2 values)");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }

            Operation::Sub => {
                if stack.len() < 2 {
                    return trap_err(current_ip, "stack underflow (Sub needs 2 values)");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }

            Operation::Mul => {
                if stack.len() < 2 {
                    return trap_err(current_ip, "stack underflow (Mul needs 2 values)");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }

            Operation::Div => {
                if stack.len() < 2 {
                    return trap_err(current_ip, "stack underflow (Div needs 2 values)");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                if b == 0 {
                    return trap_err(current_ip, "division by zero");
                }
                stack.push(a / b);
            }

            Operation::Mod => {
                if stack.len() < 2 {
                    return trap_err(current_ip, "stack underflow (Mod needs 2 values)");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                if b == 0 {
                    return trap_err(current_ip, "modulo by zero");
                }
                stack.push(a % b);
            }

            Operation::Neg => {
                if stack.is_empty() {
                    return trap_err(current_ip, "stack underflow (Neg on empty stack)");
                }
                let a = stack.pop().unwrap();
                stack.push(-a);
            }

            Operation::Load(slot) => {
                if stack.len() >= 1024 {
                    return trap_err(current_ip, "stack overflow (Load on full stack)");
                }
                stack.push(globals[slot as usize]);
            }

            Operation::Store(slot) => {
                if stack.is_empty() {
                    return trap_err(current_ip, "stack underflow (Store on empty stack)");
                }
                let val = stack.pop().unwrap();
                globals[slot as usize] = val;
            }

            Operation::Print => {
                if stack.is_empty() {
                    return trap_err(current_ip, "stack underflow (Print on empty stack)");
                }
                let val = stack.pop().unwrap();
                println!("{}", val);
            }

            Operation::Halt => break,
        }
    }

    Ok(())
}

fn main() {
    // collect all command-line arguments into a Vec<String>
    // args()[0] is always the program name itself, so we skip it
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: minivm <subcommand> [options]");
        eprintln!("  minivm run <file.tbc> [--trace]");
        std::process::exit(1);
    }

    let subcommand = &args[1];

    match subcommand.as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("Usage: minivm run <file.tbc> [--trace]");
                std::process::exit(1);
            }

            let filename = &args[2];
            // check if --trace flag is present anywhere in the args
            let trace = args.contains(&"--trace".to_string());

            println!("would run: {} (trace={})", filename, trace);
            // we'll replace this println with the real run() call
            // once we build the assembler and have real .tbc files
        }

        "asm" => {
            println!("assembler not built yet");
        }

        "dis" => {
            println!("disassembler not built yet");
        }

        _ => {
            eprintln!("unknown subcommand: {}", subcommand);
            eprintln!("valid subcommands: run, asm, dis");
            std::process::exit(1);
        }
    }
}