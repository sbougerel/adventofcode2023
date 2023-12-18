advent_of_code::solution!(17);

use std::cmp::min;
use std::collections::{HashMap, VecDeque};

type HeatMap = HashMap<(isize, isize), u32>;

fn parse(input: &str) -> (HeatMap, usize, usize) {
    (
        input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as isize, y as isize), c.to_digit(10).unwrap() as u32))
            })
            .flatten()
            .collect::<HeatMap>(),
        input.lines().next().unwrap().len(),
        input.lines().count(),
    )
}

const NORTH: (isize, isize) = (0, -1);
const SOUTH: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (-1, 0);
const EAST: (isize, isize) = (1, 0);

fn print_heatmap(heatmap: &HeatMap, start: &(isize, isize), end: &(isize, isize)) {
    for j in start.1..end.1 {
        for i in start.0..end.0 {
            match heatmap.get(&(i, j)) {
                Some(v) => print!("{:3},", v),
                None => print!("."),
            }
        }
        println!();
    }
    println!();
}

fn least_heatloss(
    heatmap: &HeatMap,
    start: (isize, isize),
    dir: (isize, isize),
    target: (isize, isize),
) -> Option<u32> {
    // When matching in states visited,
    let mut open = VecDeque::from([(start, dir, 0_u32, 0_u32)]);
    let mut visited = HashMap::new();
    while !open.is_empty() {
        let (pos, dir, heat, fwd) = open.pop_front().unwrap();
        if !heatmap.contains_key(&(pos)) {
            continue;
        }
        let heat = heat + heatmap[&pos];
        if visited.contains_key(&pos) {
            let vheat = visited[&pos];
            if vheat < heat {
                continue;
            }
        }
        visited.insert(pos, heat);
        if pos == target {
            continue;
        }
        for new_dir in [NORTH, SOUTH, EAST, WEST] {
            if new_dir == (-dir.0, -dir.1) {
                continue;
            }
            let mut new_fwd = 0;
            if dir == new_dir {
                if fwd == 2 {
                    continue;
                } else {
                    new_fwd = fwd + 1;
                }
            }
            open.push_back((
                (pos.0 + new_dir.0, pos.1 + new_dir.1),
                new_dir,
                heat,
                new_fwd,
            ));
        }
    }
    print_heatmap(&visited, &start, &(target.0 + 1, target.1 + 1));
    // min(
    //     visited.get(&(target, NORTH)).and_then(|(h, _)| Some(h)),
    //     min(
    //         visited.get(&(target, SOUTH)).and_then(|(h, _)| Some(h)),
    //         min(
    //             visited.get(&(target, WEST)).and_then(|(h, _)| Some(h)),
    //             visited.get(&(target, EAST)).and_then(|(h, _)| Some(h)),
    //         ),
    //     ),
    // )
    visited.get(&target).and_then(|h| Some(*h))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (heatmap, width, height) = parse(&input);
    print_heatmap(&heatmap, &(0, 0), &(width as isize, height as isize));
    min(
        least_heatloss(
            &heatmap,
            (0, 0),
            EAST,
            (width as isize - 1, height as isize - 1),
        ),
        least_heatloss(
            &heatmap,
            (0, 0),
            SOUTH,
            (width as isize - 1, height as isize - 1),
        ),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
