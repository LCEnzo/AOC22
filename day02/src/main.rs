use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn calc_score(opp: char, my: char) -> i32 {
	let mut score = 0;

	return score;
}

fn main() {
	let input_path = Path::new("input.txt");
	let input_file = File::open(input_path).expect("Could not open file input.txt");
	let reader = BufReader::new(input_file);

	let it = vec![vec!['A', 'B', 'C'], vec!['X', 'Y', 'Z']].into_iter();
	for v in Itertools::multi_cartesian_product(it) {
		let score = calc_score(v[0], v[1]);
		let str = v.iter().join(" ");
		println!("{str}");
	}
}
