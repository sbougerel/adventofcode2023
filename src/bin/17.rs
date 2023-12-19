advent_of_code::solution!(17);

use num::abs;
use std::collections::HashMap;

type _2D = (isize, isize);

type HeatMap = HashMap<_2D, u32>;

struct Field {
    heatmap: HeatMap,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> Field {
    Field {
        heatmap: input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as isize, y as isize), c.to_digit(10).unwrap() as u32))
            })
            .flatten()
            .collect::<HeatMap>(),
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

fn dist((ax, ay): _2D, (bx, by): _2D) -> u32 {
    (abs(ax - bx) + abs(ay - by)) as u32
}

fn start_moving(
    field: &Field,
    start: _2D,
    target: _2D,
) -> Vec<(u32, _2D, _2D, u32, u32, Vec<_2D>)> {
    [NORTH, SOUTH, EAST, WEST]
        .iter()
        .filter(|dir| field.heatmap.contains_key(&add(start, **dir)))
        .map(|dir| {
            let next = add(start, *dir);
            (
                dist(target, next),
                next,
                *dir,
                field.heatmap[&next],
                0_u32,
                vec![],
            )
        })
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
fn print_debug(field: &Field, path: &Vec<_2D>) {
    for y in 0..field.height as isize {
        for x in 0..field.width as isize {
            if path.contains(&(x, y)) {
                print!("#");
            } else {
                print!("{}", field.heatmap[&(x, y)]);
            }
        }
        println!();
    }
    println!();
}

fn least_heatloss(field: &Field, start: _2D, target: _2D) -> u32 {
    // Implement A*.  Heurisitic (h) is current heatloss + taxicab distance.
    // The idea is to make the algorithm first explore paths that are not
    // deviating too much from the straight line.  This way we quickly get a
    // better "best" which makes us prune more, etc.  Search stops when runing
    // into more optimal path.
    let mut open = start_moving(&field, start, target);
    let mut best = u32::MAX;
    let mut visited: HashMap<(_2D, _2D), [u32; 3]> = HashMap::new();
    while !open.is_empty() {
        let (_h, head, dir, heat, fwd, _path) = open.pop().unwrap();
        if heat + dist(head, target) >= best {
            continue; // Will never beat best
        }
        if head == target {
            if heat < best {
                best = heat;
                //print_debug(&field, &_path);
            }
            continue;
        }

        if visited.contains_key(&(head, dir)) {
            let val = visited[&(head, dir)];
            // States with different fwd values should coexist. You can compare
            // a low fwd only with a low fwd, but a high fwd should be
            // comparable to any of the lower fwd best value.
            //
            // There's probably more optimisation: it can be observed that 2
            // adjacent dimentions cover all possible outgoing directions. Thus
            // it's probably not necessary to store separate states for each
            // directions. I couldn't really figure out the proper way to
            // express this.
            if val[0..=(fwd as usize)].iter().min().unwrap() <= &heat {
                continue; // Will never beat best
            } else {
                // improve value
                let mut val = val;
                val[fwd as usize] = heat;
                visited.insert((head, dir), val);
            }
        } else {
            let mut val = [u32::MAX; 3];
            val[fwd as usize] = heat;
            visited.insert((head, dir), val);
        }

        for new_dir in [NORTH, SOUTH, EAST, WEST] {
            if new_dir == (-dir.0, -dir.1) {
                continue; // don't go backward
            }
            let mut new_fwd = 0;
            if dir == new_dir {
                if fwd == 2 {
                    continue; // don't go forward
                } else {
                    new_fwd = fwd + 1;
                }
            }
            let new_head = (head.0 + new_dir.0, head.1 + new_dir.1);
            if !field.heatmap.contains_key(&new_head) {
                continue; // don't go out
            }
            let new_heat = heat + field.heatmap[&new_head];
            let new_h = new_heat + dist(new_head, target);
            let pos = open.partition_point(|a| new_h < a.0);
            let mut new_path = _path.clone();
            new_path.push(head);
            open.insert(pos, (new_h, new_head, new_dir, new_heat, new_fwd, new_path));
        }
    }
    best
}

fn least_heatloss_ultra(field: &Field, start: _2D, target: _2D) -> u32 {
    let mut open = start_moving(&field, start, target);
    let mut best = u32::MAX;
    let mut visited: HashMap<(_2D, _2D), [u32; 10]> = HashMap::new();
    while !open.is_empty() {
        let (_h, head, dir, heat, fwd, _path) = open.pop().unwrap();
        if heat + dist(head, target) >= best {
            continue; // Will never beat best
        }
        if head == target {
            if heat < best && fwd >= 3 {
                best = heat;
                //print_debug(&field, &_path);
            }
            continue;
        }

        if visited.contains_key(&(head, dir)) {
            let val = visited[&(head, dir)];
            // An ultra crucible cannot compare values below fwd=3 the same way
            if fwd >= 3 && val[3..=(fwd as usize)].iter().min().unwrap() <= &heat
                || val[fwd as usize] <= heat
            {
                continue; // Will never beat best
            } else {
                // improve value
                let mut val = val;
                val[fwd as usize] = heat;
                visited.insert((head, dir), val);
            }
        } else {
            let mut val = [u32::MAX; 10];
            val[fwd as usize] = heat;
            visited.insert((head, dir), val);
        }

        for new_dir in [NORTH, SOUTH, EAST, WEST] {
            if new_dir == (-dir.0, -dir.1) {
                continue; // don't go backward
            }
            let mut new_fwd = 0;
            if dir == new_dir {
                if fwd == 9 {
                    continue; // don't go forward
                } else {
                    new_fwd = fwd + 1;
                }
            } else {
                if fwd < 3 {
                    continue; // can't turn
                }
            }
            let new_head = (head.0 + new_dir.0, head.1 + new_dir.1);
            if !field.heatmap.contains_key(&new_head) {
                continue; // don't go away
            }
            let new_heat = heat + field.heatmap[&new_head];
            let new_h = new_heat + dist(new_head, target);
            let pos = open.partition_point(|a| new_h < a.0);
            let mut new_path = _path.clone();
            new_path.push(head);
            open.insert(pos, (new_h, new_head, new_dir, new_heat, new_fwd, new_path));
        }
    }
    best
}

pub fn part_one(input: &str) -> Option<u32> {
    let field = parse(&input);
    least_heatloss(
        &field,
        (0, 0),
        (field.width as isize - 1, field.height as isize - 1),
    )
    .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = parse(&input);
    least_heatloss_ultra(
        &field,
        (0, 0),
        (field.width as isize - 1, field.height as isize - 1),
    )
    .into()
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
        assert_eq!(result, Some(94));
    }
}
