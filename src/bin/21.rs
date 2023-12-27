advent_of_code::solution!(21);

use std::collections::{HashMap, HashSet};

type _2D = (isize, isize);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Garden,
    Start,
    Rock,
}

type Tiles = HashMap<_2D, Tile>;

struct Field {
    tiles: Tiles,
    start: _2D,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> Field {
    let tiles = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                (
                    (x as isize, y as isize),
                    match c {
                        'S' => Tile::Start,
                        '.' => Tile::Garden,
                        _ => Tile::Rock,
                    },
                )
            })
        })
        .flatten()
        .collect::<Tiles>();
    let start = tiles
        .iter()
        .find(|(_, v)| *v == &Tile::Start)
        .unwrap()
        .0
        .clone();
    Field {
        tiles,
        start,
        width: input.lines().next().unwrap().len(),
        height: input.lines().count(),
    }
}

const NORTH: _2D = (0, -1);
const SOUTH: _2D = (0, 1);
const WEST: _2D = (-1, 0);
const EAST: _2D = (1, 0);

fn add((ax, ay): _2D, (bx, by): _2D) -> _2D {
    (ax + bx, ay + by)
}

#[allow(dead_code)]
fn print_reached(field: &Field, reached: &HashSet<_2D>, repeat: usize) {
    for y in (0 - (repeat * field.height) as isize)..(field.height * (repeat + 1)) as isize {
        for x in (0 - (repeat * field.width) as isize)..(field.width * (repeat + 1)) as isize {
            match reached.get(&(x, y)) {
                Some(_) => print!("O"),
                None => match field
                    .tiles
                    .get(&(
                        x.rem_euclid(field.width as isize),
                        y.rem_euclid(field.height as isize),
                    ))
                    .unwrap()
                {
                    Tile::Start => print!("S"),
                    Tile::Garden => print!("."),
                    Tile::Rock => print!("#"),
                },
            }
        }
        println!();
    }
}

fn generalized_reach(field: &Field, start: _2D, step_count: usize, clip: bool) -> HashSet<_2D> {
    // This solution is generalized to any position, albeit slow.
    assert!(step_count > 0);
    let mut even_reached = HashSet::new();
    let mut odd_reached = HashSet::new();
    let (width, height) = (field.width as isize, field.height as isize);
    let mut envelope = HashSet::from([start]);
    for run in 0..step_count {
        let mut next_envelope = HashSet::new();
        for pos in envelope {
            if run % 2 == 0 {
                if even_reached.contains(&pos) {
                    continue;
                }
                even_reached.insert(pos);
            } else {
                if odd_reached.contains(&pos) {
                    continue;
                }
                odd_reached.insert(pos);
            }
            for dir in [NORTH, SOUTH, EAST, WEST] {
                let next_pos = add(pos, dir);
                if run % 2 == 0 {
                    if odd_reached.contains(&next_pos) {
                        continue;
                    }
                } else {
                    if even_reached.contains(&next_pos) {
                        continue;
                    }
                }
                let mod_pos = if clip {
                    next_pos
                } else {
                    (next_pos.0.rem_euclid(width), next_pos.1.rem_euclid(height))
                };
                if clip && !field.tiles.contains_key(&mod_pos) {
                    continue;
                }
                if field.tiles[&mod_pos] == Tile::Rock {
                    continue;
                }
                next_envelope.insert(next_pos); // avoid duplicates
            }
        }
        envelope = next_envelope;
    }
    if step_count % 2 == 0 {
        even_reached.extend(envelope.iter());
        even_reached
    } else {
        odd_reached.extend(envelope.iter());
        odd_reached
    }
}

fn clipping_reach(field: &Field, start: _2D, step_count: usize) -> usize {
    generalized_reach(field, start, step_count, true).len()
}

