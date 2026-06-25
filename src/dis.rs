use crate::isa::op_to_string;
use crate::bytecode::{read_bytecode, decode_all};

pub fn disassemble(file_bytes: &[u8]) -> Result<String, String> {
    let code = read_bytecode(file_bytes)?;
    let program = decode_all(&code)?;

    let mut output = String::new();
    for op in &program {
        output.push_str(&op_to_string(op));
        output.push('\n');
    }

    Ok(output)
}