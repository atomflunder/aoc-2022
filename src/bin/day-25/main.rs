// Day 25 of Advent of Code 2022.
// https://adventofcode.com/2022/day/25

#[derive(Debug)]
struct Number {
    base_snafu: String,
    base_10: isize,
}

impl Number {
    fn from_base_10(num: isize) -> Self {
        let mut i = num;

        let mut snafu: Vec<char> = Vec::new();

        while i > 0 {
            let buffer = i;
            i /= 5;
            i += match buffer % 5 {
                0 => {
                    snafu.push('0');
                    0
                }
                1 => {
                    snafu.push('1');
                    0
                }
                2 => {
                    snafu.push('2');
                    0
                }
                3 => {
                    snafu.push('=');
                    1
                }
                4 => {
                    snafu.push('-');
                    1
                }
                _ => unreachable!(),
            }
        }

        snafu.reverse();

        let str: String = snafu.iter().collect();

        Self {
            base_snafu: str,
            base_10: num,
        }
    }

    fn from_base_snafu(snafu: String) -> Self {
        let mut i = 0;
        let mut radix = 1;

        for c in snafu.chars().into_iter().rev() {
            match c {
                '0' => (),
                '1' => i += radix,
                '2' => i += 2 * radix,
                '-' => i -= radix,
                '=' => i -= 2 * radix,
                _ => unreachable!(),
            }
            radix *= 5;
        }

        Self {
            base_snafu: snafu,
            base_10: i,
        }
    }
}

// There is no part two on Day 25.
fn main() {
    let input = include_str!("snafu.txt");

    let snafu_result = input
        .lines()
        .map(|i| {
            let s = Number::from_base_snafu(i.to_string());
            s.base_10
        })
        .sum::<isize>();

    println!("{}", Number::from_base_10(snafu_result).base_snafu);
}
