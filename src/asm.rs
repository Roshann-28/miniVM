use crate::isa::{Operation, encode};
use crate::bytecode::write_bytecode;

// assembles a .tasm text file into a Vec<u8> ready to write to disk
pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut program: Vec<Operation> = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        // strip comments — everything after `;` is ignored
        let line = match line.find(';') {
            Some(i) => &line[..i],
            None => line,
        };

        // trim whitespace
        let line = line.trim();

        // skip blank lines
        if line.is_empty() {
            continue;
        }

        // split into mnemonic and optional operand
        // e.g. "PUSH 7" -> ["PUSH", "7"]
        //      "ADD"    -> ["ADD"]
        let mut parts = line.splitn(2, char::is_whitespace);
        let mnemonic = parts.next().unwrap().to_uppercase();
        let operand = parts.next().map(|s| s.trim());

        let op = match mnemonic.as_str() {
            "PUSH" => {
                let n = parse_i64(operand, line_num, "PUSH")?;
                Operation::Push(n)
            }
            "POP"   => Operation::Pop,
            "DUP"   => Operation::Dup,
            "SWAP"  => Operation::Swap,
            "ADD"   => Operation::Add,
            "SUB"   => Operation::Sub,
            "MUL"   => Operation::Mul,
            "DIV"   => Operation::Div,
            "MOD"   => Operation::Mod,
            "NEG"   => Operation::Neg,
            "LOAD"  => {
                let s = parse_u8(operand, line_num, "LOAD")?;
                Operation::Load(s)
            }
            "STORE" => {
                let s = parse_u8(operand, line_num, "STORE")?;
                Operation::Store(s)
            }
            "PRINT" => Operation::Print,
            "HALT"  => Operation::Halt,
            _ => {
                return Err(format!(
                    "line {}: unknown mnemonic '{}'",
                    line_num + 1, mnemonic
                ));
            }
        };

        program.push(op);
    }

    // warn if program doesn't end with HALT
    if !matches!(program.last(), Some(Operation::Halt)) {
        eprintln!("warning: program does not end with HALT");
    }

    Ok(write_bytecode(&program))
}

// helper: parse an i64 operand (for PUSH)
fn parse_i64(operand: Option<&str>, line_num: usize, mnemonic: &str) -> Result<i64, String> {
    match operand {
        None => Err(format!(
            "line {}: {} requires an operand (e.g. {} 42)",
            line_num + 1, mnemonic, mnemonic
        )),
        Some(s) => s.parse::<i64>().map_err(|_| {
            format!(
                "line {}: invalid operand '{}' for {} (expected integer)",
                line_num + 1, s, mnemonic
            )
        }),
    }
}

// helper: parse a u8 operand (for LOAD/STORE slot numbers 0-255)
fn parse_u8(operand: Option<&str>, line_num: usize, mnemonic: &str) -> Result<u8, String> {
    match operand {
        None => Err(format!(
            "line {}: {} requires a slot number (0-255)",
            line_num + 1, mnemonic
        )),
        Some(s) => s.parse::<u8>().map_err(|_| {
            format!(
                "line {}: invalid slot '{}' for {} (expected 0-255)",
                line_num + 1, s, mnemonic
            )
        }),
    }
}