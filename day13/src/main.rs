use std::str::Lines;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Elem {
    Num(u32),
    List(Vec<Elem>)
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Elem::Num(n1), Elem::Num(n2)) => n1.partial_cmp(n2),
            (Elem::Num(num), Elem::List(l2)) => {
                if l2.is_empty() {
                    Some(Ordering::Greater)
                } else {
                    Elem::List(vec![Elem::Num(*num)]).partial_cmp(other)
                }
            },
            (Elem::List(_), Elem::Num(_)) => Some(other.partial_cmp(self).unwrap().reverse()),
            (Elem::List(l1), Elem::List(l2)) => l1.partial_cmp(l2),
        }
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Elem {
    fn match_tokens(tokens: &[char]) -> (Elem, usize) {
        let len = tokens.len();
        let mut index = 0;
        let mut list: Vec<Elem> = vec![];
        let mut num = None;

        while index < len {
            match tokens[index] {
                '[' if index == 0 => { list = vec![] },
                '[' => {
                    let (el, parsed_count) = Elem::match_tokens(&tokens[index..]);
                    index += parsed_count;
                    list.push(el);
                }
                ',' | ' ' => {
                    if let Some(num) = num {
                        list.push(Elem::Num(num));
                    }
                    
                    num = None;
                },
                ']' => {
                    if let Some(num) = num {
                        list.push(Elem::Num(num));
                    }
                    
                    return (Elem::List(list), index)
                },
                c => {
                    if num == None {
                        num = c.to_digit(10)
                    } else {
                        num = Some(10 * num.unwrap() + c.to_digit(10).unwrap())
                    }
                    
                }
            }

            index += 1;
        }

        (Elem::List(list), index)
    }

    fn parse_list(input: &str) -> Option<Self> {
        let trimmed = input.trim();
        let chars: Vec<_> = trimmed.chars().collect();

        let (list, index) = Self::match_tokens(&chars);

        println!("Parse list {} \n\t(len, end_index): ({}, {})", trimmed, chars.len(), index);
        print!("\t");
        list.print();
        println!("");
        assert_eq!(chars.len() - 1, index);

        Some(list)
    }

    fn print(&self) {
        match self {
            Elem::Num(num) => print!("{} ", num),
            Elem::List(vec) => {
                print!("[");
                for el in vec {
                    el.print();
                }
                print!("]");
            }
        }
    }
}

struct Pair {
    l1: Elem,
    l2: Elem
}

impl Pair {
    fn is_in_order(self: &Self) -> bool {
        self.l1 < self.l2
    }

    fn parse_pair(lines: &mut Lines) -> Option<Self> {
        let l1 = Elem::parse_list(lines.next()?)?;
        let l2 = Elem::parse_list(lines.next()?)?;

        Some(Pair { l1, l2 })
    }
}

fn parse_pairs(input: &str) -> Vec<Pair> {
    input.split("\n\n")
        .filter_map(|section| Pair::parse_pair(&mut section.lines()))
        .collect()
}

fn calc_target_sum(pairs: &Vec<Pair>) -> u32 {
    pairs.iter()
        .enumerate()
        .filter(|(_, pair)| pair.is_in_order())
        .map(|(ind, _)| (ind + 1) as u32)
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    let pairs: Vec<Pair> = parse_pairs(&input);
    let sum_of_right_ordered_pairs_indices = calc_target_sum(&pairs);

    println!("{}", sum_of_right_ordered_pairs_indices);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        let pairs: Vec<Pair> = parse_pairs(&input);
        let sum_of_right_ordered_pairs_indices = calc_target_sum(&pairs);

        assert_eq!(13, sum_of_right_ordered_pairs_indices);
    }
}
