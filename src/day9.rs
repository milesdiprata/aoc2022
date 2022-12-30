use std::collections::HashSet;
use std::io;

use anyhow::Result;

use day9::{Motion, Rope};

mod day9 {
    use std::io::{BufRead, Stdin};
    use std::str::FromStr;

    use anyhow::{anyhow, Error, Result};

    #[derive(Clone, Copy, Debug)]
    pub enum Direction {
        Up,
        Left,
        Right,
        Down,
    }

    #[derive(Debug)]
    pub struct Motion {
        dir: Direction,
        steps: isize,
    }

    #[derive(Debug, Default)]
    pub struct Rope {
        head: (isize, isize),
        tail: (isize, isize),
    }

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
            let steps = split
                .next()
                .map(isize::from_str)
                .ok_or_else(|| anyhow!("Missing steps!"))??;

            Ok(Self { dir, steps })
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

    impl Rope {
        pub fn step(&mut self, motion: &Motion) -> Vec<(isize, isize)> {
            (0..motion.steps)
                .map(|_| self.sub_step(motion.dir))
                .collect()
        }

        fn sub_step(&mut self, dir: Direction) -> (isize, isize) {
            self.head = Self::straight_sub_step(self.head, dir);

            if self.head.0.abs_diff(self.tail.0) > 1 || self.head.1.abs_diff(self.tail.1) > 1 {
                self.tail = match self.head.0 == self.tail.0 || self.head.1 == self.tail.1 {
                    true => Self::straight_sub_step(self.tail, dir),
                    _ => Self::diagonal_sub_step(self.tail, self.head),
                };
            }

            self.tail
        }

        fn straight_sub_step((i, j): (isize, isize), dir: Direction) -> (isize, isize) {
            match dir {
                Direction::Up => (i + 1, j),
                Direction::Left => (i, j - 1),
                Direction::Right => (i, j + 1),
                Direction::Down => (i - 1, j),
            }
        }

        fn diagonal_sub_step(
            (i, j): (isize, isize),
            (i_target, j_target): (isize, isize),
        ) -> (isize, isize) {
            let i = match i_target > i {
                true => i + 1,
                _ => i - 1,
            };

            let j = match j_target > j {
                true => j + 1,
                false => j - 1,
            };

            (i, j)
        }
    }
}

fn part_one(rope: &mut Rope, motions: &[Motion]) -> usize {
    motions
        .iter()
        .flat_map(|motion| rope.step(motion))
        .collect::<HashSet<_>>()
        .len()
}

fn main() -> Result<()> {
    let motions = Motion::from_stdin(io::stdin())?;
    let mut rope = Rope::default();

    println!("Part one: {}", part_one(&mut rope, motions.as_slice()));

    Ok(())
}
