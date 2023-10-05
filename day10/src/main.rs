use core::str::FromStr;
use anyhow::{Result, Error, anyhow};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Op {
    Noop, 
    Addx(i32)
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim().split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["noop"] => Ok(Op::Noop),
            ["addx", num] => {
                let num: i32 = num.parse().map_err(|_| anyhow!("Could not parse number component of add op"))?;
                Ok(Op::Addx(num))
            },
            _ => Err(anyhow!("Invalid string as input for parsing Op enum"))
        }
    }
}

fn get_signal_strengths(ops: &Vec<Op>) -> Vec<i32> {
    let mut cycle_count = 0;
    let mut x = 1;
    let mut res = vec![];

    for op in ops {
        match op {
            Op::Noop => {
                cycle_count += 1;

                if cycle_count == 20 || (cycle_count - 20) % 40 == 0 {
                    // print!("{:4}|{:5} ", x, x * cycle_count);
                    res.push(x * cycle_count);
                    print!("X: {:+3}, CC: {:4}, last: {:+5}\n", x, cycle_count, res.last().unwrap_or(&0));
                }
            },
            Op::Addx(num) => {
                if cycle_count == 19 || (cycle_count - 20) % 40 == 39 {
                    // print!("{:4}|{:5} ", x, x * (cycle_count + 1));

                    res.push(x * (cycle_count + 1));
                    print!("X: {:+3}, CC: {:4}, last: {:+5}\n", x, cycle_count + 1, res.last().unwrap_or(&0));
                }


                cycle_count += 2;
                

                if cycle_count == 20 || (cycle_count - 20) % 40 == 0  {
                    // print!("{:4}|{:5} ", x, x * cycle_count);
                    res.push(x * cycle_count);
                    print!("X: {:+3}, CC: {:4}, last: {:+5}\n", x, cycle_count, res.last().unwrap_or(&0));
                }

                x += num;
            }
        }
    }

    res
}

fn main() {
    let input = include_str!("input.txt");
    let ops: Vec<Op> = input.lines().filter_map(|line| line.parse::<Op>().ok()).collect();
    let signal_strengths = get_signal_strengths(&ops);
    let sum: i32 = signal_strengths.iter().sum();
    
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        let ops: Vec<Op> = input.lines().filter_map(|line| line.parse::<Op>().ok()).collect();
        println!("OPs: ");
        for op in ops.iter() {
            match op {
                Op::Noop => {},
                Op::Addx(num) => print!("a{:3} ", num)
            }
        }
        println!("");
        let sum_of_signal_strengths = get_signal_strengths(&ops);
        
        assert_eq!(13_140, sum_of_signal_strengths.iter().sum());
    }
}