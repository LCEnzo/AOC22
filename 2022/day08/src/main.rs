use std::cmp::max;

fn _print_sightline_maps(matrix: &Vec<Vec<[u32; 4]>>) {
    for k in 0..4 {
        match k {
            0 => println!("North"),
            1 => println!("East"),
            2 => println!("South"),
            3 => println!("West"),
            _ => panic!("k should not reach this value"),
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                print!("{}", matrix[i][j][k]);
            }
            println!("");
        }
        println!("");
    }
    println!("");
}

fn _print_map(matrix: &Vec<Vec<u32>>) {
    println!("Map");

    for row in matrix.iter() {
        for el in row.iter() {
            print!("{}", el);
        }
        println!("");
    }
    println!("");
}

fn map_input_to_matrix(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Expected a digit"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn calc_sightlines(map: &Vec<Vec<u32>>) -> Vec<Vec<[u32; 4]>> {
    let height = map.len();
    let width = map[0].len();
    let mut sightlines_map = vec![vec![[0, 0, 0, 0 as u32]; width - 2]; height - 2];

    for j in 0..width - 2 {
        // north
        sightlines_map[0][j][0] = map[0][j+1];
        // south
        sightlines_map[height - 3][j][2] = map[height - 1][j+1];
    }

    for i in 1..height - 2 {
        for j in 0..width - 2 {
            // north
            sightlines_map[i][j][0] = max(map[i][j+1], sightlines_map[i - 1][j][0]);
            // south
            sightlines_map[height - i - 3][j][2] =
                max(map[height - i - 1][j+1], sightlines_map[height - i - 2][j][2]);
        }
    }

    for i in 0..height - 2 {
        // east
        sightlines_map[i][width - 3][1] = map[i+1][width - 1];
        // west
        sightlines_map[i][0][3] = map[i+1][0];
    }

    for i in 0..height - 2 {
        for j in 1..width - 2 {
            // east
            sightlines_map[i][width - j - 3][1] =
                max(map[i+1][width - j - 1], sightlines_map[i][width - j - 2][1]);
            // west
            sightlines_map[i][j][3] = max(map[i+1][j], sightlines_map[i][j - 1][3]);
        }
    }

    sightlines_map
}

fn find_visible_tree_count(map: &Vec<Vec<u32>>) -> u32 {
    let height = map.len();
    let width = map[0].len();

    // Each position on the map is 4 numbers. It's the highest tree encountered
    // in the north, east, south, west direction, respectivly
    // Includes only the inner rectangle, trees on the edges which are always visible and
    // as such don't need to be checked
    let sightlines_map = calc_sightlines(&map);

    // Init with edges, subtract 4 to remove double counting
    let mut count = 2 * height as u32 + 2 * width as u32 - 4;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if &map[i][j] > sightlines_map[i - 1][j - 1].iter().min().unwrap_or(&10) {
                count += 1;
            }
        }
    }

    count
}

fn calc_tree_scenic_score(map: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let height = map.len();
    let width = map[0].len();
    let mut score: usize = 1;
    
    let mut multiplier = 0;
    for i in x+1..height {
        multiplier += 1;

        if map[i][y] >= map[x][y] {
            break;    
        }
    }
    score *= multiplier;

    multiplier = 0;
    for i in (0..x).rev() {
        multiplier += 1;
        if map[i][y] >= map[x][y] {
            break;
        }
    }
    score *= multiplier;

    multiplier = 0;
    for j in y+1..width {
        multiplier += 1;

        if map[x][j] >= map[x][y] {
            break;
        }
    }
    score *= multiplier;

    multiplier = 0;
    for j in (0..y).rev() {
        multiplier += 1;

        if map[x][j] >= map[x][y] {
            break;    
        }
    }
    score *= multiplier;

    // println!("{} has score {}", map[x][y], score);

    score as u32
}

fn find_max_scenic_score(map: &Vec<Vec<u32>>) -> u32 {
    map
        .iter()
        .enumerate()
        .map(|(x, row)| 
            row
                .iter()
                .enumerate()
                .map(|(y, _)| calc_tree_scenic_score(map, x, y))
                .max()
                .unwrap()
        )
        .max()
        .unwrap()
}

fn main() {
    let input_txt = include_str!("input.txt");
    let map = map_input_to_matrix(&input_txt);

    println!("{}", find_visible_tree_count(&map));
    println!("{}", find_max_scenic_score(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    static TEST_INPUT2: &str = "3037330373
2451225512
6533265332
3354933549
3539035390";

    static TEST_INPUT3: &str = "0123456789
0210000210
2090000002
9876543210";

    #[test]
    fn test() {
        let map = map_input_to_matrix(&TEST_INPUT);
        assert_eq!(21, find_visible_tree_count(&map));
    }

    #[test]
    fn test_scenic_pick() {
        let map = map_input_to_matrix(&TEST_INPUT);
        assert_eq!(8, find_max_scenic_score(&map));
    }

    #[test]
    fn test2() {
        let map = map_input_to_matrix(&TEST_INPUT2);
        assert_eq!(37, find_visible_tree_count(&map));
    }

    #[test]
    fn test3() {
        let map = map_input_to_matrix(&TEST_INPUT3);
        assert_eq!(28, find_visible_tree_count(&map));
    }
}
