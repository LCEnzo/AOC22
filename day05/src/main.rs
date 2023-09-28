use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Move {
    count: usize,
    origin: usize,
    destination: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct MoveParseError;

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, origin, destination) = s
            .split(' ')
            .take(7)
            .filter_map(|x| x.parse().ok())
            .collect_tuple()
            .ok_or(MoveParseError)?;
        // reduce by one since towers start at 1, and indices at 0
        let origin = origin - 1;
        let destination = destination -1;

        Ok(Move{count, origin, destination})
    }
}

fn main() {
    let input_txt = include_str!("input.txt");
    // Split input text into initial state (towers) and moves to be executed
    let (towers, moves) = input_txt.split("\n\n").take(2).collect_tuple().unwrap();

    // Parse moves
    let moves = moves
        .split('\n')
        .flat_map(|line| Move::from_str(line).ok())
        .collect::<Vec<Move>>();

    // Get number of towers by getting bottom lines with tower numbers and getting max. 
    // The iterator left over will be used to read the towers.
    let mut tower_and_places = towers.split('\n').rev();
    let top = tower_and_places.next().expect("Input_txt should split into moves and the towers. The towers should have at least one line.");
    let tower_count = top.split(' ').flat_map(|s| s.parse::<usize>().ok()).max().unwrap_or(0);

    // Create vector of vectors to be the state
    let mut towers: Vec<Vec<char>> = Vec::new();
    for _ in 0..tower_count {
        towers.push(vec![])
    }

    for line in tower_and_places {
        line
            .chars()
            .enumerate()
            .filter(|(index, c)| index % 4 == 1 && c.is_alphabetic())
            .for_each(|(index, el)| {
                towers[(index-1)/4].push(el)
            });
    }
    
    let first_answer = apply_moves_to_towers(&towers, &moves, true);
    let second_answer = apply_moves_to_towers(&towers, &moves, false);

    let first_output = first_answer.iter().map(|vec| vec.into_iter().rev().next().unwrap_or(&' ')).join("");
    let second_output = second_answer.iter().map(|vec| vec.into_iter().rev().next().unwrap_or(&' ')).join("");

    println!("{first_output}");
    println!("{second_output}");
}

fn apply_moves_to_towers(towers: &Vec<Vec<char>>, moves: &Vec<Move>, reverse_appending: bool) -> Vec<Vec<char>> {
    let mut towers = towers.to_vec();

    for mv in moves {
        let top_of_the_tower_range = (towers[mv.origin].len() - mv.count)..towers[mv.origin].len();
        let to_move = towers[mv.origin].drain(top_of_the_tower_range).collect::<Vec<char>>();
        if reverse_appending {
            towers[mv.destination].extend(to_move.iter().rev());
        } else {
            towers[mv.destination].extend(to_move.iter());
        }
    }

    towers
}
