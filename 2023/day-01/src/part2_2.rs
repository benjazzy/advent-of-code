const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug)]
struct NumberWord {
    position: usize,
    value: usize,
}

pub fn process(input: &str) -> String {
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

fn get_digits(input: &str) -> Option<usize> {
    assert!(input.is_ascii(), "Input needs to be ascii");

    let first_digit_pos = input.chars().position(|c| c.is_ascii_digit());

    let first_word: Option<NumberWord> =
        NUMBERS
            .iter()
            .enumerate()
            .fold(None, |acc, (value, num)| match (acc, input.find(num)) {
                (Some(num_word), Some(pos)) if pos < num_word.position => Some(NumberWord {
                    position: pos,
                    value,
                }),
                (None, Some(pos)) => Some(NumberWord {
                    position: pos,
                    value,
                }),
                (acc, _) => acc,
            });

    let first = match (first_digit_pos, first_word) {
        (Some(d), Some(w)) if d < w.position => get_digit(input, d),
        (Some(d), Some(w)) if d > w.position => w.value,
        (Some(d), Some(w)) if d == w.position => {
            panic!(
                "Digit and word position cant be equal. digit: {d}, word {:?}",
                w
            )
        }
        (Some(d), None) => get_digit(input, d),
        (None, Some(w)) => w.value,
        _ => panic!("Something went wrong in first"),
    };

    let last_digit_pos = input.bytes().rposition(|b| b.is_ascii_digit());

    let last_word: Option<NumberWord> =
        NUMBERS
            .iter()
            .enumerate()
            .fold(None, |acc, (value, num)| match (acc, input.rfind(num)) {
                (Some(num_word), Some(pos)) if pos > num_word.position => Some(NumberWord {
                    position: pos,
                    value,
                }),
                (None, Some(pos)) => Some(NumberWord {
                    position: pos,
                    value,
                }),
                (acc, _) => acc,
            });

    let last = match (last_digit_pos, last_word) {
        (Some(d), Some(w)) if d > w.position => get_digit(input, d),
        (Some(d), Some(w)) if d < w.position => w.value,
        (Some(d), Some(w)) if d == w.position => {
            panic!(
                "Digit and word position cant be equal. digit: {d}, word {:?}",
                w
            )
        }
        (Some(d), None) => get_digit(input, d),
        (None, Some(w)) => w.value,
        _ => panic!("Something went wrong in last"),
    };

    println!("input: {input}\tfirst: {first}\tsecond: {last}");

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
