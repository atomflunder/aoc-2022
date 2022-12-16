// Day 16 of Advent of Code 2022.
// https://adventofcode.com/2022/day/16

use std::collections::{HashMap, HashSet};

struct Valve {
    flow_rate: usize,
    leads_to: Vec<String>,
}

fn get_valves() -> HashMap<String, Valve> {
    let input = include_str!("valves.txt");
    let mut valves: HashMap<String, Valve> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("; ").unwrap();
        let (a, flow_rate) = a.split_once(" has flow rate=").unwrap();
        let (_, valve_name) = a.split_once(' ').unwrap();

        let (_, b) = b.split_once(" to ").unwrap();
        let (_, tunnels) = b.split_once(' ').unwrap();

        let valve = Valve {
            flow_rate: flow_rate.parse().unwrap(),
            leads_to: tunnels.split(", ").map(|s| s.to_string()).collect(),
        };

        valves.insert(valve_name.to_string(), valve);
    }

    valves
}

fn part_one() {
    let valves: HashMap<String, Valve> = get_valves();
    let mut cache: HashMap<(usize, String, usize), usize> = HashMap::new();
    let open_valves: HashSet<String> = HashSet::new();

    fn brute_force(
        minute: usize,
        current_location: &str,
        flow_rate: usize,
        current_score: usize,
        open_valves: &HashSet<String>,
        valves: &HashMap<String, Valve>,
        cache: &mut HashMap<(usize, String, usize), usize>,
    ) -> Option<usize> {
        if minute > 30 {
            return Some(current_score);
        }

        let cache_key = (minute, current_location.to_string(), flow_rate);
        if let Some(cached_value) = cache.get(&cache_key) {
            if *cached_value >= current_score {
                return None;
            }
        }
        cache.insert(cache_key, current_score);

        let current_valve = valves.get(current_location).unwrap();

        let best_result_open_current =
            if current_valve.flow_rate > 0 && !open_valves.contains(current_location) {
                let mut new_open_valves = open_valves.iter().cloned().collect::<HashSet<_>>();
                new_open_valves.insert(current_location.to_string());

                let new_score = current_score + flow_rate;
                let new_flow_rate = flow_rate + current_valve.flow_rate;
                brute_force(
                    minute + 1,
                    current_location,
                    new_flow_rate,
                    new_score,
                    &new_open_valves,
                    valves,
                    cache,
                )
            } else {
                None
            };

        let best_result_down_tunnels = current_valve
            .leads_to
            .iter()
            .filter_map(|next_valve_name| {
                brute_force(
                    minute + 1,
                    next_valve_name,
                    flow_rate,
                    current_score + flow_rate,
                    open_valves,
                    valves,
                    cache,
                )
            })
            .max();

        best_result_down_tunnels.max(best_result_open_current)
    }

    let result = brute_force(1, "AA", 0, 0, &open_valves, &valves, &mut cache).unwrap();

    println!("{:?}", result);
}

fn part_two() {
    let valves: HashMap<String, Valve> = get_valves();
    let mut cache: HashMap<(usize, String, String, usize), usize> = HashMap::new();
    let open_valves: HashSet<String> = HashSet::new();

    #[allow(clippy::too_many_arguments)]
    fn brute_force(
        minute: usize,
        current_location: &str,
        elephant_location: &str,
        flow_rate: usize,
        current_score: usize,
        open_valves: &HashSet<String>,
        valves: &HashMap<String, Valve>,
        cache: &mut HashMap<(usize, String, String, usize), usize>,
    ) -> Option<usize> {
        if minute > 26 {
            return Some(current_score);
        }

        let cache_key = (
            minute,
            current_location.to_string(),
            elephant_location.to_string(),
            flow_rate,
        );
        if let Some(cached_value) = cache.get(&cache_key) {
            if *cached_value >= current_score {
                return None;
            }
        }
        cache.insert(cache_key, current_score);

        let (my_flow_rate, my_tunnels) = {
            let valve = valves.get(current_location).unwrap();
            (valve.flow_rate, valve.leads_to.to_vec())
        };
        let (elephant_flow_rate, elephant_tunnels) = {
            let valve = valves.get(elephant_location).unwrap();
            (valve.flow_rate, valve.leads_to.to_vec())
        };

        let can_open_my_valve = my_flow_rate > 0 && !open_valves.contains(current_location);
        let can_open_elephant_valve =
            elephant_flow_rate > 0 && !open_valves.contains(elephant_location);
        let mut results = Vec::new();

        if can_open_my_valve {
            // open my valve, elephant moves
            let mut new_open_valves = open_valves.iter().cloned().collect::<HashSet<_>>();
            new_open_valves.insert(current_location.to_string());

            for new_elephant_location in elephant_tunnels.iter() {
                results.push(brute_force(
                    minute + 1,
                    current_location,
                    new_elephant_location,
                    flow_rate + my_flow_rate,
                    current_score + flow_rate,
                    &new_open_valves,
                    valves,
                    cache,
                ));
            }
        }

        if can_open_elephant_valve {
            // open elephant valve, i move
            let mut new_open_valves = open_valves.iter().cloned().collect::<HashSet<_>>();
            new_open_valves.insert(elephant_location.to_string());

            for new_my_location in my_tunnels.iter() {
                results.push(brute_force(
                    minute + 1,
                    new_my_location,
                    elephant_location,
                    flow_rate + elephant_flow_rate,
                    current_score + flow_rate,
                    &new_open_valves,
                    valves,
                    cache,
                ));
            }
        }

        if can_open_elephant_valve && can_open_my_valve && current_location != elephant_location {
            let mut new_open_valves = open_valves.iter().cloned().collect::<HashSet<_>>();
            new_open_valves.insert(elephant_location.to_string());
            new_open_valves.insert(current_location.to_string());

            results.push(brute_force(
                minute + 1,
                current_location,
                elephant_location,
                flow_rate + my_flow_rate + elephant_flow_rate,
                current_score + flow_rate,
                &new_open_valves,
                valves,
                cache,
            ));
        }

        for new_elephant_location in elephant_tunnels.iter() {
            for new_my_location in my_tunnels.iter() {
                results.push(brute_force(
                    minute + 1,
                    new_my_location,
                    new_elephant_location,
                    flow_rate,
                    current_score + flow_rate,
                    open_valves,
                    valves,
                    cache,
                ));
            }
        }

        results.into_iter().flatten().max()
    }

    let result = brute_force(1, "AA", "AA", 0, 0, &open_valves, &valves, &mut cache).unwrap();

    println!("{:?}", result);
}

fn main() {
    part_one();
    part_two();
}
