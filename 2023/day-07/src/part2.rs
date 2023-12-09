use std::collections::HashMap;

use nom::character::complete::{self, alphanumeric1, newline, space1};
use nom::combinator::{self, into};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, InputTake};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(format!("Unknown char {value}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CardSet<const N: usize> {
    sorted: [Card; N],
    unsorted: [Card; N],
}

impl<const N: usize> CardSet<N> {
    pub fn new(unsorted: [Card; N]) -> Self {
        let mut sorted = unsorted;
        sorted.sort_unstable();

        CardSet { sorted, unsorted }
    }

    pub fn get_type(&self) -> HandType {
        let cards = self.sorted.as_slice();

        let jacks = cards.iter().filter(|c| **c == Card::Jack).count();

        let (acc, mut first, mut second) = cards
            .windows(2)
            .filter(|cards| cards[0] != Card::Jack)
            .fold((1, 1, 1), |(acc, first, second), cards| {
                let one = &cards[0];

                if let Some(two) = cards.get(1) {
                    if one == two {
                        return (acc + 1, first, second);
                    }
                }

                if acc > first {
                    (1, acc, first)
                } else if acc > second {
                    (1, first, acc)
                } else {
                    (1, first, second)
                }
            });

        // println!("Acc: {acc}, first: {first}, second: {second}");

        if acc > first {
            second = first;
            first = acc;
        } else if acc > second {
            second = acc;
        }

        first += jacks;
        if first > 5 {
            first = 5;
        }

        (first, second).try_into().expect("Invalid hand")
    }
}

impl<const N: usize> PartialOrd for CardSet<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for CardSet<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let our_type = self.get_type();
        let other_type = other.get_type();

        if our_type == other_type {
            self.unsorted.cmp(&other.unsorted)
        } else {
            our_type.cmp(&other_type)
        }
    }
}

impl<const N: usize> TryFrom<&str> for CardSet<N> {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cards: Vec<Card> = value
            .chars()
            .take(N)
            .to_owned()
            .map(Card::try_from)
            .try_collect()?;

        let cards: [Card; N] = cards
            .try_into()
            .map_err(|_| "Could not turn vec of cards into array".to_string())?;

        Ok(CardSet::new(cards))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<const N: usize> {
    cards: CardSet<N>,
    bet: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl TryFrom<(usize, usize)> for HandType {
    type Error = String;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        match value {
            (5, _) => Ok(HandType::FiveOfAKind),
            (4, _) => Ok(HandType::FourOfAKind),
            (3, 2) => Ok(HandType::FullHouse),
            (3, _) => Ok(HandType::ThreeOfAKind),
            (2, 2) => Ok(HandType::TwoPair),
            (2, _) => Ok(HandType::OnePair),
            (1, 1) => Ok(HandType::HighCard),
            _ => Err(format!("Invalid hand {:?}", value)),
        }
    }
}

fn parse_hand<const N: usize>(input: &str) -> IResult<&str, Hand<N>> {
    let (rest, (cards, bet)) = separated_pair(alphanumeric1, space1, complete::u64)(input)?;
    let cards = cards.try_into().expect("Hand should be valid");

    Ok((rest, Hand { cards, bet }))
}

fn parse<const N: usize>(input: &str) -> IResult<&str, Vec<Hand<N>>> {
    separated_list1(newline, parse_hand)(input)
}

pub fn process(input: &str) -> String {
    const HAND_SIZE: usize = 5;

    let (_, mut hands) = parse::<HAND_SIZE>(input).expect("Input should be valid");
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bet)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input));
    }

    #[rstest]
    #[case("32T3K", HandType::OnePair)]
    #[case("T55J5", HandType::FourOfAKind)]
    #[case("KK677", HandType::TwoPair)]
    #[case("KTJJT", HandType::FourOfAKind)]
    #[case("QQQJA", HandType::FourOfAKind)]
    #[case("TKKKK", HandType::FourOfAKind)]
    #[case("KKKKT", HandType::FourOfAKind)]
    #[case("KKKTT", HandType::FullHouse)]
    #[case("JJJJJ", HandType::FiveOfAKind)]
    fn check_card_type(#[case] input: &str, #[case] expected: HandType) {
        let card_set: CardSet<5> = CardSet::try_from(input).unwrap();

        assert_eq!(expected, card_set.get_type(), "{input}");
    }
}
