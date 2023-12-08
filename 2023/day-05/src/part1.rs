use std::collections::HashMap;

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

fn load_maps(map_items: Vec<Vec<MapItem>>) -> Vec<HashMap<u32, u32>> {
    let mut maps = Vec::new();

    for map_set in map_items {
        let mut combined_map = HashMap::new();
        for map in map_set {
            let map = process_map(map);
            combined_map.extend(map);
        }

        maps.push(combined_map);
    }

    maps
}

fn process_map(map_item: MapItem) -> HashMap<u32, u32> {
    let mut items = HashMap::new();

    for i in 0..map_item.range_len {
        items.insert(i + map_item.source_start, i + map_item.dest_start);
    }

    items
}

pub fn process(input: &str) -> String {
    let (_, (seeds, maps)) = parse(input).expect("Input should be valid");
    let maps = load_maps(maps);
    println!("Maps len {}", maps.len());

    println!("Test: {:?}", maps[1]);

    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(seed, |acc, map| {
                println!("{acc}");
                map.get(acc).expect("Map should contain seed")
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
}
