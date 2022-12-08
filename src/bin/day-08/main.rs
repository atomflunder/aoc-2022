// Day 7 of Advent of Code 2022.
// https://adventofcode.com/2022/day/7

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: u32,
    coordinates: (usize, usize),
}

impl Tree {
    fn is_visible(&self, grid: &Vec<Vec<Tree>>) -> bool {
        if self.coordinates.0 == 0
            || self.coordinates.1 == 0
            || self.coordinates.0 + 1 == grid.len()
            || self.coordinates.1 + 1 == grid[0].len()
        {
            return true;
        }

        let mut compare_trees_up: Vec<Tree> = Vec::new();
        for i in 0..self.coordinates.0 {
            compare_trees_up.push(grid[i][self.coordinates.1]);
        }

        let mut compare_trees_down: Vec<Tree> = Vec::new();
        for i in self.coordinates.0 + 1..grid.len() {
            compare_trees_down.push(grid[i][self.coordinates.1]);
        }

        let mut compare_trees_left: Vec<Tree> = Vec::new();
        for i in 0..self.coordinates.1 {
            compare_trees_left.push(grid[self.coordinates.0][i])
        }

        let mut compare_trees_right: Vec<Tree> = Vec::new();
        for i in self.coordinates.1 + 1..grid[0].len() {
            compare_trees_right.push(grid[self.coordinates.0][i])
        }

        if !compare_trees_up.iter().any(|n| n.height >= self.height) {
            return true;
        }

        if !compare_trees_down.iter().any(|n| n.height >= self.height) {
            return true;
        }

        if !compare_trees_left.iter().any(|n| n.height >= self.height) {
            return true;
        }

        if !compare_trees_right.iter().any(|n| n.height >= self.height) {
            return true;
        }

        false
    }

    fn get_viewing_distance(&self, grid: &Vec<Vec<Tree>>) -> u32 {
        let mut viewing_distance_up = 0;
        let mut viewing_distance_down = 0;
        let mut viewing_distance_left = 0;
        let mut viewing_distance_right = 0;

        if self.coordinates.0 != 0 {
            for i in (0..self.coordinates.0).rev() {
                viewing_distance_up += 1;
                if grid[i][self.coordinates.1].height >= self.height {
                    break;
                }
            }
        }

        if self.coordinates.1 != 0 {
            for i in (0..self.coordinates.1).rev() {
                viewing_distance_left += 1;
                if grid[self.coordinates.0][i].height >= self.height {
                    break;
                }
            }
        }

        if self.coordinates.0 + 1 != grid.len() {
            for i in self.coordinates.0 + 1..grid.len() {
                viewing_distance_down += 1;
                if grid[i][self.coordinates.1].height >= self.height {
                    break;
                }
            }
        }

        if self.coordinates.1 + 1 != grid[0].len() {
            for i in self.coordinates.1 + 1..grid[0].len() {
                viewing_distance_right += 1;
                if grid[self.coordinates.0][i].height >= self.height {
                    break;
                }
            }
        }

        viewing_distance_up * viewing_distance_down * viewing_distance_left * viewing_distance_right
    }
}

fn get_tree_grid() -> Vec<Vec<Tree>> {
    let input = include_str!("trees.txt");

    let mut trees: Vec<Vec<Tree>> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut tree_line: Vec<Tree> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let t = Tree {
                height: c.to_digit(10).unwrap(),
                coordinates: (i, j),
            };
            tree_line.push(t);
        }
        trees.push(tree_line);
    }

    trees
}

fn part_one() {
    let trees = get_tree_grid();

    let mut count = 0;

    for line in &trees {
        for tree in line {
            if tree.is_visible(&trees) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
fn part_two() {
    let trees = get_tree_grid();

    let mut distances: Vec<u32> = Vec::new();

    for line in &trees {
        for tree in line {
            distances.push(tree.get_viewing_distance(&trees));
        }
    }

    distances.sort();
    distances.reverse();

    println!("{:?}", distances[0])
}

fn main() {
    part_one();
    part_two();
}
