use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct MoveParseError;

#[derive(Debug, PartialEq)]
struct Move {
    count: usize,
    origin: usize,
    destination: usize,
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, origin, destination) = s
            .split(' ')
            .take(7)
            .filter_map(|x| x.parse().ok())
            .collect_tuple()
            .ok_or(MoveParseError)?;
        // reduce by one since stacks start at 1, and indices at 0
        let origin = origin - 1;
        let destination = destination - 1;

        Ok(Move {
            count,
            origin,
            destination,
        })
    }
}

fn parse_moves(moves_str: String) -> Vec<Move> {
    moves_str
        .split('\n')
        .flat_map(|line| Move::from_str(line).ok())
        .collect::<Vec<Move>>()
}

fn parse_stacks(stack_str: String) -> Vec<Vec<char>> {
    // Get number of stacks by getting bottom lines with stack numbers and getting max.
    // The iterator left over will be used to read the stacks.
    let mut stacks_and_indices = stack_str.split('\n').rev();
    let top = stacks_and_indices.next().expect("Input_txt should split into moves and the stacks. The stacks should have at least one line.");
    let stack_count = top
        .split(' ')
        .flat_map(|s| s.parse::<usize>().ok())
        .max()
        .unwrap_or(0);

    // Create vector of vectors to be the state
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];

    for line in stacks_and_indices {
        line.chars()
            .enumerate()
            .filter(|(index, c)| index % 4 == 1 && c.is_alphabetic())
            .for_each(|(index, el)| stacks[(index - 1) / 4].push(el));
    }

    stacks
}

fn apply_moves_to_stacks(
    stacks: &Vec<Vec<char>>,
    moves: &Vec<Move>,
    reverse_appending: bool,
) -> Vec<Vec<char>> {
    let mut stacks = stacks.to_vec();

    for mv in moves {
        let top_of_the_stack_range = (stacks[mv.origin].len() - mv.count)..stacks[mv.origin].len();
        let to_move = stacks[mv.origin]
            .drain(top_of_the_stack_range)
            .collect::<Vec<char>>();
        if reverse_appending {
            stacks[mv.destination].extend(to_move.iter().rev());
        } else {
            stacks[mv.destination].extend(to_move.iter());
        }
    }

    stacks
}

fn main() {
    let input_txt = include_str!("input.txt");
    // Split input text into initial state (stacks) and moves to be executed
    let (stacks, moves) = input_txt.split("\n\n").take(2).collect_tuple().unwrap();

    // Parse moves and stacks
    let moves = parse_moves(moves.to_string());
    let stacks = parse_stacks(stacks.to_string());

    // stacks after moves as per first and second half rules
    let first_answer = apply_moves_to_stacks(&stacks, &moves, true);
    let second_answer = apply_moves_to_stacks(&stacks, &moves, false);

    // stringify
    let first_output = first_answer
        .iter()
        .map(|vec| vec.into_iter().rev().next().unwrap_or(&' '))
        .join("");
    let second_output = second_answer
        .iter()
        .map(|vec| vec.into_iter().rev().next().unwrap_or(&' '))
        .join("");

    println!("{first_output}");
    println!("{second_output}");
}
