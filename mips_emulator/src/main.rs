use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please run mips_emulator with a single argument containing the MIPS object code");
        process::exit(1);
    }

    let object_file = &args[1];
    println!("Running MIPS object file {object_file}");

    // TODO: Load in the MIPS object file, write driver code, emulate everything
}
