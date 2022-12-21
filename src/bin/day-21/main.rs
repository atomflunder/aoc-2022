// Day 21 of Advent of Code 2022.
// https://adventofcode.com/2022/day/21

use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Debug)]
enum Operation {
    Number(isize),
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
struct Monkey {
    other_monkey1: Option<String>,
    other_monkey2: Option<String>,
    operation: Operation,
}

impl Monkey {
    fn get_number(&self, all_monkeys: &HashMap<String, Monkey>) -> isize {
        match self.operation {
            Operation::Number(i) => i,
            Operation::Add => {
                get_monkey(self.other_monkey1.clone().unwrap(), all_monkeys).get_number(all_monkeys)
                    + get_monkey(self.other_monkey2.clone().unwrap(), all_monkeys)
                        .get_number(all_monkeys)
            }
            Operation::Subtract => {
                get_monkey(self.other_monkey1.clone().unwrap(), all_monkeys).get_number(all_monkeys)
                    - get_monkey(self.other_monkey2.clone().unwrap(), all_monkeys)
                        .get_number(all_monkeys)
            }
            Operation::Multiply => {
                get_monkey(self.other_monkey1.clone().unwrap(), all_monkeys).get_number(all_monkeys)
                    * get_monkey(self.other_monkey2.clone().unwrap(), all_monkeys)
                        .get_number(all_monkeys)
            }
            Operation::Divide => {
                get_monkey(self.other_monkey1.clone().unwrap(), all_monkeys).get_number(all_monkeys)
                    / get_monkey(self.other_monkey2.clone().unwrap(), all_monkeys)
                        .get_number(all_monkeys)
            }
        }
    }
}

fn get_all_monkeys() -> HashMap<String, Monkey> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    let input = include_str!("monkeys.txt");

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().trim_end_matches(':');
        let v1 = parts.next().unwrap();

        if let Ok(n) = v1.parse::<isize>() {
            monkeys.insert(
                name.to_string(),
                Monkey {
                    other_monkey1: None,
                    other_monkey2: None,
                    operation: Operation::Number(n),
                },
            );
        } else {
            let other_monkey1 = v1;
            let operation = match parts.next().unwrap() {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => unreachable!(),
            };
            let other_monkey2 = parts.next().unwrap();

            monkeys.insert(
                name.to_string(),
                Monkey {
                    other_monkey1: Some(other_monkey1.to_string()),
                    other_monkey2: Some(other_monkey2.to_string()),
                    operation,
                },
            );
        }
    }

    monkeys
}

fn get_monkey(monkey_name: String, all_monkeys: &HashMap<String, Monkey>) -> &Monkey {
    all_monkeys.get(&monkey_name).unwrap()
}

fn solve_for_human(all_monkeys: &mut HashMap<String, Monkey>) -> isize {
    let root_monkey = all_monkeys.get("root").unwrap();
    let mut new_monkeys = all_monkeys.clone();

    // If we change one side and the calculations stay the same, the human has to be on the other side.
    let right_side =
        get_monkey(root_monkey.clone().other_monkey2.unwrap(), all_monkeys).get_number(all_monkeys);
    new_monkeys.get_mut("humn").unwrap().operation = Operation::Number(0);
    let new_right_side = get_monkey(root_monkey.clone().other_monkey2.unwrap(), &new_monkeys)
        .get_number(&new_monkeys);

    let (new_root_monkey, comparison_monkey) = if right_side == new_right_side {
        (
            root_monkey.clone().other_monkey1.unwrap(),
            root_monkey.clone().other_monkey2.unwrap(),
        )
    } else {
        (
            root_monkey.clone().other_monkey2.unwrap(),
            root_monkey.clone().other_monkey1.unwrap(),
        )
    };

    let mut low_bound = 0;
    let mut high_bound = 194_058_098_264_286; // The result from part one.

    // Brute forcing our way there.
    // The whole function only takes ~33ms in debug and ~7ms on release mode.
    loop {
        let human_value = (low_bound + high_bound) / 2;
        all_monkeys.get_mut("humn").unwrap().operation = Operation::Number(human_value);

        match all_monkeys
            .get(&new_root_monkey)
            .unwrap()
            .get_number(all_monkeys)
            .cmp(
                &all_monkeys
                    .get(&comparison_monkey)
                    .unwrap()
                    .get_number(all_monkeys),
            ) {
            Ordering::Greater => low_bound = human_value + 1,
            Ordering::Less => high_bound = human_value - 1,
            Ordering::Equal => return human_value - 1,
        }
    }
}

fn part_one() {
    let monkeys = get_all_monkeys();

    let result = monkeys.get("root").unwrap().get_number(&monkeys);

    println!("{:?}", result);
}

fn part_two() {
    let mut monkeys = get_all_monkeys();

    let result = solve_for_human(&mut monkeys);

    println!("{:?}", result);
}

fn main() {
    part_one();
    part_two();
}
