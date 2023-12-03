advent_of_code::solution!(3);

use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = input.lines().collect::<Vec<_>>();
    let part_re = Regex::new(r"\d+").unwrap();
    let symbol_re = Regex::new(r"[^0-9.]").unwrap();
    let mut sum = 0;
    for (row, content) in schematic.iter().enumerate() {
        for part in part_re.find_iter(content) {
            // check for symbols up from part
            if row > 0
                && symbol_re.is_match(
                    &schematic[row - 1]
                        [max(part.start(), 1) - 1..min(part.end() + 1, content.len())],
                )
                || row < schematic.len() - 1
                    && symbol_re.is_match(
                        &schematic[row + 1]
                            [max(part.start(), 1) - 1..min(part.end() + 1, content.len())],
                    )
                || symbol_re.is_match(
                    &schematic[row][max(part.start(), 1) - 1..min(part.end() + 1, content.len())],
                )
            {
                sum += part.as_str().parse::<u32>().unwrap();
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = input.lines().collect::<Vec<_>>();
    let part_re = Regex::new(r"\d+").unwrap();
    let gear_re = Regex::new(r"\*").unwrap();
    // Map of gear (x, y) to times they've been used and values
    let mut gear_map: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    for (row, content) in schematic.iter().enumerate() {
        for gear in gear_re.find_iter(content) {
            gear_map.insert((row as u32, gear.start() as u32), (0, 0));
        }
    }
    let mut sum = 0;
    let update_sum = |sum: &mut u32, times: &mut u32, value: &mut u32, part: &str| {
        *times += 1;
        // Gear should only be used twice, not more
        if *times == 1 {
            *value = part.parse::<u32>().unwrap();
        }
        if *times == 2 {
            *sum += *value * part.parse::<u32>().unwrap();
        }
        if *times == 3 {
            *sum -= *value * part.parse::<u32>().unwrap();
        }
    };
    for (row, content) in schematic.iter().enumerate() {
        for part in part_re.find_iter(content) {
            // Check for gears around part
            for col in max(part.start(), 1) - 1..min(part.end() + 1, content.len()) {
                if row > 0 {
                    if let Some((times, value)) = gear_map.get_mut(&(row as u32 - 1, col as u32)) {
                        update_sum(&mut sum, times, value, part.as_str());
                    }
                }
                if row < schematic.len() - 1 {
                    if let Some((times, value)) = gear_map.get_mut(&(row as u32 + 1, col as u32)) {
                        update_sum(&mut sum, times, value, part.as_str());
                    }
                }
            }
            if part.start() > 0 {
                if let Some((times, value)) =
                    gear_map.get_mut(&(row as u32, part.start() as u32 - 1))
                {
                    update_sum(&mut sum, times, value, part.as_str());
                }
            }
            if part.end() < content.len() {
                if let Some((times, value)) = gear_map.get_mut(&(row as u32, part.end() as u32)) {
                    update_sum(&mut sum, times, value, part.as_str());
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
