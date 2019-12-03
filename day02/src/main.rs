use clap::{App, Arg};
use common::{error, Res};
use std::fs;

type IntCode = i32;

/*
 * Validate args, parse input, and run program.
 */
fn main() -> Res<()> {
    let args = App::new("Advent of Code, Day 2")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = args.value_of("INPUT").unwrap();

    // Load and parse list of numbers.
    let contents =
        fs::read_to_string(filename).expect(&format!("IO error reading file: {:?}", filename));

    let mut intcodes: Vec<IntCode> = {
        let mut intcodes: Vec<IntCode> = vec![];
        for (index, line) in contents.trim().split(",").enumerate() {
            if let Ok(parsed) = line.parse::<IntCode>() {
                intcodes.push(parsed);
            } else {
                println!("Error on integer #{}: could not parse \"{}\"", index, line);
                std::process::exit(1);
            }
        }
        intcodes
    };
    println!("Loaded {} intcodes.", intcodes.len());

    // Replace values for 1202 state:
    intcodes[1] = 12;
    intcodes[2] = 2;

    run(&mut intcodes)?;

    println!("Value at position 0: {}", intcodes[0]);

    Ok(())
}

/// Run the given IntCode program.
fn run(mut intcodes: &mut Vec<IntCode>) -> Res<()> {
    let mut pc = 0;

    while pc < intcodes.len() {
        let op_code = intcodes[pc];
        match op_code {
            1 => {
                let (src0, src1, dst_addr) = get_operands_and_dst(&mut intcodes, pc)?;
                intcodes[dst_addr] = src0 + src1;
            }
            2 => {
                let (src0, src1, dst_addr) = get_operands_and_dst(&mut intcodes, pc)?;
                intcodes[dst_addr] = src0 * src1;
            }
            99 => {
                // Reached the end; exit.
                break;
            }
            _ => {
                return error(&format!("Invalid opcode: {}", op_code));
            }
        }
        pc += 4;
    }
    Ok(())
}

/// Get the two operands and destination address for the given program counter.
fn get_operands_and_dst(intcodes: &mut Vec<IntCode>, pc: usize) -> Res<(IntCode, IntCode, usize)> {
    let src0_addr = intcodes[pc + 1] as usize;
    let src1_addr = intcodes[pc + 2] as usize;
    if src0_addr >= intcodes.len() {
        return error(&format!("Invalid src address: {}", src0_addr));
    }
    if src1_addr >= intcodes.len() {
        return error(&format!("Invalid src address: {}", src1_addr));
    }
    let src0 = intcodes[intcodes[pc + 1] as usize];
    let src1 = intcodes[intcodes[pc + 2] as usize];
    let dst_addr = intcodes[pc + 3] as usize;
    if dst_addr >= intcodes.len() {
        return error(&format!("Invalid dest address: {}", dst_addr));
    }
    Ok((src0, src1, dst_addr))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_example_1() {
        let mut intcodes: Vec<IntCode> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run(&mut intcodes).unwrap();
        assert_eq!(intcodes[0], 3500);
    }
    #[test]
    fn given_example_2() {
        let mut intcodes: Vec<IntCode> = vec![1, 0, 0, 0, 99];
        run(&mut intcodes).unwrap();
        assert_eq!(intcodes[0], 2);
    }
    #[test]
    fn given_example_3() {
        let mut intcodes: Vec<IntCode> = vec![2, 3, 0, 3, 99];
        run(&mut intcodes).unwrap();
        assert_eq!(intcodes[3], 6);
    }
    #[test]
    fn given_example_4() {
        let mut intcodes: Vec<IntCode> = vec![2, 4, 4, 5, 99, 0];
        run(&mut intcodes).unwrap();
        assert_eq!(intcodes[5], 9801);
    }
    #[test]
    fn given_example_5() {
        let mut intcodes: Vec<IntCode> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut intcodes).unwrap();
        assert_eq!(intcodes[0], 30);
        assert_eq!(intcodes[4], 2);
    }
}
