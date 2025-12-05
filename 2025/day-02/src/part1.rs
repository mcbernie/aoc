use std::ops::RangeInclusive;

use nom::{
    IResult, Parser, bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_rest, ranges) = separated_list1(tag(","), parse_range).parse(input).unwrap();

    // jetzt habe ich ein vec mit tuple ranges
    let mut ids = 0;

    for range in ranges {
        for v in range {
            let l = v.ilog10() + 1;
            let half = l / 2;
            let p = 10u64.pow(half);

            if v / p == v % p {
                ids += v
            }

        }
    }

    Ok(ids.to_string())
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(
        complete::u64, 
        tag("-"), 
        complete::u64
    ).map(
        |(f,t)| 
        f..=t
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}