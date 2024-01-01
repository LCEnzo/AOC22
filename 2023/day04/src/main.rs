use std::time::Instant;

fn parse(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(&[':', '|'][..]).skip(1);

            (
                it.next().expect("Expected : to split input line"),
                it.next().expect("Expected | to split input line into 3"),
            )
        })
        .map(|(winning_nums, chosen_numbers)| {
            (
                winning_nums
                    .split(' ')
                    .map(|num| num.trim())
                    .filter(|num| !num.is_empty())
                    .map(|num| {
                        num.trim()
                            .parse::<u32>()
                            .expect("winning_nums should include only numbers")
                    })
                    .collect::<Vec<u32>>(),
                chosen_numbers
                    .split(' ')
                    .map(|num| num.trim())
                    .filter(|num| !num.is_empty())
                    .map(|num| {
                        num.trim()
                            .parse::<u32>()
                            .expect("chosen_numbers should include only numbers")
                    })
                    .collect::<Vec<u32>>(),
            )
        })
        .collect()
}

fn calc_solution_1(input: &str) -> u32 {
    let cards = parse(input);

    cards
        .iter()
        .map(|(winning_numbers, chosen_numbers)| {
            chosen_numbers
                .iter()
                .filter(|num| winning_numbers.contains(num))
                .count()
        })
        .filter(|num| *num > 0)
        .map(|num| (2_u32).pow(num as u32 - 1))
        .sum()
}

fn calc_solution_2(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = include_str!("input.txt");

    let start = Instant::now();
    let solution = calc_solution_1(input);
    let elapsed1 = start.elapsed();
    println!(
        "1 took: {}s {}ms {}μs\nSolution:\n\t{}\n",
        elapsed1.as_secs(),
        elapsed1.subsec_millis(),
        elapsed1.subsec_micros() % 1000,
        solution
    );

    // let start = Instant::now();
    // let solution = calc_solution_2(input);
    // let elapsed2 = start.elapsed();
    // println!(
    //     "2 took: {}s {}ms {}μs\nSolution:\n\t{}\n",
    //     elapsed2.as_secs(),
    //     elapsed2.subsec_millis(),
    //     elapsed2.subsec_micros() % 1000,
    //     solution
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(13, calc_solution_1(input));
    }

    #[test]
    fn test_first_half_on_real_input() {
        let input = include_str!("input.txt");
        assert_eq!(21821, calc_solution_1(input));
    }

    // #[test]
    // fn test_second_half() {
    //     let input = include_str!("test_input.txt");
    //     assert_eq!(467835, calc_solution_2(input));
    // }

    // #[test]
    // fn test_second_half_on_real_input() {
    //     let input = include_str!("input.txt");
    //     assert_eq!(80179647, calc_solution_2(input));
    // }
}
