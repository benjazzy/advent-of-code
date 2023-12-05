use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(Default, Debug)]
struct Number {
    value: usize,
    length: usize,
}

fn get_adjacent(position: Position, length: usize) -> Vec<Position> {
    let mut adjacent = Vec::new();

    // Left
    adjacent.push(Position {
        col: position.col - 1,
        ..position
    });

    // Right
    adjacent.push(Position {
        col: position.col + length as i32,
        ..position
    });

    // Top and bottom
    for col in (position.col - 1)..=(length as i32 + position.col) {
        let row = position.row - 1;
        adjacent.push(Position { row, col });

        let row = position.row + 1;
        adjacent.push(Position { row, col });
    }

    adjacent
}

fn parse_numbers(input: &str) -> HashMap<Position, Number> {
    let mut numbers = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, number) in line
            .chars()
            .enumerate()
            .group_by(|(_, c)| c.is_ascii_digit())
            .into_iter()
            .filter_map(|(key, digit)| if key { Some(digit) } else { None })
            .map(|mut digits| {
                let col = digits.nth(0).expect("Digit should not be empty").0;

                let number = digits.fold(Number::default(), |acc, digit| {
                    let value = digit.1.to_digit(10).expect("Char should be digit");
                    Number {
                        value: acc.value * 10 + value as usize,
                        length: acc.length + 1,
                    }
                });

                (col, number)
            })
        {
            let row = row as i32;
            let col = col as i32;
            numbers.insert(Position { row, col }, number);
        }
    }

    numbers
}

fn parse_symbols(input: &str) -> HashSet<Position> {
    let mut symbols = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line
            .chars()
            .enumerate()
            .filter(|(_, c)| !(c.is_ascii_digit()) && !(*c == '.'))
        {
            let row = row as i32;
            let col = col as i32;
            symbols.insert(Position { row, col });
        }
    }

    symbols
}

pub fn process(input: &str) -> String {
    let numbers = parse_numbers(input);
    let symbols = parse_symbols(input);

    for symbol in symbols.iter() {
        println!("Symbol: {:?}", symbol);
    }

    for adjacent in get_adjacent(Position { row: 0, col: 5 }, 3) {
        if symbols.contains(&adjacent) {
            println!("114 adjacent: {:?}", adjacent);
        }
    }

    let sum: usize = numbers
        .iter()
        // .inspect(|(position, number)| {
        //     for adjacent in get_adjacent(**position, number.length) {
        //         println!(
        //             "Number: {:?}, Position: {:?}, Adjacent position: {:?}",
        //             number, position, adjacent
        //         )
        //     }
        // })
        .filter(|(position, number)| {
            get_adjacent(**position, number.length)
                .iter()
                .any(|p| symbols.contains(p))
        })
        .map(|(_, number)| number.value)
        .inspect(|num| println!("Adjacent number: {num}"))
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input));
    }

    #[test]
    fn check_line() {
        let input = "..\n1";
        assert_eq!("0", process(input));
    }

    #[test]
    fn check_adjacent() {
        let test_adjacent = &[
            Position { row: 0, col: -1 },
            Position { row: 0, col: 3 },
            Position { row: -1, col: -1 },
            Position { row: -1, col: 0 },
            Position { row: -1, col: 1 },
            Position { row: -1, col: 2 },
            Position { row: -1, col: 3 },
            Position { row: 1, col: -1 },
            Position { row: 1, col: 0 },
            Position { row: 1, col: 1 },
            Position { row: 1, col: 2 },
            Position { row: 1, col: 3 },
        ];
        let test_length = 3;

        let adjacent = get_adjacent(Position { row: 0, col: 0 }, test_length);

        assert_eq!(test_adjacent.len(), adjacent.len());

        for position in test_adjacent {
            assert!(adjacent.contains(position));
        }
    }
}
