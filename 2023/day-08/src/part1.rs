use std::collections::HashMap;

use itertools::{FoldWhile, Itertools};
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{alphanumeric1, newline};
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    pub fn turn(&self, turn: &Turn) -> &str {
        match turn {
            Turn::Left => &self.left,
            Turn::Right => &self.right,
        }
    }
}

pub fn parse_node_pair(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(alphanumeric1, tag(" = "), parse_node_pair)(input)
}

fn parse_map(input: &str) -> IResult<&str, HashMap<&str, Node>> {
    let (rest, lines) = separated_list1(newline, parse_line)(input)?;

    let nodes: HashMap<&str, Node> = lines
        .into_iter()
        .map(|(key, (left, right))| (key, Node { left, right }))
        .collect();

    Ok((rest, nodes))
}

fn parse(input: &str) -> IResult<&str, (Vec<Turn>, HashMap<&str, Node>)> {
    // let (rest, turns) = take_until1("\n\n")(input)?;
    let (rest, (turns, nodes)) =
        separated_pair(alphanumeric1, pair(newline, newline), parse_map)(input)?;
    let turns: Vec<Turn> = turns
        .chars()
        .map(|c| match c {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => panic!("Expected valid turn direction got {c}"),
        })
        .collect();

    Ok((rest, (turns, nodes)))
}

fn process_loop(turns: Vec<Turn>, nodes: HashMap<&str, Node>) -> u64 {
    let mut next = "AAA";
    let mut count = 1_u64;
    loop {
        let result = turns
            .iter()
            .fold_while((count, next), |(count, prev), turn| {
                let next = nodes
                    .get(prev)
                    .expect("Node was not found in map")
                    .turn(&turn);

                // println!("Turn: {:?}, {prev} maps to {next}", turn);

                if next == "ZZZ" {
                    FoldWhile::Done((count, prev))
                } else {
                    FoldWhile::Continue((count + 1, next))
                }
            });

        match result {
            FoldWhile::Continue((c, n)) => {
                next = n;
                count = c;
            }
            FoldWhile::Done((c, _)) => return c,
        }
    }
}

pub fn process(input: &str) -> String {
    let (_, (turns, nodes)) = parse(input).expect("Input should be valid");

    println!("\nTurns: {:?}\n\nNodes: {:?}", turns, nodes);

    let count = process_loop(turns, nodes);

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input));
    }
}
