use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::line_ending, multi::separated_list1};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {

}

/// which direction to turn how many steps
/// starts by 50. after 0 we wrap around to 99
/// at the end we count the number of time we cross 0
enum Direction {
    Left(i32),
    Right(i32),
}

fn parse_direction(input: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(line_ending, parse_line)
}

fn parse_line(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = alt((tag("L"), tag("R"))).parse(input);

    match dir {
        "L" => {
            Direction::Left(1),
        },
        "R" => {
            Direction::Right(2),
        }
        _ => {
            println!("fehler")
        }
    }

    return Direction::Left(1);
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