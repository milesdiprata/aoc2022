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
    fn find_packet_start(&self, distinct_len: usize) -> Option<usize> {
        self.0
            .char_indices()
            .collect::<Vec<_>>()
            .windows(distinct_len)
            .find(|&chars| {
                (0..distinct_len)
                    .flat_map(|i| (0..distinct_len).map(move |j| (i, j)))
                    .filter(|&(i, j)| i != j)
                    .all(|(i, j)| chars[i].1 != chars[j].1)
            })
            .map(|chars| chars[distinct_len - 1].0)
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
    const PACKET_START_LEN: usize = 4;
    signal.find_packet_start(PACKET_START_LEN)
}

fn part_two(signal: &Signal) -> Option<usize> {
    const MESSAGE_START_LEN: usize = 14;
    signal.find_packet_start(MESSAGE_START_LEN)
}

fn main() -> Result<()> {
    let signal = read_signal()?;

    println!(
        "Part one: {}",
        part_one(&signal).ok_or_else(|| anyhow!("No start-of-packet marker found!"))?
    );

    println!(
        "Part two: {}",
        part_two(&signal).ok_or_else(|| anyhow!("No start-of-message marker found!"))?
    );

    Ok(())
}
