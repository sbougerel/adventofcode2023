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
    let mut gear_map: HashMap<(isize, isize), (u32, u32)> = HashMap::new();
    for (row, content) in schematic.iter().enumerate() {
        for gear in gear_re.find_iter(content) {
            gear_map.insert((row as isize, gear.start() as isize), (0, 0));
        }
    }
    let mut sum = 0;
    let mut update_sum = |sum: &mut u32, part: &str, row: isize, col: isize| {
        if let Some((times, value)) = gear_map.get_mut(&(row, col)) {
            *times += 1;
            // Gear should only be used twice, not more
            match *times {
                1 => *value = part.parse::<u32>().unwrap(),
                2 => *sum += *value * part.parse::<u32>().unwrap(),
                3 => *sum -= *value * part.parse::<u32>().unwrap(),
                _ => return,
            }
        }
    };
    for (row, content) in schematic.iter().enumerate() {
        let row = row as isize;
        for part in part_re.find_iter(content) {
            let start = part.start() as isize;
            let end = part.end() as isize;
            // Check for gears around part
            for col in start - 1..end + 1 {
                update_sum(&mut sum, part.as_str(), row - 1, col);
                update_sum(&mut sum, part.as_str(), row + 1, col);
            }
            update_sum(&mut sum, part.as_str(), row, start - 1);
            update_sum(&mut sum, part.as_str(), row, end);
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
