struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl Hand {
    pub fn from_str(hand_str: &str) -> Self {
        let mut hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };
        hand_str.split(',').for_each(|color| {
            let color = color.trim();

            match color.split_once(' ').map(|(num, color)| {
                (
                    num.parse::<usize>().expect("Color amount is not valid"),
                    color,
                )
            }) {
                Some((num, "red")) => hand.red = num,
                Some((num, "green")) => hand.green = num,
                Some((num, "blue")) => hand.blue = num,
                Some((_, color)) => panic!("Color {color} is not valid"),
                None => {}
            }
        });

        hand
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: usize,
    hands: Vec<Hand>,
}

impl Game {
    pub fn new(id: usize, hands: Vec<Hand>) -> Self {
        Game { id, hands }
    }

    pub fn from_str(game_string: &str) -> Option<Self> {
        let rest = game_string.strip_prefix("Game ")?;
        let (id, rest) = rest.split_once(':')?;
        let id = id.parse::<usize>().ok()?;

        let hands: Vec<Hand> = rest.split(';').map(Hand::from_str).collect();

        Some(Game { id, hands })
    }

    fn power(&self) -> usize {
        self.get_min_possible_hand().power()
    }

    fn get_min_possible_hand(&self) -> Hand {
        const STARTING_HAND: Hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };

        let min_possible_hand = self.hands.iter().fold(STARTING_HAND, |acc, hand| {
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
        .filter_map(|game| {
            if let Some(game) = game {
                Some(game.power())
            } else {
                None
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input));
    }
}
