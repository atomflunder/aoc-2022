use std::collections::{hash_map, HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Blizzard {
    start: (isize, isize),
    direction: Direction,
}

impl Blizzard {
    fn new(position: (isize, isize), direction: Direction) -> Self {
        Self {
            start: position,
            direction,
        }
    }

    fn move_step(&self, time: usize) -> (isize, isize) {
        match self.direction {
            Direction::Up => (self.start.0, self.start.1 - time as isize),
            Direction::Down => (self.start.0, self.start.1 + time as isize),
            Direction::Left => (self.start.0 - time as isize, self.start.1),
            Direction::Right => (self.start.0 + time as isize, self.start.1),
        }
    }
}

struct Valley {
    start: (isize, isize),
    end: (isize, isize),
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
    cache: HashMap<usize, HashSet<Blizzard>>,
}

impl Valley {
    fn in_bounds(&self, square: (isize, isize)) -> bool {
        square.0 > 0
            && square.0 < self.width as isize - 1
            && (square.1 > 0 || square == self.start)
            && (square.1 < self.height as isize - 1 || square == self.end)
    }

    fn travel(&mut self, start: (isize, isize), goal: (isize, isize), time: usize) -> usize {
        let mut queue: VecDeque<((isize, isize), usize)> = VecDeque::new();
        queue.push_back((start, time));

        let mut visited: HashSet<((isize, isize), usize)> = HashSet::new();
        visited.insert((start, time));

        while let Some(step) = queue.pop_front() {
            if step.0 == goal {
                return step.1;
            }

            if let hash_map::Entry::Vacant(e) = self.cache.entry(step.1 + 1) {
                let mut next_blizzards = HashSet::new();

                for blizzard in &self.blizzards {
                    let mut blizzard_pos = blizzard.move_step(step.1 + 1);

                    blizzard_pos.0 = (blizzard_pos.0 - 1).rem_euclid(self.width as isize - 2) + 1;
                    blizzard_pos.1 = (blizzard_pos.1 - 1).rem_euclid(self.height as isize - 2) + 1;
                    next_blizzards.insert(Blizzard::new(blizzard_pos, Direction::Down));
                }

                e.insert(next_blizzards);
            }

            let next_step = [
                (step.0, step.1 + 1),
                ((step.0 .0, step.0 .1 - 1), step.1 + 1),
                ((step.0 .0, step.0 .1 + 1), step.1 + 1),
                ((step.0 .0 - 1, step.0 .1), step.1 + 1),
                ((step.0 .0 + 1, step.0 .1), step.1 + 1),
            ];

            for next in next_step.iter() {
                if self.in_bounds(next.0)
                    && !visited.contains(next)
                    && !self
                        .cache
                        .get(&next.1)
                        .unwrap()
                        .contains(&Blizzard::new(next.0, Direction::Down))
                {
                    queue.push_back(*next);
                    visited.insert(*next);
                }
            }
        }

        unreachable!()
    }
}

fn get_valley(input: &str) -> Valley {
    let rows: Vec<&str> = input.lines().collect();

    let start = (input.find('.').unwrap() as isize, 0);
    let end = (
        rows.last().unwrap().rfind('.').unwrap() as isize,
        rows.len() as isize - 1,
    );

    let width = rows.first().unwrap().len();
    let height = (end.1 - start.1) as usize + 1;

    let mut blizzards = vec![];

    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' || c == '.' {
                continue;
            }

            let direction = match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => unreachable!(),
            };

            let blizzard = Blizzard {
                start: (x as isize, y as isize),
                direction,
            };

            blizzards.push(blizzard);
        }
    }

    let blizzard_cache = HashMap::new();

    Valley {
        start,
        end,
        width,
        height,
        blizzards,
        cache: blizzard_cache,
    }
}

pub fn part_one() {
    let input = include_str!("valley.txt");
    let mut mountain = get_valley(input);

    let solution = mountain.travel(mountain.start, mountain.end, 0);

    println!("{:?}", solution);
}

pub fn part_two() {
    let input = include_str!("valley.txt");
    let mut mountain = get_valley(input);

    let first_travel = mountain.travel(mountain.start, mountain.end, 0);
    let second_travel = mountain.travel(mountain.end, mountain.start, first_travel);
    let third_travel = mountain.travel(mountain.start, mountain.end, second_travel);

    println!("{:?}", third_travel);
}

fn main() {
    part_one();
    part_two();
}
