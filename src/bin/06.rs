advent_of_code::solution!(6);

const RE: &str = r"Time: ([\d\s]+)
Distance: ([\d\s]+)
";

pub fn part_one(input: &str) -> Option<u32> {
    let captures = regex::Regex::new(RE).unwrap().captures(input).unwrap();
    let times = captures[1]
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let dists = captures[2]
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let mut mul = 1;
    for i in 0..times.len() {
        let mut sum = 0;
        for j in 0..times[i] {
            if j * (times[i] - j) > dists[i] {
                sum += 1;
            }
        }
        mul *= sum;
    }
    Some(mul)
}

pub fn part_two(input: &str) -> Option<u64> {
    let captures = regex::Regex::new(RE).unwrap().captures(input).unwrap();
    let time = captures[1].replace(" ", "").parse::<u64>().unwrap();
    let dist = captures[2].replace(" ", "").parse::<u64>().unwrap();
    // I orginally brute-forced this as above (easy with Rust)...
    // let mut sum = 0;
    // for j in (dist/time)..time {
    //     if j * (time - j) > dist {
    //         sum += 1;
    //     }
    // }
    // And after, only, investigated and used the quadratic forumla:
    // j * (time - j) > dist
    // j * time - j^2 > dist
    // j^2 - time * j + dist < 0
    // The values of j for which == 0 are the bounds:
    // j = (time +- sqrt(time^2 - 4 * dist)) / 2
    let j_lo =
        (time as f64 - ((time as f64).powi(2) - 4.0 * dist as f64).sqrt() / 2.0).ceil() as u64;
    let j_hi =
        (time as f64 + ((time as f64).powi(2) - 4.0 * dist as f64).sqrt() / 2.0).ceil() as u64;
    // The crazy part is that the brute force version is only 500 times slower,
    // so I guess computing those square roots is expensive.
    Some(j_hi - j_lo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
