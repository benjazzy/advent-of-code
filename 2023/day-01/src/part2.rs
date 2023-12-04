const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn process(input: &str) -> String {
    assert!(input.is_ascii(), "Input needs to be valid ascii");
    let result: usize = input
        .lines()
        .map(|l| get_digits(l).unwrap_or_default())
        .sum();

    result.to_string()
}

fn get_digit(input: &str, pos: usize) -> usize {
    input
        .chars()
        .nth(pos)
        .expect("Digit index out of range")
        .to_digit(10)
        .expect("Digit is not valid") as usize
}

fn get_word(input: &str, pos: usize, word_size: usize) -> usize {
    let full_num = input
        .as_bytes()
        .windows(word_size)
        .map(|w| std::str::from_utf8(w).expect("Byte was not valid ascii"))
        .nth(pos)
        .expect("Word out of range");

    NUMBERS
        .iter()
        .position(|n| full_num.contains(n))
        .expect("Word does not contain a digit")
}

fn get_digits(input: &str) -> Option<usize> {
    let first_digit_pos = input.chars().position(|c| c.is_ascii_digit());
    let mut first_word_pos = None;
    let mut first_word_size = 0;
    for num in NUMBERS {
        if let Some(i) = input.find(num) {
            match first_word_pos {
                Some(w) if i < w => {
                    first_word_pos = Some(i);
                    first_word_size = num.len();
                }
                None => {
                    first_word_pos = Some(i);
                    first_word_size = num.len();
                }
                _ => {}
            }
        }
    }
    let first = match (first_digit_pos, first_word_pos) {
        (Some(d), Some(w)) if d < w => get_digit(input, d),
        (Some(d), Some(w)) if d > w => get_word(input, w, first_word_size),
        (Some(d), Some(w)) if d == w => {
            panic!("Digit and word position cant be equal. digit: {d}, word {w}")
        }
        (Some(d), None) => get_digit(input, d),
        (None, Some(w)) => get_word(input, w, first_word_size),
        _ => panic!("Something went wrong in first"),
    };

    let last_digit_pos = input.bytes().rposition(|b| b.is_ascii_digit());
    let mut last_word_pos = None;
    let mut last_word_size = 0;
    for num in NUMBERS {
        if let Some(i) = input.rfind(num) {
            match last_word_pos {
                Some(w) if i > w => {
                    last_word_pos = Some(i);
                    last_word_size = num.len();
                }
                None => {
                    last_word_pos = Some(i);
                    last_word_size = num.len();
                }
                _ => {}
            }
        }
    }
    let last = match (last_digit_pos, last_word_pos) {
        (Some(d), Some(w)) if d > w => get_digit(input, d),
        (Some(d), Some(w)) if d < w => get_word(input, w, last_word_size),
        (Some(d), Some(w)) if d == w => {
            panic!("Digit and word position cant be equal. digit: {d}, word {w}")
        }
        (Some(d), None) => get_digit(input, d),
        (None, Some(w)) => get_word(input, w, last_word_size),
        _ => panic!("Something went wrong in last"),
    };

    Some(
        format!("{first}{last}")
            .parse::<usize>()
            .expect("Could not parse digits as usize"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_individual() {
        let test_inputs: &[(&str, usize)] = &[
            ("kqrxsgtrrjsix9", 69),
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        for (text, num) in test_inputs {
            let digits = get_digits(text);
            assert_eq!(Some(*num), digits);
        }
    }

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input));
    }
}
