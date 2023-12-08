advent_of_code::solution!(7);

use regex::Regex;
use std::collections::HashMap;

const FORMAT: &str = r"([AKQJT98765432]+) (\d+)";

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(FORMAT).unwrap();
    let mut bids = re
        .captures_iter(input)
        .map(|caps| caps.extract::<2>())
        .map(|(_, cap)| {
            let mut freq = HashMap::new();
            for c in cap[0].chars() {
                freq.entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
            let kind = match freq.len() {
                5 => 0, // High card
                4 => 1, // One pair
                3 => {
                    match freq.values().max() {
                        Some(2) => 2, // Two pair
                        _ => 3,       // Three of a kind
                    }
                }
                2 => {
                    match freq.values().max() {
                        Some(3) => 4, // Full house
                        _ => 5,       // Four of a kind
                    }
                }
                _ => 6, // Five of a kind
            };
            let value = cap[0]
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap(),
                })
                .enumerate()
                .map(|(i, v)| v * 15_u32.pow(4 - i as u32))
                .sum::<u32>();
            (cap[0], cap[1].parse::<u32>().unwrap(), kind, value)
        })
        .collect::<Vec<_>>();
    bids.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.3.cmp(&b.3)));
    bids.iter()
        .enumerate()
        .map(|(i, bid)| bid.1 * (i + 1) as u32)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(FORMAT).unwrap();
    let mut bids = re
        .captures_iter(input)
        .map(|caps| caps.extract::<2>())
        .map(|(_, cap)| {
            let mut freq = HashMap::new();
            for c in cap[0].chars() {
                freq.entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
            let jokers = freq.remove(&'J').unwrap_or(0);
            freq.values_mut().max().map(|v| *v += jokers);
            let kind = match freq.len() {
                5 => 0, // High card
                4 => 1, // One pair
                3 => {
                    match freq.values().max() {
                        Some(2) => 2, // Two pair
                        _ => 3,       // Three of a kind
                    }
                }
                2 => {
                    match freq.values().max() {
                        Some(3) => 4, // Full house
                        _ => 5,       // Four of a kind
                    }
                }
                _ => 6, // Five of a kind
            };
            let value = cap[0]
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1, // Jokers are weakest
                    'T' => 10,
                    _ => c.to_digit(10).unwrap(),
                })
                .enumerate()
                .map(|(i, v)| v * 15_u32.pow(4 - i as u32))
                .sum::<u32>();
            (cap[0], cap[1].parse::<u32>().unwrap(), kind, value)
        })
        .collect::<Vec<_>>();
    bids.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.3.cmp(&b.3)));
    bids.iter()
        .enumerate()
        .map(|(i, bid)| bid.1 * (i + 1) as u32)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
