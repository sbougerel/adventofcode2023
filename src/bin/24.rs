advent_of_code::solution!(24);

#[derive(Debug, Copy, Clone)]
struct _3D {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Copy, Clone)]
struct _3Df {
    x: f64,
    y: f64,
    _z: f64,
}

impl std::ops::Add for _3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Hailstone {
    pos: _3D,
    vel: _3D,
}

fn parse(input: &str) -> Vec<Hailstone> {
    let re =
        regex::Regex::new(r"(?m)^(-?\d+),\s+(-?\d+),\s+(-?\d+) @\s+(-?\d+),\s+(-?\d+),\s+(-?\d+)$")
            .unwrap();
    re.captures_iter(input)
        .map(|cap| Hailstone {
            pos: _3D {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                z: cap[3].parse().unwrap(),
            },
            vel: _3D {
                x: cap[4].parse().unwrap(),
                y: cap[5].parse().unwrap(),
                z: cap[6].parse().unwrap(),
            },
        })
        .collect::<Vec<_>>()
}

enum Trajectory {
    Parallel,
    Intersect,
}

fn xy_intersect(a: &Hailstone, b: &Hailstone) -> (Trajectory, Option<(_3Df, f64, f64)>) {
    // To find intersection (if any) between two hailstones (but not necessarily
    // collision), we only need afine equations. So convert parametric
    // equations to afine equations by eliminating t:
    //
    //   (1)         a.pos.x + a.vel.x * t == x  ->  t == (x - a.pos.x) / a.vel.x
    //   (2)         a.pos.y + a.vel.y * t == y  ->  t == (y - a.pos.y) / a.vel.y
    //   (1) == (2)  (x - a.pos.x) / a.vel.x == (y - a.pos.y) / a.vel.y
    //   (1) == (2)  y == a.vel.y * (x - a.pos.x) / a.vel.x + a.pos.y
    //   (1) == (2)  y == a.vel.y / a.vel.x * x - a.vel.y * a.pos.x / a.vel.x + a.pos.y
    //
    // That's for a, but we can do the same for b. We now have two equations.
    // Solve them:
    //
    //   (3)          y == a.vel.y / a.vel.x * x - a.vel.y * a.pos.x / a.vel.x + a.pos.y
    //   (4)          y == b.vel.y / b.vel.x * x - b.vel.y * b.pos.x / b.vel.x + b.pos.y
    //   (3) == (4)   a.vel.y / a.vel.x * x - a.vel.y * a.pos.x / a.vel.x + a.pos.y
    //                == b.vel.y / b.vel.x * x - b.vel.y * b.pos.x / b.vel.x + b.pos.y
    //   (3) == (4)   0 == a.vel.y / a.vel.x * x - b.vel.y / b.vel.x * x
    //                - a.vel.y * a.pos.x / a.vel.x + a.pos.y + b.vel.y * b.pos.x / b.vel.x - b.pos.y
    //   (3) == (4)   0 == (a.vel.y / a.vel.x - b.vel.y / b.vel.x) * x
    //                + b.vel.y * b.pos.x / b.vel.x - a.vel.y * a.pos.x / a.vel.x + a.pos.y - b.pos.y
    //   (3) == (4)   x == (a.vel.y * a.pos.x / a.vel.x - b.vel.y * b.pos.x / b.vel.x + b.pos.y - a.pos.y)
    //                / (a.vel.y / a.vel.x - b.vel.y / b.vel.x)
    //
    // Once we get x, plug it back into (3) or (4) to get y.
    //
    // Compute with integers when possible, give approx floating point answers
    // for return value. Hoping for no overflow/underflow
    assert_ne!(a.vel.x, 0);
    assert_ne!(a.vel.y, 0);
    if a.vel.y * b.vel.x == b.vel.y * a.vel.x {
        // ra == rb ?
        return (Trajectory::Parallel, None);
    }
    let ra = a.vel.y as f64 / a.vel.x as f64;
    let rb = b.vel.y as f64 / b.vel.x as f64;
    let x = (ra * a.pos.x as f64 - rb * b.pos.x as f64 + (b.pos.y - a.pos.y) as f64) / (ra - rb);
    let y = ra * x + a.pos.y as f64 - ra * a.pos.x as f64;
    let z = f64::default(); // x, y only
    let ta = (x - a.pos.x as f64) / a.vel.x as f64;
    let tb = (x - b.pos.x as f64) / b.vel.x as f64;
    return (Trajectory::Intersect, Some((_3Df { x, y, _z: z }, ta, tb)));
}

