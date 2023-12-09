advent_of_code::solution!(8);

use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let split = input.find("\n\n").unwrap();
    let instructions = input[0..split].trim().chars().collect::<Vec<_>>();
    let mut adj_list = Vec::new();
    let mut vertices = HashMap::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for (id, (_, [vertex, left, right])) in re
        .captures_iter(&input[split..])
        .map(|c| c.extract())
        .enumerate()
    {
        vertices.insert(vertex, id);
        adj_list.push((left, right));
    }
    let mut pos = "AAA";
    let mut count = 0;
    while pos != "ZZZ" {
        let index = vertices[&pos];
        match instructions[count % instructions.len()] {
            'L' => pos = adj_list[index].0,
            _ => pos = adj_list[index].1,
        }
        count += 1;
    }
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let split = input.find("\n\n").unwrap();
    let instructions = input[0..split].trim().chars().collect::<Vec<_>>();
    let mut adj_list = Vec::new();
    let mut vertices = HashMap::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for (id, (_, [vertex, left, right])) in re
        .captures_iter(&input[split..])
        .map(|c| c.extract())
        .enumerate()
    {
        vertices.insert(vertex, id);
        adj_list.push((left, right));
    }
    let mut pos = vertices
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<_>>();
    let mut periods = vec![0; pos.len()];
    let mut periods_start = vec![0; pos.len()];
    let mut count = 0;
    let mut stop = false;
    while !stop {
        for (i, p) in pos.iter_mut().enumerate() {
            let index = vertices[*p];
            match instructions[count % instructions.len()] {
                'L' => *p = &adj_list[index].0,
                _ => *p = &adj_list[index].1,
            }
            if p.ends_with("Z") {
                if periods_start[i] == 0 {
                    periods_start[i] = count + 1;
                } else {
                    if periods[i] == 0 {
                        periods[i] = count - periods_start[i] + 1;
                        if periods.iter().filter(|x| **x != 0).count() == periods.len() {
                            stop = true;
                        }
                    }
                }
            }
        }
        count += 1;
    }
    periods
        .iter()
        .fold(1_u64, |acc: u64, e| lcm(acc, *e as u64))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
