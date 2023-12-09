advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let seqs = input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    seqs.iter()
        .map(|seq| {
            let mut rows = vec![seq.clone()];
            while rows[rows.len() - 1].iter().filter(|e| **e != 0).count() > 0 {
                let row = (&rows[rows.len() - 1][..])
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<_>>();
                rows.push(row);
            }
            // Add up to the final value
            (0..rows.len())
                .map(|y| rows[y][rows[y].len() - 1])
                .sum::<i64>()
        })
        .sum::<i64>()
        .into()
}

pub fn part_two(input: &str) -> Option<i64> {
    let seqs = input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    seqs.iter()
        .map(|seq| {
            let mut rows = vec![seq.clone()];
            while rows[rows.len() - 1].iter().filter(|e| **e != 0).count() > 0 {
                let row = (&rows[rows.len() - 1][..])
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<_>>();
                rows.push(row);
            }
            // Add up to the final value
            (0..rows.len()).rev().fold(0, |acc, i| rows[i][0] - acc)
        })
        .sum::<i64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
