use clap::{App, Arg};
use common::{error, Res};
use std::fs;

type IntCode = i32;

/*
 * Validate args, parse input, and run program.
 */
fn main() -> Res<()> {
    let args = App::new("Advent of Code, Day 5")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("inputs")
                .help("Comma-separated list of inputs.")
                .required(true),
        )
        .get_matches();

    let filename = args.value_of("INPUT").unwrap();
    let inputs = args.value_of("inputs").unwrap_or("");

    // Load and parse list of numbers.
    let contents = fs::read_to_string(filename)?;

    let intcodes: Vec<IntCode> = parse(contents)?;
    println!("Loaded {} intcodes.", intcodes.len());
    let inputs: Vec<IntCode> = parse(inputs)?;
    let mut intcodes = intcodes.clone();
    println!("outputs: {:?}", run(&mut intcodes, inputs));

    Ok(())
}

fn parse<S: Into<String>>(contents: S) -> Res<Vec<IntCode>> {
    let contents = contents.into();
    let contents = contents.trim();
    if contents.len() == 0 {
        return Ok(vec![]);
    }
    let mut intcodes: Vec<IntCode> = vec![];
    for (index, line) in contents.split(",").enumerate() {
        if let Ok(parsed) = line.parse::<IntCode>() {
            intcodes.push(parsed);
        } else {
            return error(format!(
                "Error on integer #{}: could not parse \"{}\"",
                index, line
            ));
        }
    }
    Ok(intcodes)
}

#[derive(Clone, Copy, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from_int(value: i32) -> Res<ParameterMode> {
        match value {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => error("Unsupported parameter mode."),
        }
    }
}

/// Run the given IntCode program.
fn run(mut intcodes: &mut Vec<IntCode>, inputs: Vec<IntCode>) -> Res<Vec<IntCode>> {
    let mut pc = 0;

    let mut inputs = inputs.clone();
    let mut outputs = vec![];

    while pc < intcodes.len() {
        let instruction = intcodes[pc];
        let op_code = instruction % 100;
        let parameter_mode0 = ParameterMode::from_int((instruction / 100) % 10)?;
        let parameter_mode1 = ParameterMode::from_int((instruction / 1000) % 10)?;
        // Unused: let parameter_mode2 = ParameterMode::from_int((instruction / 10000) % 10);
        let modes = (parameter_mode0, parameter_mode1);
        match op_code {
            // Add
            1 => {
                let (src0, src1, dst_addr) = get_two_operands_and_dst(&mut intcodes, pc, modes)?;
                intcodes[dst_addr] = src0 + src1;
                pc += 4;
            }
            // Multiply
            2 => {
                let (src0, src1, dst_addr) = get_two_operands_and_dst(&mut intcodes, pc, modes)?;
                intcodes[dst_addr] = src0 * src1;
                pc += 4;
            }
            // Input
            3 => {
                let next_item: Vec<_> = inputs.drain(0..1).collect();
                let inp = next_item[0];
                println!("input {}", inp);
                let dst_addr = intcodes[pc + 1] as usize;
                intcodes[dst_addr] = inp;
                pc += 2;
            }
            // Output
            4 => {
                let src_addr = intcodes[pc + 1] as usize;
                outputs.push(intcodes[src_addr]);
                pc += 2;
            }
            // Jump if True
            5 => {
                let condition = get_op(intcodes, pc + 1, parameter_mode0)?;
                let new_pc = get_op(intcodes, pc + 2, parameter_mode1)?;
                if condition != 0 {
                    pc = new_pc as usize;
                } else {
                    pc += 3;
                }
            }
            // Jump if False
            6 => {
                let condition = get_op(intcodes, pc + 1, parameter_mode0)?;
                let new_pc = get_op(intcodes, pc + 2, parameter_mode1)?;
                if condition == 0 {
                    pc = new_pc as usize;
                } else {
                    pc += 3;
                }
            }
            // Less than
            7 => {
                let (src0, src1, dst_addr) = get_two_operands_and_dst(&mut intcodes, pc, modes)?;
                intcodes[dst_addr] = if src0 < src1 { 1 } else { 0 };
                pc += 4;
            }
            // Equals
            8 => {
                let (src0, src1, dst_addr) = get_two_operands_and_dst(&mut intcodes, pc, modes)?;
                intcodes[dst_addr] = if src0 == src1 { 1 } else { 0 };
                pc += 4;
            }
            99 => {
                // Reached the end; exit.
                break;
            }
            _ => {
                return error(format!("Invalid opcode: {}", op_code));
            }
        }
    }
    Ok(outputs)
}

