use std::cmp::{max, min};
use std::collections::{HashSet, HashMap, VecDeque};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Sensor {
    self_pos: Position,
    beacon: Position,
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| line.split(&['=', ',', ':']))
        .map(|fragments| {
            let v: Vec<&str> = fragments.collect();
            return (
                v[1].parse::<i64>().unwrap(),
                v[3].parse::<i64>().unwrap(),
                v[5].parse::<i64>().unwrap(),
                v[7].parse::<i64>().unwrap(),
            );
        })
        .map(|coords| Sensor {
            self_pos: Position {
                x: coords.0,
                y: coords.1,
            },
            beacon: Position {
                x: coords.2,
                y: coords.3,
            },
        })
        .collect()
}

fn get_minmax_xy(sensors: &Vec<Sensor>) -> Option<(i64, i64, i64, i64)> {
    if sensors.is_empty() {
        return None;
    }

    let (mut minX, mut minY, mut maxX, mut maxY) = (
        sensors[0].self_pos.x,
        sensors[0].self_pos.y,
        sensors[0].self_pos.x,
        sensors[0].self_pos.y,
    );

    for s in sensors {
        minX = min(minX, min(s.self_pos.x, s.beacon.x));
        minY = min(minY, min(s.self_pos.y, s.beacon.y));

        maxX = max(maxX, max(s.self_pos.x, s.beacon.x));
        maxY = max(maxY, max(s.self_pos.y, s.beacon.y));
    }

    Some((minX, minY, maxX, maxY))
}

fn print_map(sensors: &Vec<Sensor>) -> () {
    let (minX, minY, maxX, maxY) = get_minmax_xy(sensors).unwrap();
    let mut sensor_set = HashSet::new();
    let mut beacons = HashSet::new();

    for sensor in sensors {
        sensor_set.insert((sensor.self_pos.x, sensor.self_pos.y));
        beacons.insert((sensor.beacon.x, sensor.beacon.y));
    }

    print!("    min: {}, {}\n    max: {}, {}\n", minX, minY, maxX, maxY);

    print!("{:>8}: ", 'x');
    for (ind, x) in (minX..=maxX).enumerate() {
        if ind == 0 || (ind as i64) == maxX - minX - 1 {
            print!("{}", (x % 10).abs());
        } else if ind % 5 == 0 || x % 5 == 0 {
            print!("{}", (x % 10).abs());
        } else {
            print!(" ");
        }
    }
    print!("\n");

    for y in minY..=maxY {
        print!("{:>8}: ", y);

        for x in minX..=maxX {
            if sensor_set.contains(&(x, y)) {
                print!("S")
            } else if beacons.contains(&(x, y)) {
                print!("B")
            } else {
                print!(".")
            }
        }

        print!("\n");
    }
}

fn flood_fill(orig_x: i64, orig_y: i64, dist_left: i64, occupied: &mut HashMap<(i64, i64), i64>) -> () {
    if dist_left < 0 {
        return;
    }

    occupied.insert((orig_x, orig_y), dist_left);

    for (x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let new_x = orig_x + x;
        let new_y = orig_y + y;

        if let Some(existing_dist) = occupied.get(&(new_x, new_y)) {
            if *existing_dist < dist_left - 1 {
                flood_fill(new_x, new_y, dist_left - 1, occupied);
            }
        } else {
            flood_fill(new_x, new_y, dist_left - 1, occupied);
        }
    }
}

fn flood_fill_iterative(orig_x: i64, orig_y: i64, start_dist: i64, occupied: &mut HashMap<(i64, i64), i64>) -> () {
    let mut candidate_moves = VecDeque::new();
    let diffs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    candidate_moves.push_back((orig_x, orig_y, start_dist));

    print!("{} {} - {} => ", orig_x, orig_y, start_dist);

    let mut counter = 0;
    loop {
        // Stop search if there are no more candidates for moves 
        let op = candidate_moves.pop_front();
        if op.is_none() {
            break
        }

        // Skip candidate if it's farther than permissable
        let (x, y, dist) = op.unwrap();
        if dist < 0 {
            continue
        }

        let existing = occupied.get(&(x, y));
        // Only note the position if it's no already filled, or we can travel farther (we have larger dist)
        if existing.is_none() || (*existing.unwrap() < dist) {
            occupied.insert((x, y), dist);
            counter += 1;

            for (x_diff, y_diff) in diffs {
                candidate_moves.push_back((x_diff + x, y_diff + y, dist - 1));
            }
        }
    }

    print!("{} | end loop\n", counter);
}

fn calc_solution_1(sensors: &Vec<Sensor>, target_row: i64) -> u32 {
    let (mut minX, mut minY, mut maxX, mut maxY) = get_minmax_xy(sensors).unwrap();
    let mut positions: HashMap<(i64, i64), i64> = HashMap::new();
    let mut beacons = HashSet::new();

    for sensor in sensors {
        let dist = (sensor.self_pos.x - sensor.beacon.x).abs()
            + (sensor.self_pos.y - sensor.beacon.y).abs();

        beacons.insert((sensor.beacon.x, sensor.beacon.y));

        minX = min(sensor.self_pos.x - dist, minX);
        maxX = max(sensor.self_pos.x + dist, maxX);
        minY = min(sensor.self_pos.y - dist, minY);
        maxY = max(sensor.self_pos.y + dist, maxY);

        flood_fill_iterative(sensor.self_pos.x, sensor.self_pos.y, dist, &mut positions);

        // for i in 0..=dist {
        //     for j in 0..=(dist - i) {
        //         positions.insert((sensor.self_pos.x + i, sensor.self_pos.y + j));
        //         positions.insert((sensor.self_pos.x + i, sensor.self_pos.y - j));
        //         positions.insert((sensor.self_pos.x - i, sensor.self_pos.y + j));
        //         positions.insert((sensor.self_pos.x - i, sensor.self_pos.y - j));
        //     }
        // }
    }

    // print!("\n");
    // // positions.keys().into_iter().map(|el| el.clone()).collect::<Vec<(i64, i64)>>()

    // println!("minmax X: {}, {}", minX, maxX);
    // println!("res: {}", (minX..=maxX).into_iter().filter(|x| positions.contains_key(&(*x, target_row))).count());

    // for y in minY..=maxY {
    //     println!("Y: {} -> count: {}", y, (minX..=maxX).into_iter().filter(|x| positions.contains_key(&(*x, y))).count())
    // }

    // let min = (minX..=maxX).into_iter().filter(|x| positions.contains_key(&(*x, target_row))).min().unwrap();
    // let max = (minX..=maxX).into_iter().filter(|x| positions.contains_key(&(*x, target_row))).max().unwrap();
    // print!("min {} max {}\n", min, max);

    (minX..=maxX)
        .into_iter()
        .filter(|x| positions.get(&(*x, target_row)).is_some())
        .filter(|x| !beacons.contains(&(*x, target_row)))
        .count() as u32
}

fn main() {
    let input = include_str!("input.txt");
    let sensors = parse_sensors(input);

    // print_map(&sensors);
    println!("{}", calc_solution_1(&sensors, 10_000));
    // println!(
    //     "{}",
    //     calc_sand_grain_count_until_filled(&positions)
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        let sensors = parse_sensors(input);

        // print_map(&sensors);

        assert_eq!(26, calc_solution_1(&sensors, 10));
    }

    // #[test]
    // fn test_second_half() {
    //     let input = include_str!("test_input.txt");
    //     let mut positions = parse_positions(input);

    //     print_map(&positions, None);

    //     assert_eq!(93, calc_sand_grain_count_until_filled(&mut positions));
    // }
}
