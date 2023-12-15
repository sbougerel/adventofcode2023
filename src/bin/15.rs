advent_of_code::solution!(15);

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0_usize, |a, b| ((a + (*b as usize)) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<usize> {
    input
        .split(',')
        .map(|s| hash(s.trim()))
        .sum::<usize>()
        .into()
}

#[allow(dead_code)]
fn print_boxes(boxes: &[Vec<(&str, usize)>; 256]) {
    for (i, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            println!("Box {}: {:?}", i, b);
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: [Vec<(&str, usize)>; 256] = std::array::from_fn(|_| Vec::new());
    for cmd in input.split(',') {
        let cmd = cmd.trim();
        let (lbl, ins) = cmd.split_at(cmd.find(['=', '-']).unwrap());
        let (sep, foc) = ins.split_at(1);
        let bxn = hash(lbl);
        let pos = boxes[bxn].iter().position(|(s, _)| *s == lbl);
        match sep {
            "=" => match pos {
                Some(i) => {
                    boxes[bxn].remove(i);
                    boxes[bxn].insert(i, (lbl, foc.parse::<usize>().unwrap()));
                }
                None => boxes[bxn].push((lbl, foc.parse::<usize>().unwrap())),
            },
            _ => match pos {
                Some(i) => {
                    boxes[bxn].remove(i);
                }
                None => {}
            },
        }
    }
    // print_boxes(&boxes);
    boxes
        .iter()
        .enumerate()
        .map(|(bxn, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(ln, (_, foc))| (bxn + 1) * (ln + 1) * foc)
                .sum::<usize>()
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(144));
    }
}
