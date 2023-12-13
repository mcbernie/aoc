use tracing::debug;

use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]

pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {

    let output: u32 = input.lines().map(|line| {
        
        let mut all_numbers = Vec::new();
        for (i,ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                all_numbers.push(ch.to_string().parse::<u32>().unwrap());
            } else {
                match &line[i..] {
                    c if c.starts_with("one") => all_numbers.push(1),
                    c if c.starts_with("two") => all_numbers.push(2),
                    c if c.starts_with("three") => all_numbers.push(3),
                    c if c.starts_with("four") => all_numbers.push(4),
                    c if c.starts_with("five") => all_numbers.push(5),
                    c if c.starts_with("six") => all_numbers.push(6),
                    c if c.starts_with("seven") => all_numbers.push(7),
                    c if c.starts_with("eight") => all_numbers.push(8),
                    c if c.starts_with("nine") => all_numbers.push(9),
                    _ => {}
                };
            }
        }


        let mut all_numbers = all_numbers.iter();

        let first_number = all_numbers.next().expect("first number should exists");
        if let Some(next_number) = all_numbers.last() {
            (first_number * 10) + next_number
        } else {
            (first_number * 10) + first_number
        }

    }).sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }

}
