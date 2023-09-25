use itertools::Itertools;

fn main() {
	let input = include_str!("input.txt");
	let mut sum: u32 = 0;

	for line in input.lines() {
		if !line.is_empty() {
			let rucksack_sum = sum_of_common_priorities_of_rucksack(line.to_string());
			// println!("{rucksack_sum}");
			sum += rucksack_sum;
		}
	}

	println!("\nSum: {sum}")
}

fn sum_of_common_priorities_of_rucksack(rucksack: String) -> u32 {
	println!("\t {rucksack}");
	let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);

	find_common_elements(&comp1, &comp2)
		.iter()
		.map(|x| type_to_score(*x))
		.fold(0, |acc, x| acc + x)
}

fn find_common_elements(left_side: &str, right_side: &str) -> Vec<char> { 
	// println!("\t - intersection of {left_side} {right_side}:");
	let mut intersection = vec![];
	let left_side = left_side.chars().sorted().collect::<String>();
	let right_side = right_side.chars().sorted().collect::<String>();

	println!("\t - sorted          {left_side} {right_side} = | ");

	let bytes = right_side.as_bytes();
	let mut index = 0;
	'outer: for c in left_side.chars() {
		let mut c2 = bytes[index] as char;

		while c > c2 {
			index += 1;
			if index >= bytes.len() {
				break 'outer;
			}

			c2 = bytes[index] as char;
		}

		if c == c2 {
			intersection.push(c);
			break;
		}
	}

	let c = intersection[0];
	let val = type_to_score(c);
	println!("\t intersection = {c} = {val}\n");

	intersection
}


fn type_to_score(t: char) -> u32 {
	let letter_value = t.to_ascii_lowercase() as u32 - 'a' as u32 + 1;
	let case_value = if t.is_lowercase() { 0 } else { 26 };
	
	letter_value + case_value
}
