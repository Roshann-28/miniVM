use crate::isa::{encode, decode, Operation};

// the magic bytes that identify a valid .tbc file — spells "MVM\0"
pub const MAGIC: [u8; 4] = [0x4D, 0x56, 0x4D, 0x00];
pub const VERSION: u8 = 0x01;

// write: takes a list of Operations, encodes them all,
// wraps them in the header, returns the final bytes ready to write to disk
pub fn write_bytecode(program: &[Operation]) -> Vec<u8> {
    // encode every instruction into raw bytes
    let mut code: Vec<u8> = Vec::new();
    for op in program {
        code.extend_from_slice(&encode(op));
    }

    let mut file: Vec<u8> = Vec::new();

    // header: magic (4 bytes)
    file.extend_from_slice(&MAGIC);

    // header: version (1 byte)
    file.push(VERSION);

    // header: code length as u32 little-endian (4 bytes)
    let length = code.len() as u32;
    file.extend_from_slice(&length.to_le_bytes());

    // body: the actual code
    file.extend_from_slice(&code);

    file
}

// read: takes raw file bytes, validates the header,
// returns the code bytes ready to execute
pub fn read_bytecode(file: &[u8]) -> Result<Vec<u8>, String> {
    // check file is long enough to even have a header (4 + 1 + 4 = 9 bytes)
    if file.len() < 9 {
        return Err("invalid .tbc file: too short to contain a header".to_string());
    }

    // check magic bytes
    if file[0..4] != MAGIC {
        return Err("invalid .tbc file: wrong magic bytes (not a .tbc file)".to_string());
    }

    // check version
    if file[4] != VERSION {
        return Err(format!(
            "invalid .tbc file: unsupported version 0x{:02X} (expected 0x01)",
            file[4]
        ));
    }

    // read code length from bytes 5..9 (u32 little-endian)
    let length = u32::from_le_bytes(file[5..9].try_into().unwrap()) as usize;

    // check the file actually contains that many bytes after the header
    if file.len() < 9 + length {
        return Err(format!(
            "invalid .tbc file: header says {} bytes of code but file only has {}",
            length,
            file.len() - 9
        ));
    }

    // return just the code bytes
    Ok(file[9..9 + length].to_vec())
}

// decode_all: turns raw code bytes into a Vec<Operation>
// used by both the VM and the disassembler
pub fn decode_all(code: &[u8]) -> Result<Vec<Operation>, String> {
    let mut ops = Vec::new();
    let mut pos = 0;

    while pos < code.len() {
        let (op, size) = decode(code, pos)?;
        ops.push(op);
        pos += size;

        if let Operation::Halt = op {
            break;
        }
    }

    Ok(ops)
}