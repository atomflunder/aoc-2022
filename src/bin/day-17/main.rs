// Day 17 of Advent of Code 2022.
// https://adventofcode.com/2022/day/17

use std::collections::HashMap;

#[derive(Clone)]
struct Rock {
    left: Vec<(usize, usize)>,
    extra: Vec<(usize, usize)>,
    right: Vec<(usize, usize)>,
    bottom: Vec<(usize, usize)>,
    coordinates: Option<(usize, usize)>,
}

impl Rock {
    fn get_all_rocks() -> Vec<Self> {
        vec![
            Rock {
                left: vec![(0, 0)],
                right: vec![(0, 3)],
                extra: vec![(0, 1), (0, 2)],
                bottom: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                coordinates: None,
            },
            Rock {
                left: vec![(0, 1), (1, 0), (2, 1)],
                right: vec![(0, 1), (1, 2), (2, 1)],
                extra: vec![(1, 1)],
                bottom: vec![(1, 0), (0, 1), (1, 2)],
                coordinates: None,
            },
            Rock {
                left: vec![(0, 0), (1, 2), (2, 2)],
                right: vec![(0, 2), (1, 2), (2, 2)],
                extra: vec![(0, 1)],
                bottom: vec![(0, 0), (0, 1), (0, 2)],
                coordinates: None,
            },
            Rock {
                left: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                right: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                extra: vec![],
                bottom: vec![(0, 0)],
                coordinates: None,
            },
            Rock {
                left: vec![(0, 0), (1, 0)],
                right: vec![(0, 1), (1, 1)],
                extra: vec![],
                bottom: vec![(0, 0), (0, 1)],
                coordinates: None,
            },
        ]
    }

    fn get_rock(ss: &[Rock], next: &mut usize, bottom: usize) -> Self {
        let mut obj = ss[*next % 5].clone();
        obj.coordinates.replace((bottom + 3, 2));
        *next += 1;
        obj
    }

    fn can_move(&self, m: &Vec<Vec<bool>>, to_right: bool) -> bool {
        let start_coords = self.coordinates.as_ref().unwrap();
        if to_right {
            let mut can_move = true;
            for i in self.right.iter() {
                let right = i.1 + start_coords.1 + 1;
                if i.0 + start_coords.0 < m.len() {
                    can_move &= right < 7 && !m[i.0 + start_coords.0][right];
                }
            }
            can_move
        } else {
            let mut can_move = true;
            for i in self.left.iter() {
                let left = (i.1 + start_coords.1) as i64 - 1;
                if i.0 + start_coords.0 < m.len() {
                    can_move &= left >= 0 && !m[i.0 + start_coords.0][left as usize];
                }
            }
            can_move
        }
    }

    fn can_fall(&self, m: &Vec<Vec<bool>>) -> bool {
        let start_coords = self.coordinates.as_ref().unwrap();
        let mut can_fall = true;
        for i in self.bottom.iter() {
            let bottom: i64 = (i.0 + start_coords.0) as i64 - 1;
            if bottom < m.len() as i64 {
                can_fall &= bottom >= 0 && !m[bottom as usize][i.1 + start_coords.1];
            }
        }
        can_fall
    }

    fn fall_one_step(&mut self) {
        let mut x = self.coordinates.as_mut().unwrap();
        x.0 -= 1;
    }

    fn move_one_step(&mut self, to_right: bool) {
        let mut x = self.coordinates.as_mut().unwrap();
        if to_right {
            x.1 = (x.1 as i64 + 1) as usize;
        } else {
            x.1 = (x.1 as i64 - 1) as usize;
        }
    }
}

fn get_last(m: &Vec<Vec<bool>>, last: i32) -> String {
    let mut s = String::new();

    m.iter()
        .skip(i32::max(0, m.len() as i32 - last) as usize)
        .for_each(|x| {
            x.iter().for_each(|&y| {
                if y {
                    s.push('#');
                } else {
                    s.push('.');
                }
            })
        });

    s
}

fn new_lines(drawn: &mut Vec<Vec<bool>>, needed: usize) {
    for _ in 0..(needed - drawn.len()) {
        drawn.push(vec![false; 7]);
    }
}

fn solve(no_of_rocks: usize) {
    let shapes = Rock::get_all_rocks();
    let moves = include_str!("jet.txt").lines().next().unwrap();
    let mut grid = vec![vec![false; 7]; 3];

    let mut store = HashMap::<(usize, u8, String), (usize, usize)>::new();
    let mut next = 0;
    let mut bottom = 0;

    let mut rock = Rock::get_rock(&shapes, &mut next, bottom);
    new_lines(&mut grid, bottom + 6);
    let moves = moves.chars().collect::<Vec<char>>();
    let mut index = 0;
    let mut done = false;
    let mut height_extra = 0;

    loop {
        let action = moves[index];
        index = (index + 1) % moves.len();

        if next == no_of_rocks + 1 {
            break;
        }

        match action {
            '>' => {
                if rock.can_move(&grid, true) {
                    rock.move_one_step(true);
                }
            }
            '<' => {
                if rock.can_move(&grid, false) {
                    rock.move_one_step(false);
                }
            }
            _ => unreachable!(),
        }

        if rock.can_fall(&grid) {
            rock.fall_one_step();
        } else {
            let starting = rock.coordinates.as_ref().unwrap();
            for i in rock.left.iter() {
                grid[i.0 + starting.0][i.1 + starting.1] = true;
            }
            for i in rock.extra.iter() {
                grid[i.0 + starting.0][i.1 + starting.1] = true;
            }
            for i in rock.right.iter() {
                grid[i.0 + starting.0][i.1 + starting.1] = true;
            }

            let other = rock.right.iter().map(|(x, _)| x).max().unwrap()
                + rock.coordinates.as_ref().unwrap().0;
            bottom = usize::max(bottom, other + 1);

            let last_rows = get_last(&grid, 30);
            let details = (index, (next % 5) as u8, last_rows);
            if !done && store.get(&details).is_some() {
                let prev = store.get(&details).unwrap();
                let rock_per_cycle = next - prev.0;
                let h_per_cycle = bottom - prev.1;
                let cycles = (no_of_rocks - next) / rock_per_cycle;
                height_extra = cycles * h_per_cycle;
                next += rock_per_cycle * cycles;
                done = true;
            }
            store.insert(details, (next, bottom));
            new_lines(&mut grid, bottom + 6);
            rock = Rock::get_rock(&shapes, &mut next, bottom);
        }
    }
    println!("{}", bottom + height_extra);
}

fn part_one() {
    solve(2022);
}

fn part_two() {
    solve(1_000_000_000_000);
}

fn main() {
    part_one();
    part_two();
}
