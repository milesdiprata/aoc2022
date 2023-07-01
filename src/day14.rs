use std::io;

use anyhow::Result;

use day14::Cave;

mod day14 {
    use std::{
        fmt,
        io::{BufRead, Stdin},
        str::FromStr,
    };

    use anyhow::{anyhow, Error, Result};

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq)]
    enum Tile {
        Air,
        Rock,
        SandSource,
        Sand,
    }

    #[derive(Clone, Copy)]
    struct Point {
        x: u16,
        y: u8,
    }

    struct Path {
        coords: Vec<Point>,
    }

    pub struct Cave {
        grid: Vec<Vec<Tile>>,
    }

    impl fmt::Debug for Tile {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Air => write!(fmt, "."),
                Self::Rock => write!(fmt, "#"),
                Self::SandSource => write!(fmt, "+"),
                Self::Sand => write!(fmt, "o"),
            }
        }
    }

    impl fmt::Debug for Point {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(fmt, "{},{}", self.x, self.y)
        }
    }

    impl fmt::Debug for Path {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut sep = None;

            self.coords.iter().try_for_each(|point| {
                let res = write!(fmt, "{}{:?}", sep.unwrap_or_default(), point);
                sep = Some(" -> ");
                res
            })
        }
    }

    impl fmt::Debug for Cave {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.grid.iter().try_for_each(|row| {
                row.iter()
                    .try_for_each(|tile| write!(fmt, "{tile:?}"))
                    .and_then(|_| writeln!(fmt))
            })
        }
    }

    impl FromStr for Point {
        type Err = Error;

        fn from_str(point: &str) -> Result<Self> {
            let mut split = point.split(',');

            let x = split
                .next()
                .ok_or_else(|| anyhow!("Missing x-coordinate!"))?
                .parse()?;

            let y = split
                .next()
                .ok_or_else(|| anyhow!("Missing y-coordinate!"))?
                .parse()?;

            Ok(Self { x, y })
        }
    }

    impl FromStr for Path {
        type Err = Error;

        fn from_str(coords: &str) -> Result<Self> {
            let coords = coords
                .split(" -> ")
                .map(|point| point.parse())
                .collect::<Result<Vec<_>>>()?;

            Ok(Self { coords })
        }
    }

    impl Point {
        fn as_down(&self) -> Option<Self> {
            if self.y < u8::MAX {
                Some(Self {
                    x: self.x,
                    y: self.y + 1,
                })
            } else {
                None
            }
        }

        fn as_down_left(&self) -> Option<Self> {
            if self.x as isize - 1 > 0 && self.y < u8::MAX {
                Some(Self {
                    x: self.x - 1,
                    y: self.y + 1,
                })
            } else {
                None
            }
        }

        fn as_down_right(&self) -> Option<Self> {
            if self.x < u16::MAX && self.y < u8::MAX {
                Some(Self {
                    x: self.x + 1,
                    y: self.y + 1,
                })
            } else {
                None
            }
        }
    }

    impl Path {
        fn x_coords(&self) -> impl Iterator<Item = u16> + '_ {
            self.coords.iter().map(|point| point.x)
        }

        fn y_coords(&self) -> impl Iterator<Item = u8> + '_ {
            self.coords.iter().map(|point| point.y)
        }
    }

    impl Cave {
        pub fn from_stdin(stdin: Stdin) -> Result<Self> {
            stdin
                .lock()
                .lines()
                .take_while(|line| {
                    line.as_ref()
                        .map(|line| line.is_empty())
                        .map(|empty| !empty)
                        .unwrap_or_default()
                })
                .map(|line| line.map_err(|err| anyhow!(err)))
                .map(|line| line.and_then(|line| line.parse()))
                .collect::<Result<Vec<_>>>()
                .map(|paths| Self::from_paths(paths.as_slice()))
        }

        fn from_paths(paths: &[Path]) -> Self {
            let x_max = paths
                .iter()
                .flat_map(|path| path.x_coords())
                .max()
                .unwrap_or(Self::sand_source().x);

            let y_max = paths
                .iter()
                .flat_map(|path| path.y_coords())
                .max()
                .unwrap_or(Self::sand_source().y);

            let mut cave = Self {
                grid: (0..=y_max)
                    .map(|_| (0..=x_max).map(|_| Tile::Air).collect())
                    .collect(),
            };

            *cave
                .get_mut(Self::sand_source())
                .unwrap_or_else(|| unreachable!()) = Tile::SandSource;

            paths.iter().for_each(|path| {
                path.coords
                    .windows(2)
                    .flat_map(|points| match (points.first(), points.last()) {
                        (Some(i), Some(j)) => Some((i, j)),
                        _ => None,
                    })
                    .for_each(|(i, j)| {
                        let (x_min, x_max) = (i.x.min(j.x), i.x.max(j.x));
                        let (y_min, y_max) = (i.y.min(j.y), i.y.max(j.y));

                        (x_min..=x_max).for_each(|x| {
                            if let Some(tile) = cave.get_mut(Point { x, y: y_min }) {
                                *tile = Tile::Rock;
                            }
                        });

                        (y_min..=y_max).for_each(|y| {
                            if let Some(tile) = cave.get_mut(Point { x: x_min, y }) {
                                *tile = Tile::Rock;
                            }
                        })
                    });
            });

            cave
        }

        pub fn drop_sand(&mut self) -> Option<()> {
            let mut sand = Self::sand_source();

            loop {
                if let Tile::Air = self.get(sand.as_down()?)? {
                    sand = sand.as_down()?;
                } else if let Tile::Air = self.get(sand.as_down_left()?)? {
                    sand = sand.as_down_left()?;
                } else if let Tile::Air = self.get(sand.as_down_right()?)? {
                    sand = sand.as_down_right()?;
                } else {
                    break;
                }
            }

            *self.get_mut(sand)? = Tile::Sand;

            Some(())
        }

        fn get(&self, point: Point) -> Option<Tile> {
            self.grid
                .get(point.y as usize)
                .and_then(|row| row.get(point.x as usize))
                .copied()
        }

        fn get_mut(&mut self, point: Point) -> Option<&mut Tile> {
            self.grid
                .get_mut(point.y as usize)
                .and_then(|row| row.get_mut(point.x as usize))
        }

        const fn sand_source() -> Point {
            Point { x: 500, y: 0 }
        }
    }
}

fn part_one(cave: &mut Cave) -> usize {
    (1..usize::MAX)
        .take_while(|_| cave.drop_sand().is_some())
        .last()
        .unwrap_or_default()
}

fn main() -> Result<()> {
    let mut cave = Cave::from_stdin(io::stdin())?;

    // println!("{:?}", cave);

    println!("Part one: {}", part_one(&mut cave));

    Ok(())
}
