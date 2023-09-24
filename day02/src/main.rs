fn main() {
	let input = include_str!("input.txt");
	let mut old_score = 0;
	let mut new_score = 0;

	for line in input.lines() {
		if line.len() > 2 {
			let opponent_move = line.chars().nth(0).unwrap();
			let my_move = line.chars().nth(2).unwrap();

			old_score += calc_old_score(opponent_move, my_move).unwrap();
			new_score += calc_new_score(opponent_move, my_move).unwrap();
		}
	}

	println!("Old score: {old_score}");
	println!("New score: {new_score}");
}

fn calc_old_score(opp: char, my: char) -> Result<i32, String> {
	let hand_score = match my {
		'X' => 1,
		'Y' => 2,
		'Z' => 3,
		_ => return Err(format!("Invalid char as own move {my}")),
	};

	let outcome_score = match (opp, my) {
		('A', 'X') => 3,
		('A', 'Y') => 6,
		('A', 'Z') => 0,
		('B', 'X') => 0,
		('B', 'Y') => 3,
		('B', 'Z') => 6,
		('C', 'X') => 6,
		('C', 'Y') => 0,
		('C', 'Z') => 3,
		_ => return Err(format!("Invalid char/s as moves ({opp}, {my})")),
	};

	Ok(hand_score + outcome_score)
}

fn calc_new_score(opp: char, tactic: char) -> Result<i32, String> {
	let my_move: char = match (opp, tactic) {
		// (A, X) means my opponent uses rock and I must lose, ergo use Z for scissors
		('A', 'X') | ('B', 'Z') | ('C', 'Y') => 'Z',
		('A', 'Y') | ('B', 'X') | ('C', 'Z') => 'X',
		('A', 'Z') | ('B', 'Y') | ('C', 'X') => 'Y',
		_ => return Err(format!("Invalid opponent move or my tactic ({opp}, {tactic})")),
	};

	calc_old_score(opp, my_move)
}