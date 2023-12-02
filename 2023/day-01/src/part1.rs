pub fn process(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|l| get_digits(l).unwrap_or_default())
        .sum();

    result.to_string()
}

fn get_digits(input: &str) -> Option<usize> {
    let first = input.chars().find(|c| c.is_ascii_digit())?;
    let last = input.chars().rev().find(|c| c.is_ascii_digit())?;

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
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input));
    }
}
