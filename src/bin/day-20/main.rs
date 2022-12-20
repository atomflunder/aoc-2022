// Day 20 of Advent of Code 2022.
// https://adventofcode.com/2022/day/20

use std::{cell::RefCell, cmp::Ordering, rc::Rc};

#[derive(Debug)]
struct NumberPartOne {
    value: isize,
    has_moved: bool,
}

impl NumberPartOne {
    fn new(value: isize) -> Self {
        Self {
            value,
            has_moved: false,
        }
    }
}

#[derive(Debug, Clone)]
struct NumberPartTwo {
    value: isize,
    index: RefCell<usize>,
}

impl NumberPartTwo {
    fn new(value: isize, index: usize) -> Self {
        Self {
            value: value * 811589153,
            index: RefCell::new(index),
        }
    }
}

trait Number {
    fn get_value(&self) -> isize;
}

impl Number for NumberPartOne {
    fn get_value(&self) -> isize {
        self.value
    }
}
impl Number for NumberPartTwo {
    fn get_value(&self) -> isize {
        self.value
    }
}

fn get_score(nums: Vec<impl Number>) -> isize {
    let zero = nums.iter().position(|e| e.get_value() == 0).unwrap();

    nums[(zero + 1000) % nums.len()].get_value()
        + nums[(zero + 2000) % nums.len()].get_value()
        + nums[(zero + 3000) % nums.len()].get_value()
}

fn part_one() {
    let input = include_str!("numbers.txt");

    let mut nums = input
        .lines()
        .map(|i| NumberPartOne::new(i.parse().unwrap()))
        .collect::<Vec<NumberPartOne>>();

    let mut i = 0;
    while i < nums.len() {
        if !nums[i].has_moved {
            let wrap = nums.len() as isize - 1;
            let mut new_index = (i as isize + nums[i].value) % wrap;
            if new_index < 0 {
                new_index += wrap;
            }
            let mut to_move = nums.remove(i);
            to_move.has_moved = true;
            nums.insert(new_index as usize, to_move);
        } else {
            i += 1;
        }
    }

    println!("{:?}", get_score(nums));
}

fn part_two() {
    let input = include_str!("numbers.txt");

    let mut original_order = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let n = line.parse::<isize>().unwrap();

        let entry = Rc::new(NumberPartTwo::new(n, i));
        original_order.push(entry.clone());
    }
    let mut nums = original_order.clone();

    let wrap = original_order.len() as isize - 1;

    for _ in 0..10 {
        for e in &original_order {
            let mut old_index = e.index.borrow_mut();
            let mut new_index = (*old_index as isize + e.value) % wrap;
            if new_index < 0 {
                new_index += wrap
            }
            let new_index = new_index as usize;

            match new_index.cmp(&old_index) {
                Ordering::Greater => {
                    for num in nums.iter().take(new_index + 1).skip(*old_index + 1) {
                        num.index.replace_with(|&mut old| old - 1);
                    }
                }
                Ordering::Less => {
                    for num in nums.iter().take(*old_index).skip(new_index) {
                        num.index.replace_with(|&mut old| old + 1);
                    }
                }
                Ordering::Equal => (),
            }

            let temp = nums.remove(*old_index);
            *old_index = new_index;
            nums.insert(new_index as usize, temp);
        }
    }

    println!(
        "{}",
        get_score(
            nums.iter()
                .map(|i| NumberPartTwo {
                    value: i.value,
                    index: i.index.clone()
                })
                .collect::<Vec<NumberPartTwo>>()
        )
    );
}

fn main() {
    part_one();
    part_two();
}
