use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::io::Stdin;
use std::iter;
use std::str::FromStr;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: usize,
    valves: Vec<String>,
}

#[derive(Debug)]
struct ProboscideaVolcanium {
    valves: HashMap<String, Valve>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(valve: &str) -> Result<Self> {
        let id = valve
            .split("Valve ")
            .last()
            .map(|split| split.chars())
            .map(|split| split.take(2))
            .map(|split| split.collect())
            .ok_or_else(|| anyhow!("Missing valve ID!"))?;

        let flow_rate = valve
            .split('=')
            .last()
            .and_then(|split| split.split(';').next())
            .map(str::parse)
            .ok_or_else(|| anyhow!("Missing flow-rate!"))??;

        let valves = valve
            .split("to valve")
            .last()
            .map(|split| split.split("s "))
            .and_then(|split| split.last())
            .map(|split| split.trim())
            .map(|split| split.split(", "))
            .map(|split| split.map(str::to_string))
            .map(|split| split.collect())
            .ok_or_else(|| anyhow!("Missing adj valves!"))?;

        Ok(Self {
            id,
            flow_rate,
            valves,
        })
    }
}

impl Valve {
    fn pressure_released(&self, time_remaining: Duration) -> usize {
        self.flow_rate * time_remaining.as_secs() as usize
    }
}

impl ProboscideaVolcanium {
    fn from_stdin(stdin: Stdin) -> Result<Self> {
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
            .map(|line| line.and_then(|line| line.parse::<Valve>()))
            .collect::<Result<Vec<_>>>()
            .map(|valves| valves.into_iter())
            .map(|valves| valves.map(|valve| (valve.id.clone(), valve)))
            .map(|valves| valves.collect())
            .map(|valves| Self { valves })
    }
}

fn part_one(proboscidea_volcanium: &ProboscideaVolcanium) -> Option<usize> {
    const MINUTE: Duration = Duration::from_secs(60);
    const VALVE_ID_START: &str = "AA";

    let mut time_remaining = 30 * MINUTE;
    let mut pressure_released = 0;

    let mut frontier = iter::once(VALVE_ID_START).collect::<VecDeque<_>>();
    let mut visited = iter::once(VALVE_ID_START).collect::<HashSet<_>>();

    while let Some(i) = frontier.pop_front() {
        if time_remaining <= Duration::ZERO {
            break;
        }

        time_remaining -= MINUTE;
    }

    Some(pressure_released)
}

fn main() -> Result<()> {
    let proboscidea_volcanium = ProboscideaVolcanium::from_stdin(io::stdin())?;

    println!("{proboscidea_volcanium:#?}");

    println!("Part one: {:?}", part_one(&proboscidea_volcanium));

    Ok(())
}
