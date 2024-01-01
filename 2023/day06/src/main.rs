use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    fn does_time_win(&self, holding_time: u64) -> bool {
        holding_time * (self.time - holding_time) > self.dist
    }

    fn search_for_bound(&self, left: bool) -> u64 {
        let mut l = if left { 0 } else { self.time / 2 };
        let mut r = if left { self.time / 2 } else { self.time };

        while l <= r {
            let s = (r + l) / 2;

            if self.does_time_win(s) {
                if left {
                    r = s - 1;
                } else {
                    l = s + 1;
                }
            } else if left {
                l = s + 1;
            } else {
                r = s - 1;
            }
        }

        if left {
            l
        } else {
            r
        }
    }

    fn count_ways_to_win(&self) -> u64 {
        let left_bound = self.search_for_bound(true);
        let right_bound = self.search_for_bound(false);
        right_bound - left_bound + 1
    }
}

fn parse(input: &str) -> Option<Vec<Race>> {
    let mut lines = input.lines();
    let times = lines
        .next()?
        .split(':')
        .nth(1)?
        .split_ascii_whitespace()
        .filter_map(|num| num.trim().parse::<u64>().ok());
    let dists = lines
        .next()?
        .split(':')
        .nth(1)?
        .split_ascii_whitespace()
        .filter_map(|num| num.trim().parse::<u64>().ok());

    Some(
        times
            .zip(dists)
            .map(|(time, dist)| Race { time, dist })
            .collect(),
    )
}

fn calc_solution_1(input: &str) -> u64 {
    let races = parse(input).expect("Expected to successfully parse input in calc solution 1");
    races.iter().map(|race| race.count_ways_to_win()).product()
}

fn parse2(input: &str) -> Option<Race> {
    let mut lines = input.lines();
    let time_parts: Vec<_> = lines
        .next()?
        .split(':')
        .nth(1)?
        .split_ascii_whitespace()
        .map(|num| num.trim())
        .collect();
    let time = time_parts.join("").parse::<u64>().ok()?;

    println!("Got time {}", time);

    let dist_parts: Vec<_> = lines
        .next()?
        .split(':')
        .nth(1)?
        .split_ascii_whitespace()
        .map(|num| num.trim())
        .collect();
    let dist = dist_parts.join("").parse::<u64>().ok()?;

    println!("Got dist {}", dist);

    Some(Race { time, dist })
}

fn calc_solution_2(input: &str) -> u64 {
    let race = parse2(input).expect("Expected to properly parse race");
    race.count_ways_to_win()
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

    let start = Instant::now();
    let solution = calc_solution_2(input);
    let elapsed2 = start.elapsed();
    println!(
        "2 took: {}s {}ms {}μs\nSolution:\n\t{}\n",
        elapsed2.as_secs(),
        elapsed2.subsec_millis(),
        elapsed2.subsec_micros() % 1000,
        solution
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_binary_search_left_bound() {
        let input = include_str!("test_input.txt");
        let races = parse(input).unwrap();
        for (race, res) in races.iter().take(3).zip([2, 4, 11]) {
            assert_eq!(res, race.search_for_bound(true));
        }
    }

    #[test]
    fn test_first_binary_search_right_bound() {
        let input = include_str!("test_input.txt");
        let races = parse(input).unwrap();
        for (race, res) in races.iter().take(3).zip([5, 11, 19]) {
            assert_eq!(res, race.search_for_bound(false));
        }
    }

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(288, calc_solution_1(input));
    }

    #[test]
    fn test_first_half_on_real_input() {
        let input = include_str!("input.txt");
        assert_eq!(3317888, calc_solution_1(input));
    }

    #[test]
    fn test_second_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(71503, calc_solution_2(input));
    }

    #[test]
    fn test_second_half_on_real_input() {
        let input = include_str!("input.txt");
        assert_eq!(24655068, calc_solution_2(input));
    }
}
