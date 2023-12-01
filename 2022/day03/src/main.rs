use itertools::Itertools;

fn main() {
	let input = include_str!("input.txt");
	let mut rucksack_sum: u32 = 0;
	let mut group_sum: u32 = 0;
	let mut lines = input.lines().peekable();

	while lines.peek() != None {
		let line1 = lines.next();
		if let Some(li) = line1 {
			rucksack_sum += sum_of_common_priorities_of_rucksack(li.to_string());
		} else {
			break;
		}

		let line2 = lines.next();
		if let Some(li) = line2 {
			rucksack_sum += sum_of_common_priorities_of_rucksack(li.to_string());
		} else {
			break;
		}

		let line3 = lines.next();
		if let Some(li) = line3 {
			rucksack_sum += sum_of_common_priorities_of_rucksack(li.to_string());
		} else {
			break;
		}

		group_sum += find_group_score(line1.unwrap(), line2.unwrap(), line3.unwrap());
	}

	println!("\nrucksack_sum: {rucksack_sum}");
	println!("group_sum: {group_sum}");
}

fn find_group_score(l1: &str, l2: &str, l3: &str) -> u32 {
	let top = l1.chars().sorted().collect::<String>();
	let mid = l2.chars().sorted().collect::<String>();
	let bot = l3.chars().sorted().collect::<String>();

	let mid = mid.as_bytes();
	let bot = bot.as_bytes();
	let mut mid_index = 0;
	let mut bot_index = 0;

	'outer: for c in top.chars() {
		let mut c2 = mid[mid_index] as char;
		let mut c3 = bot[bot_index] as char;

		while c > c2 {
			mid_index += 1;
			if mid_index >= mid.len() {
				break 'outer;
			}

			c2 = mid[mid_index] as char;
		}

		while c2 > c3 {
			bot_index += 1;
			if bot_index >= bot.len() {
				break 'outer;
			}

			c3 = bot[bot_index] as char;
		}

		if c == c2 && c2 == c3 {
			return type_to_score(c);
		}
	}

	let top = l1.chars().sorted().collect::<String>();
	let mid = l2.chars().sorted().collect::<String>();
	let bot = l3.chars().sorted().collect::<String>();
	println!("Intersection not found");
	println!("\t - {top}");
	println!("\t - {mid}");
	println!("\t - {bot}");
	panic!("Did not find common elements between 3 elves");
}

fn sum_of_common_priorities_of_rucksack(rucksack: String) -> u32 {
	let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
	type_to_score(find_common_elements(&comp1, &comp2))
}

fn find_common_elements(left_side: &str, right_side: &str) -> char { 
	let left_side = left_side.chars().sorted().collect::<String>();
	let right_side = right_side.chars().sorted().collect::<String>();

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
			return c;
		}
	}

	panic!("Did not find common elements between 2 compartments of a ruckasck");
}


fn type_to_score(t: char) -> u32 {
	let letter_value = t.to_ascii_lowercase() as u32 - 'a' as u32 + 1;
	let case_value = if t.is_lowercase() { 0 } else { 26 };
	
	letter_value + case_value
}
