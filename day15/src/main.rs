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

impl Sensor {
    fn dist(self) -> i64 {
        (self.self_pos.x - self.beacon.x).abs() + (self.self_pos.y - self.beacon.y).abs()
    }
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

fn print_map(sensors: &Vec<Sensor>, filled: Option<&HashMap<(i64, i64), i64>>) -> () {
    let (minX, minY, maxX, maxY) = (-10, -12, 30, 27);//get_minmax_xy(sensors).unwrap();
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
            } else if let Some(ref occupied) = filled {
                if occupied.contains_key(&(x, y)) {
                    print!("#")
                } else {
                    print!(".")
                }
            } else {
                print!(".")
            }
        }

        print!("\n");
    }
}

// Too slow for actual solution, and can cause stack overflows. Used to create testcases and paint a filled map based on test input given.
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

// Function for creating and printing a filled map
fn make_print_map(sensors: &Vec<Sensor>) {
    let (mut minX, mut minY, mut maxX, mut maxY) = get_minmax_xy(sensors).unwrap();
    let mut positions: HashMap<(i64, i64), i64> = HashMap::new();
    // a unknown sensor can't be on a place occupied by a known beacon or sensor
    let mut occupied = HashSet::new();

    for sensor in sensors {
        let dist = sensor.dist();

        occupied.insert((sensor.beacon.x, sensor.beacon.y));
        occupied.insert((sensor.self_pos.x, sensor.self_pos.y));

        minX = min(sensor.self_pos.x - dist, minX);
        maxX = max(sensor.self_pos.x + dist, maxX);
        minY = min(sensor.self_pos.y - dist, minY);
        maxY = max(sensor.self_pos.y + dist, maxY);

        flood_fill(sensor.self_pos.x, sensor.self_pos.y, dist, &mut positions);
    }

    print_map(&sensors, Some(&positions));

    for y in minY..=maxY {
        let count = (minX..=maxX)
            .into_iter()
            .filter(|x| positions.get(&(*x, y)).is_some())
            .filter(|x| !occupied.contains(&(*x, y)))
            .count() as u32;

        // print!("{}: {}\n", y, count);
        // Useful for making tests for faster or partial algos
        print!("assert_eq!({}, calc_solution_1(&sensors, {}));\n", count, y);
    } 
    
    print!("\n");
}

// Actual solution logic, fast enough
fn calc_solution_1(sensors: &Vec<Sensor>, target_row: i64) -> u32 {
    let mut positions: HashSet<(i64, i64)> = HashSet::new();
    // a unknown sensor can't be on a place occupied by a known beacon or sensor
    let mut occupied = HashSet::new();

    for sensor in sensors {
        let dist = sensor.dist();

        occupied.insert((sensor.beacon.x, sensor.beacon.y));
        occupied.insert((sensor.self_pos.x, sensor.self_pos.y));

        let y_dist = (sensor.self_pos.y - target_row).abs();
        if y_dist > dist {
            continue
        }

        // This is not optimal, there are better, more efficient, algos for finding unions of ranges
        let x_diff = dist - y_dist;
        for x in (sensor.self_pos.x - x_diff)..=(sensor.self_pos.x + x_diff) {
            positions.insert((x, target_row));
        }
    }

    positions
        .iter()
        .filter(|(x, y)| !occupied.contains(&(*x, *y)))
        .count() as u32
}

fn main() {
    // Used to create test cases
    // let input = include_str!("test_input.txt");
    // let sensors = parse_sensors(input);
    // make_print_map(&sensors);

    let input = include_str!("input.txt");
    let sensors = parse_sensors(input);
    println!("{}", calc_solution_1(&sensors, 2_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        let sensors = parse_sensors(input);

        assert_eq!(1, calc_solution_1(&sensors, -10));
        assert_eq!(3, calc_solution_1(&sensors, -9));
        assert_eq!(5, calc_solution_1(&sensors, -8));
        assert_eq!(7, calc_solution_1(&sensors, -7));
        assert_eq!(10, calc_solution_1(&sensors, -6));
        assert_eq!(14, calc_solution_1(&sensors, -5));
        assert_eq!(18, calc_solution_1(&sensors, -4));
        assert_eq!(22, calc_solution_1(&sensors, -3));
        assert_eq!(26, calc_solution_1(&sensors, -2));
        assert_eq!(31, calc_solution_1(&sensors, -1));
        assert_eq!(34, calc_solution_1(&sensors, 0));
        assert_eq!(34, calc_solution_1(&sensors, 1));
        assert_eq!(32, calc_solution_1(&sensors, 2));
        assert_eq!(29, calc_solution_1(&sensors, 3));
        assert_eq!(29, calc_solution_1(&sensors, 4));
        assert_eq!(27, calc_solution_1(&sensors, 5));
        assert_eq!(25, calc_solution_1(&sensors, 6));
        assert_eq!(21, calc_solution_1(&sensors, 7));
        assert_eq!(23, calc_solution_1(&sensors, 8));
        assert_eq!(25, calc_solution_1(&sensors, 9));
        assert_eq!(26, calc_solution_1(&sensors, 10));
        assert_eq!(27, calc_solution_1(&sensors, 11));
        assert_eq!(29, calc_solution_1(&sensors, 12));
        assert_eq!(29, calc_solution_1(&sensors, 13));
        assert_eq!(28, calc_solution_1(&sensors, 14));
        assert_eq!(29, calc_solution_1(&sensors, 15));
        assert_eq!(28, calc_solution_1(&sensors, 16));
        assert_eq!(28, calc_solution_1(&sensors, 17));
        assert_eq!(29, calc_solution_1(&sensors, 18));
        assert_eq!(28, calc_solution_1(&sensors, 19));
        assert_eq!(25, calc_solution_1(&sensors, 20));
        assert_eq!(25, calc_solution_1(&sensors, 21));
        assert_eq!(20, calc_solution_1(&sensors, 22));
        assert_eq!(15, calc_solution_1(&sensors, 23));
        assert_eq!(9, calc_solution_1(&sensors, 24));
        assert_eq!(4, calc_solution_1(&sensors, 25));
        assert_eq!(1, calc_solution_1(&sensors, 26));
    }
}
