// src/main.rs

mod isa;
mod vm;

use vm::run;
use isa::Operation;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: minivm <subcommand> [options]");
        eprintln!("  minivm run <file.tbc> [--trace]");
        std::process::exit(1);
    }

    let subcommand = &args[1];

    match subcommand.as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("Usage: minivm run <file.tbc> [--trace]");
                std::process::exit(1);
            }

            let filename = &args[2];
            let trace = args.contains(&"--trace".to_string());

            println!("would run: {} (trace={})", filename, trace);

            // temporary test program
            let program = vec![
                Operation::Push(10),
                Operation::Push(20),
                Operation::Add,
                Operation::Print,
                Operation::Halt,
            ];

            if let Err(err) = run(program, trace) {
                eprintln!("{}", err);
            }
        }

        "asm" => {
            println!("assembler not built yet");
        }

        "dis" => {
            println!("disassembler not built yet");
        }

        _ => {
            eprintln!("unknown subcommand: {}", subcommand);
            eprintln!("valid subcommands: run, asm, dis");
            std::process::exit(1);
        }
    }
}