// src/vm.rs

use crate::isa::{op_to_string, Operation};

fn trap_err(ip: usize, msg: &str) -> Result<(), String> {
    Err(format!("trap at ip=0x{:04X}: {}", ip, msg))
}

pub fn run(program: Vec<Operation>, trace: bool) -> Result<(), String> {
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

                let value = stack.pop().unwrap();
                globals[slot as usize] = value;
            }

            Operation::Print => {
                if stack.is_empty() {
                    return trap_err(current_ip, "stack underflow (Print on empty stack)");
                }

                let value = stack.pop().unwrap();
                println!("{}", value);
            }

            Operation::Halt => break,
        }
    }

    Ok(())
}