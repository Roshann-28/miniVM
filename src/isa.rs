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

// encode: turns one Operation into bytes
// e.g. Push(7) -> [0x01, 7, 0, 0, 0, 0, 0, 0, 0]  (opcode + 8 bytes for i64)
//      Add     -> [0x10]                             (opcode only)
pub fn encode(op: &Operation) -> Vec<u8> {
    match op {
        Operation::Push(n) => {
            let mut bytes = vec![0x01];
            // i64 is 8 bytes, stored little-endian
            bytes.extend_from_slice(&n.to_le_bytes());
            bytes
        }
        Operation::Pop   => vec![0x02],
        Operation::Dup   => vec![0x03],
        Operation::Swap  => vec![0x04],
        Operation::Add   => vec![0x10],
        Operation::Sub   => vec![0x11],
        Operation::Mul   => vec![0x12],
        Operation::Div   => vec![0x13],
        Operation::Mod   => vec![0x14],
        Operation::Neg   => vec![0x15],
        Operation::Load(s)  => vec![0x40, *s],
        Operation::Store(s) => vec![0x41, *s],
        Operation::Print => vec![0x60],
        Operation::Halt  => vec![0xFF],
    }
}

// decode: reads bytes starting at `pos`, returns the Operation
// and how many bytes it consumed
// e.g. bytes=[0x01, 7, 0, 0, 0, 0, 0, 0, 0] -> Ok((Push(7), 9))
//      bytes=[0x10]                           -> Ok((Add, 1))
pub fn decode(bytes: &[u8], pos: usize) -> Result<(Operation, usize), String> {
    if pos >= bytes.len() {
        return Err(format!(
            "trap at ip=0x{:04X}: ip past end of code without HALT",
            pos
        ));
    }

    let opcode = bytes[pos];

    match opcode {
        0x01 => {
            // PUSH needs 8 more bytes for the i64 operand
            if pos + 9 > bytes.len() {
                return Err(format!(
                    "trap at ip=0x{:04X}: truncated instruction (PUSH needs 8 bytes)",
                    pos
                ));
            }
            // read 8 bytes and convert to i64 (little-endian)
            let n = i64::from_le_bytes(bytes[pos+1..pos+9].try_into().unwrap());
            Ok((Operation::Push(n), 9))
        }
        0x02 => Ok((Operation::Pop,  1)),
        0x03 => Ok((Operation::Dup,  1)),
        0x04 => Ok((Operation::Swap, 1)),
        0x10 => Ok((Operation::Add,  1)),
        0x11 => Ok((Operation::Sub,  1)),
        0x12 => Ok((Operation::Mul,  1)),
        0x13 => Ok((Operation::Div,  1)),
        0x14 => Ok((Operation::Mod,  1)),
        0x15 => Ok((Operation::Neg,  1)),
        0x40 => {
            // LOAD needs 1 more byte for the slot number
            if pos + 2 > bytes.len() {
                return Err(format!(
                    "trap at ip=0x{:04X}: truncated instruction (LOAD needs 1 byte)",
                    pos
                ));
            }
            Ok((Operation::Load(bytes[pos+1]), 2))
        }
        0x41 => {
            // STORE needs 1 more byte for the slot number
            if pos + 2 > bytes.len() {
                return Err(format!(
                    "trap at ip=0x{:04X}: truncated instruction (STORE needs 1 byte)",
                    pos
                ));
            }
            Ok((Operation::Store(bytes[pos+1]), 2))
        }
        0x60 => Ok((Operation::Print, 1)),
        0xFF => Ok((Operation::Halt,  1)),
        _ => Err(format!(
            "trap at ip=0x{:04X}: unknown opcode 0x{:02X}",
            pos, opcode
        )),
    }
}