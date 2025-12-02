#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {

    input.lines().

}

/// which direction to turn how many steps
/// starts by 50. after 0 we wrap around to 99
/// at the end we count the number of time we cross 0
enum Direction {
    Left(i32),
    Right(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        ";
        assert_eq!("", process(input)?);
        Ok(())
    }
}