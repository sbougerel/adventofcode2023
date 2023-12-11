advent_of_code::solution!(11);

use num::abs;
use std::collections::HashMap;

pub fn distance(input: &str, expand: i64) -> Option<i64> {
    let mut stars = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    // Do row expension and stars
    let mut add = 0;
    for (y, l) in input.lines().enumerate() {
        let mut empty = true;
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    stars.insert((x, y), (0, add));
                    empty = false;
                }
                _ => {}
            }
            width = x; // hackish, but ok
        }
        if empty {
            add += 1;
        }
        height = y;
    }
    width += 1;
    height += 1;
    // Do column expension now
    add = 0;
    for x in 0..width {
        let mut empty = true;
        for y in 0..height {
            if stars.contains_key(&(x, y)) {
                stars.insert((x, y), (add, stars[&(x, y)].1));
                empty = false;
            }
        }
        if empty {
            add += 1;
        }
    }
    // Compute distance for each pairs
    let mut star_vec = stars.keys().collect::<Vec<_>>();
    let mut dist = 0;
    while star_vec.len() > 0 {
        let star = star_vec.pop().unwrap();
        for o in &star_vec {
            let (i, j) = stars[&star];
            let (k, l) = stars[&o];
            dist += abs((star.0 as i64 + (expand - 1) * i) - (o.0 as i64 + (expand - 1) * k))
                + abs((star.1 as i64 + (expand - 1) * j) - (o.1 as i64 + (expand - 1) * l));
        }
    }
    Some(dist)
}

pub fn part_one(input: &str) -> Option<i64> {
    distance(input, 2)
}
pub fn part_two(input: &str) -> Option<i64> {
    distance(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
