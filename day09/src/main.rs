use std::collections::HashSet;
use std::str::FromStr;
use std::io::Error;
use std::io::ErrorKind::InvalidInput;

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    R, U, L, D
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "R" => Ok(Direction::R),
            "U" => Ok(Direction::U),
            "L" => Ok(Direction::L),
            "D" => Ok(Direction::D),
            _ => Err(Error::new(InvalidInput, "Invalid direction"))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    direction: Direction,
    count: u32,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(" ");
        let direction_str = parts.next().ok_or(Error::new(InvalidInput, "Empty String"))?;
        let direction = direction_str.parse()?;
        
        let count_str = parts.next().ok_or(Error::new(InvalidInput, "String has been split into two parts due to the lack of whitespace"))?;
        let count = count_str.parse().map_err(|_| Error::new(InvalidInput, "Invalid move count input"))?;
        
        Ok(Move { direction, count })
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Position(i32, i32);

fn adjust_tail_pos_based_on_head(tail: &mut Position, head: &Position, direction: &Direction) {
    let y = tail.0 - head.0;
    let x = tail.1 - head.1;

    if (-1..=1).contains(&y) && (-1..=1).contains(&x) {
        return
    }

    match direction {
        Direction::R => {
            tail.0 = head.0;
            tail.1 = head.1 - 1;
        },
        Direction::U => {
            tail.0 = head.0 - 1;
            tail.1 = head.1;
        },
        Direction::L => {
            tail.0 = head.0;
            tail.1 = head.1 + 1;
        },
        Direction::D => {
            tail.0 = head.0 + 1;
            tail.1 = head.1;
        },
    }
}

fn calc_position_count(moves: &Vec<Move>) -> u32 {
    let mut set = HashSet::new();
    let mut head = Position(0, 0);
    let mut tail = Position(0, 0);

    set.insert(tail);

    for m in moves {
        for _ in 0..m.count {
            match m.direction {
                Direction::R => head.1 += 1,
                Direction::U => head.0 += 1,
                Direction::L => head.1 -= 1,
                Direction::D => head.0 -= 1,
            }
            
            adjust_tail_pos_based_on_head(&mut tail, &head, &m.direction);
            set.insert(tail);
    
            // dbg!(head);
            // dbg!(tail);
        }
    }

    set.len() as u32
}

fn main() {
    let input = include_str!("input.txt");
    let moves: Vec<Move> = input
        .lines()
        .filter_map(|line| 
            match line.parse::<Move>() {
                Ok(r#move) => Some(r#move),
                Err(_) => None
            }
        )
        .collect();

    println!("{}", calc_position_count(&moves));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_example_test() {
        let input = include_str!("test_input.txt");
        let moves: Vec<Move> = input
            .lines()
            .filter_map(|line| 
                match line.parse::<Move>() {
                    Ok(r#move) => Some(r#move),
                    Err(_) => None
                }
            )
            .collect();

        assert_eq!(13, calc_position_count(&moves));
    }
}