use std::time::Instant;
use std::ops::Range;

type Seeds = Vec<i64>;

#[derive(Clone, Copy, Debug)]
struct Mapping {
    src_start: i64,
    dst_start: i64,
    len: i64
}

impl Mapping {
    fn src(&self) -> Range<i64> {
        return self.src_start..(self.src_start + self.len)
    }

    fn dst(&self) -> Range<i64> {
        return self.dst_start..(self.dst_start + self.len)
    }

    fn map(&self, num: i64) -> i64 {
        // println!("\t\t\tmapped by: {}, {}, {}", self.src_start, self.dst_start, self.len);
        if self.src().contains(&num) {
            num + (self.dst_start - self.src_start)
        } else {
            num
        }
    }

    fn rev_map(&self, num: i64) -> i64 {
        if self.dst().contains(&num) {
            num + (self.src_start - self.dst_start)
        } else {
            num
        }
    }
}

type Map = Vec<Mapping>;

fn apply_map(val: i64, map: &Map) -> i64 {
    map
        .iter()
        .filter(|mapping| mapping.src().contains(&val))
        .map(|mapping| mapping.map(val))
        .last()
        .unwrap_or_else(|| val)
} 

fn rev_apply_map(val: i64, map: &Map) -> i64 {
    todo!();
} 

type Maps = Vec<Map>;

fn parse_input(input: &str) -> Option<(Seeds, Maps)> {
    let mut sections = if !input.contains("\r\n") { input.split("\n\n") } else { input.split("\r\n\r\n") };

    let seeds: Seeds = sections
        .next()?
        .split_whitespace()
        .flat_map(|el| el.parse::<i64>())
        .collect();

    let maps: Maps = 
        sections
        .map(|section| section.lines().skip(1))
        .map(|lines| 
            lines.filter_map(|line| {
                let mut nums = line
                    .split_ascii_whitespace()
                    .filter_map(|num| num.parse::<i64>().ok());

                Some(Mapping{dst_start: nums.next()?, src_start: nums.next()?, len: nums.next()?})
            })
            .collect::<Map>()
        )
        .collect();

    // TODO: Try and collapse multiple maps into one

    return Some((seeds, maps));
}

fn calc_solution_1(input: &str) -> i64 {
    let (seeds, maps) = parse_input(input).unwrap();

    // println!("Seeds: ");
    // for seed in seeds.iter() {
    //     print!("{} ", seed);
    // }
    // println!("\nMaps:");
    // for map in maps.iter() {
    //     println!("\tMap: ");
    //     for mapping in map {
    //         println!("\t\tMapping: {}, {}, {}", mapping.src_start, mapping.dst_start, mapping.len);
    //     }
    // }
    // println!("");

    let mut result = i64::MAX;
    for seed in seeds.iter() {
        let mut acc = *seed;
        // println!("\tSeed {}: ", seed);
        for map in maps.iter() {
            acc = apply_map(acc, &map);
            // println!("\t\t -> {}", acc);
        }

        // println!("\tSeed {} --> {}", seed, acc);
        // println!("");

        result = std::cmp::min(result, acc);
    }

    result
}

fn calc_solution_2(input: &str) -> i64 {
    // TODO: Speed up, this is MEGA GIGA ULTRA SLOW, takes like 280s to compute
    let (seeds, maps) = parse_input(input).unwrap();

    let mut result = i64::MAX;
    let mut seeds_iter = seeds.iter();
    loop{
        let a = seeds_iter.next();
        let b = seeds_iter.next();

        if a.is_none() || b.is_none() {
            return result;
        }

        let a = *a.unwrap();
        let b = *b.unwrap();

        for seed in a..(a + b) {
            let mut acc = seed;
            for map in maps.iter() {
                acc = apply_map(acc, &map);
                // println!("\t\t -> {}", acc);
            }

            result = std::cmp::min(result, acc);
        }
    }
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
        assert_eq!(35, calc_solution_1(input));
    }

    #[test]
    fn test_second_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(46, calc_solution_2(input));
    }
}
