#[derive(Default)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl Hand {
    pub fn from_str(hand_str: &str) -> Self {
        hand_str.split(',').fold(Default::default(), |acc, color| {
            let color = color.trim_start();

            match color.split_once(' ').map(|(num, color)| {
                (
                    num.parse::<usize>().expect("Color amount is not valid"),
                    color,
                )
            }) {
                Some((num, "red")) => Hand { red: num, ..acc },
                Some((num, "green")) => Hand { green: num, ..acc },
                Some((num, "blue")) => Hand { blue: num, ..acc },
                Some((_, color)) => panic!("Color {color} is not valid"),
                None => acc,
            }
        })
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

struct Game {
    hands: Vec<Hand>,
}

impl Game {
    pub fn from_str(game_string: &str) -> Option<Self> {
        let (_, rest) = game_string.split_once(':')?;

        let hands: Vec<Hand> = rest.split(';').map(Hand::from_str).collect();

        Some(Game { hands })
    }

    fn power(&self) -> usize {
        self.get_min_possible_hand().power()
    }

    fn get_min_possible_hand(&self) -> Hand {
        let min_possible_hand = self.hands.iter().fold(Hand::default(), |acc, hand| {
            let red = if hand.red > acc.red {
                hand.red
            } else {
                acc.red
            };
            let green = if hand.green > acc.green {
                hand.green
            } else {
                acc.green
            };
            let blue = if hand.blue > acc.blue {
                hand.blue
            } else {
                acc.blue
            };

            Hand { red, green, blue }
        });

        min_possible_hand
    }
}

pub fn process(input: &str) -> String {
    let sum: usize = input
        .lines()
        .map(Game::from_str)
        .filter_map(|game| game.map(|game| game.power()))
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input));
    }
}
