// fn main() {
//     // why vec<i64> for the stack?
//     // A stack only needs to add/remove from the top.
//     // both push() and pop() to do on a Vec are fast     (O(1)).
//     // No need for a special "stack" type — Vec already behaves like one
//     let mut stack: Vec<i64> = Vec::new();

//     stack.push(7);
//     stack.push(3);

//     // let b: Option<i64>= stack.pop();
//     // let a: Option<i64>= stack.pop(); -> cannot add a + b as (Option<i64> + Option<i64>) can't be added as Option<i64> returns a container either Some(T), or None

//     //That's why we need .unwrap() first — it opens the box and gives you the i64 inside (or crashes if the box was empty/None):

//     let b = stack.pop().unwrap();
//     let a = stack.pop().unwrap();

//     stack.push(a + b);

//     let result = stack.pop().unwrap();
//     println!("result: {}", result);
// }

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

// helper to build trap error strings cleanly
fn trap_err(ip: usize, msg: &str) -> Result<(), String> {
    Err(format!("trap at ip=0x{:04X}: {}", ip, msg))
}

fn run(program: Vec<Operation>) -> Result<(), String> {
    let mut stack: Vec<i64> = Vec::new();
    let mut ip: usize = 0;
    let mut globals: [i64; 256] = [0; 256];

    while ip < program.len() {
        let instr = program[ip];
        let current_ip = ip; // save before advancing for error messages
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
    let program = vec![
        Operation::Push(10),
        Operation::Push(0),
        Operation::Div, // division by zero
        Operation::Print,
        Operation::Halt,
    ];
    // should print: trap at ip=0x0002: division by zero

    // if run() returns an Err, print it and exit with code 1
    if let Err(e) = run(program) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
