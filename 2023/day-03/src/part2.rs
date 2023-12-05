use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(Default, Debug, Clone, Copy)]
struct Number {
    id: usize,
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
    let mut next_id = 0;

    for (row, line) in input.lines().enumerate() {
        line.chars()
            .enumerate()
            .group_by(|(_, c)| c.is_ascii_digit())
            .into_iter()
            .filter_map(|(key, digit)| if key { Some(digit) } else { None })
            .for_each(|mut digits| {
                let digits = digits.collect_vec();
                let start_col = digits.get(0).expect("Digits should not be empty").0;

                let id = next_id;
                next_id += 1;

                let (length, value) = digits.iter().fold((1, 0), |acc, digit| {
                    (
                        acc.0 + 1,
                        (acc.1 * 10) + digit.1.to_digit(10).expect("A valid digit"),
                    )
                });
                let number = Number {
                    length,
                    value: value as usize,
                    id,
                };

                let row = row as i32;

                for (col, _) in digits.iter() {
                    let col = *col as i32;
                    numbers.insert(Position { row, col }, number);
                }
            })
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

fn parse_gears(input: &str) -> HashSet<Position> {
    let mut symbols = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate().filter(|(_, c)| *c == '*') {
            let row = row as i32;
            let col = col as i32;
            symbols.insert(Position { row, col });
        }
    }

    symbols
}

pub fn process(input: &str) -> String {
    let numbers = parse_numbers(input);
    println!("Numbers: {:?}", numbers);
    let gears = parse_gears(input);

    let sum: usize = gears
        .iter()
        .map(|gear_pos| {
            let adjacent = get_adjacent(*gear_pos, 1);
            let values = adjacent
                .iter()
                .filter_map(|a| numbers.get(a))
                .unique_by(|n| n.id)
                .map(|n| n.value)
                .collect_vec();
            if values.len() == 2 {
                values.iter().product()
            } else {
                0
            }
        })
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
        assert_eq!("467835", process(input));
    }

    #[test]
    fn check_small() {
        let input = "2*2";
        assert_eq!("4", process(input))
    }
}
