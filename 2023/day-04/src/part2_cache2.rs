use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Card {
    id: usize,
    numbers: HashSet<usize>,
    winning: Vec<usize>,
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
        let winning: Vec<usize> = winning
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

    pub fn number_of_winnings(&self) -> usize {
        self.winning
            .iter()
            .filter(|w| self.numbers.contains(w))
            .count()
    }
}

fn process_children(id: usize, cards: &[Card], cache: &mut HashMap<usize, usize>) -> usize {
    let card = &cards[id];

    if let Some(sum) = cache.get(&id) {
        return *sum;
    }

    let number_of_winnings = card.number_of_winnings();
    if number_of_winnings == 0 {
        return 1;
    }

    let last_card = number_of_winnings + id;
    let sum = ((id + 1)..=last_card)
        .filter(|i| *i < cards.len())
        .map(|i| process_children(i, cards, cache))
        .sum::<usize>()
        + 1;

    cache.insert(id, sum);

    sum
}

pub fn process(input: &str) -> String {
    let mut cache = HashMap::new();
    let cards: Vec<Card> = input.lines().map(|l| Card::from_str(l)).collect();

    cards
        .iter()
        .enumerate()
        .map(|(i, _)| process_children(i, cards.as_slice(), &mut cache))
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
        assert_eq!("30", process(input));
    }
}
