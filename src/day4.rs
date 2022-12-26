use anyhow::{anyhow, Error, Result};
use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct SectionAssignment {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct ElfPair {
    first: SectionAssignment,
    second: SectionAssignment,
}

impl FromStr for SectionAssignment {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        let mut split = str.split('-');

        let start = usize::from_str(
            split
                .next()
                .ok_or_else(|| anyhow!("Missing first section ID!"))?,
        )?;

        let end = usize::from_str(
            split
                .next()
                .ok_or_else(|| anyhow!("Missing last section ID!"))?,
        )?;

        Ok(Self { start, end })
    }
}

impl FromStr for ElfPair {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        let mut split = str.split(',');

        let first = split
            .next()
            .ok_or_else(|| anyhow!("Missing first section assignment!"))?
            .parse()?;

        let second = split
            .next()
            .ok_or_else(|| anyhow!("Missing second section assignment!"))?
            .parse()?;

        Ok(Self { first, second })
    }
}

impl SectionAssignment {
    fn as_range(&self) -> RangeInclusive<usize> {
        self.start..=self.end
    }
}

impl ElfPair {
    fn any_complete_overlap(&self) -> bool {
        self.first
            .as_range()
            .all(|assignment| self.second.as_range().contains(&assignment))
            || self
                .second
                .as_range()
                .all(|assignment| self.first.as_range().contains(&assignment))
    }

    fn any_overlap(&self) -> bool {
        self.first
            .as_range()
            .any(|assignment| self.second.as_range().contains(&assignment))
            || self
                .second
                .as_range()
                .any(|assignment| self.first.as_range().contains(&assignment))
    }
}

fn read_elf_pairs() -> Result<Vec<ElfPair>> {
    let mut lines = io::stdin().lock().lines();
    let mut pairs = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        pairs.push(line.parse()?);
    }

    Ok(pairs)
}

fn part_one(pairs: &[ElfPair]) -> usize {
    pairs
        .iter()
        .filter(|&pair| pair.any_complete_overlap())
        .count()
}

fn part_two(pairs: &[ElfPair]) -> usize {
    pairs.iter().filter(|&pair| pair.any_overlap()).count()
}

fn main() -> Result<()> {
    let pairs = read_elf_pairs()?;

    println!("Part one: {}", part_one(pairs.as_slice()));
    println!("Part two: {}", part_two(pairs.as_slice()));

    Ok(())
}
