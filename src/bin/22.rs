use std::collections::VecDeque;

advent_of_code::solution!(22);

#[derive(Debug, PartialEq, Copy, Clone)]
struct _3D {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Brick {
    beg: _3D,
    end: _3D,
}
type Bricks = Vec<Brick>;

fn parse(input: &str) -> Bricks {
    let re = regex::Regex::new(r"(?m)^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let mut brick = Brick {
                beg: _3D {
                    x: cap[1].parse().unwrap(),
                    y: cap[2].parse().unwrap(),
                    z: cap[3].parse().unwrap(),
                },
                end: _3D {
                    x: cap[4].parse().unwrap(),
                    y: cap[5].parse().unwrap(),
                    z: cap[6].parse().unwrap(),
                },
            };
            // Verify my understanding of the inputs
            assert!(brick.beg.z > 0);
            assert!(brick.end.z > 0);
            assert!(
                1 >= (brick.beg.x == brick.end.x).then(|| 0).unwrap_or(1)
                    + (brick.beg.y == brick.end.y).then(|| 0).unwrap_or(1)
                    + (brick.beg.z == brick.end.z).then(|| 0).unwrap_or(1)
            );
            // Ensure that beg/end is oriented min/max
            if brick.beg.x > brick.end.x || brick.beg.y > brick.end.y || brick.beg.z > brick.end.z {
                std::mem::swap(&mut brick.beg, &mut brick.end);
            }
            // Can't test whether the brick overlap here, but I will *trust* it's not the case
            brick
        })
        .collect()
}

fn collide(a: &Brick, b: &Brick) -> bool {
    (a.beg.x <= b.end.x && a.end.x >= b.beg.x) && (a.beg.y <= b.end.y && a.end.y >= b.beg.y)
}

fn snap(a: &Brick, z: isize) -> Brick {
    Brick {
        beg: _3D {
            x: a.beg.x,
            y: a.beg.y,
            z,
        },
        end: _3D {
            x: a.end.x,
            y: a.end.y,
            z: a.end.z - a.beg.z + z,
        },
    }
}

fn at_rest(bricks: &Bricks) -> Bricks {
    let mut falling = bricks.clone();
    falling.sort_by_key(|b| b.beg.z);
    // Snap all bricks to their lowest z-order, scan in front.
    for i in 0..falling.len() {
        let brick = &falling[i];
        let mut z = 1;
        for j in 0..i {
            let test = falling[i - j - 1];
            if test.end.z >= brick.beg.z {
                continue; // test already at same or higher level
            }
            if collide(&brick, &test) {
                z = std::cmp::max(z, test.end.z + 1);
            }
        }
        falling[i] = snap(&brick, z);
    }
    falling
}

#[derive(Debug, Default)]
struct Edges {
    supporting: Vec<usize>,
    supported_by: Vec<usize>,
}
type Graph = std::collections::HashMap<usize, Edges>;

fn supporting(ordered_bricks: &Bricks) -> Graph {
    let mut graph = Graph::new();
    for i in 0..ordered_bricks.len() {
        let brick = &ordered_bricks[i];
        graph.entry(i).or_default();
        for j in i + 1..ordered_bricks.len() {
            let test = &ordered_bricks[j];
            if test.beg.z != brick.end.z + 1 {
                continue; // not interested
            }
            if test.beg.z > brick.end.z + 1 {
                break; // no more collision possible
            }
            if collide(&brick, &test) {
                graph.entry(i).and_modify(|v| v.supporting.push(j));
                graph.entry(j).or_default().supported_by.push(i);
            }
        }
    }
    assert_eq!(ordered_bricks.len(), graph.len());
    graph
}

fn can_desintegrate(support_graph: &Graph) -> usize {
    support_graph
        .iter()
        .filter(|(_, edges)| {
            edges.supporting.len() == 0
            ||
            // Can be desintegrated if the supported bricks have all more than 1 support
            edges.supporting.iter().all(|brick| support_graph[&brick].supported_by.len() > 1)
        })
        .count()
}

fn chain_desintegration(support_graph: &Graph) -> usize {
    support_graph
        .iter()
        .map(|(this_brick, its_edges)| {
            // Breadth-first search of bricks to fall
            let mut open = its_edges
                .supporting
                .iter()
                .copied()
                .collect::<VecDeque<_>>();
            let mut falling = std::collections::HashSet::from([*this_brick]);
            while let Some(brick) = open.pop_front() {
                if falling.contains(&brick) {
                    continue;
                }
                let edges = &support_graph[&brick];
                if edges.supported_by.iter().any(|b| !falling.contains(b)) {
                    continue;
                }
                open.extend(&edges.supporting);
                falling.insert(brick);
            }
            //println!("Brick {} causes {} to fall", this_brick, falling.len());
            falling.len() - 1 // We count the *other* bricks only
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let bricks = parse(&input);
    let ordered_bricks = at_rest(&bricks);
    //println!("{:?}", ordered_bricks);
    let support_graph = supporting(&ordered_bricks);
    //println!("{:?}", support_graph);
    Some(can_desintegrate(&support_graph))
}

pub fn part_two(input: &str) -> Option<usize> {
    let bricks = parse(&input);
    let ordered_bricks = at_rest(&bricks);
    //println!("{:?}", ordered_bricks);
    let support_graph = supporting(&ordered_bricks);
    //println!("{:?}", support_graph);
    Some(chain_desintegration(&support_graph))
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
        assert_eq!(result, Some(8));
    }
}
