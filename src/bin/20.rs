advent_of_code::solution!(20);

use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone)]
enum Cat {
    Broad,
    Flip,
    Con,
}

#[derive(Debug)]
struct Mod {
    cat: Cat,
    dests: Vec<String>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

type FlipStates = HashMap<String, bool>;
type ConStates = HashMap<String, HashMap<String, Pulse>>;
type Mods = HashMap<String, Mod>;

fn parse(input: &str) -> Mods {
    input
        .lines()
        .map(|l| {
            let (module, dests) = l.trim().split_once(" -> ").unwrap();
            let (cat, address) = match &module[0..1] {
                "b" => (Cat::Broad, "broadcast".to_string()),
                "%" => (Cat::Flip, module[1..].to_string()),
                "&" => (Cat::Con, module[1..].to_string()),
                _ => panic!("(╯°□°)╯︵ ┻━┻"),
            };
            (
                address,
                Mod {
                    cat,
                    dests: dests.split(", ").map(|s| s.to_string()).collect::<Vec<_>>(),
                },
            )
        })
        .collect::<Mods>()
}

fn states_init(mods: &Mods) -> (FlipStates, ConStates) {
    let mut flips = FlipStates::new();
    let mut cons = ConStates::new();
    for (k, v) in mods.iter() {
        match v.cat {
            Cat::Broad => {}
            Cat::Flip => {
                flips.insert(k.to_string(), false);
            }
            Cat::Con => {
                cons.insert(k.to_string(), HashMap::new());
            }
        }
    }
    // Set states for ConStates, iterate through all dests
    for (k, v) in mods.iter() {
        for dest in v.dests.iter() {
            if let Some(con) = cons.get_mut(dest) {
                con.insert(k.to_string(), Pulse::Low);
            }
        }
    }
    (flips, cons)
}

fn send_pulse(
    mods: &Mods,
    flips: &mut FlipStates,
    cons: &mut ConStates,
    watch: Option<&str>,
) -> (u64, u64, HashMap<String, (u64, u64)>) {
    let mut count_low = 0;
    let mut count_high = 0;
    let mut pending = VecDeque::from([(Pulse::Low, "button".to_string(), "broadcast".to_string())]);
    let mut watch_list = HashMap::new();
    while !pending.is_empty() {
        let (pulse, src, here) = pending.pop_front().unwrap();
        // println!("{} --{:?}-> {}", src, pulse, here);
        match pulse {
            Pulse::High => count_high += 1,
            Pulse::Low => count_low += 1,
        }
        if let Some(w) = watch {
            if here == w {
                let (inc_low, inc_high) = match pulse {
                    Pulse::High => (0, 1),
                    Pulse::Low => (1, 0),
                };
                watch_list
                    .entry(src.clone())
                    .and_modify(|(low, high)| {
                        *low += inc_low;
                        *high += inc_high;
                    })
                    .or_insert((inc_low, inc_high));
            }
        }
        if !mods.contains_key(&here) {
            // Handles "rx" modules, e.g.
            continue;
        }
        let module = &mods[&here];
        match module.cat {
            Cat::Broad => {
                // Broadcast the *same* pulse to dest
                for dest in module.dests.iter() {
                    pending.push_back((pulse, here.clone(), dest.to_string()));
                }
            }
            Cat::Con => {
                // Remember most recent pulse, if all high -> low pulse, else -> high pulse
                cons.get_mut(&here)
                    .unwrap()
                    .entry(src)
                    .and_modify(|v| *v = pulse);
                let inv = match cons[&here].values().all(|v| *v == Pulse::High) {
                    true => Pulse::Low,
                    false => Pulse::High,
                };
                for dest in module.dests.iter() {
                    pending.push_back((inv, here.clone(), dest.to_string()));
                }
            }
            Cat::Flip => {
                // Ignore high pulse, flip on low pulse, true -> high pulse, false -> low pulse
                if pulse == Pulse::High {
                    continue;
                }
                flips.entry(here.clone()).and_modify(|v| *v = !*v);
                let same = match flips[&here] {
                    true => Pulse::High,
                    false => Pulse::Low,
                };
                for dest in module.dests.iter() {
                    pending.push_back((same, here.clone(), dest.to_string()));
                }
            }
        }
    }
    //println!("{}, {}", count_low, count_high);
    (count_low, count_high, watch_list)
}

#[allow(dead_code)]
fn print_graph(modules: &Mods) {
    let mut keys = modules.keys().collect::<Vec<_>>();
    keys.sort();
    for key in keys {
        let m = &modules[key];
        match m.cat {
            Cat::Broad => {}
            Cat::Con => {
                print!("&");
            }
            Cat::Flip => {
                print!("%");
            }
        }
        print!("{} -> {}", key, m.dests[0]);
        for d in &m.dests[1..] {
            print!(", {}", d);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mods = parse(&input);
    //print_graph(&mods);
    let (mut flipstates, mut constates) = states_init(&mods);
    let (low, high) = (0..1000)
        .map(|_| send_pulse(&mods, &mut flipstates, &mut constates, None))
        .fold((0, 0), |acc, e| (e.0 + acc.0, e.1 + acc.1));
    Some(low * high)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mods = parse(&input);
    //print_graph(&mods);
    // I do not know how to solve this in the general case.  After pondering for
    // a while what is the solution, I just looked at the tree of modules that
    // output to rx:
    //
    //       0      1       2      3...
    // rx <- &gf <- &qs <- &mh <- %nr, %jh, ...
    //           <- &sv <- &jt <- %nk, ...
    //           <- &pg <- &pz <- ...
    //           <- &sp <- &rn <- ...
    //
    // At the third level, it's a long serie of %flip modules.  So if all these
    // layer-3 modules most recent pulses where high pulses, the layer-2 &conj
    // modules will all send a low pulse, which will get inverted to high pulse
    // by layer-1s and result in layer-0 &gf sending a low pulse.
    //
    // So it's clear that layer-1 sends mostly a bunch of low pulses, and once
    // in a while, when all the layer-3 modules sync on a high pulse, layer-0
    // receives a single high pulse.
    //
    // Therefore in the code below, we're going to simply watch the frequency at
    // which qs, sv, pg and sp send a high pulse to gf, and bet it's periodic,
    // then compute the LCM of the period.
    let (mut flipstates, mut constates) = states_init(&mods);

    let mut first_high_pulse: HashMap<String, usize> = HashMap::new();
    let mut press_count = 0;

    let output = "rx"; // For a slightly more general solution, find "rx"'s source.
    let watch = mods
        .iter()
        .find(|(_, m)| m.dests.contains(&output.to_owned()))
        .unwrap()
        .0;
    let watch_len = constates[watch].len();
    while first_high_pulse.len() < watch_len {
        press_count += 1;
        let (_, _, src_count) = send_pulse(&mods, &mut flipstates, &mut constates, Some(watch));

        for (src, (_, high)) in src_count.iter() {
            if *high > 0 {
                assert!(*high == 1); // Test assumptions
                let first = first_high_pulse
                    .entry(src.to_string())
                    .or_insert(press_count);
                assert!(press_count % *first == 0); // Test assumptions
            }
        }
    }

    Some(first_high_pulse.values().fold(1, |a, e| lcm(a, *e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
