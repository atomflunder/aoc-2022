// Day 5 of Advent of Code 2022.
// https://adventofcode.com/2022/day/5

fn get_crates() -> [Vec<char>; 9] {
    // Could not be bothered reading them from input.
    // So I implemented them manually and removed them from the text file.
    [
        vec!['G', 'W', 'L', 'J', 'B', 'R', 'T', 'D'],
        vec!['C', 'W', 'S'],
        vec!['M', 'T', 'Z', 'R'],
        vec!['V', 'P', 'S', 'H', 'C', 'T', 'D'],
        vec!['Z', 'D', 'L', 'T', 'P', 'G'],
        vec!['D', 'C', 'Q', 'J', 'Z', 'R', 'B', 'F'],
        vec!['R', 'T', 'F', 'M', 'J', 'D', 'B', 'S'],
        vec!['M', 'V', 'T', 'B', 'R', 'H', 'L'],
        vec!['V', 'S', 'D', 'P', 'Q'],
    ]
}

fn part_one(crates: &mut [Vec<char>; 9]) {
    let moves = include_str!("crates.txt");

    for line in moves.lines() {
        let instructions: Vec<&str> = line.split(' ').collect();
        let amount = instructions[1].parse::<usize>().unwrap();
        let source = instructions[3].parse::<usize>().unwrap() - 1;
        let destination = instructions[5].parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let item = crates[source].remove(0);
            crates[destination].insert(0, item);
        }
    }

    let mut solution = String::new();

    for c in crates {
        solution.push(c[0]);
    }

    println!("{}", solution);
}
fn part_two(crates: &mut [Vec<char>; 9]) {
    let moves = include_str!("crates.txt");

    for line in moves.lines() {
        let instructions: Vec<&str> = line.split(' ').collect();
        let amount = instructions[1].parse::<usize>().unwrap();
        let source = instructions[3].parse::<usize>().unwrap() - 1;
        let destination = instructions[5].parse::<usize>().unwrap() - 1;

        let mut items = crates[source][0..amount].to_vec();

        for _ in 0..amount {
            crates[source].remove(0);
        }

        items.append(&mut crates[destination]);
        crates[destination] = items.clone();
    }

    let mut solution = String::new();

    for c in crates {
        solution.push(c[0]);
    }

    println!("{}", solution);
}

fn main() {
    part_one(&mut get_crates());
    part_two(&mut get_crates());
}
