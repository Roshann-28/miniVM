mod isa;
mod vm;
mod bytecode;
mod asm;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: minivm <subcommand> [options]");
        eprintln!("  minivm run <file.tbc> [--trace]");
        eprintln!("  minivm asm <file.tasm> -o <file.tbc>");
        eprintln!("  minivm dis <file.tbc>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "run" => cmd_run(&args),
        "asm" => cmd_asm(&args),
        "dis" => println!("disassembler not built yet"),
        _ => {
            eprintln!("unknown subcommand: {}", args[1]);
            std::process::exit(1);
        }
    }
}

fn cmd_asm(args: &[String]) {
    // minivm asm <file.tasm> -o <file.tbc>
    if args.len() < 5 || args[3] != "-o" {
        eprintln!("Usage: minivm asm <file.tasm> -o <file.tbc>");
        std::process::exit(1);
    }

    let input_file = &args[2];   // the .tasm file to read
    let output_file = &args[4];  // the .tbc file to write

    // read the .tasm source text
    let source = std::fs::read_to_string(input_file).unwrap_or_else(|e| {
        eprintln!("error: could not read '{}': {}", input_file, e);
        std::process::exit(1);
    });

    // assemble it into bytes
    let bytes = asm::assemble(&source).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        std::process::exit(1);
    });

    // write the .tbc file to disk
    std::fs::write(output_file, bytes).unwrap_or_else(|e| {
        eprintln!("error: could not write '{}': {}", output_file, e);
        std::process::exit(1);
    });

    println!("assembled '{}' -> '{}'", input_file, output_file);
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