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
