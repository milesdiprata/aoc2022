use std::io;
use std::io::BufRead;
use std::io::Stdin;
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

struct Sensor {
    position: Point,
    beacon: Point,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

struct SubterraneanTunnels {
    sensors: Vec<Sensor>,
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

        let position = split
            .next()
            .map(|position| position.split_at("Sensor at".len()))
            .map(|(_, position)| position)
            .map(|position| position.parse())
            .ok_or_else(|| anyhow!("Missing sensor position!"))??;

        let beacon = split
            .next()
            .map(|beacon| beacon.split_at("closest beacon is at".len()))
            .map(|(_, beacon)| beacon)
            .map(|beacon| beacon.parse())
            .ok_or_else(|| anyhow!("Missing closest beacon location!"))??;

        Ok(Self { position, beacon })
    }
}

impl Point {
    fn dist(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl SubterraneanTunnels {
    fn from_stdin(stdin: Stdin) -> Result<Self> {
        let sensors = stdin
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
            .iter()
            .map(String::as_str)
            .map(Sensor::from_str)
            .collect::<Result<_>>()?;

        Ok(Self { sensors })
    }

    fn is_beacon(&self, position: &Point) -> bool {
        self.sensors
            .iter()
            .map(|sensor| &sensor.beacon)
            .any(|beacon| beacon == position)
    }

    fn is_within_dist(&self, position: &Point) -> bool {
        self.sensors
            .iter()
            .map(|sensor| (&sensor.position, &sensor.beacon))
            .any(|(sensor, beacon)| sensor.dist(position) <= sensor.dist(beacon))
    }

    fn find_x_range(&self, y: isize) -> RangeInclusive<isize> {
        let mut x_min = isize::MAX;
        let mut x_max = isize::MIN;

        self.sensors
            .iter()
            .map(|sensor| (&sensor.position, &sensor.beacon))
            .for_each(|(sensor, beacon)| {
                let dist = sensor.dist(beacon);

                if (y - beacon.y).abs() <= dist {
                    let middle = sensor.x;
                    let start = middle - dist;
                    let end = middle + dist;

                    x_min = x_min.min(start);
                    x_max = x_max.max(end);
                }
            });

        x_min..=x_max
    }

    fn find_beaconless_locations(&self, y: isize) -> impl Iterator<Item = Point> + '_ {
        self.find_x_range(y)
            .map(move |x| Point { x, y })
            .filter(|position| !self.is_beacon(position))
            .filter(|position| self.is_within_dist(position))
    }
}

fn part_one(tunnels: &SubterraneanTunnels) -> usize {
    const Y_TARGET: isize = 2_000_000;
    tunnels.find_beaconless_locations(Y_TARGET).count()
}

fn main() -> Result<()> {
    let tunnels = SubterraneanTunnels::from_stdin(io::stdin())?;

    println!("Part one: {}", part_one(&tunnels));

    Ok(())
}
