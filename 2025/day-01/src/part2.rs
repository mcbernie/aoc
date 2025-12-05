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
        let num = match dir {
            Direction::Left(num) => -num,
            Direction::Right(num) => num
        };
        
        let (new_c, count) = moving(cursor, num);
        cursor = new_c;
        crossings += count;
    }

    Ok(crossings.to_string())
}

fn moving(cur_cur: i32, mov: i32) -> (i32, i32) {

    let new_cur = cur_cur + mov;
    let mut revolutions = (new_cur / 100).abs();

    if (cur_cur > 0 && new_cur < 0) || new_cur == 0 {
        revolutions += 1
    }

    (
        new_cur.rem_euclid(100),
        revolutions
    )
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
    //#[ignore]
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
        assert_eq!("6", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn test_mover() -> miette::Result<()> {
        assert_eq!((82,1), moving(50, -68));
        assert_eq!((52,0), moving(82, -30));
        assert_eq!((0,1), moving(52, 48));
        assert_eq!((95,0), moving(0, -5));
        assert_eq!((55,1), moving(95, 60));
        assert_eq!((0,1), moving(55, -55));
        assert_eq!((99,0), moving(0, -1));
        assert_eq!((0,1), moving(99, -99));
        assert_eq!((14,0), moving(0, 14));
        assert_eq!((32,1), moving(14, -82));
        Ok(())
    }

    #[test_log::test]
    fn test_mover_160() -> miette::Result<()> {
        assert_eq!((10,2), moving(50, 160));
        Ok(())
    }
    
    #[test_log::test]
    fn test_mover_845() -> miette::Result<()> {
        assert_eq!((95,8), moving(50, 845));
        Ok(())
    }
}