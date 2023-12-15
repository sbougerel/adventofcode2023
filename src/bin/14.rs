advent_of_code::solution!(14);

use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone)]
enum Rock {
    Round,
    Square,
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Field = Vec<Vec<Option<Rock>>>;

fn parse(input: &str) -> Field {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'O' => Some(Rock::Round),
                    '#' => Some(Rock::Square),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn step_direction(field: &mut Field, direction: Direction) -> bool {
    let mut moved = false;
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            if let Some(rock) = field[y][x] {
                if rock == Rock::Square {
                    continue;
                }
                if direction == Direction::North && y == 0
                    || direction == Direction::West && x == 0
                    || direction == Direction::South && y == field.len() - 1
                    || direction == Direction::East && x == field[y].len() - 1
                {
                    continue;
                }
                let (i, j) = match direction {
                    Direction::North => (x + 0, y - 1),
                    Direction::East => (x + 1, y + 0),
                    Direction::South => (x + 0, y + 1),
                    Direction::West => (x - 1, y + 0),
                };
                if field[j][i].is_some() {
                    continue;
                }
                // Move rock to the top
                field[j][i] = Some(rock);
                field[y][x] = None;
                moved = true;
            }
        }
    }
    moved
}

fn tilt(field: &mut Field, direction: Direction) {
    while step_direction(field, direction) {}
}

fn cycle(field: &mut Field) {
    tilt(field, Direction::North);
    tilt(field, Direction::West);
    tilt(field, Direction::South);
    tilt(field, Direction::East);
}

fn weight_north(field: &Field) -> usize {
    field
        .iter()
        .rev() // start from south
        .enumerate()
        .map(|(w, l)| {
            l.iter()
                .filter_map(|r| match r {
                    Some(Rock::Round) => Some(w + 1),
                    _ => None,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[allow(dead_code)]
fn print_field(field: &Field) {
    for l in field.iter() {
        println!(
            "{}",
            l.iter()
                .map(|r| match r {
                    Some(Rock::Round) => 'O',
                    Some(Rock::Square) => '#',
                    _ => '.',
                })
                .collect::<String>()
        );
    }
}

fn field_key(field: &Field) -> Vec<bool> {
    field
        .iter()
        .map(|l| {
            l.iter().map(|r| match r {
                Some(Rock::Round) => true,
                _ => false,
            })
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn cycle_until(field: &mut Field, n: usize) {
    let mut seen = HashMap::new();
    let mut count = 0;
    while count < n {
        let key = field_key(&field);
        if let Some(prev_count) = seen.get(&key) {
            let period = count - prev_count;
            let remaining = (n - count) % period;
            for _ in 0..remaining {
                cycle(field);
            }
            return;
        }
        seen.insert(key, count);
        cycle(field);
        count += 1;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut field = parse(input);
    tilt(&mut field, Direction::North);
    // print_field(&field);
    weight_north(&field).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut field = parse(input);
    // Find the period and extrapolate...
    cycle_until(&mut field, 1000000000);
    // print_field(&field);
    weight_north(&field).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
