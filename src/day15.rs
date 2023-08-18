use std::io;

use anyhow::Result;

use day15::SubterraneanTunnels;

mod day15 {
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        fmt,
        io::{BufRead, Stdin},
        str::FromStr,
    };

    use anyhow::{anyhow, Error, Result};

    enum Position {
        Sensor(Point),
        Beacon,
        Visited,
    }

    struct Sensor {
        location: Point,
        closest_beacon: Point,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Point {
        x: isize,
        y: isize,
    }

    pub struct SubterraneanTunnels {
        grid: HashMap<Point, Position>,
    }

    impl fmt::Debug for Position {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Position::Sensor(_) => write!(fmt, "S"),
                Position::Beacon => write!(fmt, "B"),
                Position::Visited => write!(fmt, "#"),
            }
        }
    }

    impl fmt::Debug for SubterraneanTunnels {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            (-2..=22).try_for_each(|y| {
                (-2..=25)
                    .map(|x| Point { x, y })
                    .map(|point| self.grid.get(&point))
                    .try_for_each(|position| match position {
                        Some(position) => write!(fmt, "{position:?}"),
                        None => write!(fmt, "."),
                    })
                    .and_then(|_| writeln!(fmt))
            })
        }
    }

    impl FromStr for Point {
        type Err = Error;

        fn from_str(point: &str) -> Result<Self> {
            let mut split = point.trim().split(", ");

            let x = split
                .next()
                .map(|x| x.split('='))
                .and_then(|x| x.last())
                .map(|x| x.parse())
                .ok_or_else(|| anyhow!("Missing x-coordinate!"))??;

            let y = split
                .next()
                .map(|y| y.split('='))
                .and_then(|y| y.last())
                .map(|y| y.parse())
                .ok_or_else(|| anyhow!("Missing y-coordinate!"))??;

            Ok(Self { x, y })
        }
    }

    impl FromStr for Sensor {
        type Err = Error;

        fn from_str(sensor: &str) -> Result<Self> {
            let mut split = sensor.trim().split(": ");

            let location = split
                .next()
                .map(|location| location.split_at("Sensor at".len()))
                .map(|(_, location)| location)
                .map(|location| location.parse())
                .ok_or_else(|| anyhow!("Missing sensor location!"))??;

            let closest_beacon = split
                .next()
                .map(|beacon| beacon.split_at("closest beacon is at".len()))
                .map(|(_, beacon)| beacon)
                .map(|beacon| beacon.parse())
                .ok_or_else(|| anyhow!("Missing closest beacon location!"))??;

            Ok(Self {
                location,
                closest_beacon,
            })
        }
    }

    impl Sensor {
        fn into_positions(self) -> impl Iterator<Item = (Point, Position)> {
            [
                (self.location, Position::Sensor(self.closest_beacon.clone())),
                (self.closest_beacon, Position::Beacon),
            ]
            .into_iter()
        }
    }

    impl Point {
        fn as_up(&self) -> Self {
            Self {
                x: self.x,
                y: self.y - 1,
            }
        }

        fn as_down(&self) -> Self {
            Self {
                x: self.x,
                y: self.y + 1,
            }
        }

        fn as_left(&self) -> Self {
            Self {
                x: self.x - 1,
                y: self.y,
            }
        }

        fn as_right(&self) -> Self {
            Self {
                x: self.x + 1,
                y: self.y,
            }
        }

        fn adjacencies(&self) -> impl Iterator<Item = Self> {
            [
                self.as_up(),
                self.as_down(),
                self.as_left(),
                self.as_right(),
            ]
            .into_iter()
        }

        fn manhattan_distance(&self, other: &Self) -> isize {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }
    }

    impl SubterraneanTunnels {
        pub fn from_stdin(stdin: Stdin) -> Result<Self> {
            let grid = stdin
                .lock()
                .lines()
                .take_while(|line| {
                    line.as_ref()
                        .map(|line| line.is_empty())
                        .map(|empty| !empty)
                        .unwrap_or_default()
                })
                .map(|line| line.map_err(|err| anyhow!(err)))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(|line| line.parse::<Sensor>())
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flat_map(|sensor| sensor.into_positions())
                .collect();

            Ok(Self { grid })
        }

        pub fn find_beaconless_locations(&self, row_y: isize) -> impl Iterator<Item = Point> + '_ {
            self.grid
                .iter()
                .filter_map(|(sensor, position)| match position {
                    Position::Sensor(beacon) => Some((sensor, beacon)),
                    _ => None,
                })
                .map(|(sensor, beacon)| (sensor, sensor.manhattan_distance(beacon)))
                .filter(|&(sensor, distance)| (sensor.y - row_y).abs() <= distance)
                .flat_map(|(sensor, distance)| self.bfs_root(sensor.clone(), distance, row_y))
                .collect::<HashSet<_>>()
                .into_iter()
        }

        fn bfs_root(
            &self,
            start: Point,
            distance: isize,
            row_y: isize,
        ) -> impl Iterator<Item = Point> + '_ {
            let mut frontier = VecDeque::new();
            let mut visited = HashSet::new();

            frontier.push_back(start.clone());
            visited.insert(start.clone());

            while let Some(i) = frontier.pop_front() {
                i.adjacencies()
                    .filter(|j| (j.y - row_y).abs() <= distance)
                    .filter(|j| start.manhattan_distance(j) <= distance)
                    .filter(|j| !visited.contains(j))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .for_each(|j| {
                        frontier.push_back(j.clone());
                        visited.insert(j);
                    });
            }

            visited
                .into_iter()
                .filter(|location| self.grid.get(location).is_none())
                .filter(move |location| location.y == row_y)
        }
    }
}

fn part_one(tunnels: &SubterraneanTunnels) -> usize {
    // const ROW_Y: isize = 2_000_000;
    const ROW_Y: isize = 10;

    tunnels.find_beaconless_locations(ROW_Y).count()
}

fn main() -> Result<()> {
    let tunnels = SubterraneanTunnels::from_stdin(io::stdin())?;

    println!("Part one: {}", part_one(&tunnels));

    Ok(())
}
