// Day 22 of Advent of Code 2022.
// https://adventofcode.com/2022/day/22

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn to_score(self) -> isize {
        match &self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn turn(&self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Move(_) => *self,
            Instruction::TurnLeft => self.turn_left(),
            Instruction::TurnRight => self.turn_right(),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnRight,
    TurnLeft,
}

impl Instruction {
    fn handle(
        &self,
        grid: &Vec<Vec<char>>,
        direction: &mut Direction,
        x: &mut isize,
        y: &mut isize,
    ) {
        match self {
            Instruction::Move(n) => {
                for _ in 0..*n {
                    let mut temp_x = *x;
                    let mut temp_y = *y;

                    match direction {
                        Direction::Right => {
                            temp_x += 1;
                            if temp_x >= grid[*y as usize].len() as isize {
                                temp_x = grid[*y as usize].iter().position(|&c| c != ' ').unwrap()
                                    as isize;
                            }
                        }
                        Direction::Down => {
                            temp_y += 1;
                            if temp_y >= grid.len() as isize
                                || grid[temp_y as usize].len() <= *x as usize
                                || grid[temp_y as usize][*x as usize] == ' '
                            {
                                temp_y = grid
                                    .iter()
                                    .position(|row| {
                                        row.len() > *x as usize && row[*x as usize] != ' '
                                    })
                                    .unwrap() as isize;
                            }
                        }
                        Direction::Left => {
                            temp_x -= 1;
                            if temp_x < 0 || grid[*y as usize][temp_x as usize] == ' ' {
                                temp_x = grid[*y as usize].iter().rposition(|&c| c != ' ').unwrap()
                                    as isize;
                            }
                        }
                        Direction::Up => {
                            temp_y -= 1;
                            if temp_y < 0
                                || grid[temp_y as usize].len() <= *x as usize
                                || grid[temp_y as usize][*x as usize] == ' '
                            {
                                temp_y = grid
                                    .iter()
                                    .rposition(|row| {
                                        row.len() > *x as usize && row[*x as usize] != ' '
                                    })
                                    .unwrap() as isize;
                            }
                        }
                    }

                    if grid[temp_y as usize][temp_x as usize] == '#' {
                        break;
                    }

                    *x = temp_x;
                    *y = temp_y;
                }
            }
            _ => {
                *direction = direction.turn(self);
            }
        }
    }
}

fn get_grid_and_instructions() -> (Vec<Vec<char>>, Vec<Instruction>) {
    let (part1, part2) = include_str!("map.txt").split_once("\r\n\r\n").unwrap();
    let grid = {
        part1
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    };

    let mut instructions = Vec::new();
    let mut i = 0;
    for c in part2.chars() {
        if c.is_ascii_digit() {
            // Pushing the existing digit one spot to the left, if there is another digit coming up.
            i = i * 10 + c.to_digit(10).unwrap();
        } else {
            if i != 0 {
                instructions.push(Instruction::Move(i as usize));
                i = 0;
            }
            match c {
                'R' => instructions.push(Instruction::TurnRight),
                'L' => instructions.push(Instruction::TurnLeft),
                _ => (),
            }
        }
    }
    instructions.push(Instruction::Move(i as usize));

    (grid, instructions)
}

fn part_one() {
    let (grid, instructions) = get_grid_and_instructions();

    let mut x = grid[0].iter().position(|&c| c != ' ').unwrap() as isize;
    let mut y = 0;
    let mut direction = Direction::Right;
    for instruction in instructions {
        instruction.handle(&grid, &mut direction, &mut x, &mut y);
    }

    let solution = (y + 1) * 1000 + (x + 1) * 4 + direction.to_score();

    println!("{:?}", solution);
}

fn part_two() {
    todo!("Part 2 not implemented yet.")
}

fn main() {
    part_one();
    part_two();
}
