use nom::bytes::complete::{is_a, is_not, tag, take_until};
use nom::character::complete::{self, alphanumeric1, line_ending, space1};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use nom_supreme::ParserExt;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn get_beatable_ways(&self) -> u64 {
        (1..self.time)
            .filter(|time_held| {
                let time_remaining = self.time - time_held;

                let distance = time_held * time_remaining;

                distance > self.distance
            })
            .count() as u64
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LineType {
    Time,
    Distance,
}

impl TryFrom<&str> for LineType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Time" => Ok(LineType::Time),
            "Distance" => Ok(LineType::Distance),
            _ => Err("Invalid line type"),
        }
    }
}

//Time:      7  15   30
fn parse_line(line: &str) -> IResult<&str, (LineType, Vec<u64>)> {
    let (rest, line_type) = context("line type", terminated(alphanumeric1, tag(":")))(line)?;
    println!("Line type: {line_type}");

    let (rest, values) = context(
        "line values",
        is_not("0123456789").precedes(separated_list1(space1, complete::u64)),
    )(rest)?;

    Ok((
        rest,
        (
            line_type
                .try_into()
                .expect("Line should start with Time or Distance"),
            values,
        ),
    ))
}

fn parse_times(line: &str) -> IResult<&str, Vec<u64>> {
    parse_line(line).map(|(rest, (line_type, values))| {
        assert_eq!(line_type, LineType::Time);

        (rest, values)
    })
}

fn parse_distances(line: &str) -> IResult<&str, Vec<u64>> {
    parse_line(line).map(|(rest, (line_type, values))| {
        assert_eq!(line_type, LineType::Distance);

        (rest, values)
    })
}

fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    let (rest, (times, distances)) =
        separated_pair(parse_times, line_ending, parse_distances)(input)?;

    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect();

    Ok((rest, races))
}

pub fn process(input: &str) -> String {
    let (_, races) = parse(input).expect("Input should be valid");
    println!("Races: {:?}", races);

    races
        .iter()
        .map(|race| race.get_beatable_ways())
        .product::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input));
    }
}
