advent_of_code::solution!(2);

use regex::Regex;
use std::cmp::max;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    Regex::new(r"(?m)^Game (\d+): (\d+ \w+(?:[,;] \d+ \w+)*)$")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            let mut game = Game {
                id: cap[1].parse().unwrap(),
                red: 0,
                green: 0,
                blue: 0,
            };
            for pick in cap[2].split("; ").map(|s| s.split(", ")).flatten() {
                let mut split = pick.split(' ');
                let count = split.next().unwrap().parse().unwrap();
                let color = split.next().unwrap();
                match color {
                    "red" => game.red = max(count, game.red),
                    "green" => game.green = max(count, game.green),
                    "blue" => game.blue = max(count, game.blue),
                    _ => panic!("Unknown color: {}", color),
                }
            }
            game
        })
        .filter(|game| game.red <= MAX_RED && game.green <= MAX_GREEN && game.blue <= MAX_BLUE)
        .map(|game| game.id)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    Regex::new(r"(?m)^Game (\d+): (\d+ \w+(?:[,;] \d+ \w+)*)$")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            let mut game = Game {
                id: cap[1].parse().unwrap(),
                red: 0,
                green: 0,
                blue: 0,
            };
            for pick in cap[2].split("; ").map(|s| s.split(", ")).flatten() {
                let mut split = pick.split(' ');
                let count = split.next().unwrap().parse().unwrap();
                let color = split.next().unwrap();
                match color {
                    "red" => game.red = max(count, game.red),
                    "green" => game.green = max(count, game.green),
                    "blue" => game.blue = max(count, game.blue),
                    _ => panic!("Unknown color: {}", color),
                }
            }
            game
        })
        .map(|game| game.red * game.green * game.blue)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
