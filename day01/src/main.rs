use clap::{App, Arg};
use std::fs;

type FuelType = i64;

fn main() {
    let args = App::new("Advent of Code, Day 1")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("fuel_mass")
                .short("f")
                .help("Factor in mass of fuel."),
        )
        .get_matches();

    let filename = args.value_of("INPUT").unwrap();
    let calculate_fuel_mass = args.is_present("fuel_mass");

    // Load and parse list of numbers.
    let contents =
        fs::read_to_string(filename).expect(&format!("IO error reading file: {:?}", filename));
    let numbers: Vec<FuelType> = {
        let mut numbers: Vec<FuelType> = vec![];
        for (index, line) in contents.trim().split("\n").enumerate() {
            if let Ok(parsed) = line.parse::<FuelType>() {
                numbers.push(parsed);
            } else {
                println!("Error on line {}: could not parse \"{}\"", index, line);
                std::process::exit(1);
            }
        }
        numbers
    };
    println!("Loaded {} numbers.", numbers.len());

    // Compute and print the result:
    let total_fuel_required = numbers
        .iter()
        .map(|mass| {
            if calculate_fuel_mass {
                fuel_required_with_fuel_weight(*mass)
            } else {
                fuel_required(*mass)
            }
        })
        .fold(0, |s, i| s + i);
    println!("Total fuel required: {}", total_fuel_required);
}

fn fuel_required(mass: FuelType) -> FuelType {
    (mass as f64 / 3.0) as FuelType - 2
}

fn fuel_required_with_fuel_weight(mass: FuelType) -> FuelType {
    // Compute the base fuel.
    let fuel = fuel_required(mass);

    // And the fuel required for that.
    let mut fuel_delta = fuel_required(fuel);
    let mut net_fuel_delta: FuelType = 0;

    while fuel_delta > 0 {
        net_fuel_delta += fuel_delta;
        fuel_delta = fuel_required(fuel_delta);
    }
    fuel + net_fuel_delta
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_examples_part_one() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn given_examples_part_two() {
        assert_eq!(fuel_required_with_fuel_weight(14), 2);
        assert_eq!(fuel_required_with_fuel_weight(1969), 966);
        assert_eq!(fuel_required_with_fuel_weight(100756), 50346);
    }
}
