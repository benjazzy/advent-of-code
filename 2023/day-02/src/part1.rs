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
}

struct Game {
    id: usize,
    hands: Vec<Hand>,
}

impl Game {
    pub fn from_str(game_string: &str) -> Option<Self> {
        let rest = game_string.strip_prefix("Game ")?;
        let (id, rest) = rest.split_once(':')?;
        let id = id.parse::<usize>().ok()?;

        let hands: Vec<Hand> = rest.split(';').map(Hand::from_str).collect();

        Some(Game { id, hands })
    }

    pub fn is_possible(&self, max_hand: Hand) -> bool {
        for hand in self.hands.iter() {
            if hand.red > max_hand.red {
                return false;
            }
            if hand.green > max_hand.green {
                return false;
            }
            if hand.blue > max_hand.blue {
                return false;
            }
        }

        true
    }

    pub fn filter_game(&self, max_hand: Hand) -> Option<usize> {
        if self.is_possible(max_hand) {
            return Some(self.id);
        }

        None
    }
}

pub fn process(input: &str) -> String {
    const MAX_HAND: Hand = Hand {
        red: 12,
        green: 13,
        blue: 14,
    };

    let sum: usize = input
        .lines()
        .map(Game::from_str)
        .filter_map(|game| {
            if let Some(game) = game {
                game.filter_game(MAX_HAND)
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
        assert_eq!("8", process(input));
    }
}
