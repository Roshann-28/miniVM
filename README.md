# miniVM

A stack-based bytecode virtual machine written in Rust. Write programs in a custom assembly language, compile them to binary bytecode, and execute them on a stack machine — all in one binary with three subcommands: `asm`, `run`, and `dis`.

## Approach

Built incrementally — starting from the smallest working piece and growing it outward.

**1. Hardcoded stack machine**
A `Vec<i64>` as the stack, with a fixed sequence (`push 7, push 3, add, print`) written directly in Rust. Proved the core idea before adding any complexity.

**2. Instructions as data**
Introduced the `Operation` enum and rewrote the hardcoded sequence as a `Vec<Operation>` executed by a loop via `match`. Code became data the program interprets.

**3. Instruction pointer (ip)**
Replaced the `for` loop with a `while` loop using `ip`, implementing the fetch → execute → advance cycle every real VM uses. Added `Halt` as an explicit stopping point.

**4. Full ISA**
Expanded `Operation` to all 14 instructions from the spec — stack manipulation (`Push/Pop/Dup/Swap`), arithmetic (`Add/Sub/Mul/Div/Mod/Neg`), and I/O/control (`Print/Halt`).

**5. Global slots (Load/Store)**
Added a `[i64; 256]` zero-initialized `globals` array. `Store(slot)` pops into a slot, `Load(slot)` pushes from a slot — gives programs variable-like storage.

**6. Trap handling with Result**
Replaced `.unwrap()` calls with `Result<(), String>`. A `trap_err()` helper builds messages in the spec's exact format (`trap at ip=0x0003: stack underflow (Pop on empty stack)`). Covers stack overflow/underflow and division/modulo by zero. `main()` prints the error and exits with code 1 on failure.

**7. --trace mode**
Added a `trace: bool` parameter to `run()`, printing `ip`, the instruction, and the stack before every step. Added an `op_to_string()` function to render instructions as real assembly text (`PUSH 7` instead of `Push(7)`) — reused directly by the disassembler.

Example:
ip=0x0000 PUSH 7 stack=[]
ip=0x0001 PUSH 3 stack=[7]
ip=0x0002 ADD stack=[7, 3]
ip=0x0003 PRINT stack=[10]

**8. CLI argument parsing**
Real command-line argument parsing using `std::env::args()`. Three subcommands:

- `minivm run <file.tbc> [--trace]` — runs a bytecode file
- `minivm asm <file.tasm> -o <file.tbc>` — assembles text to bytecode
- `minivm dis <file.tbc>` — disassembles bytecode back to text

**9. File format (`bytecode.rs`)**
Implemented the `.tbc` binary format: magic bytes `MVM\0`, version `0x01`, code length as `u32` LE, then raw code. `write_bytecode()` encodes a program and wraps it in the header. `read_bytecode()` validates the header and returns the code bytes. `decode_all()` walks the code bytes and returns a `Vec<Operation>`.

**10. Assembler (`minivm asm`)**
Single-pass assembler in `asm.rs`. Reads `.tasm` files line by line, strips `;` comments, skips blank lines, matches mnemonics case-insensitively, and calls `encode()` from `isa.rs`. Line-numbered error messages for unknown mnemonics or bad operands. Warns if the program doesn't end with `HALT`.

**11. Disassembler (`minivm dis`)**
`dis.rs` reuses `decode_all()` from `bytecode.rs` and `op_to_string()` from `isa.rs` to walk the bytecode and print each instruction as assembly text. Round-trip verified: `asm → dis → asm` output matches the original `.tasm` for `arith.tasm`.

## Test programs

| File         | Expression              | Output |
| ------------ | ----------------------- | ------ |
| `arith.tasm` | `(7 + 3) * (9 - 4) / 5` | `10`   |
