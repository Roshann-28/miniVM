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
    // Stack manipulation
    Push(i64),
    Pop,
    Dup,
    Swap,
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    // I/O and control
    Print,
    Halt,
}

fn main() {
    let program = vec![
        Operation::Push(7),
        Operation::Dup,
        Operation::Mul,
        Operation::Print,
        Operation::Halt,
    ];

    let mut stack: Vec<i64> = Vec::new();
    let mut ip: usize = 0;

    while ip < program.len() {
        let instr = program[ip];
        ip += 1;

        match instr {
            Operation::Push(n) => stack.push(n),

            Operation::Pop => {
                stack.pop().unwrap(); // discard top value
            }

            Operation::Dup => {
                let top = stack.last().unwrap(); // peek without removing
                stack.push(*top); // push a copy
            }

            Operation::Swap => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(b); // b goes down
                stack.push(a); // a goes on top
            }

            Operation::Add => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }

            Operation::Sub => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }

            Operation::Mul => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }

            Operation::Div => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b); // we'll add div-by-zero trap later
            }

            Operation::Mod => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a % b); // we'll add mod-by-zero trap later
            }

            Operation::Neg => {
                let a = stack.pop().unwrap();
                stack.push(-a); // flip the sign
            }

            Operation::Print => {
                let val = stack.pop().unwrap();
                println!("{}", val);
            }

            Operation::Halt => break,
        }
    }
}
