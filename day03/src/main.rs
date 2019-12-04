use clap::{App, Arg};
use common::{error, Res};
use std::collections::HashSet;
use std::fs;

type Position = (i32, i32);

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str<S: Into<String>>(dir_str: S) -> Res<Self> {
        let dir_str = dir_str.into();
        match dir_str.as_ref() {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => error(format!("Invalid direction: {}", dir_str)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Span {
    direction: Direction,
    dist: i32,
}

impl Span {
    fn from_str<S: Into<String>>(span_str: S) -> Res<Self> {
        let span_str: String = span_str.into();
        let mut iter = span_str.chars();
        if span_str.len() == 0 {
            return error(format!("Empty span string: {}", span_str));
        }
        let first = iter.next().unwrap().to_string();
        let rest: String = iter.collect();

        Ok(Span {
            direction: Direction::from_str(first)?,
            dist: rest.parse::<i32>()?,
        })
    }
}

fn main() -> Res<()> {
    let args = App::new("Advent of Code, Day 3")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = args.value_of("INPUT").unwrap();

    // Load and parse list of numbers.
    let contents = fs::read_to_string(filename)?;

    // Parse into spans.
    let wires: Vec<Vec<Span>> = parse_wires(contents)?;
    println!("Loaded {} wires.", wires.len());

    let intersections = all_intersections(&wires);
    println!("Intersections: {:?}", intersections);

    let closest_dist = closest_intersection(&intersections);
    println!("closest intersection manhattan dist: {}", closest_dist);

    let closest_by_wire_length = closest_length(&wires, &intersections);
    println!("closest by wire length: {}", closest_by_wire_length);

    Ok(())
}

fn parse_wires<S: Into<String>>(contents: S) -> Res<Vec<Vec<Span>>> {
    let contents = contents.into();
    let wire_defs: Vec<&str> = contents.split("\n").filter(|&s| s.len() > 0).collect();
    let mut wires: Vec<Vec<Span>> = vec![];
    for wire_def in wire_defs {
        let mut spans: Vec<Span> = vec![];
        for span_str in wire_def.split(",").filter(|&s| s.len() > 0) {
            spans.push(Span::from_str(span_str)?);
        }
        wires.push(spans);
    }
    Ok(wires)
}

fn get_positions(wire: &Vec<Span>) -> Vec<Position> {
    // Create a set of all positions "visited" by the wire.
    let mut positions = vec![];
    let mut current_position: Position = (0, 0);
    for span in wire {
        for _ in 0..span.dist {
            let new_position = match &span.direction {
                Direction::Up => (current_position.0, current_position.1 - 1),
                Direction::Down => (current_position.0, current_position.1 + 1),
                Direction::Right => (current_position.0 + 1, current_position.1),
                Direction::Left => (current_position.0 - 1, current_position.1),
            };
            positions.push(new_position);
            current_position = new_position;
        }
    }
    positions
}

fn get_positions_set(wire: &Vec<Span>) -> HashSet<Position> {
    get_positions(wire).into_iter().collect()
}

fn all_intersections(wires: &Vec<Vec<Span>>) -> HashSet<Position> {
    let sets: Vec<HashSet<Position>> = wires.iter().map(|wire| get_positions_set(&wire)).collect();

    // Now determine all common points in the sets.
    let mut common_values = sets[0].clone();
    for other in &sets[1..] {
        common_values = common_values
            .intersection(other)
            .map(|item| *item) // TODO: Why doesn't into_iter work here?
            .collect();
    }

    common_values
}

fn closest_intersection(positions: &HashSet<Position>) -> i32 {
    let mut min: i32 = std::i32::MAX;
    for pos in positions {
        let dist = manhattan_dist(pos);
        if dist < min {
            min = dist;
        }
    }
    min
}

fn manhattan_dist(position: &Position) -> i32 {
    let (x, y) = position;
    x.abs() + y.abs()
}

// Compute the shortest wire length for the given intersection.
fn closest_length(wires: &Vec<Vec<Span>>, positions: &HashSet<Position>) -> i32 {
    // Now do the same but based on wire length.
    let mut min: i32 = std::i32::MAX;

    for pos in positions {
        let mut dist: i32 = 0;
        // for each wire, compute the distance to the intersection points. we'll store those.
        for (index, wire) in wires.iter().enumerate() {
            let wire_pos = get_positions(&wire);
            println!("wire #{} has length {}", index, wire_pos.len());

            // Find the closest intersection distance.
            if let Some(m) = wire_pos.iter().position(|&item| item == *pos) {
                let m = m + 1; // Account for zero index.
                println!("Wire #{}: dist to {:?}: {:?}", index, pos, m);
                dist += m as i32;
            }
        }
        if dist < min {
            min = dist;
        }
    }

    min
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() -> Res<()> {
        let wires = parse_wires("U10,L5,R3\nD5")?;
        assert_eq!(
            wires[0],
            vec![
                Span {
                    direction: Direction::Up,
                    dist: 10
                },
                Span {
                    direction: Direction::Left,
                    dist: 5
                },
                Span {
                    direction: Direction::Right,
                    dist: 3
                }
            ]
        );
        assert_eq!(
            wires[1],
            vec![Span {
                direction: Direction::Down,
                dist: 5
            }]
        );
        Ok(())
    }

    #[test]
    fn positions_up() -> Res<()> {
        let wire = vec![Span {
            direction: Direction::Up,
            dist: 3,
        }];
        let positions = get_positions_set(&wire);
        let expected: HashSet<Position> = vec![(0, -1), (0, -2), (0, -3)].into_iter().collect();

        assert_eq!(positions, expected);
        Ok(())
    }
    #[test]
    fn positions_down() -> Res<()> {
        let wire = vec![Span {
            direction: Direction::Down,
            dist: 3,
        }];
        let positions = get_positions(&wire);
        let expected = vec![(0, 1), (0, 2), (0, 3)];

        assert_eq!(positions, expected);
        Ok(())
    }
    #[test]
    fn positions_left() -> Res<()> {
        let wire = vec![Span {
            direction: Direction::Left,
            dist: 3,
        }];
        let positions = get_positions(&wire);
        let expected = vec![(-1, 0), (-2, 0), (-3, 0)];

        assert_eq!(positions, expected);
        Ok(())
    }
    #[test]
    fn positions_right() -> Res<()> {
        let wire = vec![Span {
            direction: Direction::Right,
            dist: 3,
        }];
        let positions = get_positions(&wire);
        let expected = vec![(1, 0), (2, 0), (3, 0)];

        assert_eq!(positions, expected);
        Ok(())
    }
    #[test]
    fn positions_left_right() -> Res<()> {
        let wire = vec![
            Span {
                direction: Direction::Right,
                dist: 3,
            },
            Span {
                direction: Direction::Up,
                dist: 2,
            },
        ];
        let positions = get_positions(&wire);
        let expected = vec![(1, 0), (2, 0), (3, 0), (3, -1), (3, -2)];

        assert_eq!(positions, expected);
        Ok(())
    }

    #[test]
    fn intersections_1() -> Res<()> {
        let wires = parse_wires("R8,U5,L5,D3\nU7,R6,D4,L4")?;
        let intersections = all_intersections(&wires);
        let expected: HashSet<Position> = vec![(3, -3), (6, -5)].into_iter().collect();
        assert_eq!(intersections, expected);
        Ok(())
    }
    #[test]
    fn closest_intersection_1() -> Res<()> {
        let pos: HashSet<Position> = vec![(3, -3), (6, -5)].into_iter().collect();
        let closest_dist = closest_intersection(&pos);
        assert_eq!(closest_dist, 6);
        Ok(())
    }
    #[test]
    fn given_example_1() -> Res<()> {
        let wires = parse_wires("R8,U5,L5,D3\nU7,R6,D4,L4")?;
        let intersections = all_intersections(&wires);
        let expected: HashSet<Position> = vec![(3, -3), (6, -5)].into_iter().collect();
        assert_eq!(intersections, expected);
        let closest_by_wire_length = closest_length(&wires, &intersections);
        assert_eq!(closest_by_wire_length, 30);
        Ok(())
    }
    #[test]
    fn given_example_2() -> Res<()> {
        let wires = parse_wires(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
        )?;
        let intersections = all_intersections(&wires);
        let closest_by_wire_length = closest_length(&wires, &intersections);
        assert_eq!(closest_by_wire_length, 610);
        Ok(())
    }
    #[test]
    fn given_example_3() -> Res<()> {
        let wires = parse_wires(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )?;
        let intersections = all_intersections(&wires);
        let closest_by_wire_length = closest_length(&wires, &intersections);
        assert_eq!(closest_by_wire_length, 410);
        Ok(())
    }
}
