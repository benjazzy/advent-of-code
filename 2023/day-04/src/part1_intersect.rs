use std::collections::HashSet;

struct Card {
    id: usize,
    numbers: HashSet<usize>,
    winning: HashSet<usize>,
}

impl Card {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .strip_prefix("Card ")
            .expect("Card input should start with 'Card '")
            .trim_start();

        let (id, rest) = input
            .split_once(':')
            .expect("Card input should contain ':'");
        let id = id.parse::<usize>().expect("Card id should be a number");

        let (winning, rest) = rest.split_once('|').expect("Card input should contain '|'");
        let winning: HashSet<usize> = winning
            .split(' ')
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();

        let numbers: HashSet<usize> = rest
            .split(' ')
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();

        Card {
            id,
            numbers,
            winning,
        }
    }

    pub fn get_points(&self) -> usize {
        let mut winning_numbers = self.winning.intersection(&self.numbers);
        if let Some(_) = winning_numbers.next() {
            winning_numbers.fold(1, |acc, _| acc * 2)
        } else {
            0
        }
    }
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|l| Card::from_str(l).get_points())
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input));
    }
}
