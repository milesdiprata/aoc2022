use anyhow::{anyhow, Error, Result};
use std::env;
use std::str::FromStr;

#[derive(Debug)]
struct Signal(String);

impl FromStr for Signal {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        if !str.chars().all(|char| char.is_ascii_lowercase()) {
            return Err(anyhow!("Signal is not all lowercase ASCII!"));
        }

        Ok(Self(str.to_owned()))
    }
}

impl Signal {
    fn find_packet_start(&self) -> Option<usize> {
        self.0
            .char_indices()
            .collect::<Vec<_>>()
            .windows(4)
            .find(|&chars| {
                (0..4)
                    .map(|i| i as usize)
                    .flat_map(|i| (0..4).map(|j| j as usize).map(move |j| (i, j)))
                    .filter(|&(i, j)| i != j)
                    .all(|(i, j)| chars[i].1 != chars[j].1)
            })
            .map(|chars| chars[3].0)
            .map(|idx| idx + 1)
    }
}

fn read_signal() -> Result<Signal> {
    let signal = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Missing signal!"))?;

    Ok(Signal(signal))
}

fn part_one(signal: &Signal) -> Option<usize> {
    signal.find_packet_start()
}

fn main() -> Result<()> {
    let signal = read_signal()?;

    println!(
        "Part one: {}",
        part_one(&signal).ok_or_else(|| anyhow!("No start-of-packet marker found!"))?
    );

    Ok(())
}
