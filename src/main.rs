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
    Load(u8),  // u8 because slot number is 0-255
    Store(u8), // u8 because slot number is 0-255
    Print,
    Halt,
}

fn main() {
    let program = vec![
        Operation::Push(10), // push 10
        Operation::Store(0), // save in slot 0
        Operation::Push(20), // push 20
        Operation::Store(1), // save in slot 1
        Operation::Load(0),  // load slot 0 → stack: [10]
        Operation::Load(1),  // load slot 1 → stack: [10, 20]
        Operation::Add,      // stack: [30]
        Operation::Print,    // prints 30
        Operation::Halt,
    ];

    let mut stack: Vec<i64> = Vec::new();
    let mut ip: usize = 0;

    // 256 global slots, all starting at zero
    let mut globals: [i64; 256] = [0; 256];

    while ip < program.len() {
        let instr = program[ip];
        ip += 1;

        match instr {
            Operation::Push(n) => stack.push(n),

            Operation::Pop => {
                stack.pop().unwrap();
            }

            Operation::Dup => {
                let top = stack.last().unwrap();
                stack.push(*top);
            }

            Operation::Swap => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(b);
                stack.push(a);
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
                stack.push(a / b);
            }

            Operation::Mod => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a % b);
            }

            Operation::Neg => {
                let a = stack.pop().unwrap();
                stack.push(-a);
            }

            Operation::Load(slot) => {
                // read from globals array, push onto stack
                stack.push(globals[slot as usize]);
            }

            Operation::Store(slot) => {
                // pop from stack, save into globals array
                let val = stack.pop().unwrap();
                globals[slot as usize] = val;
            }

            Operation::Print => {
                let val = stack.pop().unwrap();
                println!("{}", val);
            }

            Operation::Halt => break,
        }
    }
}
