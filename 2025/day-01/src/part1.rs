use nom::{
    IResult, 
    Parser, 
    branch::alt, 
    bytes::complete::tag, 
    character::complete::{self,line_ending}, 
    multi::separated_list1
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, directions)= parse_directions(input).unwrap();

    let mut cursor = 50;
    let mut crossings = 0;

    for dir in directions {
        let (mov, _num) = match dir {
            Direction::Left(num) => ((cursor - num).rem_euclid(100), num),
            Direction::Right(num) => ((cursor + num).rem_euclid(100), num)
        };

        cursor = mov;
        if cursor == 0 {
            crossings += 1;
        }
    }

    Ok(crossings.to_string())
}

/// which direction to turn how many steps
/// starts by 50. after 0 we wrap around to 99
/// at the end we count the number of time we cross 0
#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = alt((tag("L"), tag("R"))).parse(input)?;
    let (input, num) = complete::i32(input)?;

    let direction = match dir {
        "L" => {
            Direction::Left(num)
        },
        "R" => {
            Direction::Right(num)
        }
        a => {
            panic!("fehler {a}")
        }
    };

    Ok((input, direction))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("3", process(input)?);
        Ok(())
    }
}