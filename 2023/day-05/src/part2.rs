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
    dest_start: i64,
    source_start: i64,
    range_len: i64,
}

#[derive(Debug)]
struct MapRange {
    range: Range<i64>,
    offset: i64,
}

impl MapRange {
    pub fn new(map_item: &MapItem) -> Self {
        let range = map_item.source_start..(map_item.source_start + map_item.range_len);
        let offset = map_item.dest_start - map_item.source_start;

        MapRange { range, offset }
    }

    pub fn map(&self, item: &i64) -> Option<i64> {
        if self.range.contains(item) {
            return Some((self.offset + *item));
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

    pub fn map(&self, item: i64) -> i64 {
        self.maps
            .iter()
            .find_map(|m| m.map(&item))
            .map_or(item, |mapped| mapped)
    }
}

fn line(input: &str) -> IResult<&str, MapItem> {
    let (input, (dest_start, source_start, range_len)) = tuple((
        complete::i64,
        complete::i64.preceded_by(tag(" ")),
        complete::i64.preceded_by(tag(" ")),
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

fn parse(input: &str) -> IResult<&str, (Vec<i64>, Vec<Vec<MapItem>>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::i64))
        .parse(input)?;

    let (input, maps) = many1(parse_map)(input)?;

    Ok((input, (seeds, maps)))
}

fn load_maps(map_items: Vec<Vec<MapItem>>) -> Vec<Map> {
    map_items.into_iter().map(Map::new).collect()
}

pub fn process(input: &str) -> String {
    let (_, (seeds, maps)) = parse(input).expect("Input should be valid");
    let seed_ranges: Vec<Range<i64>> = seeds
        .chunks(2)
        .map(|ends| ends[0]..(ends[0] + ends[1]))
        .collect();

    let maps = load_maps(maps);

    seed_ranges
        .into_iter()
        .map(|seed_range| {
            seed_range
                .map(|seed| maps.iter().fold(seed, |acc, map| map.map(acc)))
                .min()
                .expect("Seed should map to a location")
        })
        .min()
        .expect("Seed range should map to a location")
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
        assert_eq!("46", process(input));
    }
}
