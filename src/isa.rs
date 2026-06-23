// src/isa.rs

#[derive(Debug, Clone, Copy)]
pub enum Operation {
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

pub fn op_to_string(op: &Operation) -> String {
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