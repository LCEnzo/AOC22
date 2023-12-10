use std::time::Instant;
use regex::Regex;
use core::str::FromStr;

struct Draw {
    red: u32,
    green: u32, 
    blue: u32,
}

struct Game {
    index: u32,
    draws: Vec<Draw>
}

impl FromStr for Draw {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<number>\d+) (?P<color>green|blue|red)")?;
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        for cap in re.captures_iter(s) {
            let number: u32 = cap["number"].parse().unwrap_or(0);
            match &cap["color"] {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                _ => {}
            }
        }

        Ok(Draw { red, green, blue })
    }
}

impl Draw {
    fn is_valid(&self, max_blue: u32, max_green: u32, max_red: u32) -> bool {
        self.blue <= max_blue && self.green <= max_green && self.red <= max_red
    }
}

fn parse_input(input: &str) -> Option<Vec<Game>> {
    Some(
        input
            .lines()
            .flat_map(|line| // Skip over the 'Game index:' part
                line.split(':').skip(1).next()
            )
            .map(|line| // parse game into a vector of Draw structs
                    line.split(';')
                    .flat_map(|draw_str|
                        draw_str.parse::<Draw>().ok()
                    )
                    .collect::<Vec<Draw>>()
            )
            .enumerate() // enumerate to get indices of Games
            .map(|(index, draws)| Game{index: index as u32 + 1, draws: draws})
            .collect::<Vec<_>>()
    )
}

impl Game {
    fn is_valid(&self, max_blue: u32, max_green: u32, max_red: u32) -> bool {
        self.draws.iter().all(|draw| draw.is_valid(max_blue, max_green, max_red))
    }

    fn minimum_cubes(&self) -> Draw {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        self.draws.iter().for_each(|draw| {
            red = std::cmp::max(red, draw.red);
            green = std::cmp::max(green, draw.green);
            blue = std::cmp::max(blue, draw.blue);
        });

        Draw {red, green, blue}
    }

    fn power(&self) -> u32 {
        let min = self.minimum_cubes();
        min.red * min.green * min.blue
    }
}

fn calc_solution_1_with_args(input: &str, max_blue: u32, max_green: u32, max_red: u32) -> u32 {
    parse_input(input).unwrap().iter().filter(|game| game.is_valid(max_blue, max_green, max_red)).map(|game| game.index).sum()
}

fn calc_solution_1(input: &str) -> u32 {
    calc_solution_1_with_args(input, 14, 13, 12)
}

fn calc_solution_2(input: &str) -> u32 {
    parse_input(input).unwrap().iter().map(|game| game.power()).sum()
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
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(8, calc_solution_1(input));
    }

    #[test]
    fn test_first_half_on_real_input() {
        let input = include_str!("input.txt");
        assert_eq!(2239, calc_solution_1(input));
    }

    #[test]
    fn test_second_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(2286, calc_solution_2(input));
    }

    #[test]
    fn test_second_half_on_real_input() {
        let input = include_str!("input.txt");
        assert_eq!(83435, calc_solution_2(input));
    }
}
