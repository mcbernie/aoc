use nom::{
    multi::separated_list1, 
    character::complete::{self, line_ending, alpha1},
    IResult, 
    sequence::{preceded, separated_pair}, bytes::complete::tag, 
};

use crate::custom_error::AocError;


#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}
impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Unknown color: {}", s),
        }
    }
}

#[derive(Debug)]
struct Cube {
    color: Color,
    count: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Vec<Cube>>,
}

impl Game {
    fn check_if_it_works(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        let result: (u32,u32,u32) = self.rounds.iter().map(| cubes | {

            let mut current_red = 0;
            let mut current_green = 0;
            let mut current_blue = 0;
            for cube in cubes.iter() {
                match cube.color {
                    Color::Red => {
                        if cube.count > max_red { current_red = 1 };
                     } ,
                    Color::Green => {
                        if cube.count > max_green { current_green = 1 };
                    } ,
                    Color::Blue => {
                        if cube.count > max_blue { current_blue = 1 };
                    },
                }
            }

            (current_red, current_green, current_blue)
        }).fold((0,0,0),|acc, (r,g,b)| {
            (acc.0 + r, acc.1 + g, acc.2 + b)
        });

        !(result.0 > 0 || result.1 > 0 || result.2 > 0)
    }
}

fn cube(input: &str) -> IResult<&str, Cube> {
    
    let (input, (count, color)) =  separated_pair(
        complete::u32, // number
        tag(" "), // space
        alpha1 // color
    )(input)?;

    Ok((input, Cube {
        color: Color::from(color),
        count,
    }))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), complete::u32)(input)?;
    let (input, rounds) = preceded(
        tag(": "), 
        separated_list1(tag("; "), 
            separated_list1(
                tag(", "),
                cube
            )
        )
    )(input)?;

    //dbg!(id);
    //dbg!(&rounds);

    Ok((input, Game{
        id,
        rounds,
    }))
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    

    let (_, games) = separated_list1(line_ending, game)(input).expect("should always work");

    let score = games.iter().map(| game | {
        match game.check_if_it_works(12, 13, 14) {
            true => {
                //dbg!(game);
                game.id
            },
            false => {

                //dbg!("not possible");
                0
            },
        }
    }).sum::<u32>();

    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("8", process(input)?);
        Ok(())
    }
}
