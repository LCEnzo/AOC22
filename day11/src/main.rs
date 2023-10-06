use std::{str::FromStr, cmp::Ordering};

use anyhow::{Result, anyhow, Error, Ok};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Add,
    Mult
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim() {
            "*" => Ok(Op::Mult),
            "+" => Ok(Op::Add),
            _ => Err(anyhow!("Could not parse op."))
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Operand {
    Num(i64),
    Old
}

impl FromStr for Operand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim() {
            "old" => Ok(Operand::Old),
            num => Ok(Operand::Num(num.parse::<i64>()?))
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct MonkeyOp {
    operand1: Operand, 
    op: Op, 
    operand2: Operand
}

impl FromStr for MonkeyOp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.split(" = ").last().ok_or(anyhow!("Could not split line by '=' to parse MonkeyOp"))?;
        let os: Vec<_> = s.split_ascii_whitespace().take(3).collect();

        if os.len() < 3 {
            return Err(anyhow!("Couldn't parse Monkey operation"));
        }
        
        let oper1 = os[0].parse::<Operand>()?;
        let op = os[1].parse::<Op>()?;
        let oper2 = os[2].parse::<Operand>()?;
    
        Ok(MonkeyOp{operand1: oper1, op, operand2: oper2})
    }
}

impl MonkeyOp {
    fn apply_op(&self, old_val: i64) -> i64 {
        let op1 = match self.operand1 {
            Operand::Old => old_val,
            Operand::Num(num) => num,
        };

        let op2 = match self.operand2 {
            Operand::Old => old_val,
            Operand::Num(num) => num,
        };

        match self.op {
            Op::Add => op1 + op2,
            Op::Mult => op1 * op2,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Test {
    div: i64,
    truthy: usize,
    falsy: usize
}

impl FromStr for Test {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let nums = s
            .lines()
            // .map(|line| line.split_ascii_whitespace())
            .filter_map(|line|
                line
                    .split_ascii_whitespace()
                    .filter_map(|word| word.parse::<u32>().ok())
                    .fold(None, |_acc, x| Some(x))
            )
            .collect::<Vec<u32>>();

        let mut nums_iter = nums.iter();

        let div = nums_iter.next().ok_or(anyhow!("Could not parse Test"))?.clone() as i64;
        let truthy = nums_iter.next().ok_or(anyhow!("Could not parse Test"))?.clone() as usize;
        let falsy = nums_iter.next().ok_or(anyhow!("Could not parse Test"))?.clone() as usize;

        Ok(Test{div, truthy, falsy})
    }
}

impl Test {
    fn test(self, val: i64) -> usize {
        if val % self.div == 0 {
            self.truthy
        } 
        else {
            self.falsy
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey {
    index: usize,
    items: Vec<i64>,
    operation: MonkeyOp,
    test: Test,
    inspect_count: i64,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let index_regex = regex::Regex::new(r"^Monkey (\d+):$").unwrap();
        let numbers_regex = regex::Regex::new(r"(\d+)").unwrap();
        let mut lines = s.lines();

        let index = index_regex
            .captures(
                lines.next()
                .ok_or(anyhow!("String to parse as Monkey did not include one line."))?
            ).and_then(|capture| 
                capture
                    .get(1)
                    .and_then(|match_| match_.as_str().parse::<usize>().ok())
            ).ok_or(anyhow!("Could not parse monkey index"))?;

        let items = numbers_regex
            .captures_iter(lines.next().ok_or(anyhow!("String to parse as Monkey did not include enough lines."))?)
            .filter_map(|captures| 
                captures.get(0).map(|match_| 
                    match_.as_str().parse::<i64>()
                    .ok()
                )
            )
            .collect::<Option<Vec<i64>>>()
            .ok_or(anyhow!("Failed to parse items"))?;

        let operation = lines
            .next()
            .ok_or(anyhow!("String to parse as Monkey did not include enough lines."))?
            .parse::<MonkeyOp>()?;

        let test = lines
            .take(3)
            .map(|line| line.to_string())
            .fold("".to_string(), |acc, line| acc.to_owned() + &line + "\n")
            .parse::<Test>()?;

        Ok(Monkey { index, items, operation, test, inspect_count: 0 })
    }
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}: ", self.index)?;
        let items_str: Vec<String> = self.items.iter().map(|&item| item.to_string()).collect();
        write!(f, "{}", items_str.join(", "))
    }
}

fn parse_input_to_monkeys(string: &str) -> Vec<Monkey> {
    string
        .split("\n\n")
        .filter_map(|monkey_str| monkey_str.parse::<Monkey>().ok())
        .collect()
}

static ROUNDS: u32 = 20;

fn simulate_round(monkeys: &mut Vec<Monkey>) {
    let len = monkeys.len();

    for ind in 0..len {
        // Assumes a monkey can't pass to himself
        for item_ind in 0..monkeys[ind].items.len() {
            let new_val = monkeys[ind].operation.apply_op(monkeys[ind].items[item_ind]) / 3;
            let new_ind = monkeys[ind].test.test(new_val);

            assert_ne!(new_ind, monkeys[ind].index);
            assert!(new_ind < len);

            monkeys[new_ind].items.push(new_val);
            monkeys[ind].inspect_count += 1;
        }

        monkeys[ind].items.clear();
    }

    for mon in monkeys.iter() {
        println!("{}", mon);
    }

    println!("");
}

fn calc_score(monkeys: &Vec<Monkey>) -> i64 {
    let mut monkeys = monkeys.clone();

    monkeys.sort_by(|a, b| {
        let cm = b.inspect_count.cmp(&a.inspect_count);

        if cm == Ordering::Equal {
            let asum: i64 = a.items.iter().sum();
            let bsum: i64 = b.items.iter().sum();
            return bsum.cmp(&asum);
        }

        cm
    });

    monkeys.iter()
        .take(2)
        .fold(1, |acc, x| acc * x.inspect_count)
}

fn main() {
    let input = include_str!("input.txt");
    let mut monkeys = parse_input_to_monkeys(input);

    for mon in monkeys.iter() {
        println!("{}", mon);
    }

    println!("");

    for _ in 0..ROUNDS {
        simulate_round(&mut monkeys);
    }

    let score = calc_score(&monkeys);
    println!("{}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        let mut monkeys = parse_input_to_monkeys(input);

        for _ in 0..ROUNDS {
            simulate_round(&mut monkeys);
        }
    
        let res = calc_score(&monkeys);
        assert_eq!(10_605, res);
    }
}
