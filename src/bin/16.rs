advent_of_code::solution!(16);

use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

type Field = HashMap<(isize, isize), char>;
type Beam = ((isize, isize), (isize, isize));

fn parse(input: &str) -> (Field, usize, usize) {
    (
        input
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .map(|(y, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as isize, y as isize), c))
            })
            .flatten()
            .collect::<Field>(),
        input.lines().count(),
        input.lines().next().unwrap().len(),
    )
}

const EAST: (isize, isize) = (1, 0);
const WEST: (isize, isize) = (-1, 0);
const NORTH: (isize, isize) = (0, -1);
const SOUTH: (isize, isize) = (0, 1);

fn dt(((x, y), (dx, dy)): Beam) -> Beam {
    ((x + dx, y + dy), (dx, dy))
}

fn propagate(beam: &Beam, field: &Field, explore: &mut VecDeque<Beam>) {
    match field.get(&beam.0) {
        Some('/') => match beam.1 {
            NORTH => explore.push_back(dt((beam.0, EAST))),
            EAST => explore.push_back(dt((beam.0, NORTH))),
            SOUTH => explore.push_back(dt((beam.0, WEST))),
            _ => explore.push_back(dt((beam.0, SOUTH))),
        },
        Some('\\') => match beam.1 {
            NORTH => explore.push_back(dt((beam.0, WEST))),
            EAST => explore.push_back(dt((beam.0, SOUTH))),
            SOUTH => explore.push_back(dt((beam.0, EAST))),
            _ => explore.push_back(dt((beam.0, NORTH))),
        },
        Some('|') => match beam.1 {
            EAST | WEST => {
                explore.push_back(dt((beam.0, NORTH)));
                explore.push_back(dt((beam.0, SOUTH)));
            }
            _ => {
                explore.push_back(dt(*beam));
            }
        },
        Some('-') => match beam.1 {
            NORTH | SOUTH => {
                explore.push_back(dt((beam.0, EAST)));
                explore.push_back(dt((beam.0, WEST)));
            }
            _ => {
                explore.push_back(dt(*beam));
            }
        },
        Some('.') => explore.push_back(dt(*beam)),
        _ => {}
    }
}

fn energized(field: &Field, start: Beam) -> HashSet<(isize, isize)> {
    let mut beams = HashSet::new();
    let mut explore = VecDeque::from([start]);
    while !explore.is_empty() {
        let beam = explore.pop_front().unwrap();
        if beams.contains(&beam) || !field.contains_key(&beam.0) {
            continue;
        }
        beams.insert(beam);
        propagate(&beam, field, &mut explore);
    }
    beams.iter().map(|(p, _)| p.clone()).collect::<HashSet<_>>()
}

#[allow(dead_code)]
fn print_energized(tiles: &HashSet<(isize, isize)>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                tiles
                    .contains(&(x as isize, y as isize))
                    .then(|| '#')
                    .unwrap_or('.')
            );
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (field, _, _) = parse(input);
    let tiles = energized(&field, ((0, 0), EAST));
    //print_energized(&tiles, width, height);
    Some(tiles.len())
}

fn max_energized(field: &Field, width: usize, height: usize) -> usize {
    let mut m = 0;
    let width = width as isize;
    let height = height as isize;
    for y in 0..height {
        m = max(m, energized(&field, ((0, y), EAST)).len());
        m = max(m, energized(&field, ((width - 1, y), WEST)).len());
    }
    for x in 0..width {
        m = max(m, energized(&field, ((x, 0), SOUTH)).len());
        m = max(m, energized(&field, ((x, height - 1), NORTH)).len());
    }
    m
}

pub fn part_two(input: &str) -> Option<usize> {
    let (field, width, height) = parse(input);
    max_energized(&field, width, height).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
