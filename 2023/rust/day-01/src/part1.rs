use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {

    let output: u32 = input.lines().map(|line| {
        let mut all_numbers = line.chars()
            .filter(|c| c.is_numeric());

        let mut numbers = Vec::new();
        let first_number = all_numbers.next().expect("first number should exists");
        numbers.push(first_number);
        numbers.push(all_numbers.last().unwrap_or(first_number));
        let result = numbers.into_iter().collect::<String>().parse::<u32>();
        result.expect("should be a number")
    }).sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
