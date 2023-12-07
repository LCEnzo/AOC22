use std::time::Instant;

fn calc_solution_1(input: &str) -> u32 {
    todo!()
}

fn calc_solution_2(input: &str) -> u64 {
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
    //     solution / repeat_amount as u64
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(142, calc_solution_1(input));
    }
}
