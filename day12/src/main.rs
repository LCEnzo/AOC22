use std::{collections::VecDeque, fmt::Display};

// Fields are x and y
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn get_neighbours(self: &Self, height: &usize, width: &usize) -> [Option<Self>; 4] {
        let top = if self.y > 0 {
            Some(new_pos(self.x, self.y - 1))
        } else {
            None
        };

        let left = if self.x > 0 {
            Some(new_pos(self.x - 1, self.y))
        } else {
            None
        };

        let bot = if self.y < height - 1 {
            Some(new_pos(self.x, self.y + 1))
        } else {
            None
        };

        let right = if self.x < width - 1 {
            Some(new_pos(self.x + 1, self.y))
        } else {
            None
        };

        [top, left, bot, right]
    }
}

fn new_pos(x: usize, y: usize) -> Position {
    Position { x, y }
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Position, Position) {
    let mut start = new_pos(0, 0);
    let mut end = new_pos(0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, ch)| match ch {
                    b'S' => {
                        start.x = x;
                        start.y = y;
                        0
                    }
                    b'E' => {
                        end.x = x;
                        end.y = y;
                        b'z' - b'a' + 1
                    }
                    _ => ch - b'a' + 1,
                })
                .collect()
        })
        .collect();

    assert_ne!(start, end);

    (map, start, end)
}

fn bfs(map: &Vec<Vec<u8>>, start: &Position, end: &Position) -> Option<u32> {
    let height = map.len();
    let width = map[0].len();
    
    let mut dist_map = vec![vec![u32::MAX; width]; height];
    dist_map[start.y][start.x] = 0;

    let mut open: VecDeque<Position> = VecDeque::new();

    open.push_back(start.clone());

    while !open.is_empty() {
        let curr = open.pop_front()?;

        let cands = curr
            .get_neighbours(&height, &width);
        let cands_iter = cands.iter()
            .filter_map(|&pos| pos)
            .filter(|pos| (map[pos.y][pos.x] <= map[curr.y][curr.x] + 1));
        for cand in cands_iter {
            if dist_map[cand.y][cand.x] == u32::MAX || (dist_map[cand.y][cand.x] > dist_map[curr.y][curr.x] + 1) {
                // let curr_c = (map[curr.y][curr.x] + b'a' - 1) as char;
                // let cand_c = (map[cand.y][cand.x] + b'a' - 1) as char;
                // println!("From {:2}, {:2} ({:1}) to {:2}, {:2} ({:1})", 
                //     curr.y, curr.x, curr_c, cand.y, cand.x, cand_c);

                open.push_back(cand);
                dist_map[cand.y][cand.x] = dist_map[curr.y][curr.x] + 1;
            }
        }

        print_input(&dist_map, &start, &end);
    }

    print_input(&dist_map, &start, &end);

    if dist_map[end.y][end.x] != u32::MAX {
        Some(dist_map[end.y][end.x])
    } else {
        None
    }
}

fn print_input<T>(map: &Vec<Vec<T>>, start: &Position, end: &Position)
where
    T: Copy + From<u8> + PartialOrd + Into<i64>
{
    println!("start: {} {}\nend: {} {}", start.y, start.x, end.y, end.x);
    for line in map.iter() {
        for &pos in line {
            let p: i64 = pos.into();
            let p = if p > 300 { -1 } else { pos.into() }; 
            print!("{:3} ", p);
        }

        println!("");
    }

    println!("");
}

fn navigate(input: &str) -> u32 {
    let (map, start, end) = parse_input(&input);
    print_input(&map, &start, &end);
    bfs(&map, &start, &end).unwrap()
}

fn main() {
    // x, y == (0, 0) at the top left corner
    // y up as index goes up, so the y is in effect reversed
    let input = include_str!("input.txt");
    println!("{}", navigate(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        let step_count = navigate(&input);
        assert_eq!(31, step_count);
    }

    // #[test]
    // fn test_second_half() {
    //     let input = include_str!("test_input.txt");
    //     todo!("Did not solve first half");
    // }
}
