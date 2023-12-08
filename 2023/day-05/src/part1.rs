use std::{collections::HashMap, ops::Range};

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

struct MapItem {
    dest_start: u32,
    source_start: u32,
    range_len: u32,
}

#[derive(Debug)]
struct MapRange {
    range: Range<u32>,
    offset: i32,
}

impl MapRange {
    pub fn new(map_item: &MapItem) -> Self {
        let range = map_item.source_start..(map_item.source_start + map_item.range_len);
        let offset = map_item.dest_start as i32 - map_item.source_start as i32;

        MapRange { range, offset }
    }

    pub fn map(&self, item: &u32) -> Option<u32> {
        if self.range.contains(item) {
            return Some((self.offset + *item as i32) as u32);
        }

        None
    }
}

#[derive(Debug)]
struct Map {
    maps: Vec<MapRange>,
}

impl Map {
    pub fn new(map_items: Vec<MapItem>) -> Self {
        let maps = map_items.iter().map(MapRange::new).collect();

        Map { maps }
    }

    pub fn map(&self, item: u32) -> u32 {
        self.maps
            .iter()
            .filter_map(|m| m.map(&item))
            .next()
            .map_or(item, |mapped| mapped)
    }
}

fn line(input: &str) -> IResult<&str, MapItem> {
    let (input, (dest_start, source_start, range_len)) = tuple((
        complete::u32,
        complete::u32.preceded_by(tag(" ")),
        complete::u32.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        MapItem {
            dest_start,
            source_start,
            range_len,
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, Vec<MapItem>> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)))
        .parse(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<Vec<MapItem>>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)?;

    let (input, maps) = many1(parse_map)(input)?;

    Ok((input, (seeds, maps)))
}

fn load_maps(map_items: Vec<Vec<MapItem>>) -> Vec<Map> {
    map_items.into_iter().map(Map::new).collect()
}

pub fn process(input: &str) -> String {
    let (_, (seeds, maps)) = parse(input).expect("Input should be valid");
    let maps = load_maps(maps);
    println!("Maps len {}", maps.len());

    seeds
        .into_iter()
        .map(|seed| {
            println!("\nSeed: {seed}");
            maps.iter().fold(seed, |acc, map| {
                let value = map.map(acc);
                println!("{acc} maps to {value}");

                value
            })
        })
        .inspect(|l| println!("Location: {l}"))
        .min()
        .expect("Seed should map to a location")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", process(input));
    }

    #[test]
    fn check_edges() {
        let input = "seeds: 1 2 3

seed-to-soil map:
4 1 1
1 2 1";

        assert_eq!("1", process(input))
    }
}