# miniVM

A stack-based bytecode virtual machine written in Rust. Write programs in a custom assembly language, compile them to binary bytecode, and execute them on a stack machine — all in one binary with three subcommands: `asm`, `run`, and `dis`.

Approach
I built this incrementally, starting from the smallest possible working piece and growing it outward, rather than trying to write the full pipeline at once.

1. Started with a hardcoded stack machine

Before touching files or bytecode, I wrote the simplest possible version: a Vec<i64> as the stack, with a fixed sequence of operations (push 7, push 3, add, print) written directly in Rust. This proved the core idea — a stack where you push, pop, and operate on the top values — before adding any complexity.

2. Represented instructions as data

Next, I introduced the Op enum (Push, Add, Print, etc.) and rewrote the hardcoded sequence as a Vec<Op> that a loop executes via match. This turned "code written in Rust" into "data my program interprets" — the core idea behind any VM.

3. Added an instruction pointer (ip)

Replaced the `for` loop with a `while` loop using an `ip` (instruction pointer) —
a `usize` index that tracks which instruction is currently executing. This is the
foundation of the fetch-decode-execute cycle that every real VM uses:

- **fetch** — read `program[ip]`
- **execute** — run it via `match`
- **advance** — `ip += 1`

Added `Halt` as an explicit instruction to stop the loop cleanly, since every real
program needs a defined stopping point. Without it, the VM would just fall off the
end of the program silently.

4. Added the full ISA (Instruction Set Architecture)

Expanded the `Operation` enum to include all 14 instructions from the spec, grouped by purpose:

- **Stack manipulation** — `Push`, `Pop`, `Dup`, `Swap`
- **Arithmetic** — `Add`, `Sub`, `Mul`, `Div`, `Mod`, `Neg`
- **I/O and control** — `Print`, `Halt`

Each instruction is handled in the `match` block inside the execute loop. At this point the VM can run any straight-line arithmetic program — the program is still written directly as a `Vec<Operation>` in Rust, but the machine itself is complete.

5. Added 256 global slots (Load and Store)

Added a `[i64; 256]` array called `globals`, zero-initialized, alongside the stack.
Two new instructions access it:

- **`Store(slot)`** — pops a value off the stack and saves it into `globals[slot]`
- **`Load(slot)`** — reads a value from `globals[slot]` and pushes it onto the stack

This gives programs a way to save and reuse values — like variables. The slot number
is a `u8` (0–255) because the spec allows exactly 256 slots and `u8` naturally
enforces that range without any extra checks.

6. Added trap handling with Result

Replaced all `.unwrap()` calls with proper error handling using Rust's `Result` type.
The `run()` function now returns `Result<(), String>` — `Ok(())` on success, or
`Err(String)` carrying a formatted trap message on failure.

A small helper `trap_err()` builds the message in the exact format the spec requires:
`trap at ip=0x0003: stack underflow (Pop on empty stack)`

Traps covered:

- **Stack overflow** — pushing onto a full stack (max 1024)
- **Stack underflow** — popping/operating on too few values
- **Division by zero** — `Div` or `Mod` with b=0

6. Added trap handling with Result

Replaced all `.unwrap()` calls with proper error handling using Rust's `Result` type.
The `run()` function now returns `Result<(), String>` — `Ok(())` on success, or
`Err(String)` carrying a formatted trap message on failure.

A small helper `trap_err()` builds the message in the exact format the spec requires:
`trap at ip=0x0003: stack underflow (Pop on empty stack)`

Traps covered:

- **Stack overflow** — pushing onto a full stack (max 1024)
- **Stack underflow** — popping/operating on too few values
- **Division by zero** — `Div` or `Mod` with b=0

`main()` handles the result — if `run()` returns an `Err`, it prints to stderr and
exits with code 1, as the spec requires.

7. Added --trace mode

Added a `trace: bool` parameter to `run()`. When enabled, it prints `ip`, the current
instruction, and the stack contents _before_ every instruction executes — exactly as
the spec requires for `minivm run --trace`.

To make the trace output read like real assembly instead of Rust's debug format,
implemented the `Display` trait for `Operation` — so `Push(7)` prints as `PUSH 7`,
`Add` prints as `ADD`, and so on. This isn't just cosmetic: the disassembler
(`minivm dis`) will need this exact same instruction-to-text logic, so building it
now means it's reused later instead of duplicated.

Example trace output:
\`\`\`
ip=0x0000 PUSH 7 stack=[]
ip=0x0001 PUSH 3 stack=[7]
ip=0x0002 ADD stack=[7, 3]
ip=0x0003 PRINT stack=[10]
\`\`\`
