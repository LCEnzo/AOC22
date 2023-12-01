use std::cmp::{min, max};
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

fn adjust_tail_pos_based_on_head(tail: &mut Position, head: &Position) {
    let y = tail.0 - head.0;
    let x = tail.1 - head.1;

    if (-1..=1).contains(&y) && (-1..=1).contains(&x) {
        return
    } 
    else if x.abs() == y.abs() {
        tail.0 -= if y.is_negative() { y + 1 } else { y - 1 };
        tail.1 -= if x.is_negative() { x + 1 } else { x - 1 };
    } 
    else if x.abs() > y.abs() {
        tail.0 = head.0;
        tail.1 = if x.is_negative() { head.1 - 1 } else { head.1 + 1 };
    }
    else {
        tail.0 = if y.is_negative() { head.0 - 1 } else { head.0 + 1 };
        tail.1 = head.1;
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
            
            adjust_tail_pos_based_on_head(&mut tail, &head);
            set.insert(tail);
        }
    }

    set.len() as u32
}

fn print_snake(snake: &Vec<Position>) {
    // Finds max and min height and width to form bounds of the box to be drawn
    let min_h = min(snake.iter().map(|pos| pos.0).min().unwrap(), -2);
    let max_h = max(snake.iter().map(|pos| pos.0).max().unwrap(), 2);
    let min_w = min(snake.iter().map(|pos| pos.1).min().unwrap(), -2);
    let max_w = max(snake.iter().map(|pos| pos.1).max().unwrap(), 2);
    let tail_ind: usize = snake.len() - 1;

    for h in min_h..=max_h {
        'draw_loop: for w in min_w..=max_w {
            if h == 0 && w == 0 {
                print!("s");
                continue 'draw_loop;
            }

            for (ind, pos) in snake.iter().enumerate() {
                if pos.0 == h && pos.1 == w {
                    let marker = match ind { 
                        0 => "H".to_string(), 
                        _ if ind == tail_ind => "T".to_string(), 
                        _ => ind.to_string(),
                    };
                    
                    print!("{}", marker);
                    continue 'draw_loop;
                }
            }

            print!("_");
        }

        println!("");
    }

    println!("");
    std::thread::sleep(std::time::Duration::from_millis(250));
}

fn calc_snake_tail_positions_count(moves: &Vec<Move>, snake_len: usize) -> u32 {
    // Head is 0, Tail is len() - 1
    if snake_len < 2 {
        return 0;
    }

    let mut snake = vec![Position(0, 0); snake_len];
    let mut set = HashSet::new();

    set.insert(snake.last().unwrap().clone());

    for m in moves {
        for _ in 0..m.count {
            match m.direction {
                Direction::R => snake[0].1 += 1,
                Direction::U => snake[0].0 += 1,
                Direction::L => snake[0].1 -= 1,
                Direction::D => snake[0].0 -= 1,
            }

            for (curr, prev) in (0..snake_len-1).zip(1..snake_len) {
                let curr_clone = snake[curr].clone();
                adjust_tail_pos_based_on_head(&mut snake[prev], &curr_clone);
            }

            set.insert(snake.last().unwrap().clone());

            // print_snake(&snake);
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
    println!("{}", calc_snake_tail_positions_count(&moves, 10));
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

    #[test]
    fn second_half_example_test() {
        let input = include_str!("test_input2.txt");
        let moves: Vec<Move> = input
            .lines()
            .filter_map(|line| 
                match line.parse::<Move>() {
                    Ok(r#move) => Some(r#move),
                    Err(_) => None
                }
            )
            .collect();

        assert_eq!(36, calc_snake_tail_positions_count(&moves, 10));
    }

    #[test]
    fn check_generality_of_snake_fn() {
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

        assert_eq!(13, calc_snake_tail_positions_count(&moves, 2));
    }
}
