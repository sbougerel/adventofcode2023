advent_of_code::solution!(13);

use std::cmp::min;
use std::iter::zip;

type Pattern = Vec<Vec<bool>>;
type Patterns = Vec<Pattern>;

fn parse_patterns(input: &str) -> Patterns {
    let mut patterns = Patterns::new();
    let mut new_pattern = true;
    for l in input.lines() {
        if l.is_empty() {
            new_pattern = true;
        } else {
            if new_pattern {
                new_pattern = false;
                patterns.push(Pattern::new());
            }
            patterns
                .last_mut()
                .unwrap()
                .push(l.chars().map(|c| (c == '#')).collect::<Vec<_>>());
        }
    }
    patterns
}

#[allow(dead_code)]
fn print_pattern(p: &Pattern) {
    for l in p.iter() {
        println!(
            "{}",
            l.iter()
                .map(|b| b.then(|| '#').unwrap_or('.'))
                .collect::<String>()
        );
    }
}

fn find_horiz_reflections(pattern: &Pattern) -> Vec<usize> {
    let mid_iter = pattern
        .as_slice()
        .windows(2)
        .enumerate()
        .filter_map(|(i, l)| (l[0] == l[1]).then(|| Some(i)));
    let mut results = Vec::new();
    for mid in mid_iter {
        let mid = mid.unwrap() + 1;
        let size = min(mid, pattern.len() - mid);
        // Validate reflection
        if pattern[mid - size..mid]
            .iter()
            .eq(pattern[mid..mid + size].iter().rev())
        {
            results.push(mid);
        }
    }
    results
}

fn rotate(pattern: &Pattern) -> Pattern {
    let mut vertical: Pattern = Pattern::new();
    for row in 0..pattern.len() {
        for col in 0..pattern[row].len() {
            if vertical.len() <= col {
                vertical.push(Vec::new());
            }
            vertical[col].push(pattern[row][col]);
        }
    }
    vertical
}

fn one_reflection(pattern: &Pattern) -> Option<(bool, usize)> {
    let result = find_horiz_reflections(pattern)
        .get(0)
        .and_then(|x| Some((true, *x)));
    if result.is_some() {
        return result;
    }
    // Rotate 90 degrees
    let vertical = rotate(pattern);
    find_horiz_reflections(&vertical)
        .get(0)
        .and_then(|x| Some((false, *x)))
}

fn reflections(patterns: &Patterns) -> Vec<Option<(bool, usize)>> {
    let mut results: Vec<Option<(bool, usize)>> = vec![None; patterns.len()];
    for (pattern, result) in zip(patterns.iter(), results.iter_mut()) {
        //print_pattern(pattern);
        *result = one_reflection(pattern);
    }
    results
}

fn desmudged_reflections(patterns: &Patterns) -> Vec<Option<(bool, usize)>> {
    let mut results: Vec<Option<(bool, usize)>> = vec![None; patterns.len()];
    for (pattern, result) in zip(patterns.iter(), results.iter_mut()) {
        // Compute initial reflection first
        let (horiz, mid) = one_reflection(pattern).unwrap();
        // Rotate 90 degrees
        let mut desmudged = pattern
            .iter()
            .map(|l| l.iter().map(|b| *b).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        //print_pattern(&desmudged);
        let mut vertical = rotate(&desmudged);
        // Brute force? Okay
        for r in 0..desmudged.len() {
            for c in 0..desmudged[r].len() {
                // flip
                desmudged[r][c] = !desmudged[r][c];
                vertical[c][r] = !vertical[c][r];

                let candidates = find_horiz_reflections(&desmudged);
                for candidate in candidates {
                    if !horiz || candidate != mid {
                        // Must differ from initial reflection
                        *result = Some((true, candidate));
                        break;
                    }
                }

                let candidates = find_horiz_reflections(&vertical);
                for candidate in candidates {
                    if horiz || candidate != mid {
                        // Must differ from initial reflection
                        *result = Some((false, candidate));
                        break;
                    }
                }

                // flip back - okay if not executed on break
                desmudged[r][c] = !desmudged[r][c];
                vertical[c][r] = !vertical[c][r];
            }
            if result.is_some() {
                break;
            }
        }
    }
    results
}

pub fn part_one(input: &str) -> Option<usize> {
    // Identify lines repeated line (the middle of the reflection), go towards
    // the edge to check.
    let patterns: Patterns = parse_patterns(input);
    let results = reflections(&patterns);

    // println!("{:?}", results);
    results
        .iter()
        .map(|x| {
            let (h, v) = x.unwrap();
            v * h.then(|| 100).unwrap_or(1)
        })
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let patterns: Patterns = parse_patterns(input);

    // Brute-force? Flip each bit and test in both directions?
    // Optimize with bit-wise operations later if needed
    let results = desmudged_reflections(&patterns);

    // println!("{:?}", results);
    results
        .iter()
        .map(|x| {
            let (h, v) = x.unwrap();
            v * h.then(|| 100).unwrap_or(1)
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
