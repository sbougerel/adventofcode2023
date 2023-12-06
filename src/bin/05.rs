advent_of_code::solution!(5);

use regex::Regex;

const FORMAT: &str = r"seeds: ([\d ]+)

seed-to-soil map:
([\d\s]+)

soil-to-fertilizer map:
([\d\s]+)

fertilizer-to-water map:
([\d\s]+)

water-to-light map:
([\d\s]+)

light-to-temperature map:
([\d\s]+)

temperature-to-humidity map:
([\d\s]+)

humidity-to-location map:
([\d\s]+)
";

pub fn parse(input: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let re = Regex::new(FORMAT).unwrap();
    let captures = re.captures(input).unwrap();
    let extract_map = |cap: &str| {
        let mut v = cap
            .split('\n')
            .map(|s| {
                s.split(' ')
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        v.sort_by_key(|a| a[1]);
        v
    };

    let seeds = captures[1]
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let maps = (2..9)
        .map(|i| extract_map(&captures[i]))
        .collect::<Vec<_>>();

    (seeds, maps)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse(input);
    seeds
        .iter()
        .map(|s| {
            let mut r = *s;
            for m in &maps {
                let mut i = 0; // assuming there's at least one rule
                while i + 1 < m.len() && m[i + 1][1] <= r {
                    i += 1;
                }
                if r >= m[i][1] && r < m[i][1] + m[i][2] {
                    r = m[i][0] + (r - m[i][1]);
                }
            }
            r
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse(input);
    let mut open_intervals = seeds
        .chunks(2)
        .map(|s| (0_usize, s[0], s[0] + s[1]))
        .collect::<Vec<_>>();
    // Find the minimum start amongs all final intervals
    let mut final_intervals: Vec<(u64, u64)> = Vec::new();
    while open_intervals.len() > 0 {
        // Break up the interval into smaller intervals
        let (map, start, end) = open_intervals.pop().unwrap();
        if map == maps.len() {
            // This interval has been translated by all maps
            final_intervals.push((start, end));
            continue;
        }
        let m = &maps[map];
        let mut i = 0;
        let mut s = start; // break intervals from the start
        while i < m.len() && end > m[i][1] {
            // end is after this mapping's start
            if s >= m[i][1] + m[i][2] {
                // start is after this mapping's end -> next mapping
                i += 1;
            } else {
                // start is before this mapping's end
                if s < m[i][1] {
                    // start is before this mapping's start -> split and stay on mapping
                    open_intervals.push((map + 1, s, m[i][1]));
                    s = m[i][1];
                } else {
                    // start is within this mapping's range
                    if end <= m[i][1] + m[i][2] {
                        // end is also within this mapping's range -> translate and stop
                        open_intervals.push((
                            map + 1,
                            m[i][0] + (s - m[i][1]),
                            m[i][0] + (end - m[i][1]),
                        ));
                        break;
                    } else {
                        // end is after this mapping's range -> translate, split and continue
                        open_intervals.push((map + 1, m[i][0] + (s - m[i][1]), m[i][0] + m[i][2]));
                        s = m[i][1] + m[i][2];
                        i += 1;
                    }
                }
            }
        }
        if i == m.len() {
            // interval was beyond any mapping -> continue
            open_intervals.push((map + 1, s, end));
        }
    }
    final_intervals.iter().map(|(s, _)| s).min().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
