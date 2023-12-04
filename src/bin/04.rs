advent_of_code::solution!(4);

use regex::Regex;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let card_re = Regex::new(r"Card +\d+:([\d ]+)\|([\d ]+)").unwrap();
    let as_set = |s: &str| {
        s.split(' ')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<HashSet<u32>>()
    };
    input
        .lines()
        .map(|line| {
            let cap = card_re.captures(line).unwrap();
            let win = as_set(&cap[1]);
            let have = as_set(&cap[2]);
            let count = win.intersection(&have).count() as u32;
            if count == 0 {
                0
            } else {
                2_u32.pow(count - 1)
            }
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let card_re = Regex::new(r"Card +\d+:([\d ]+)\|([\d ]+)").unwrap();
    let as_set = |s: &str| {
        s.split(' ')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<HashSet<u32>>()
    };
    let wins = input
        .lines()
        .map(|line| {
            let cap = card_re.captures(line).unwrap();
            let win = as_set(&cap[1]);
            let have = as_set(&cap[2]);
            win.intersection(&have).count()
        })
        .collect::<Vec<_>>();
    let mut copies = vec![1; wins.len()];
    for i in 0..wins.len() {
        for j in 0..wins[i] {
            copies[i + j + 1] += copies[i];
        }
    }
    copies.iter().sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