/// Get the two operands and destination address for the given program counter.
fn get_two_operands_and_dst(
    intcodes: &mut Vec<IntCode>,
    pc: usize,
    modes: (ParameterMode, ParameterMode),
) -> Res<(IntCode, IntCode, usize)> {
    let (mode0, mode1) = modes;
    let src0 = get_op(intcodes, pc + 1, mode0)?;
    let src1 = get_op(intcodes, pc + 2, mode1)?;
    let dst_addr = intcodes[pc + 3] as usize;
    if dst_addr >= intcodes.len() {
        return error(format!("Invalid dest address: {}", dst_addr));
    }
    Ok((src0, src1, dst_addr))
}

fn get_op(intcodes: &mut Vec<IntCode>, addr: usize, mode: ParameterMode) -> Res<IntCode> {
    let value = intcodes[addr];

    match mode {
        ParameterMode::Position => {
            let value = value as usize;
            if value >= intcodes.len() {
                return error(format!("Invalid src address: {}", value));
            }
            Ok(intcodes[value])
        }
        ParameterMode::Immediate => Ok(value),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_example_1() {
        let mut intcodes: Vec<IntCode> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run(&mut intcodes, vec![]).unwrap();
        assert_eq!(intcodes[0], 3500);
    }
    #[test]
    fn given_example_2() {
        let mut intcodes: Vec<IntCode> = vec![1, 0, 0, 0, 99];
        run(&mut intcodes, vec![]).unwrap();
        assert_eq!(intcodes[0], 2);
    }
    #[test]
    fn given_example_3() {
        let mut intcodes: Vec<IntCode> = vec![2, 3, 0, 3, 99];
        run(&mut intcodes, vec![]).unwrap();
        assert_eq!(intcodes[3], 6);
    }
    #[test]
    fn given_example_4() {
        let mut intcodes: Vec<IntCode> = vec![2, 4, 4, 5, 99, 0];
        run(&mut intcodes, vec![]).unwrap();
        assert_eq!(intcodes[5], 9801);
    }
    #[test]
    fn given_example_5() {
        let mut intcodes: Vec<IntCode> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut intcodes, vec![]).unwrap();
        assert_eq!(intcodes[0], 30);
        assert_eq!(intcodes[4], 2);
    }

    #[test]
    fn cmp_equal_to_8_pos() -> Res<()> {
        let mut intcodes: Vec<IntCode> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run(&mut intcodes, vec![1])?, vec![0]);
        assert_eq!(run(&mut intcodes, vec![8])?, vec![1]);
        Ok(())
    }

    #[test]
    fn cmp_less_than_8_pos() -> Res<()> {
        let mut intcodes: Vec<IntCode> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run(&mut intcodes, vec![1])?, vec![1]);
        assert_eq!(run(&mut intcodes, vec![7])?, vec![1]);
        assert_eq!(run(&mut intcodes, vec![8])?, vec![0]);
        assert_eq!(run(&mut intcodes, vec![20])?, vec![0]);
        Ok(())
    }
    #[test]
    fn cmp_equal_to_8_imm() -> Res<()> {
        let mut intcodes: Vec<IntCode> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(run(&mut intcodes, vec![1])?, vec![0]);
        assert_eq!(run(&mut intcodes, vec![8])?, vec![1]);
        Ok(())
    }

    #[test]
    fn cmp_less_than_8_imm() -> Res<()> {
        let mut intcodes: Vec<IntCode> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(run(&mut intcodes, vec![1])?, vec![1]);
        assert_eq!(run(&mut intcodes, vec![7])?, vec![1]);
        assert_eq!(run(&mut intcodes, vec![8])?, vec![0]);
        assert_eq!(run(&mut intcodes, vec![20])?, vec![0]);
        Ok(())
    }
}
