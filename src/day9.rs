use std::collections::HashSet;
use std::io;

use anyhow::Result;

use day9::{Motion, Rope};

mod day9 {
    use std::collections::VecDeque;
    use std::io::{BufRead, Stdin};
    use std::str::FromStr;

    use anyhow::{anyhow, Error, Result};

    #[derive(Clone, Copy, Debug)]
    enum Direction {
        Up,
        Left,
        Right,
        Down,
    }

    #[derive(Debug)]
    pub struct Motion {
        dir: Direction,
        len: usize,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Knot(isize, isize);

    #[derive(Debug)]
    pub struct Rope(VecDeque<Knot>);

    impl FromStr for Direction {
        type Err = Error;

        fn from_str(str: &str) -> Result<Self> {
            match str.len() {
                1 => match str.chars().next().unwrap() {
                    'U' => Ok(Self::Up),
                    'L' => Ok(Self::Left),
                    'R' => Ok(Self::Right),
                    'D' => Ok(Self::Down),
                    _ => Err(anyhow!("Invalid direction!")),
                },
                _ => Err(anyhow!("Expected single character for direction!")),
            }
        }
    }

    impl FromStr for Motion {
        type Err = Error;

        fn from_str(str: &str) -> Result<Self> {
            let mut split = str.split_whitespace();

            let dir = split
                .next()
                .map(|dir| dir.parse())
                .ok_or_else(|| anyhow!("Missing direction!"))??;
            let len = split
                .next()
                .map(usize::from_str)
                .ok_or_else(|| anyhow!("Missing len!"))??;

            Ok(Self { dir, len })
        }
    }

    impl Motion {
        pub fn from_stdin(stdin: Stdin) -> Result<Vec<Self>> {
            let mut lines = stdin.lock().lines();
            let mut motions = Vec::new();

            while let Some(Ok(line)) = lines.next() {
                if line.is_empty() {
                    break;
                }

                motions.push(line.parse()?);
            }

            Ok(motions)
        }
    }

    impl Knot {
        fn move_head(&mut self, dir: Direction) {
            match dir {
                Direction::Up => self.0 += 1,
                Direction::Left => self.1 -= 1,
                Direction::Right => self.1 += 1,
                Direction::Down => self.0 -= 1,
            }
        }

        fn follow(&mut self, head: &Self) {
            let i_diff = head.0.abs_diff(self.0);
            let j_diff = head.1.abs_diff(self.1);

            if (i_diff == 2 && j_diff == 0) || (i_diff == 0 && j_diff == 2) {
                self.step(head);
            } else if i_diff > 1 || j_diff > 1 {
                self.diagonal_step(head);
            }
        }

        fn step(&mut self, head: &Self) {
            if head.0 > self.0 {
                self.0 += 1;
            } else if self.0 > head.0 {
                self.0 -= 1;
            } else if head.1 > self.1 {
                self.1 += 1;
            } else if self.1 > head.1 {
                self.1 -= 1;
            }
        }

        fn diagonal_step(&mut self, head: &Self) {
            match head.0 > self.0 {
                true => self.0 += 1,
                _ => self.0 -= 1,
            };

            match head.1 > self.1 {
                true => self.1 += 1,
                _ => self.1 -= 1,
            };
        }
    }

    impl Rope {
        pub fn with_knots(knots: usize) -> Result<Self> {
            match knots {
                (2..) => Ok(Self((0..knots).map(|_| Knot::default()).collect())),
                _ => Err(anyhow!("Minimum of two knots required!")),
            }
        }

        pub fn step(&mut self, motion: &Motion) -> Option<Vec<Knot>> {
            let knots_len = self.0.len();
            let mut tails = Vec::with_capacity(motion.len);

            (0..motion.len)
                .map(|_| {
                    self.0.front_mut()?.move_head(motion.dir);

                    (1..knots_len)
                        .map(|i| {
                            let head = self.0.get(i - 1).cloned()?;
                            let tail = self.0.get_mut(i)?;

                            tail.follow(&head);

                            Some(())
                        })
                        .collect::<Option<_>>()?;

                    tails.push(self.0.back().cloned()?);

                    Some(())
                })
                .collect::<Option<_>>()?;

            Some(tails)
        }
    }
}

fn count_tails(mut rope: Rope, motions: &[Motion]) -> usize {
    motions
        .iter()
        .flat_map(|motion| rope.step(motion))
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}

fn part_one(rope: Rope, motions: &[Motion]) -> usize {
    count_tails(rope, motions)
}

fn part_two(rope: Rope, motions: &[Motion]) -> usize {
    count_tails(rope, motions)
}

fn main() -> Result<()> {
    let motions = Motion::from_stdin(io::stdin())?;

    println!(
        "Part one: {}",
        part_one(Rope::with_knots(2)?, motions.as_slice())
    );

    println!(
        "Part two: {}",
        part_two(Rope::with_knots(10)?, motions.as_slice())
    );

    Ok(())
}
