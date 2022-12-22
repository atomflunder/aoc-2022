// Day 22 of Advent of Code 2022.
// https://adventofcode.com/2022/day/22

type CubeSide = Vec<Option<(Vec<Vec<char>>, [Option<(usize, usize, usize)>; 4])>>;

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

    fn from_score(score: &isize) -> Self {
        match score % 4 {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!(),
        }
    }

    fn add_score(&self, score: &isize) -> Self {
        Self::from_score(&(self.to_score() + score))
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
    fn handle_part_one(
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

    fn handle_part_two(
        &self,
        grid: &Vec<Vec<char>>,
        direction: &mut Direction,
        x: &mut isize,
        y: &mut isize,
        cube: &[CubeSide],
    ) {
        match self {
            Instruction::Move(n) => {
                for _ in 0..*n {
                    let (mut temp_x, mut temp_y) = match direction {
                        Direction::Right => (*x + 1, *y),
                        Direction::Down => (*x, *y + 1),
                        Direction::Left => (*x - 1, *y),
                        Direction::Up => (*x, *y - 1),
                    };
                    let mut temp_direction = *direction;

                    if temp_x < 0
                        || temp_y < 0
                        || temp_y >= grid.len() as isize
                        || temp_x >= grid[temp_y as usize].len() as isize
                        || grid[temp_y as usize][temp_x as usize] == ' '
                    {
                        let (cx, cy) = (*x / 50, *y / 50);
                        let cube_side = cube[cy as usize][cx as usize].as_ref().unwrap();
                        let (ncx, ncy, ndir) = cube_side.1[direction.to_score() as usize].unwrap();

                        let edge_offset = match direction {
                            Direction::Right => temp_y - cy * 50,
                            Direction::Down => (cx + 1) * 50 - temp_x - 1,
                            Direction::Left => (cy + 1) * 50 - temp_y - 1,
                            Direction::Up => temp_x - cx * 50,
                        };

                        (temp_x, temp_y) = match direction.add_score(&(ndir as isize)) {
                            Direction::Right => {
                                (ncx as isize * 50, ncy as isize * 50 + edge_offset)
                            }
                            Direction::Down => {
                                ((ncx as isize + 1) * 50 - edge_offset - 1, ncy as isize * 50)
                            }
                            Direction::Left => (
                                (ncx as isize + 1) * 50 - 1,
                                (ncy as isize + 1) * 50 - edge_offset - 1,
                            ),
                            Direction::Up => {
                                (ncx as isize * 50 + edge_offset, (ncy as isize + 1) * 50 - 1)
                            }
                        };
                        temp_direction =
                            Direction::from_score(&(direction.to_score() + ndir as isize));
                    }

                    if grid[temp_y as usize][temp_x as usize] == '#' {
                        break;
                    }

                    *x = temp_x;
                    *y = temp_y;
                    *direction = temp_direction;
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

fn get_cube_from_grid(grid: &Vec<Vec<char>>) -> Vec<CubeSide> {
    let mut cube: Vec<CubeSide> = vec![vec![None; 4]; 4];

    let (mut x, mut y) = (0, 0);
    while y < grid.len() {
        let mut cube_side = vec![vec![' '; 50]; 50];
        for i in 0..50 {
            for j in 0..50 {
                if y + i < grid.len() && x + j < grid[y + i].len() {
                    cube_side[i][j] = grid[y + i][x + j];
                }
            }
        }
        if cube_side[0][0] != ' ' {
            cube[y / 50][x / 50] = Some((cube_side, [None; 4]));
        }
        x += 50;
        if x >= grid[y].len() {
            x = 0;
            y += 50;
        }
    }

    for y in 0..cube.len() {
        for x in 0..cube[y].len() {
            if let Some((_, [c_right, c_down, c_left, c_up])) = unsafe {
                &mut *(&mut cube[y][x]
                    as *mut Option<(Vec<Vec<char>>, [Option<(usize, usize, usize)>; 4])>)
            } {
                if x + 1 < cube[y].len() && cube[y][x + 1].is_some() {
                    *c_right = Some((x + 1, y, 0));
                }
                if y + 1 < cube.len() && cube[y + 1][x].is_some() {
                    *c_down = Some((x, y + 1, 0));
                }
                if x > 0 && cube[y][x - 1].is_some() {
                    *c_left = Some((x - 1, y, 0));
                }
                if y > 0 && cube[y - 1][x].is_some() {
                    *c_up = Some((x, y - 1, 0));
                }
            }
        }
    }

    let mut total_sides = 0;
    while total_sides < 6 {
        for y in 0..cube.len() {
            for x in 0..cube[y].len() {
                if let Some((_, [c_right, c_down, c_left, c_up])) = unsafe {
                    &mut *(&mut cube[y][x]
                        as *mut Option<(Vec<Vec<char>>, [Option<(usize, usize, usize)>; 4])>)
                } {
                    match (c_right, c_down, c_left, c_up) {
                        (Some(c_right), Some(c_down), _, _)
                            if {
                                cube[c_right.1][c_right.0].as_ref().unwrap().1[(1 + c_right.2) % 4]
                                    .is_none()
                                    && cube[c_down.1][c_down.0].as_ref().unwrap().1[c_down.2 % 4]
                                        .is_none()
                            } =>
                        {
                            total_sides += 1;
                            cube[c_right.1][c_right.0].as_mut().unwrap().1[(1 + c_right.2) % 4] =
                                Some((c_down.0, c_down.1, c_down.2 + (4 - c_right.2 % 4) + 1));
                            cube[c_down.1][c_down.0].as_mut().unwrap().1[c_down.2 % 4] =
                                Some((c_right.0, c_right.1, c_right.2 + (4 - c_down.2 % 4) + 3));
                        }
                        (_, Some(c_down), Some(c_left), _)
                            if {
                                cube[c_down.1][c_down.0].as_ref().unwrap().1[(2 + c_down.2) % 4]
                                    .is_none()
                                    && cube[c_left.1][c_left.0].as_ref().unwrap().1
                                        [(1 + c_left.2) % 4]
                                        .is_none()
                            } =>
                        {
                            total_sides += 1;
                            cube[c_down.1][c_down.0].as_mut().unwrap().1[(2 + c_down.2) % 4] =
                                Some((c_left.0, c_left.1, c_left.2 + (4 - c_down.2 % 4) + 1));
                            cube[c_left.1][c_left.0].as_mut().unwrap().1[(1 + c_left.2) % 4] =
                                Some((c_down.0, c_down.1, c_down.2 + (4 - c_left.2 % 4) + 3));
                        }
                        (_, _, Some(c_left), Some(c_up))
                            if {
                                cube[c_left.1][c_left.0].as_ref().unwrap().1[(3 + c_left.2) % 4]
                                    .is_none()
                                    && cube[c_up.1][c_up.0].as_ref().unwrap().1[(2 + c_up.2) % 4]
                                        .is_none()
                            } =>
                        {
                            total_sides += 1;
                            cube[c_left.1][c_left.0].as_mut().unwrap().1[(3 + c_left.2) % 4] =
                                Some((c_up.0, c_up.1, c_up.2 + (4 - c_left.2 % 4) + 1));
                            cube[c_up.1][c_up.0].as_mut().unwrap().1[(2 + c_up.2) % 4] =
                                Some((c_left.0, c_left.1, c_left.2 + (4 - c_up.2 % 4) + 3));
                        }
                        (Some(c_right), _, _, Some(c_up))
                            if {
                                cube[c_up.1][c_up.0].as_ref().unwrap().1[c_up.2 % 4].is_none()
                                    && cube[c_right.1][c_right.0].as_ref().unwrap().1
                                        [(3 + c_right.2) % 4]
                                        .is_none()
                            } =>
                        {
                            total_sides += 1;
                            cube[c_up.1][c_up.0].as_mut().unwrap().1[c_up.2 % 4] =
                                Some((c_right.0, c_right.1, c_right.2 + (4 - c_up.2 % 4) + 1));
                            cube[c_right.1][c_right.0].as_mut().unwrap().1[(3 + c_right.2) % 4] =
                                Some((c_up.0, c_up.1, c_up.2 + (4 - c_right.2 % 4) + 3));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    cube
}

fn part_one() {
    let (grid, instructions) = get_grid_and_instructions();

    let mut x = grid[0].iter().position(|&c| c != ' ').unwrap() as isize;
    let mut y = 0;
    let mut direction = Direction::Right;
    for instruction in instructions {
        instruction.handle_part_one(&grid, &mut direction, &mut x, &mut y);
    }

    let solution = (y + 1) * 1000 + (x + 1) * 4 + direction.to_score();

    println!("{:?}", solution);
}

fn part_two() {
    let (grid, instructions) = get_grid_and_instructions();

    let cube = get_cube_from_grid(&grid);

    let mut x = grid[0].iter().position(|&c| c != ' ').unwrap() as isize;
    let mut y = 0;
    let mut direction = Direction::Right;

    for instruction in instructions {
        instruction.handle_part_two(&grid, &mut direction, &mut x, &mut y, &cube);
    }

    let solution = (y + 1) * 1000 + (x + 1) * 4 + direction as isize;

    println!("{:?}", solution);
}

fn main() {
    part_one();
    part_two();
}
