// Day 15 of Advent of Code 2022.
// https://adventofcode.com/2022/day/15

#[derive(Debug)]
struct Sensor {
    coordinates: (isize, isize),
    closest_beacon: (isize, isize),
    distance: isize,
}

impl Sensor {
    fn point_inside_range(&self, point: (isize, isize)) -> bool {
        if self.closest_beacon == point {
            false
        } else {
            self.distance
                >= (self.coordinates.0.abs_diff(point.0) + self.coordinates.1.abs_diff(point.1))
                    as isize
        }
    }
}

fn get_sensors() -> Vec<Sensor> {
    let sensors = include_str!("sensors.txt");
    let mut all_sensors: Vec<Sensor> = Vec::new();

    for line in sensors.lines() {
        if let [_, _, sensor_x, sensor_y, _, _, _, _, beacon_x, beacon_y] = line
            .split(' ')
            .map(|s| {
                let split = s.split_once('=').unwrap_or(("0", "0"));
                let mut string = split.1.to_string();
                if !string.chars().last().unwrap().is_ascii_digit() {
                    string.pop();
                }
                string.parse::<isize>().unwrap_or(0)
            })
            .collect::<Vec<isize>>()[..]
        {
            let sensor = Sensor {
                coordinates: (sensor_x, sensor_y),
                closest_beacon: (beacon_x, beacon_y),
                distance: sensor_x.abs_diff(beacon_x) as isize
                    + sensor_y.abs_diff(beacon_y) as isize,
            };
            all_sensors.push(sensor);
        }
    }

    all_sensors
}

fn part_one() {
    let sensors = get_sensors();

    let min = sensors
        .iter()
        .map(|s| s.coordinates.0 - s.distance)
        .min()
        .unwrap();
    let max = sensors
        .iter()
        .map(|s| s.coordinates.0 + s.distance)
        .max()
        .unwrap();

    let amount = (min..=max)
        .filter(|&i| sensors.iter().any(|s| s.point_inside_range((i, 2_000_000))))
        .count();

    println!("{:?}", amount);
}

fn part_two() {
    let sensors = get_sensors();

    let frequency = sensors
        .iter()
        .find_map(|s| {
            ((s.coordinates.0 - s.distance - 1).max(0)..=s.coordinates.0.min(4_000_000))
                .zip(s.coordinates.1..=4_000_000)
                .find_map(|p| {
                    sensors
                        .iter()
                        .all(|s| !s.point_inside_range(p))
                        .then_some(p.0 * 4_000_000 + p.1)
                })
        })
        .unwrap();

    println!("{}", frequency);
}

fn main() {
    part_one();
    part_two();
}
