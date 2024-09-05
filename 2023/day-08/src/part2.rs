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
            Turn::Left => self.left,
            Turn::Right => self.right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Route<'a> {
    turns: &'a [Turn],
    position: usize,
}

impl<'a> Route<'a> {
    pub fn new(turns: &'a [Turn]) -> Self {
        Route { turns, position: 0 }
    }

    pub fn next_turn(&mut self) -> Turn {
        let idx = self.position % self.turns.len();
        self.position += 1;

        self.turns[idx]
    }
}

impl Iterator for Route<'_> {
    type Item = Turn;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_turn())
    }
}

#[derive(Debug, Clone, Copy)]
struct Path<'a> {
    current_node: &'a str,
    map: &'a HashMap<&'a str, Node<'a>>,
}

impl<'a> Path<'a> {
    pub fn new(current_node: &'a str, map: &'a HashMap<&'a str, Node<'a>>) -> Self {
        Path { current_node, map }
    }

    pub fn is_end(&self) -> bool {
        self.current_node.ends_with("Z")
    }

    pub fn next(&mut self, turn: &Turn) {
        let next = self
            .map
            .get(self.current_node)
            .expect("Node should be valid on the map");
        self.current_node = next.turn(turn);
    }
}

#[derive(Debug)]
struct Solver<'a> {
    paths: &'a mut [Path<'a>],
    route: Route<'a>,
    count: u64,
}

impl<'a> Solver<'a> {
    pub fn new(paths: &'a mut [Path<'a>], route: Route<'a>) -> Self {
        Solver {
            paths,
            route,
            count: 0,
        }
    }
}

impl<'a> Iterator for Solver<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        let turn = self.route.next_turn();

        // for path in self.paths.iter_mut() {
        //     path.next(&turn);
        // }
        //
        // if self.paths.iter().all(|path| path.is_end()) {
        //     Some(self.count)
        // } else {
        //     None
        // }

        let mut iter = self.paths.iter_mut();

        if iter.any(|path| {
            path.next(&turn);

            !path.is_end()
        }) {
            for path in iter {
                path.next(&turn);
            }

            None
        } else {
            Some(self.count)
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

fn find_starts<'a>(nodes: &'a HashMap<&str, Node>) -> Box<[Path<'a>]> {
    nodes
        .keys()
        .filter_map(|key| {
            if key.ends_with("A") {
                Some(Path::new(key, nodes))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

pub fn process(input: &str) -> String {
    let (_, (turns, nodes)) = parse(input).expect("Input should be valid");

    println!("\nTurns: {:?}\n\nNodes: {:?}", turns, nodes);

    let route = Route::new(turns.as_ref());
    let mut paths = find_starts(&nodes);

    let mut solver = Solver::new(&mut paths, route);

    loop {
        if let Some(count) = solver.next() {
            return count.to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input));
    }

    #[test]
    fn check_end() {
        let map = &HashMap::new();
        let end = Path {
            current_node: "AAZ",
            map,
        };

        let not_end = Path {
            current_node: "AAA",
            map,
        };

        assert!(!not_end.is_end());
        assert!(end.is_end());
    }

    #[test]
    fn check_path() {
        let mut map = HashMap::new();
        map.insert(
            "AAA",
            Node {
                left: "NA",
                right: "BBB",
            },
        );
        map.insert(
            "BBB",
            Node {
                left: "ZZZ",
                right: "NA",
            },
        );

        let mut path = Path {
            current_node: "AAA",
            map: &map,
        };

        path.next(&Turn::Right);
        assert!(!path.is_end());
        assert_eq!(path.current_node, "BBB");

        path.next(&Turn::Left);
        assert!(path.is_end());
        assert_eq!(path.current_node, "ZZZ");
    }

    #[test]
    fn check_solver() {
        let mut map = HashMap::new();
        map.insert(
            "11A",
            Node {
                left: "NA",
                right: "11B",
            },
        );
        map.insert(
            "11B",
            Node {
                left: "11Z",
                right: "NA",
            },
        );
        map.insert(
            "22A",
            Node {
                left: "NA",
                right: "22B",
            },
        );
        map.insert(
            "22B",
            Node {
                left: "22Z",
                right: "NA",
            },
        );

        let turns = vec![Turn::Right, Turn::Left];

        let route = Route {
            turns: turns.as_ref(),
            position: 0,
        };

        let path1 = Path {
            current_node: "11A",
            map: &map,
        };

        let path2 = Path {
            current_node: "22A",
            map: &map,
        };

        let mut paths = vec![path1, path2];

        let mut solver = Solver {
            paths: paths.as_mut(),
            route,
            count: 0,
        };

        assert_eq!(solver.next(), None);
        assert_eq!(solver.paths[0].current_node, "11B");
        assert_eq!(solver.paths[1].current_node, "22B");

        solver.next();
        assert_eq!(solver.paths[0].current_node, "11Z");
        assert_eq!(solver.paths[1].current_node, "22Z");
        // assert_eq!(solver.next(), Some(2));
    }

    #[test]
    fn check_route() {
        let turns = vec![Turn::Right, Turn::Left];
        let mut route = Route {
            turns: turns.as_ref(),
            position: 0,
        };

        assert_eq!(route.next_turn(), Turn::Right);
        assert_eq!(route.next_turn(), Turn::Left);
        assert_eq!(route.next_turn(), Turn::Right);
        assert_eq!(route.next_turn(), Turn::Left);
    }
}
