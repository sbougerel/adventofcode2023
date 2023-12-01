advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            // Clearly, not the most efficient way to do this, a better approach
            // might have been use the leading character to determine which word
            // to check, and advance the slice by 1 or by the len of word found.
            // This was only efficent to type.
            let tok2u32 = [
                ("0", 0u32),
                ("1", 1),
                ("2", 2),
                ("3", 3),
                ("4", 4),
                ("5", 5),
                ("6", 6),
                ("7", 7),
                ("8", 8),
                ("9", 9),
                ("zero", 0),
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ];
            let mut first_digit = 0;
            let mut first_index = line.len();
            let mut last_digit = 0;
            let mut last_index = 0;
            // Assume that the first and last word never overlap! Gasp!
            for (word, val) in tok2u32.iter() {
                if let Some(index) = line.find(word) {
                    if index < first_index {
                        first_digit = *val;
                        first_index = index;
                    }
                }
                if let Some(index) = line.rfind(word) {
                    if last_index <= index {
                        last_digit = *val;
                        last_index = index;
                    }
                }
            }
            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(242));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
