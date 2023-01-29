use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process, u8};

#[derive(Debug)]
enum Value {
    Literal(u32),
    Label(String),
}

#[derive(Debug, Default)]
enum Instruction {
    Add {
        d: u8,
        s: u8,
        t: u8,
    },
    Sub {
        d: u8,
        s: u8,
        t: u8,
    },
    Slt {
        d: u8,
        s: u8,
        t: u8,
    },
    Sltu {
        d: u8,
        s: u8,
        t: u8,
    },
    Mult {
        s: u8,
        t: u8,
    },
    Multu {
        s: u8,
        t: u8,
    },
    Div {
        s: u8,
        t: u8,
    },
    Divu {
        s: u8,
        t: u8,
    },
    Mfhi {
        d: u8,
    },
    Mflo {
        d: u8,
    },
    Lis {
        d: u8,
    },
    Lw {
        t: u8,
        i: Value,
        s: u8,
    },
    Sw {
        t: u8,
        i: Value,
        s: u8,
    },
    Beq {
        s: u8,
        t: u8,
        i: Value,
    },
    Bne {
        s: u8,
        t: u8,
        i: Value,
    },
    Jr {
        s: u8,
    },
    Jalr {
        s: u8,
    },
    Word {
        i: Value,
    },
    #[default]
    Noop,
}

#[derive(Debug, Default)]
struct Line {
    text: String,
    labels: Vec<String>,
    instruction: Instruction,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_value(value: &str) -> Value {
    match value.parse::<u32>() {
        Ok(num) => Value::Literal(num),
        _ => match value.parse::<i32>() {
            Ok(num) => Value::Literal(num as u32),
            _ => Value::Label(value.to_string()),
        },
    }
}

fn parse_instruction(instruction: String) -> Instruction {
    let tokens: Vec<&str> = instruction
        .split([' ', ',', '(', ')'])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    match tokens.first() {
        None => Instruction::Noop,
        Some(&"add") => Instruction::Add {
            d: tokens[1][1..].parse().unwrap(),
            s: tokens[2][1..].parse().unwrap(),
            t: tokens[3][1..].parse().unwrap(),
        },
        Some(&"sub") => Instruction::Sub {
            d: tokens[1][1..].parse().unwrap(),
            s: tokens[2][1..].parse().unwrap(),
            t: tokens[3][1..].parse().unwrap(),
        },
        Some(&"slt") => Instruction::Slt {
            d: tokens[1][1..].parse().unwrap(),
            s: tokens[2][1..].parse().unwrap(),
            t: tokens[3][1..].parse().unwrap(),
        },
        Some(&"sltu") => Instruction::Sltu {
            d: tokens[1][1..].parse().unwrap(),
            s: tokens[2][1..].parse().unwrap(),
            t: tokens[3][1..].parse().unwrap(),
        },
        Some(&"mult") => Instruction::Mult {
            s: tokens[1][1..].parse().unwrap(),
            t: tokens[2][1..].parse().unwrap(),
        },
        Some(&"multu") => Instruction::Multu {
            s: tokens[1][1..].parse().unwrap(),
            t: tokens[2][1..].parse().unwrap(),
        },
        Some(&"div") => Instruction::Div {
            s: tokens[1][1..].parse().unwrap(),
            t: tokens[2][1..].parse().unwrap(),
        },
        Some(&"divu") => Instruction::Divu {
            s: tokens[1][1..].parse().unwrap(),
            t: tokens[2][1..].parse().unwrap(),
        },
        Some(&"mfhi") => Instruction::Mfhi {
            d: tokens[1][1..].parse().unwrap(),
        },
        Some(&"mflo") => Instruction::Mflo {
            d: tokens[1][1..].parse().unwrap(),
        },
        Some(&"lis") => Instruction::Lis {
            d: tokens[1][1..].parse().unwrap(),
        },
        Some(&"lw") => Instruction::Lw {
            t: tokens[1][1..].parse().unwrap(),
            i: parse_value(tokens[2]),
            s: tokens[3][1..].parse().unwrap(),
        },
        Some(&"sw") => Instruction::Sw {
            t: tokens[1][1..].parse().unwrap(),
            i: parse_value(tokens[2]),
            s: tokens[3][1..].parse().unwrap(),
        },
        Some(&"beq") => Instruction::Beq {
            s: tokens[1][1..].parse().unwrap(),
            t: tokens[2][1..].parse().unwrap(),
            i: parse_value(tokens[3]),
        },
        Some(&"bne") => Instruction::Bne {
            s: tokens[1][1..].parse().unwrap(),
            t: tokens[2][1..].parse().unwrap(),
            i: parse_value(tokens[3]),
        },
        Some(&"jr") => Instruction::Jr {
            s: tokens[1][1..].parse().unwrap(),
        },
        Some(&"jalr") => Instruction::Jr {
            s: tokens[1][1..].parse().unwrap(),
        },
        Some(&".word") => Instruction::Word {
            i: parse_value(tokens[1]),
        },
        Some(other) => panic!("Unrecognized instruction opcode: {other}"),
    }
}

fn parse_line(line: String) -> Line {
    lazy_static! {
        static ref LABELS_RE: Regex = Regex::new(r"[a-zA-Z][a-zA-Z0-9]*:").unwrap();
    }
    let semicolon_index = line.find(';').unwrap_or(line.len());
    let line = &line[..semicolon_index].trim();

    let last_colon_index = line.rfind(':').map(|x| x + 1).unwrap_or(0);
    let labels = &line[..last_colon_index].trim();
    let instruction = &line[last_colon_index..].trim();

    let labels: Vec<String> = LABELS_RE
        .find_iter(labels)
        .map(|s| s.as_str().to_string())
        .collect();

    println!();
    Line {
        text: line.to_string(),
        labels,
        instruction: parse_instruction(instruction.to_string()),
    }
}

fn parse_lines(lines: io::Lines<io::BufReader<File>>) -> Vec<Line> {
    lines.flatten().map(parse_line).collect()
}

fn assemble_file(mips_file: &String) {
    println!("Assembling file {mips_file}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Pass a MIPS assembly file");
        process::exit(1);
    }

    let mips_file = &args[1];
    let lines = read_lines(mips_file).expect("Could not open MIPS file");
    let lines = parse_lines(lines);

    dbg!(lines);
}
