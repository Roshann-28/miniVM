mod isa;
mod vm;
mod bytecode;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: minivm <subcommand> [options]");
        eprintln!("  minivm run <file.tbc> [--trace]");
        std::process::exit(1);
    }

    let subcommand = &args[1];

    match subcommand.as_str() {
        "run" => cmd_run(&args),

        "asm" => println!("assembler not built yet"),
        "dis" => println!("disassembler not built yet"),
        _ => {
            eprintln!("unknown subcommand: {}", subcommand);
            std::process::exit(1);
        }
    }
}

fn cmd_run(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: minivm run <file.tbc> [--trace]");
        std::process::exit(1);
    }

    let filename = &args[2];
    let trace = args.contains(&"--trace".to_string());

    let file_bytes = std::fs::read(filename).unwrap_or_else(|e| {
        eprintln!("error: could not read '{}': {}", filename, e);
        std::process::exit(1);
    });

    let code = bytecode::read_bytecode(&file_bytes).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        std::process::exit(1);
    });

    let program = bytecode::decode_all(&code).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    if let Err(e) = vm::run(program, trace) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}