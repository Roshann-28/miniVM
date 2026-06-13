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

// Adding Copy, Clone, Debug means you can freely pass Op values around, print them for tracing, without fighting Rust's ownership rules.
#[derive(Debug, Clone, Copy)]
enum Operation {
    Push(i64),
    Add,
    Print,
}

fn main() {
    let program = vec![
        Operation::Push(7),
        Operation::Push(3),
        Operation::Add,
        Operation::Print,
    ];

    let mut stack: Vec<i64> = Vec::new();

    for instr in program {
        match instr {
            Operation::Push(n) => stack.push(n),
            Operation::Add => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }

            Operation::Print => {
                let val = stack.pop().unwrap();
                println!("{}", val);
            }
        }
    }
}
