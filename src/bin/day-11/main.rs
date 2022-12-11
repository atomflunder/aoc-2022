// Day 11 of Advent of Code 2022.
// https://adventofcode.com/2022/day/11

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    inspections: usize,
    operation: Operation,
    divisible_by: usize,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Operation,
        divisible_by: usize,
        true_monkey: usize,
        false_monkey: usize,
    ) -> Self {
        Self {
            items,
            inspections: 0,
            operation,
            divisible_by,
            true_monkey,
            false_monkey,
        }
    }
}

// Again, could not be bothered with reading the input, so manual implementation it is.
fn get_monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(
            vec![52, 60, 85, 69, 75, 75],
            Operation::Multiply(17),
            13,
            6,
            7,
        ),
        Monkey::new(vec![96, 82, 61, 99, 82, 84, 85], Operation::Add(8), 7, 0, 7),
        Monkey::new(vec![95, 79], Operation::Add(6), 19, 5, 3),
        Monkey::new(vec![88, 50, 82, 65, 77], Operation::Multiply(19), 2, 4, 1),
        Monkey::new(
            vec![66, 90, 59, 90, 87, 63, 53, 88],
            Operation::Add(7),
            5,
            1,
            0,
        ),
        Monkey::new(vec![92, 75, 62], Operation::Square, 3, 3, 4),
        Monkey::new(vec![94, 86, 76, 67], Operation::Add(1), 11, 5, 2),
        Monkey::new(vec![57], Operation::Add(2), 17, 6, 2),
    ]
}

fn part_one() {
    let mut monkeys = get_monkeys();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            // Going through the items in reverse order, but it does not really matter in this case.
            while let Some(item) = monkeys[i].items.pop() {
                let monkey = &monkeys[i];
                let item = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                } / 3;
                let target = if item % monkey.divisible_by == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                monkeys[i].inspections += 1;
                monkeys[target].items.push(item);
            }
        }
    }

    let mut inspections = vec![];

    for m in monkeys {
        inspections.push(m.inspections);
    }

    inspections.sort();
    inspections.reverse();

    println!("{:?}", inspections[0] * inspections[1]);
}

fn part_two() {
    let mut monkeys = get_monkeys();

    // In order to not get numbers that are way too big and overflow the usize,
    // we just need a way to check if the current "worry level" is divisible by the monkey's divisivle_by value.
    // So we modulo everything by the product of every monkey's divisible by value.
    // In my case this is equal to 9699690.
    // Had to refresh my memory with this: https://en.wikipedia.org/wiki/Modular_arithmetic
    let modulo = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.divisible_by);

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                let monkey = &monkeys[i];
                let item = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                } % modulo;
                let target = if item % monkey.divisible_by == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                monkeys[i].inspections += 1;
                monkeys[target].items.push(item);
            }
        }
    }

    let mut inspections = vec![];

    for m in monkeys {
        inspections.push(m.inspections);
    }

    inspections.sort();
    inspections.reverse();

    println!("{:?}", inspections[0] * inspections[1]);
}

fn main() {
    part_one();
    part_two();
}
