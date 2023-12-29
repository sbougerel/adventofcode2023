advent_of_code::solution!(23);

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, Default)]
struct _2D {
    x: isize,
    y: isize,
}

impl std::ops::Add for _2D {
    type Output = _2D;

    fn add(self, other: _2D) -> _2D {
        _2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const NORTH: _2D = _2D { x: 0, y: -1 };
const SOUTH: _2D = _2D { x: 0, y: 1 };
const WEST: _2D = _2D { x: -1, y: 0 };
const EAST: _2D = _2D { x: 1, y: 0 };

type Terrain = std::collections::HashMap<_2D, _2D>;

fn parse(input: &str) -> Terrain {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '.' || *c == '>' || *c == '<' || *c == '^' || *c == 'v')
                .map(move |(x, c)| {
                    (
                        _2D {
                            x: x as isize,
                            y: y as isize,
                        },
                        match c {
                            '.' => _2D::default(), // Any direction possible
                            '>' => EAST,
                            'v' => SOUTH,
                            '^' => NORTH,
                            _ => WEST,
                        },
                    )
                })
        })
        .flatten()
        .collect::<Terrain>()
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    weight: usize,
    to: _2D,
}

type Edges = Vec<Edge>;

#[derive(Debug)]
struct Vertex {
    weight: usize,
    out: Edges,
}

type Graph = std::collections::HashMap<_2D, Vertex>;

fn simplify(mut graph: Graph) -> Graph {
    // Remove nodes to reduce computation. A path like:
    //
    //    .......>..
    //
    // Will reduce to 4 nodes with edge weight (x):
    //
    //    .(5).(0)>(1).
    //
    // Edit post solution: I was afraid to remove too many nodes before knowing
    // what part 2 was about. When I got part 2, I just ran it in release and it
    // spit out the solution in 7 seconds. I count it as a bonus for using Rust.
    // A strictly typed language cost extra time in writing the code, which you
    // can make back in execution. Thus I moved on.
    //
    // There is more simplification that can be made on the graph to further
    // speed up the computation:
    //
    //  - Remove dead-ends until none are left (except start and goal)
    //  - Merge slope vertex with a neighbor; only nodes with 3 or more
    //    neighbors should remain
    //  - Remove redundant edges, keep only the longest edge
    //
    // There are probably more.
    let mut candidates = Vec::new();
    for (tile, vertex) in graph.iter() {
        if vertex.out.len() == 2
            && graph[&vertex.out[0].to].out.iter().any(|e| e.to == *tile)
            && graph[&vertex.out[1].to].out.iter().any(|e| e.to == *tile)
        {
            candidates.push(*tile);
        }
    }
    for tile in &candidates {
        let weight = graph[tile].weight;
        let [first, second] = graph[tile].out[0..2] else {
            panic!()
        };
        for edge in graph.get_mut(&first.to).unwrap().out.iter_mut() {
            if edge.to == *tile {
                edge.to = second.to;
                edge.weight += weight + second.weight;
                break;
            }
        }
        for edge in graph.get_mut(&second.to).unwrap().out.iter_mut() {
            if edge.to == *tile {
                edge.to = first.to;
                edge.weight += weight + first.weight;
                break;
            }
        }
        graph.remove(tile);
    }
    graph
}

fn graph_with_slopes(terrain: &Terrain) -> Graph {
    // First pass: build a graph where every tile is a node
    let mut graph = Graph::new();
    for (tile, slope) in terrain.iter() {
        graph.insert(
            *tile,
            Vertex {
                weight: 1,
                out: Vec::new(),
            },
        );
        if *slope == _2D::default() {
            // Tiles wihout slope connect outward to all their neighbor
            for dir in [NORTH, SOUTH, WEST, EAST] {
                if !terrain.contains_key(&(*tile + dir)) {
                    continue;
                }
                graph.get_mut(&tile).unwrap().out.push(Edge {
                    weight: 0,
                    to: *tile + dir,
                });
            }
        } else {
            // Tiles with slope connect only in the direction of the slope
            assert!(terrain.contains_key(&(*tile + *slope)));
            graph.get_mut(&tile).unwrap().out.push(Edge {
                weight: 0,
                to: *tile + *slope,
            })
        }
    }
    simplify(graph)
}

fn graph_without_slopes(terrain: &Terrain) -> Graph {
    // First pass: build a graph where every tile is a node
    let mut graph = Graph::new();
    for (tile, _) in terrain.iter() {
        graph.insert(
            *tile,
            Vertex {
                weight: 1,
                out: Vec::new(),
            },
        );
        // All tiles are without slopes
        for dir in [NORTH, SOUTH, WEST, EAST] {
            if !terrain.contains_key(&(*tile + dir)) {
                continue;
            }
            graph.get_mut(&tile).unwrap().out.push(Edge {
                weight: 0,
                to: *tile + dir,
            });
        }
    }
    simplify(graph)
}

#[derive(Debug, Eq, Copy, Clone)]
struct State(usize, _2D, Option<usize>);

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

type Histories = Vec<(_2D, Option<usize>)>;

fn find_history(histories: &Histories, mut head: usize, tile: _2D) -> bool {
    // histories are a DAG, we explore ancesters from head to find tile
    while histories[head].1 != None {
        if histories[head].0 == tile {
            return true;
        }
        head = histories[head].1.unwrap();
    }
    histories[head].0 == tile
}

fn longest_route(graph: &Graph, start: _2D, end: _2D) -> Option<usize> {
    // Dikjstra but reverse heuristic, with individual path storage
    let mut open = std::collections::BinaryHeap::from([State(0, start, None)]);
    let mut histories = Histories::new();
    let mut best: Option<State> = None;
    while let Some(State(dist, tile, parent)) = open.pop() {
        if tile == end && (best.is_none() || best.unwrap().0 < dist) {
            best = Some(State(dist, tile, parent));
        }
        if parent.is_some() && find_history(&histories, parent.unwrap(), tile) {
            continue;
        }
        histories.push((tile, parent));
        let parent = Some(histories.len() - 1);
        let dist = dist + graph[&tile].weight;
        for edge in &graph[&tile].out {
            open.push(State(dist + edge.weight, edge.to, parent));
        }
    }
    best.and_then(|s| Some(s.0))
}

fn start(graph: &Graph) -> _2D {
    let start = _2D { x: 1, y: 0 };
    assert!(graph.contains_key(&start));
    start
}

fn goal(graph: &Graph) -> _2D {
    let mut goal = _2D { x: -1, y: -1 };
    for vertex in graph.keys() {
        if vertex.y > goal.y {
            goal = *vertex;
        }
    }
    goal
}

pub fn part_one(input: &str) -> Option<usize> {
    let terrain = parse(input);
    //println!("{:?}", terrain);
    let graph = graph_with_slopes(&terrain);
    //println!("{:?}", graph);
    longest_route(&graph, start(&graph), goal(&graph))
}

pub fn part_two(input: &str) -> Option<usize> {
    let terrain = parse(input);
    //println!("{:?}", terrain);
    let graph = graph_without_slopes(&terrain);
    //println!("{:?}", graph);
    longest_route(&graph, start(&graph), goal(&graph))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
