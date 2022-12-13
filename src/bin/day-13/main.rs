// Day 13 of Advent of Code 2022.
// https://adventofcode.com/2022/day/13

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(x), Packet::Num(y)) => x.cmp(y),
            (Packet::Num(x), Packet::List(_)) => Packet::List(vec![Packet::Num(*x)]).cmp(other),
            (Packet::List(_), Packet::Num(x)) => self.cmp(&Packet::List(vec![Packet::Num(*x)])),
            (Packet::List(x), Packet::List(y)) => {
                for i in 0..x.len().min(y.len()) {
                    match x[i].cmp(&y[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }
                x.len().cmp(&y.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::str::FromStr for Packet {
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn get_packets(tokens: &mut Vec<&str>) -> Result<Vec<Packet>, std::num::ParseIntError> {
            let mut result = Vec::new();
            while !tokens.is_empty() {
                match tokens.pop() {
                    Some("]") => {
                        return Ok(result);
                    }
                    Some("[") => {
                        result.push(Packet::List(get_packets(tokens)?));
                    }
                    Some("") => {}
                    Some(n) => {
                        result.push(Packet::Num(n.parse::<i32>()?));
                    }
                    None => unreachable!(),
                };
            }
            Ok(result)
        }

        let binding = line.replace('[', "[,").replace(']', ",]");
        let mut tokens = binding.split(',').rev().collect::<Vec<_>>();
        Ok(Packet::List(get_packets(&mut tokens)?))
    }

    type Err = std::num::ParseIntError;
}

fn part_one() {
    let packets = include_str!("packets.txt");

    let mut in_order = 0;

    for (i, pair) in packets.split("\r\n\r\n").enumerate() {
        let mut lines = pair.lines();
        let left = lines.next().unwrap().parse::<Packet>().unwrap();
        let right = lines.next().unwrap().parse::<Packet>().unwrap();
        if left < right {
            in_order += i + 1
        }
    }
    println!("{:?}", in_order);
}

fn part_two() {
    let packets = include_str!("packets.txt");

    let mut all_packets = packets
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Packet>().unwrap())
        .collect::<Vec<_>>();

    let divider1 = "[[2]]".parse::<Packet>().unwrap();
    let divider2 = "[[6]]".parse::<Packet>().unwrap();
    all_packets.push(divider1.clone());
    all_packets.push(divider2.clone());

    all_packets.sort();

    let divider1_pos = all_packets.iter().position(|p| p == &divider1).unwrap() + 1;
    let divider2_pos = all_packets.iter().position(|p| p == &divider2).unwrap() + 1;
    println!("{:?}", (divider1_pos * divider2_pos));
}

fn main() {
    part_one();
    part_two();
}
