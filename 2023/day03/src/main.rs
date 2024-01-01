use std::time::Instant;

/// get_num is meant to be given a line (Vec<char>) and a position/index (usize).
/// Based on this, it will try and calculate the number that has a digit on the given index.
/// If no digit is found, it will return None.
fn get_num(line: &Vec<char>, pos: usize) -> Option<u32> {
    if !(0..line.len()).contains(&pos) || !line[pos].is_digit(10) {
        return None;
    }

    let mut pos = pos;
    while pos > 0 && line[pos - 1].is_digit(10) {
        pos -= 1;
    }

    let mut num = 0;
    while let Some(digit) = line[pos].to_digit(10) {
        num *= 10;
        num += digit;
        pos += 1;

        if pos >= line.len() {
            break;
        }
    }

    Some(num)
}

fn get_sum_around_part(mat: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut part_sum = 0;

    // number that is left of the part at (i, j)
    // Assumes underflow is filtered out via line bounds check
    part_sum += get_num(&mat[i], j - 1).unwrap_or(0);
    // number that is right of the part at (i, j)
    // Assumes overflow won't happen since input has 140 chars per row
    part_sum += get_num(&mat[i], j + 1).unwrap_or(0);

    // positions/rows above and below part
    for row_index in [i - 1, i + 1] {
        if (0..mat.len()).contains(&row_index) {
            part_sum += get_num(&mat[row_index], j + 1).unwrap_or(0);

            // if the top right has a digit, top middle must be part of the same number, so this checks for double counting
            if j + 1 >= mat[i].len() || !mat[row_index][j + 1].is_digit(10) {
                part_sum += get_num(&mat[row_index], j).unwrap_or(0);
            }

            // same check as top middle
            if !mat[row_index][j].is_digit(10) {
                part_sum += get_num(&mat[row_index], j - 1).unwrap_or(0);
            }
        }
    }

    part_sum
}

fn calc_solution_1(input: &str) -> u32 {
    let mat: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut sum = 0;
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] != '.' && !mat[i][j].is_digit(10) {
                let part_sum = get_sum_around_part(&mat, i, j);
                sum += part_sum;
            }
        }
    }

    sum
}

fn get_gear_ratio(mat: &Vec<Vec<char>>, i: usize, j: usize) -> Option<u32> {
    let mut part_prod = 1;
    let mut num_count = 0;

    if !(0..mat.len()).contains(&i) || !(0..mat[i].len()).contains(&j) || mat[i][j] != '*' {
        return None;
    }

    // number that is left of the part at (i, j)
    // Assumes underflow is filtered out via line bounds check
    if let Some(num) = get_num(&mat[i], j - 1) {
        num_count += 1;
        if num_count > 2 {
            return None;
        }
        part_prod *= num;
    }
    // number that is right of the part at (i, j)
    // Assumes overflow won't happen since input has 140 chars per row
    if let Some(num) = get_num(&mat[i], j + 1) {
        num_count += 1;
        if num_count > 2 {
            return None;
        }
        part_prod *= num;
    }

    // positions/rows above and below part
    for row_index in [i - 1, i + 1] {
        if (0..mat.len()).contains(&row_index) {
            if let Some(num) = get_num(&mat[row_index], j + 1) {
                num_count += 1;
                if num_count > 2 {
                    return None;
                }
                part_prod *= num;
            }

            // if the top right has a digit, top middle must be part of the same number, so this checks for double counting
            if j + 1 >= mat[i].len() || !mat[row_index][j + 1].is_digit(10) {
                if let Some(num) = get_num(&mat[row_index], j) {
                    num_count += 1;
                    if num_count > 2 {
                        return None;
                    }
                    part_prod *= num;
                }
            }

            // same check as top middle
            if !mat[row_index][j].is_digit(10) {
                if let Some(num) = get_num(&mat[row_index], j - 1) {
                    num_count += 1;
                    if num_count > 2 {
                        return None;
                    }
                    part_prod *= num;
                }
            }
        }
    }

    if num_count == 2 {
        Some(part_prod)
    } else {
        None
    }
}

fn calc_solution_2(input: &str) -> u32 {
    let mat: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut sum = 0;
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] != '.' && !mat[i][j].is_digit(10) {
                let gear_ratio = get_gear_ratio(&mat, i, j);

                if let Some(gear_ratio) = gear_ratio {
                    // println!("Gear at ({:3}, {:3}) got ratio {}", i, j, gear_ratio);
                    sum += gear_ratio;
                }
            }
        }
    }

    sum
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
    fn test_get_num() {
        let input: Vec<_> = ".874.772.".chars().collect();

        // Dots
        for i in [0, 4, 8] {
            assert_eq!(None, get_num(&input, i));
        }

        // First num, 874
        for i in 1..=3 {
            assert_eq!(Some(874), get_num(&input, i));
        }

        // Second num, 772
        for i in 5..=7 {
            assert_eq!(Some(772), get_num(&input, i));
        }

        // Out of bounds
        assert_eq!(None, get_num(&input, input.len()));
    }

    #[test]
    fn test_get_sum_around_part() {
        let input: Vec<Vec<char>> = "1.2\n3*4\n.5."
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        assert_eq!(15, get_sum_around_part(&input, 1, 1));
    }

    #[test]
    fn test_get_sum_around_part_with_skipping() {
        let input: Vec<Vec<char>> = ".333.\n.*...\n222.."
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        assert_eq!(555, get_sum_around_part(&input, 1, 1));
    }

    #[test]
    fn test_first_half_basic() {
        let input = "1.2\n3*4\n.5.";
        assert_eq!(15, calc_solution_1(&input));
    }

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(4361, calc_solution_1(input));
    }

    #[test]
    fn test_first_half_on_real_input() {
        let input = include_str!("input.txt");
        assert_eq!(551094, calc_solution_1(input));
    }

    #[test]
    fn test_second_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(467835, calc_solution_2(input));
    }

    #[test]
    fn test_second_half_on_real_input() {
        let input = include_str!("input.txt");
        assert_eq!(80179647, calc_solution_2(input));
    }
}
