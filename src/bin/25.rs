advent_of_code::solution!(25);

type Graph = std::collections::HashMap<String, Vec<String>>;

// Undirected graph, Edge(a, b) == Edge(b, a)
#[derive(Debug, Eq, Copy, Clone, PartialOrd)]
struct Edge<'a>(&'a String, &'a String);

impl PartialEq for Edge<'_> {
    fn eq(&self, other: &Self) -> bool {
        let Edge(a, b) = self;
        let Edge(c, d) = other;
        (a == c && b == d) || (a == d && b == c)
    }
}

impl Ord for Edge<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Edge(a, b) = self;
        let Edge(c, d) = other;
        let min_self = std::cmp::min(a, b);
        let max_self = std::cmp::max(a, b);
        let min_other = std::cmp::min(c, d);
        let max_other = std::cmp::max(c, d);
        min_self.cmp(min_other).then(max_self.cmp(max_other))
    }
}

impl std::hash::Hash for Edge<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Edge(a, b) = self;
        let min_self = std::cmp::min(a, b);
        let max_self = std::cmp::max(a, b);
        min_self.hash(state);
        max_self.hash(state);
    }
}

fn add_edge(graph: &mut Graph, a: String, b: String) {
    graph.entry(a.clone()).or_default().push(b.clone());
    graph.entry(b).or_default().push(a);
}

type Edges<'a> = Vec<Edge<'a>>;

fn edges(graph: &Graph) -> Edges {
    let mut v = graph
        .iter()
        .map(|(v, es)| es.iter().map(move |o| Edge(v, o)))
        .flatten()
        .collect::<Vec<_>>();
    v.sort();
    v.dedup();
    v
}

fn parse(input: &str) -> Graph {
    input
        .lines()
        .map(|l| {
            let (v, ovs) = l.trim().split_once(": ").unwrap();
            ovs.trim()
                .split(" ")
                .map(|ov| (v.to_string(), ov.to_string()))
        })
        .flatten()
        .fold(Graph::new(), |mut g, (a, b)| {
            // Text said elements will only appear once
            add_edge(&mut g, a, b);
            g
        })
}

fn connectivity(graph: &Graph, ignore_edges: &[Edge]) -> usize {
    let mut queue = vec![graph.keys().next().unwrap()];
    let mut visited = std::collections::HashSet::from([queue[0]]);
    while let Some(node) = queue.pop() {
        for neighbor in &graph[node] {
            if ignore_edges.contains(&Edge(node, neighbor)) {
                continue;
            }
            if visited.contains(&neighbor) {
                continue;
            }
            visited.insert(neighbor);
            queue.push(&neighbor);
        }
    }
    visited.len()
}

#[allow(dead_code)]
fn disconnect_random(graph: &Graph) -> usize {
    let mut n = 0;
    let edges = edges(graph);
    for combi in gen_combinations::CombinationIterator::new(&edges, 3) {
        let ignore = combi.iter().map(|i| **i).collect::<Vec<_>>();
        let connectivity = connectivity(graph, ignore.as_slice());
        if connectivity != graph.len() {
            println!("Disconnected by: {:?}", combi);
            n = connectivity;
            break;
        }
    }
    n
}

fn edge_distinct_paths<'a>(graph: &'a Graph, s: &'a String, t: &'a String) -> Vec<Vec<&'a String>> {
    // BFS to find all edge-disjoint s-t paths, like Edmunds-Karp algorithm;
    // which is guaranteed to be the collection of shortest augmenting paths,
    // and the maximum numbers of s-t paths.
    let mut paths = Vec::new();
    let mut ignore = std::collections::HashSet::new();
    loop {
        let mut open = std::collections::VecDeque::from([s]);
        let mut pred = std::collections::HashMap::new();
        let mut taken = ignore.clone();
        while !open.is_empty() && !pred.contains_key(t) {
            let node = open.pop_front().unwrap();
            for n in &graph[node] {
                if taken.contains(&Edge(node, n)) {
                    continue;
                }
                taken.insert(Edge(node, n));
                pred.insert(n, node);
                open.push_back(n);
            }
        }
        if !pred.contains_key(t) {
            break;
        }
        // Recover path from pred, update ignore and restart
        let mut path = Vec::new();
        let mut node = t;
        while node != s {
            ignore.insert(Edge(node, pred[node]));
            path.push(node);
            node = pred[node];
        }
        path.push(s);
        paths.push(path);
    }
    paths
}

fn find_disconnect_by_st_cut(graph: &Graph) -> usize {
    // Since we know the global minimum cut is 3, pick any 2 nodes and compute
    // the maximum number of edge-disjoints S-T paths. According to Menger's
    // theorem, we will have the S-T cut.
    //
    // Once we have the S-T cut, we know that the edges to disconect are in one
    // of the distinct paths; test all combinations of 3 edges to find which
    // disconnects the graph.
    let vertices = graph.keys().collect::<Vec<_>>();
    let s = vertices[0];
    let mut paths = Vec::new();
    for i in 1..vertices.len() {
        let t = vertices[i];
        paths = edge_distinct_paths(&graph, s, t);
        if paths.len() == 3 {
            break;
        }
    }
    assert!(paths.len() == 3);
    for edge1 in paths[0].windows(2).map(|s| Edge(s[0], s[1])) {
        for edge2 in paths[1].windows(2).map(|s| Edge(s[0], s[1])) {
            for edge3 in paths[2].windows(2).map(|s| Edge(s[0], s[1])) {
                let connectivity = connectivity(graph, &[edge1, edge2, edge3]);
                if graph.len() != connectivity {
                    return connectivity;
                }
            }
        }
    }
    graph.len() // Should never actually reach here
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse(input);
    //let random = disconnect_random(&graph); // For initial tests
    let disconnected = find_disconnect_by_st_cut(&graph);
    Some(disconnected * (graph.len() - disconnected))
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
