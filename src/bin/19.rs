advent_of_code::solution!(19);

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Cat {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Rule {
    cat: Cat,
    less: bool,
    val: u64,
    target: String,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    target: String,
}

type Workflows = HashMap<String, Workflow>;

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}
type Parts = Vec<Part>;

fn parse(input: &str) -> (Workflows, Parts) {
    let rule = Regex::new(r"([xmsa])([><])(\d+):(\w+)").unwrap();
    let workflow = Regex::new(r"(?m)^(\w+)\{((?:[xmsa][><]\d+:\w+,)+)(\w+)\}$").unwrap();
    let part = Regex::new(r"(?m)^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    (
        workflow
            .captures_iter(&input)
            .map(|cap| {
                (
                    cap[1].to_string(),
                    Workflow {
                        rules: cap[2]
                            .trim_matches(',')
                            .split(",")
                            .map(|tok| {
                                let (_, [cat, cmp, val, target]) =
                                    rule.captures(tok).unwrap().extract();
                                Rule {
                                    cat: match cat {
                                        "x" => Cat::X,
                                        "m" => Cat::M,
                                        "a" => Cat::A,
                                        _ => Cat::S,
                                    },
                                    less: (cmp == "<"),
                                    val: val.parse::<u64>().unwrap(),
                                    target: target.to_string(),
                                }
                            })
                            .collect::<Vec<Rule>>(),
                        target: cap[3].to_string(),
                    },
                )
            })
            .collect::<Workflows>(),
        part.captures_iter(&input)
            .map(|cap| Part {
                x: cap[1].parse::<u64>().unwrap(),
                m: cap[2].parse::<u64>().unwrap(),
                a: cap[3].parse::<u64>().unwrap(),
                s: cap[4].parse::<u64>().unwrap(),
            })
            .collect::<Parts>(),
    )
}

fn sort_parts(workflows: &Workflows, parts: &Parts) -> u64 {
    let mut sum = 0;
    for part in parts {
        let mut target = "in";
        while target != "A" && target != "R" {
            let workflow = &workflows[target];
            let mut rule_target = "";
            for rule in workflow.rules.iter() {
                let part_val = match rule.cat {
                    Cat::A => part.a,
                    Cat::M => part.m,
                    Cat::S => part.s,
                    Cat::X => part.x,
                };
                if rule.less && part_val < rule.val || !rule.less && part_val > rule.val {
                    rule_target = &rule.target;
                    break;
                }
            }
            if rule_target.is_empty() {
                target = &workflow.target;
            } else {
                target = rule_target;
            }
        }
        if target == "A" {
            sum += part.a + part.m + part.s + part.x;
        }
    }
    sum
}

#[derive(Debug, Clone, Copy)]
struct PartInt {
    x: [u64; 2],
    m: [u64; 2],
    a: [u64; 2],
    s: [u64; 2],
}

#[derive(Debug, PartialEq)]
enum Outcome {
    No,
    Low,
    High,
    Yes,
}

fn accepted_intervals(workflows: &Workflows) -> Vec<PartInt> {
    let init = PartInt {
        x: [0, 4000], // 1..=4000 - 1 -> 0..4000; Need to adjust all checks by -1
        m: [0, 4000],
        a: [0, 4000],
        s: [0, 4000],
    };
    // Reduce part intervals by going throuh all workflows, starting from "in".
    // Manage a list of open (workflow, part interval), which should be sorted
    // in final "accepted" bucket.  Since each rule splits a part interval in
    // distinct non-overlapping interval, there's no need to worry about
    // overlaps.
    let mut open = vec![(init, "in")];
    let mut accepted = vec![];
    while !open.is_empty() {
        let (mut partint, target) = open.pop().unwrap();
        if target == "A" {
            accepted.push(partint);
            continue;
        }
        if target == "R" {
            continue; // discard
        }
        let workflow = &workflows[target];
        // Split interval with each rules
        let mut outcome = Outcome::No;
        for rule in workflow.rules.iter() {
            let part_val = match rule.cat {
                Cat::A => partint.a,
                Cat::M => partint.m,
                Cat::S => partint.s,
                Cat::X => partint.x,
            };
            let test_val = rule.val - 1; // 0..4000 interval
            outcome = if rule.less {
                if part_val[1] < test_val {
                    Outcome::Yes
                } else {
                    if part_val[0] < test_val {
                        Outcome::Low
                    } else {
                        Outcome::No
                    }
                }
            } else {
                if part_val[0] > test_val {
                    Outcome::Yes
                } else {
                    if part_val[1] > test_val {
                        Outcome::High
                    } else {
                        Outcome::No
                    }
                }
            };
            match outcome {
                Outcome::Yes => {
                    // The whole interval matches!
                    open.push((partint, &rule.target));
                    break;
                }
                Outcome::No => {}
                Outcome::Low => {
                    // The lower half matches with "less than" test
                    let mut copy = partint;
                    match rule.cat {
                        Cat::A => (partint.a[0], copy.a[1]) = (test_val, test_val),
                        Cat::M => (partint.m[0], copy.m[1]) = (test_val, test_val),
                        Cat::S => (partint.s[0], copy.s[1]) = (test_val, test_val),
                        Cat::X => (partint.x[0], copy.x[1]) = (test_val, test_val),
                    };
                    open.push((copy, &rule.target));
                }
                Outcome::High => {
                    // The upper half matches with "greater than" test
                    let mut copy = partint;
                    match rule.cat {
                        Cat::A => (partint.a[1], copy.a[0]) = (test_val + 1, test_val + 1),
                        Cat::M => (partint.m[1], copy.m[0]) = (test_val + 1, test_val + 1),
                        Cat::S => (partint.s[1], copy.s[0]) = (test_val + 1, test_val + 1),
                        Cat::X => (partint.x[1], copy.x[0]) = (test_val + 1, test_val + 1),
                    };
                    open.push((copy, &rule.target));
                }
            };
        }
        // If rules outcome != Yes, continue to workflow target
        if outcome != Outcome::Yes {
            open.push((partint, &workflow.target));
        }
    }
    accepted
}

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, parts) = parse(&input);
    Some(sort_parts(&workflows, &parts))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = parse(&input);
    let accepted = accepted_intervals(&workflows);
    accepted
        .iter()
        .map(|p| (p.x[1] - p.x[0]) * (p.m[1] - p.m[0]) * (p.a[1] - p.a[0]) * (p.s[1] - p.s[0]))
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
