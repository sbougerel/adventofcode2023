advent_of_code::solution!(12);

use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

type Cache = HashMap<(String, Vec<usize>), usize>;

fn permutation_count(s: &str, grps: &[usize], cache: &mut Cache) -> usize {
    let s = s.trim_start_matches('.'); // remove sourrounding '.'
    if grps.len() == 0 {
        return s.chars().any(|c| c == '#').then(|| 0).unwrap_or(1);
    }
    // Check len requirements
    let minlen = grps.iter().sum::<usize>() + grps.len() - 1;
    if s.len() < minlen {
        return 0;
    }
    // Then slide grps[0]; and split after it
    let nextsh = s.find('#').unwrap_or(s.len());
    let slide = min(s.len() - minlen, nextsh);
    let mut count = 0;
    for i in 0..=slide {
        if s[i..i + grps[0]].chars().any(|c| c == '.') || s[i + grps[0]..].starts_with('#') {
            continue;
        }
        if i + grps[0] == s.len() {
            count += (grps.len() == 1).then(|| 1).unwrap_or(0);
            break;
        }
        count += memoized_permutation_count(&s[i + grps[0] + 1..], &grps[1..], cache);
    }
    count
}

fn memoized_permutation_count(s: &str, grps: &[usize], cache: &mut Cache) -> usize {
    return match cache.get(&(s.to_string(), grps.to_vec())) {
        Some(val) => *val,
        None => {
            let val = permutation_count(s, grps, cache);
            cache.insert((s.to_string(), grps.to_vec()), val);
            val
        }
    };
}

fn parse(input: &str) -> Vec<(String, Vec<usize>)> {
    let re = Regex::new(r"(?m)([?.#]+) ([0-9,]+)$").unwrap();
    re.captures_iter(input)
        .map(|c| {
            (
                c[1].to_string(),
                c[2].split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let data = parse(input);
    data.iter()
        .map(|(s, g)| {
            let mut c = Cache::new();
            permutation_count(s, g, &mut c)
        })
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let data = parse(input);
    data.iter()
        .map(|(s, g)| {
            let mut c = Cache::new();
            let sn = format!("{}?{}?{}?{}?{}", s, s, s, s, s);
            let mut gn = g.clone();
            gn.extend(g);
            gn.extend(g);
            gn.extend(g);
            gn.extend(g);
            permutation_count(&sn, &gn, &mut c)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
