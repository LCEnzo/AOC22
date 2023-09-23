use std::cmp::max;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

fn main() {
    let input_path = Path::new("input.txt");
    let input_file = File::open(input_path).expect("Could not open file input.txt");
    let reader = BufReader::new(input_file);


}
