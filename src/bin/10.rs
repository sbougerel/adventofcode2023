advent_of_code::solution!(10);

use std::collections::{HashMap, HashSet, VecDeque};

fn build_data(
    input: &str,
) -> (
    HashMap<(isize, isize), char>,
    HashSet<(isize, isize)>,
    isize,
    isize,
    u32,
) {
    let mut start = (0, 0);
    let mut field = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '.' => {}
                'S' => {
                    field.insert((x as isize, y as isize), c);
                    start = (x as isize, y as isize);
                }
                _ => {
                    field.insert((x as isize, y as isize), c);
                }
            }
            width = x as isize;
        }
        height = y as isize;
    }
    width += 1;
    height += 1;
    // Initialize breath-first search of the loop
    let mut open = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    match field.get(&(start.0, start.1 - 1)).unwrap_or(&'.') {
        '|' | 'F' | '7' => open.push_back((start.0, start.1 - 1, 1)),
        _ => {}
    }
    match field.get(&(start.0, start.1 + 1)).unwrap_or(&'.') {
        '|' | 'L' | 'J' => open.push_back((start.0, start.1 + 1, 1)),
        _ => {}
    }
    match field.get(&(start.0 - 1, start.1)).unwrap_or(&'.') {
        '-' | 'L' | 'F' => open.push_back((start.0 - 1, start.1, 1)),
        _ => {}
    }
    match field.get(&(start.0 + 1, start.1)).unwrap_or(&'.') {
        '-' | '7' | 'J' => open.push_back((start.0 + 1, start.1, 1)),
        _ => {}
    }
    let mut distance = 0;
    while open.len() > 0 {
        let (x, y, steps) = open.pop_front().unwrap();
        if visited.contains(&(x, y)) {
            distance = steps;
            break;
        } else {
            visited.insert((x, y));
        }
        let (front, back) = match field.get(&(x, y)).unwrap() {
            '-' => ((x + 1, y), (x - 1, y)),
            '7' => ((x - 1, y), (x, y + 1)),
            'L' => ((x + 1, y), (x, y - 1)),
            'J' => ((x - 1, y), (x, y - 1)),
            'F' => ((x + 1, y), (x, y + 1)),
            _ => ((x, y + 1), (x, y - 1)), // '|'
        };
        if !visited.contains(&front) {
            open.push_back((front.0, front.1, steps + 1));
        }
        if !visited.contains(&back) {
            open.push_back((back.0, back.1, steps + 1));
        }
    }
    (field, visited, width, height, distance)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(build_data(input).4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (field, visited, width, height, _) = build_data(input);
    let mut enclosed = 0;
    for x in 0..width {
        let mut inside = false;
        let mut entry = '.';
        for y in 0..height {
            if visited.contains(&(x, y)) {
                match field[&(x, y)] {
                    '-' => {
                        inside = !inside;
                    }
                    'F' | '7' => {
                        entry = field[&(x, y)];
                    }
                    'J' => {
                        if entry == 'F' {
                            inside = !inside;
                        }
                    }
                    'L' => {
                        if entry == '7' {
                            inside = !inside;
                        }
                    }
                    _ => {}
                }
            } else {
                if inside {
                    enclosed += 1;
                }
            }
        }
    }
    Some(enclosed)
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
        assert_eq!(result, Some(1));
    }
}
