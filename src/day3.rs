use anyhow::{anyhow, Error, Result};
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Item(char);

#[derive(Debug)]
struct Rucksack(Vec<Item>);

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        if str.len() % 2 != 0 {
            return Err(anyhow!("Rucksack does not have equal size compartments!"));
        }

        Ok(Self(str.chars().map(Item).collect()))
    }
}

impl Item {
    fn as_priority(&self) -> Option<usize> {
        if self.0.is_alphabetic() {
            let offset = if self.0.is_lowercase() {
                b'a' - 1
            } else {
                b'A' - 27
            };

            Some((self.0 as u8 - offset) as usize)
        } else {
            None
        }
    }
}

impl Rucksack {
    fn find_duplicate(&self) -> Option<Item> {
        let unique_items = self
            .0
            .chunks(self.0.len() / 2)
            .map(|compartment| compartment.iter())
            .map(|compartment| compartment.map(|item| item.0))
            .map(|compartment| compartment.collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let (first, second) = (unique_items.first()?, unique_items.last()?);

        first
            .iter()
            .flat_map(|&i| second.iter().map(move |&j| (i, j)))
            .find(|(i, j)| i == j)
            .map(|(item, _)| item)
            .map(Item)
    }
}

fn read_rucksacks() -> Result<Vec<Rucksack>> {
    let mut lines = io::stdin().lock().lines();
    let mut rucksacks = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        rucksacks.push(line.parse()?);
    }

    Ok(rucksacks)
}

fn part_one(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .iter()
        .flat_map(Rucksack::find_duplicate)
        .flat_map(|item| item.as_priority())
        .sum()
}

fn part_two(rucksacks: &[Rucksack]) -> usize {
    todo!()
}

fn main() -> Result<()> {
    let rucksacks = read_rucksacks()?;

    println!("Part one: {}", part_one(rucksacks.as_slice()));
    println!("Part two: {}", part_two(rucksacks.as_slice()));

    Ok(())
}
