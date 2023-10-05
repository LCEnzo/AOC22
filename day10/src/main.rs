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

fn calc_register_values_at_cycle(ops: &Vec<Op>) -> Vec<i32> {
    let mut signals = vec![];
    let mut x = 1;

    for op in ops.iter() {
        match op {
            Op::Noop => {
                signals.push(x);
            },
            Op::Addx(num) => {
                signals.push(x);
                signals.push(x);
                x += num;
            }
        }
    }

    signals
}

fn calc_signal_strengths(register_values: &Vec<i32>) -> Vec<i32> {
    register_values
        .iter()
        .enumerate()
        .filter(|(cycle, _)| (cycle + 21) % 40 == 0)
        .map(|(cycle, register_val)| register_val * (cycle + 1) as i32)
        .collect()
}

fn get_display_pixels(register_values: &Vec<i32>) -> Vec<char> {
    register_values
        .iter()
        .enumerate()
        .map(|(cycle, val)| ((cycle + 1) % 40, val))
        .map(|(cycle, reg_val)|
            if (reg_val+0..=reg_val+2).contains(&(cycle as i32)) {
                '#'
            } else {
                '.'
            }
        ).collect()
}

fn main() {
    let input = include_str!("input.txt");
    let ops: Vec<Op> = input.lines().filter_map(|line| line.parse::<Op>().ok()).collect();
    let register_values = calc_register_values_at_cycle(&ops);
    let sum: i32 = calc_signal_strengths(&register_values).iter().sum();
    
    println!("{}", sum);

    let pixels = get_display_pixels(&register_values);
    for line in pixels.chunks(40) {
        let line: String = line.iter().collect();
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        let ops: Vec<Op> = input.lines().filter_map(|line| line.parse::<Op>().ok()).collect();
        let register_values = calc_register_values_at_cycle(&ops);
        let sum = calc_signal_strengths(&register_values).iter().sum();
        assert_eq!(13_140, sum);
    }
}