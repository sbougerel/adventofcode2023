advent_of_code::solution!(10);

use std::collections::{vec_deque, HashMap, HashSet, VecDeque};

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
    // Find the ending position in the pipe loop, each pipe is only connected with 2 other segments.
    // Breadth-first search
    let mut open = VecDeque::new();
    match field.get(&(start.0, start.1 - 1)).unwrap_or(&'.') {
        '|' => {
            open.push_back((start.0, start.1 - 1, start.0, start.1 - 2, 1));
        }
        'F' => {
            open.push_back((start.0, start.1 - 1, start.0 + 1, start.1 - 1, 1));
        }
        '7' => {
            open.push_back((start.0, start.1 - 1, start.0 - 1, start.1 - 1, 1));
        }
        _ => {}
    }
    match field.get(&(start.0, start.1 + 1)).unwrap_or(&'.') {
        '|' => {
            open.push_back((start.0, start.1 + 1, start.0, start.1 + 2, 1));
        }
        'L' => {
            open.push_back((start.0, start.1 + 1, start.0 + 1, start.1 + 1, 1));
        }
        'J' => {
            open.push_back((start.0, start.1 + 1, start.0 - 1, start.1 + 1, 1));
        }
        _ => {}
    }
    match field.get(&(start.0 - 1, start.1)).unwrap_or(&'.') {
        '-' => {
            open.push_back((start.0 - 1, start.1, start.0 - 2, start.1, 1));
        }
        'L' => {
            open.push_back((start.0 - 1, start.1, start.0 - 1, start.1 - 1, 1));
        }
        'F' => {
            open.push_back((start.0 - 1, start.1, start.0 - 1, start.1 + 1, 1));
        }
        _ => {}
    }
    match field.get(&(start.0 + 1, start.1)).unwrap_or(&'.') {
        '-' => {
            open.push_back((start.0 + 1, start.1, start.0 + 2, start.1, 1));
        }
        '7' => {
            open.push_back((start.0 + 1, start.1, start.0 + 1, start.1 + 1, 1));
        }
        'J' => {
            open.push_back((start.0 + 1, start.1, start.0 + 1, start.1 - 1, 1));
        }
        _ => {}
    }
    // println!("{:?}", field);
    // println!("{:?}", start);
    // println!("{:?}", open);
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut distance = 0;
    while open.len() > 0 {
        let (x0, y0, x1, y1, steps) = open.pop_front().unwrap();
        if visited.contains(&(x0, y0)) {
            // Stop if we've already visited that node
            distance = steps;
            break;
        } else {
            visited.insert((x0, y0));
        }
        // println!("{:?}", (x0, y0, x1, y1, steps));
        match (x1 - x0, y1 - y0) {
            (1, 0) => {
                match field[&(x1, y1)] {
                    '-' => open.push_back((x1, y1, x1 + 1, y1, steps + 1)),
                    '7' => open.push_back((x1, y1, x1, y1 + 1, steps + 1)),
                    _ => open.push_back((x1, y1, x1, y1 - 1, steps + 1)), // J
                }
            }
            (-1, 0) => {
                match field[&(x1, y1)] {
                    'L' => open.push_back((x1, y1, x1, y1 - 1, steps + 1)),
                    '-' => open.push_back((x1, y1, x1 - 1, y1, steps + 1)),
                    _ => open.push_back((x1, y1, x1, y1 + 1, steps + 1)), // F
                }
            }
            (0, 1) => {
                match field[&(x1, y1)] {
                    'L' => open.push_back((x1, y1, x1 + 1, y1, steps + 1)),
                    '|' => open.push_back((x1, y1, x1, y1 + 1, steps + 1)),
                    _ => open.push_back((x1, y1, x1 - 1, y1, steps + 1)), // J
                }
            }
            _ => {
                match field[&(x1, y1)] {
                    '7' => open.push_back((x1, y1, x1 - 1, y1, steps + 1)),
                    '|' => open.push_back((x1, y1, x1, y1 - 1, steps + 1)),
                    _ => open.push_back((x1, y1, x1 + 1, y1, steps + 1)), // F
                }
            }
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
                    println!("{:?}, {:?}", field.get(&(x, y)).unwrap_or(&'.'), &(x, y));
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
