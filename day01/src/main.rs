use std::cmp::max;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub, Mul, Rem, Div};

fn calc_new_max_sums<T: PartialEq + Ord + Add + Sub + Mul + Rem + Div + Copy>(max_sums: &mut Vec<T>, new_sum: T) {
    max_sums[0] = max(max_sums[0], new_sum);
    if max_sums[1] < max_sums[0] {
        max_sums.swap(1, 0);
    }
    if max_sums[2] < max_sums[1] {
        max_sums.swap(2, 1);
    }
}

fn main() {
    let input_path = Path::new("input_a.txt");
    let input_file = File::open(input_path).expect("Could not open file input_a.txt");
    let reader = BufReader::new(input_file);
    let mut curr_sum: u32 = 0;
    let mut max_sums = vec![0, 0, 0];

    for line in reader.lines() {
        match line {
            Ok(text) => {
                if text.is_empty() || char::is_whitespace(text.as_str().chars().nth(0).unwrap()) {
                    calc_new_max_sums(&mut max_sums, curr_sum);
	                curr_sum = 0;
                } else {
                    curr_sum += text.parse::<u32>().expect("Unable to parse value to unsigned int");
                }
            }
            Err(err) => {
                panic!("{err}");
            }
        }
    }

    for sum in max_sums.iter() {
        print!("{sum} ")
    }
    print!("\nSum: {}\n", max_sums.iter().sum::<u32>());
}
