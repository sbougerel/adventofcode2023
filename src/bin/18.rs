advent_of_code::solution!(18);

use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

type _2D = (isize, isize);

const NORTH: _2D = (0, -1);
const SOUTH: _2D = (0, 1);
const WEST: _2D = (-1, 0);
const EAST: _2D = (1, 0);

type Instr = (_2D, u64, String);

fn parse_one(input: &str) -> Vec<Instr> {
    let re = Regex::new(r"(?m)^([RDLU])\s*(\d+)\s*\(#([0-9a-f]{6})\)$").unwrap();
    re.captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [d, s, c])| {
            (
                match d {
                    "R" => EAST,
                    "D" => SOUTH,
                    "L" => WEST,
                    _ => NORTH,
                },
                s.parse::<u64>().unwrap(),
                c.to_string(),
            )
        })
        .collect::<Vec<_>>()
}

fn parse_two(input: &str) -> Vec<Instr> {
    let re = Regex::new(r"(?m)^([RDLU])\s*(\d+)\s*\(#([0-9a-f]{6})\)$").unwrap();
    re.captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [_, _, c])| {
            (
                match &c[5..] {
                    "0" => EAST,
                    "1" => SOUTH,
                    "2" => WEST,
                    _ => NORTH,
                },
                u64::from_str_radix(&c[0..5], 16).unwrap(),
                c.to_string(),
            )
        })
        .collect::<Vec<_>>()
}

type Segment = (_2D, _2D);
type Block = Segment; // A block is like ((0, 0), (1, 1))
type Walls = HashSet<Block>;

fn slide(((lx, ly), (hx, hy)): Block, (x, y): _2D, amt: isize) -> Block {
    ((lx + x * amt, ly + y * amt), (hx + x * amt, hy + y * amt))
}

#[allow(dead_code)]
fn print_walls_scaled(instrs: &Vec<Instr>, view: (u64, u64)) {
    let mut head: Block = ((0, 0), (1, 1));
    let mut bounds = ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN));
    for instr in instrs.iter() {
        head = slide(head, instr.0, instr.1 as isize);
        bounds.0 .0 = min(bounds.0 .0, head.0 .0);
        bounds.0 .1 = min(bounds.0 .1, head.0 .1);
        bounds.1 .0 = max(bounds.1 .0, head.1 .0);
        bounds.1 .1 = max(bounds.1 .1, head.1 .1);
    }
    let mut walls = Walls::new();
    let mut new_bounds = ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN));
    for instr in instrs.iter() {
        let end = match instr.0 {
            NORTH | SOUTH => (instr.1 * view.1) as isize / (bounds.1 .1 - bounds.0 .1),
            _ => (instr.1 * view.0) as isize / (bounds.1 .0 - bounds.0 .0),
        };
        for _ in 0..end {
            head = slide(head, instr.0, 1);
            new_bounds.0 .0 = min(new_bounds.0 .0, head.0 .0);
            new_bounds.0 .1 = min(new_bounds.0 .1, head.0 .1);
            new_bounds.1 .0 = max(new_bounds.1 .0, head.1 .0);
            new_bounds.1 .1 = max(new_bounds.1 .1, head.1 .1);
            walls.insert(head);
        }
    }
    for y in new_bounds.0 .1..new_bounds.1 .1 {
        for x in new_bounds.0 .0..new_bounds.1 .0 {
            match walls.contains(&((x, y), (x + 1, y + 1))) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
    println!();
}

fn corners(instrs: &Vec<Instr>) -> Vec<Block> {
    let mut head: Block = ((0, 0), (1, 1));
    let mut corners = Vec::new();
    for instr in instrs {
        head = slide(head, instr.0, instr.1 as isize);
        corners.push(head);
    }
    corners
}

fn perimeter(instrs: &Vec<Instr>, corners: &Vec<Block>) -> Vec<_2D> {
    // Following corners, computes the list of points that form the outer
    // perimeter.  Assumption: corners follow instrs in sequence and loop back.
    // Start from any top-left corner (Z-order).
    let (start, _) = corners
        .iter()
        .enumerate()
        .min_by_key(|(_, x)| x.0 .0 + x.0 .1)
        .unwrap();
    let mut points = Vec::new();
    for i in 0..corners.len() {
        let corner_type = (
            instrs[(start + i) % instrs.len()].0,
            instrs[(start + i + 1) % instrs.len()].0,
        );
        let corner = corners[(start + i) % corners.len()];
        let point = match corner_type {
            // CORNER TYPE 1   | TYPE 2   => (    X    ,    Y    )
            (NORTH, EAST) | (EAST, NORTH) => (corner.0 .0, corner.0 .1),
            (NORTH, WEST) | (WEST, NORTH) => (corner.0 .0, corner.1 .1),
            (SOUTH, WEST) | (WEST, SOUTH) => (corner.1 .0, corner.1 .1),
            (SOUTH, EAST) | (EAST, SOUTH) => (corner.1 .0, corner.0 .1),
            _ => panic!("Assumed instructions always form corners!"),
        };
        points.push(point);
    }
    points
}

fn minmax(a: isize, b: isize) -> (isize, isize) {
    (min(a, b), max(a, b))
}

fn capacity(instrs: &Vec<Instr>) -> u64 {
    let corners = corners(&instrs);
    let perimeter = perimeter(&instrs, &corners);

    // Scan vertically, column-by-column, in a segment-aligned way (so we can
    // ignore vertical segments).  Add the entire block's area to the capacity
    // when inside.
    let mut x_breakpoints = perimeter.iter().map(|p| p.0).collect::<Vec<_>>();
    x_breakpoints.sort_unstable();
    x_breakpoints.dedup();

    let mut horiz_segments = perimeter
        .as_slice()
        .windows(2)
        .filter(|w| w[0].1 == w[1].1)
        .map(|w| {
            let (lx, hx) = minmax(w[0].0, w[1].0);
            ((lx, w[0].1), (hx, w[1].1))
        })
        .collect::<Vec<_>>();
    if perimeter[0].1 == perimeter[perimeter.len() - 1].1 {
        let (lx, hx) = minmax(perimeter[0].0, perimeter[perimeter.len() - 1].0);
        let y = perimeter[0].1;
        horiz_segments.push(((lx, y), (hx, y)));
    }
    horiz_segments.sort_unstable_by_key(|k| k.0 .1);

    let mut cap: u64 = 0;
    for xs in x_breakpoints.as_slice().windows(2) {
        let (lx, hx) = (xs[0], xs[1]);
        let mut last_segment = horiz_segments[0];
        let mut inside = last_segment.1 .0 > lx && hx > last_segment.0 .0;
        // println!(
        //     "window: ({}, {}), first segment: {:?}, inside? {}",
        //     lx, hx, last_segment, inside
        // );
        for segment in horiz_segments[1..].iter() {
            if inside {
                cap += (segment.0 .1 - last_segment.0 .1) as u64 * (hx - lx) as u64;
            }
            // intersect
            if lx < segment.1 .0 && segment.0 .0 < hx {
                inside = !inside;
            }
            last_segment = *segment;
            // println!(
            //     "window: ({}, {}), segment: {:?}, inside? {}",
            //     lx, hx, segment, inside
            // );
        }
    }

    cap
}

pub fn part_one(input: &str) -> Option<u64> {
    let instrs = parse_one(&input);
    //print_walls_scaled(&instrs, (100, 100));
    Some(capacity(&instrs))
}

pub fn part_two(input: &str) -> Option<u64> {
    let instrs = parse_two(&input);
    //print_walls_scaled(&instrs, (50, 50));
    Some(capacity(&instrs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
