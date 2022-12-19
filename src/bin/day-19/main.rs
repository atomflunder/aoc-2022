// Day 19 of Advent of Code 2022.
// https://adventofcode.com/2022/day/19

use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Blueprint {
    id: isize,
    ore: isize,
    clay: isize,
    obsidian: (isize, isize),
    geode: (isize, isize),
}

impl Blueprint {
    fn new(
        id: isize,
        ore: isize,
        clay: isize,
        obsidian: (isize, isize),
        geode: (isize, isize),
    ) -> Self {
        Self {
            id,
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn work(&self, max_ticks: isize) -> isize {
        let mut geode = 0;
        let starting_state = State::new();

        let mut seen_states = HashSet::new();
        let mut unseen_states = VecDeque::new();

        unseen_states.push_back(starting_state);

        let max_ore_cost = *[self.ore, self.clay, self.obsidian.0, self.geode.0]
            .iter()
            .max()
            .unwrap();

        while let Some(mut state) = unseen_states.pop_front() {
            geode = geode.max(state.total_geode);

            if state.total_geode < geode - 1 || seen_states.contains(&state) {
                continue;
            };

            seen_states.insert(state);
            if state.tick == max_ticks {
                continue;
            }

            // Prioritizing making geode robots over any other.
            if state.total_ore >= self.geode.0 && state.total_obsidian >= self.geode.1 {
                let mut next_state = state;
                next_state.total_ore -= self.geode.0;
                next_state.total_obsidian -= self.geode.1;
                next_state.one_tick();
                next_state.geode_robots += 1;
                unseen_states.push_back(next_state);
            } else {
                if state.total_ore >= self.ore && state.ore_robots < max_ore_cost {
                    let mut next_state = state;
                    next_state.total_ore -= self.ore;
                    next_state.one_tick();
                    next_state.ore_robots += 1;
                    unseen_states.push_back(next_state);
                }
                if state.total_ore >= self.clay {
                    let mut next_state = state;
                    next_state.total_ore -= self.clay;
                    next_state.one_tick();
                    next_state.clay_robots += 1;
                    unseen_states.push_back(next_state);
                }
                if state.total_ore >= self.obsidian.0 && state.total_clay >= self.obsidian.1 {
                    let mut next_state = state;
                    next_state.total_ore -= self.obsidian.0;
                    next_state.total_clay -= self.obsidian.1;
                    next_state.one_tick();
                    next_state.obsidian_robots += 1;
                    unseen_states.push_back(next_state);
                }
                state.one_tick();
                unseen_states.push_back(state);
            }
        }

        geode
    }
}

fn get_all_blueprints() -> Vec<Blueprint> {
    let input = include_str!("blueprints.txt");
    let mut blueprints: Vec<Blueprint> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let split: Vec<isize> = line
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        blueprints.push(Blueprint::new(
            (i + 1).try_into().unwrap(),
            split[6],
            split[12],
            (split[18], split[21]),
            (split[27], split[30]),
        ));
    }

    blueprints
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    tick: isize,
    total_ore: isize,
    ore_robots: isize,
    total_clay: isize,
    clay_robots: isize,
    total_obsidian: isize,
    obsidian_robots: isize,
    total_geode: isize,
    geode_robots: isize,
}

impl State {
    fn new() -> Self {
        Self {
            tick: 0,
            total_ore: 0,
            ore_robots: 1,
            total_clay: 0,
            clay_robots: 0,
            total_obsidian: 0,
            obsidian_robots: 0,
            total_geode: 0,
            geode_robots: 0,
        }
    }

    fn one_tick(&mut self) {
        self.total_ore += self.ore_robots;
        self.total_clay += self.clay_robots;
        self.total_obsidian += self.obsidian_robots;
        self.total_geode += self.geode_robots;
        self.tick += 1;
    }
}

fn part_one() {
    let mut sum = 0;
    let blueprints = get_all_blueprints();

    for blueprint in blueprints {
        sum += blueprint.work(24) * blueprint.id;
    }

    println!("{:?}", sum);
}

fn part_two() {
    let mut product = 1;
    let binding = get_all_blueprints();
    let blueprints = binding.iter().take(3).collect::<Vec<&Blueprint>>();

    for blueprint in blueprints {
        product *= blueprint.work(32);
    }

    println!("{:?}", product);
}

fn main() {
    part_one();
    part_two();
}