fn special_reach(field: &Field, step_count: usize) -> usize {
    // This solution is special to the problem, it has 3 assumptions:
    //
    //  - fields are square (blocks), with odd length (2k + 1)
    //  - start is fully centered (at k), due to odd length sides
    //  - the layout of rocks lets you use manahattan distance between fields.
    //  - the number of steps from the start modulo size of the field is
    //    exactly 2k, i.e. the number of steps ends at the edge of the field.
    //
    // The example given does not fit the above, and actually makes things
    // harder. With the assumptions above, there are only a finite set of blocks
    // that are laid out in this fashion around the fully filled center blocks
    // (S, .), as shown for radius = 3:
    //
    //       aAb
    //      aX.Zb
    //     aX...Zb
    //     C..S..D
    //     dW...Yc
    //      dW.Yc
    //       dBc
    //
    //  A, B, C, D:    a, b, c, d:    W, X, Y, Z:
    //
    //   ...O...        .......        .O.O...
    //   ..O.O..        .......        O.O.O..
    //   .O.O.O.        .......        .O.O.O.
    //   O.O.O.O        .......        O.O.O.O
    //   .O.O.O.        O......        .O.O.O.
    //   O.O.O.O        .O.....        O.O.O.O
    //   .O.O.O.        O.O....        .O.O.O.
    //
    // We start by verifying some of the assumptions:
    assert_eq!(field.width, field.height);
    let side = field.width as isize;
    let steps = step_count as isize;
    assert_eq!(field.start.0 * 2 + 1, side);
    assert_eq!(field.start.1 * 2 + 1, side);
    assert!((field.start.0 - steps).rem_euclid(side) == 0);
    assert!((field.start.1 - steps).rem_euclid(side) == 0);
    let radius = {
        let (q, r) = num::integer::div_rem(steps + field.start.0, side);
        assert!(r == side - 1);
        q
    };
    assert!(radius > 1);

    // Compute center blocks, which are alternating series of even/odd at radius
    // - 1 from the S block (in manhattan distance):
    //
    // first serie: 8 + 16 + .. + (2k * 4)
    // other serie: 4 + 12 + .. + ((2k + 1) * 4)
    let even_reach = clipping_reach(field, field.start, field.width * 2);
    let odd_reach = clipping_reach(field, field.start, field.width * 2 + 1);
    let (serie_1_reach, serie_2_reach) = if step_count % 2 == 0 {
        (even_reach, odd_reach)
    } else {
        (odd_reach, even_reach)
    };
    let mut reached = 1 * serie_1_reach; // Start at center block
    let mut k: usize = 1; // Skip center
    while k * 2 < radius as usize {
        reached += 2 * k * 4 * serie_1_reach;
        k += 1;
    }
    let mut k: usize = 0;
    while k * 2 + 1 < radius as usize {
        reached += (2 * k + 1) * 4 * serie_2_reach;
        k += 1;
    }

    // Compute a, b, c, d blocks. There are "radius" times each.
    let remaining = (field.start.0 - 1) as usize;
    reached += (clipping_reach(&field, (side - 1, side - 1), remaining) // a
                + clipping_reach(&field, (0, side - 1), remaining)      // b
                + clipping_reach(&field, (0, 0), remaining)             // c
                + clipping_reach(&field, (side - 1, 0), remaining))     // d
        * radius as usize;

    // Compute W, X, Y, Z blocks. There are "radius - 1" times each.
    let remaining = (side + field.start.0 - 1) as usize;
    reached += (clipping_reach(&field, (side - 1, side - 1), remaining) // X
                + clipping_reach(&field, (0, side - 1), remaining)      // Z
                + clipping_reach(&field, (0, 0), remaining)             // Y
                + clipping_reach(&field, (side - 1, 0), remaining))     // W
        * (radius - 1) as usize;

    // Compute A, B, C, D blocks, they are all unique
    let remaining = (side - 1) as usize;
    reached += clipping_reach(&field, (field.start.0, side - 1), remaining) // A
        + clipping_reach(&field, (field.start.0, 0), remaining)             // B
        + clipping_reach(&field, (side - 1, field.start.1), remaining)      // C
        + clipping_reach(&field, (0, field.start.1), remaining); // D

    reached
}

pub fn part_one(input: &str) -> Option<usize> {
    let field = parse(&input);
    let reached = generalized_reach(&field, field.start, 64, true);
    //print_reached(&field, &reached, 0);
    Some(reached.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let field = parse(&input);
    //let steps = 26501365;
    let steps = field.width * 2 + field.width - 1 - field.start.0 as usize;
    let reached = special_reach(&field, steps);
    //let compare = generalized_reach(&field, field.start, steps, false);
    //assert_eq!(reached, compare.len());
    Some(reached)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(588));
    }
}
