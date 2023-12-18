use std::collections::BTreeMap;

use crate::custom_error::AocError;

#[derive(Clone, Debug)]
enum r#Type {
    Number(u32),
    Symbol(char),
}

#[derive(Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
    width: usize,
    value: r#Type,
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    
    let mut lines = input.lines().collect::<Vec<&str>>();
    let line_length = lines[0].len();

    let chars = lines.iter_mut().flat_map(| line | {
        line.chars()
    }).collect::<Vec<char>>();


    let mut all: Vec<(usize, usize, u32, bool)> = Vec::new();
    let mut next_number = false;

    for (i, c) in chars.iter().enumerate() {
        let x = i % line_length;
        let y = i / line_length;

        if c.is_ascii_digit() {
            let mut found = false;
            // check if left, top, right, bottom and all diagonals are symbols. one line is based on line_length varaiable i is current position in arra
            // hint: use modulo to check in which row we are (y) and with i you can calculate the current x position. so we can check easy for symbols

            // get the current x and y position
            if y > 0 {
                // check top
                let top = chars[(y - 1) * line_length + x];
                if top != '.' && !top.is_ascii_digit() {
                    //dbg!("we got a symbol on top",x,y,c);
                    found = true;
                }

                // now we try to check diagonal top left
                if x > 0 {
                    let top_left = chars[(y - 1) * line_length + (x - 1)];
                    if top_left != '.' && !top_left.is_ascii_digit() {
                        //dbg!("we got a symbol on top left",x,y,c);
                        found = true;
                    }
                }

                // now we try to check diagonal top right
                if x < line_length - 1 {
                    let top_right = chars[(y - 1) * line_length + (x + 1)];
                    if top_right != '.' && !top_right.is_ascii_digit() {
                        //dbg!("we got a symbol on top right",x,y,c);
                        found = true;
                    }
                }
            }
            // check left side (be sure we not jump a line (y) back!)
            if x > 0 {
                let left = chars[y * line_length + (x - 1)];
                if left != '.' && !left.is_ascii_digit() {
                    //dbg!("we got a symbol on left",x,y,c);
                    found = true;
                }
            }

            // check right side (be sure we not jump a line (y) back!)
            if x < line_length - 1 {
                let right = chars[y * line_length + (x + 1)];
                if right != '.' && !right.is_ascii_digit() {
                    //dbg!("we got a symbol on right",x,y,c);
                    found = true;
                }
            }

            // no we check bottom (be sure not to jump over the last line)
            if y < lines.len() - 1 {
                let bottom = chars[(y + 1) * line_length + x];
                if bottom != '.' && !bottom.is_ascii_digit() {
                    //dbg!("we got a symbol on bottom",x,y,c);
                    found = true;
                }

                // now we try to check diagonal bottom left
                if x > 0 {
                    let bottom_left = chars[(y + 1) * line_length + (x - 1)];
                    if bottom_left != '.' && !bottom_left.is_ascii_digit() {
                        //dbg!("we got a symbol on bottom left",x,y,c);
                        found = true;
                    }
                }

                // now we try to check diagonal bottom right
                if x < line_length - 1 {
                    let bottom_right = chars[(y + 1) * line_length + (x + 1)];
                    if bottom_right != '.' && !bottom_right.is_ascii_digit() {
                        //dbg!("we got a symbol on bottom right",x,y,c);
                        found = true;
                    }
                }
            }

            
            if let Some(a) = all.last_mut() {
                if next_number {
                    all.push((x,y,c.to_digit(10).unwrap(), found));
                    next_number = false;
                } else {
                    (*a).2 = (a.2 * 10) + c.to_digit(10).unwrap();
                    if found == true {
                        (*a).3 = true;
                    }
                }
            } else {
                all.push((x,y,c.to_digit(10).unwrap(), found));
                next_number = false;
            }
        } else {
            next_number = true;
        }


    }
    
    for (x,y,c,found) in &all {
        if *found == true {
            println!("x: {}, y: {}, value: {}, found: {}", x, y, c, found);
        }
        
    }

    let all_with_connection: u32 = all.iter().filter(|(_, _, _, found)| *found == true).map(|(_, _, a,_)| a).sum();
    //dbg!(all_with_connection);



    Ok(all_with_connection.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...............307............130..................969...601...186.........................................312....628..........878..........
......479#../..*..............................#.....*......*............../309.....484........................*......-..........+.....89....
...........726..352...461..69..............435.....390...625....................................459.........152...-....580............*.....
.......................*.......454*674.448......65................257....104*762....&..............*269.........558.&.....*907.........652..
.....................164.....-..............532*.....................................484......108........955.........252..........321.......
.......270....*..............470.......................632....................*...................$............352...................*......
";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
