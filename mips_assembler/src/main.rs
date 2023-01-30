use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::{env, process, u8};

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Literal(u32),
    Label(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Value::Literal(val) => write!(f, "{}", val as i32),
            Value::Label(ref s) => write!(f, "{s}"),
        }
    }
}

impl Value {
    fn to_u32(&self) -> u32 {
        match *self {
            Value::Literal(val) => val,
            Value::Label(ref _s) => panic!("Can't convert label to u32"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Instruction::Add { d, s, t } => write!(f, "add ${d}, ${s}, ${t}"),
            Instruction::Sub { d, s, t } => write!(f, "sub ${d}, ${s}, ${t}"),
            Instruction::Slt { d, s, t } => write!(f, "slt ${d}, ${s}, ${t}"),
            Instruction::Sltu { d, s, t } => write!(f, "sltu ${d}, ${s}, ${t}"),
            Instruction::Mult { s, t } => write!(f, "mult ${s}, ${t}"),
            Instruction::Multu { s, t } => write!(f, "multu ${s}, ${t}"),
            Instruction::Div { s, t } => write!(f, "div ${s}, ${t}"),
            Instruction::Divu { s, t } => write!(f, "divu ${s}, ${t}"),
            Instruction::Mfhi { d } => write!(f, "mfhi ${d}"),
            Instruction::Mflo { d } => write!(f, "mflo ${d}"),
            Instruction::Lis { d } => write!(f, "lis ${d}"),
            Instruction::Lw { t, ref i, s } => write!(f, "lw ${t}, {i}(${s})"),
            Instruction::Sw { t, ref i, s } => write!(f, "sw ${t}, {i}(${s})"),
            Instruction::Beq { s, t, ref i } => write!(f, "beq ${s}, ${t}, {i}"),
            Instruction::Bne { s, t, ref i } => write!(f, "bne ${s}, ${t}, {i}"),
            Instruction::Jr { s } => write!(f, "jr ${s}"),
            Instruction::Jalr { s } => write!(f, "jalr ${s}"),
            Instruction::Word { ref i } => write!(f, ".word {i}"),
            Instruction::Noop => write!(f, ""),
        }
    }
}

fn std_word(s: u8, t: u8, d: u8, opcode: u16) -> u32 {
    ((s as u32) << 21) | ((t as u32) << 16) | ((d as u32) << 11) | (opcode as u32)
}
fn sti_word(opcode: u8, s: u8, t: u8, i: u32) -> u32 {
    ((opcode as u32) << 26) | ((s as u32) << 21) | ((t as u32) << 16) | (i & 0xFFFF)
}

impl Instruction {
    fn assemble(&self) -> u32 {
        match *self {
            Instruction::Add { d, s, t } => std_word(s, t, d, 0x20),
            Instruction::Sub { d, s, t } => std_word(s, t, d, 0x22),
            Instruction::Slt { d, s, t } => std_word(s, t, d, 0x2a),
            Instruction::Sltu { d, s, t } => std_word(s, t, d, 0x2b),
            Instruction::Mult { s, t } => std_word(s, t, 0, 0x18),
            Instruction::Multu { s, t } => std_word(s, t, 0, 0x19),
            Instruction::Div { s, t } => std_word(s, t, 0, 0x1a),
            Instruction::Divu { s, t } => std_word(s, t, 0, 0x1b),
            Instruction::Mfhi { d } => std_word(0, 0, d, 0x10),
            Instruction::Mflo { d } => std_word(0, 0, d, 0x12),
            Instruction::Lis { d } => std_word(0, 0, d, 0x14),
            Instruction::Lw { t, ref i, s } => sti_word(0b100011, s, t, i.to_u32()),
            Instruction::Sw { t, ref i, s } => sti_word(0b101011, s, t, i.to_u32()),
            Instruction::Beq { s, t, ref i } => sti_word(0b000100, s, t, i.to_u32()),
            Instruction::Bne { s, t, ref i } => sti_word(0b000101, s, t, i.to_u32()),
            Instruction::Jr { s } => sti_word(0b000000, s, 0, 0b1000),
            Instruction::Jalr { s } => sti_word(0b000000, s, 0, 0b1001),
            Instruction::Word { ref i } => i.to_u32(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct Line {
    text: String,
    labels: Vec<String>,
    instruction: Instruction,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ele in &self.labels {
            write!(f, "{ele} ")?;
        }
        write!(f, "{}", self.instruction)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_value(value: &str) -> Value {
    if let Ok(num) = value.parse::<u32>() {
        Value::Literal(num)
    } else if let Ok(num) = value.parse::<i32>() {
        Value::Literal(num as u32)
    } else if let Ok(num) = u32::from_str_radix(&value[2..], 16) {
        Value::Literal(num)
    } else {
        Value::Label(value.to_string())
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
        Some(&"jalr") => Instruction::Jalr {
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
    let original_line = line.trim();
    let semicolon_index = line.find(';').unwrap_or(line.len());
    let line = &line[..semicolon_index].trim();

    let last_colon_index = line.rfind(':').map(|x| x + 1).unwrap_or(0);
    let labels = &line[..last_colon_index].trim();
    let instruction = &line[last_colon_index..].trim();

    let labels: Vec<String> = LABELS_RE
        .find_iter(labels)
        .map(|s| s.as_str().to_string())
        .collect();

    Line {
        text: original_line.to_string(),
        labels,
        instruction: parse_instruction(instruction.to_string()),
    }
}

fn parse_lines(lines: io::Lines<io::BufReader<File>>) -> Vec<Line> {
    lines.flatten().map(parse_line).collect()
}

fn extract_label_locations(lines: &Vec<Line>) -> HashMap<&str, u32> {
    let mut result = HashMap::new();
    let mut addr: u32 = 0;
    for line in lines {
        for label in &line.labels {
            let label = &label[..label.len() - 1];
            if result.contains_key(label) {
                panic!("Duplicate label {label}");
            }
            result.insert(label, addr);
        }
        if line.instruction != Instruction::Noop {
            addr += 4;
        }
    }
    result
}

fn replace_labels(lines: &Vec<Line>, labels: &HashMap<&str, u32>) -> Vec<Line> {
    let mut result = Vec::new();
    let mut addr: u32 = 0;
    for line in lines {
        if line.instruction != Instruction::Noop {
            addr += 4;
        }
        let new_instruction = match &line.instruction {
            Instruction::Lw {
                t,
                i: Value::Label(label),
                s,
            } => {
                let label_value = labels
                    .get(label.as_str())
                    .unwrap_or_else(|| panic!("Undefined label {label}"));
                Instruction::Lw {
                    t: *t,
                    i: Value::Literal(*label_value),
                    s: *s,
                }
            }
            Instruction::Sw {
                t,
                i: Value::Label(label),
                s,
            } => {
                let label_value = labels
                    .get(label.as_str())
                    .unwrap_or_else(|| panic!("Undefined label {label}"));
                Instruction::Sw {
                    t: *t,
                    i: Value::Literal(*label_value),
                    s: *s,
                }
            }
            Instruction::Beq {
                s,
                t,
                i: Value::Label(label),
            } => {
                let label_value = labels
                    .get(label.as_str())
                    .unwrap_or_else(|| panic!("Undefined label {label}"));
                let offset = ((*label_value as i32 - addr as i32) / 4) as u32;
                Instruction::Beq {
                    s: *s,
                    t: *t,
                    i: Value::Literal(offset),
                }
            }
            Instruction::Bne {
                s,
                t,
                i: Value::Label(label),
            } => {
                let label_value = labels
                    .get(label.as_str())
                    .unwrap_or_else(|| panic!("Undefined label {label}"));
                let offset = ((*label_value as i32 - addr as i32) / 4) as u32;
                Instruction::Bne {
                    s: *s,
                    t: *t,
                    i: Value::Literal(offset),
                }
            }
            Instruction::Word {
                i: Value::Label(label),
            } => {
                let label_value = labels
                    .get(label.as_str())
                    .unwrap_or_else(|| panic!("Undefined label {label}"));
                Instruction::Word {
                    i: Value::Literal(*label_value),
                }
            }
            other => other.clone(),
        };
        if new_instruction != Instruction::Noop {
            result.push(Line {
                text: line.text.clone(),
                instruction: new_instruction,
                labels: Vec::new(),
            });
        }
    }
    result
}

fn assemble(instructions: &[Line]) -> Vec<u32> {
    instructions
        .iter()
        .map(|line| line.instruction.assemble())
        .collect()
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
    let label_locations = extract_label_locations(&lines);
    let lines = replace_labels(&lines, &label_locations);
    let machine_code = assemble(&lines);

    for line in lines {
        let bytes = line.instruction.assemble().to_be_bytes();
        eprintln!(
            "{:08b} {:08b} {:08b} {:08b} | {}",
            bytes[0], bytes[1], bytes[2], bytes[3], line.text
        );
    }

    let mut bytes = Vec::<u8>::new();
    for word in machine_code {
        bytes.extend_from_slice(&word.to_be_bytes())
    }
    io::stdout()
        .write_all(bytes.as_slice())
        .expect("Writing failed");
}