fn xy_intersections_area(hail: &Vec<Hailstone>, min: i64, max: i64) -> usize {
    let mut count = 0;
    for i in 0..hail.len() - 1 {
        for j in i + 1..hail.len() {
            let a = hail[i];
            let b = hail[j];
            // println!();
            // println!("Hailstone A: {:?}", a);
            // println!("Hailstone B: {:?}", b);
            let (trajectory, intersection) = xy_intersect(&&a, &&b);
            match trajectory {
                Trajectory::Intersect => {
                    let (point, ta, tb) = intersection.unwrap();
                    let min = min as f64;
                    let max = max as f64;
                    if ta < 0.0 || tb < 0.0 {
                        // if ta < 0.0 && tb < 0.0 {
                        //     println!(
                        //         "Hailstones' paths intersected in the past for both hailstones"
                        //     );
                        // } else if ta < 0.0 {
                        //     println!("Hailstones' paths intersected in the past for hailstone A");
                        // } else {
                        //     println!("Hailstones' paths intersected in the past for hailstone B");
                        // }
                        continue;
                    }
                    if min <= point.x && point.x <= max && min <= point.y && point.y <= max {
                        // println!(
                        //     "Hailstones' paths cross inside the test area at {:?}",
                        //     point
                        // );
                        count += 1;
                    } else {
                        // println!(
                        //     "Hailstones' paths cross outside the test area at {:?}",
                        //     point
                        // );
                    }
                }
                Trajectory::Parallel => {
                    // println!("Hailstones' paths are parallel");
                }
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let hail = parse(input);
    // test
    Some(xy_intersections_area(&hail, 7, 27))
    // actual
    //Some(xy_intersections_area(
    //    &hail,
    //    200000000000000,
    //    400000000000000,
    //))
}

pub fn part_two(_input: &str) -> Option<usize> {
    // Perfect collision means there must be a t where all x, y, z, are equal
    // for any hailstones "a" we pick:
    //
    //    (1)         a.pos.x + a.vel.x * t = Px + Vx * t
    //    (2)         a.pos.y + a.vel.y * t = Py + Vy * t
    //    (3)         a.pos.z + a.vel.z * t = Pz + Vz * t
    //
    // We have these equations with any hailstones:
    //
    //    (1)         a.pos.x + a.vel.x * t1 = Px + Vx * t1
    //    (4)         b.pos.x + b.vel.x * t2 = Px + Vx * t2
    //    (5)         c.pos.x + c.vel.x * t3 = Px + Vx * t3
    //                ...
    //
    // Variables t0, t1, ... are inconvenient; we could eliminate them by picking
    // just another dimension such as:
    //
    //   (1 & 2)      (a.pos.x - Px) / (Vx - a.vel.x) = (a.pos.y - Py) / (Vy - a.vel.y)
    //
    // Now rearrange to remove nul divisors, do this for 4 hailstones, since we
    // have 4 variables to solve for:
    //
    //   (6)        0 = (a.pos.x - Px) * (Vy - a.vel.y) - (a.pos.y - Py) * (Vx - a.vel.x)
    //   (7)        0 = (b.pos.x - Px) * (Vy - b.vel.y) - (b.pos.y - Py) * (Vx - b.vel.x)
    //   (8)        0 = (c.pos.x - Px) * (Vy - c.vel.y) - (c.pos.y - Py) * (Vx - c.vel.x)
    //   (9)        0 = (d.pos.x - Px) * (Vy - d.vel.y) - (d.pos.y - Py) * (Vx - d.vel.x)
    //                ...
    //
    // This is where I stopped and plugged all the values in Wolfram Alpha to
    // make sure I'm on the right and haven't made mistakes. It spat out the
    // integral answer:
    //
    //              Px = 129723668686742 ∧ Py = 353939130278484
    //              ∧ Vx = 312 ∧ Vy = -116
    //
    // From there it was an easy path to "victory", picking 2 hailstones "a" and
    // "b":
    //
    //              t(a) = 379155208275
    //              t(b) = 495926101181
    //              Pz = 227368817349775  ∧ Vz = 109
    //
    // To find the real solution, I would need to re-arrange all the terms of
    // the equation, while taking care to not leave any fractions since we're
    // interested in integers. Then find 2 t's and get to Pz, Vz as I did above.
    //
    // This is the way I know how to resolve this, but it's tedious. I'll be
    // looking to see how others did it. My method works, but it requires an
    // equation solver like the amazing WolframAlpha
    // https://www.wolframalpha.com/
    //
    // So I'm gonna leave it at that for now, and maybe I'll come back to it.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
