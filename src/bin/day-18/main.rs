// Day 18 of Advent of Code 2022.
// https://adventofcode.com/2022/day/18

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn get_all_cubes() -> Vec<Self> {
        let mut cubes = Vec::new();
        let input = include_str!("cubes.txt");

        for line in input.lines() {
            if let [x, y, z] = line.split(',').collect::<Vec<&str>>()[..] {
                cubes.push(Cube::new(
                    x.parse().unwrap(),
                    y.parse().unwrap(),
                    z.parse().unwrap(),
                ));
            }
        }

        cubes
    }

    fn get_sides(&self) -> Vec<(i32, i32, i32)> {
        vec![
            (self.x + 1, self.y, self.z),
            (self.x - 1, self.y, self.z),
            (self.x, self.y + 1, self.z),
            (self.x, self.y - 1, self.z),
            (self.x, self.y, self.z + 1),
            (self.x, self.y, self.z - 1),
        ]
    }

    fn get_exposed_sides(
        all_cube_coords: &HashSet<(i32, i32, i32)>,
        max_x: i32,
        max_y: i32,
        max_z: i32,
    ) -> HashSet<(i32, i32, i32)> {
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        queue.push((-1, -1, -1));

        while let Some(point) = queue.pop() {
            if !visited.insert(point) {
                continue;
            }

            let x_pos = (point.0 + 1, point.1, point.2);
            if point.0 <= max_x + 1 && !all_cube_coords.contains(&x_pos) {
                queue.push(x_pos);
            }

            let x_neg = (point.0 - 1, point.1, point.2);
            if point.0 >= -1 && !all_cube_coords.contains(&x_neg) {
                queue.push(x_neg);
            }

            let y_pos = (point.0, point.1 + 1, point.2);
            if point.1 <= max_y + 1 && !all_cube_coords.contains(&y_pos) {
                queue.push(y_pos);
            }

            let y_neg = (point.0, point.1 - 1, point.2);
            if point.1 >= -1 && !all_cube_coords.contains(&y_neg) {
                queue.push(y_neg);
            }

            let z_pos = (point.0, point.1, point.2 + 1);
            if point.2 <= max_z + 1 && !all_cube_coords.contains(&z_pos) {
                queue.push(z_pos);
            }

            let z_neg = (point.0, point.1, point.2 - 1);
            if point.2 >= -1 && !all_cube_coords.contains(&z_neg) {
                queue.push(z_neg);
            }
        }
        visited
    }
}

impl From<(i32, i32, i32)> for Cube {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }
}

fn part_one() {
    let cubes = Cube::get_all_cubes();
    let mut total = 0;

    for cube in &cubes {
        for side in cube.get_sides() {
            if !cubes.contains(&Cube::from(side)) {
                total += 1;
            }
        }
    }

    println!("{:?}", total);
}

fn part_two() {
    let cubes = Cube::get_all_cubes();

    let all_cube_coords: HashSet<(i32, i32, i32)> = cubes.iter().map(|c| (c.x, c.y, c.z)).collect();
    let mut total = 0;
    let outside = Cube::get_exposed_sides(
        &all_cube_coords,
        cubes.iter().map(|c| c.x).max().unwrap(),
        cubes.iter().map(|c| c.y).max().unwrap(),
        cubes.iter().map(|c| c.z).max().unwrap(),
    );

    for p in outside {
        let cube = Cube::from(p);
        for side in cube.get_sides() {
            if all_cube_coords.contains(&side) {
                total += 1;
            }
        }
    }

    println!("{:?}", total);
}

fn main() {
    part_one();
    part_two();
}
