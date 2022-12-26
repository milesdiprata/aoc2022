use anyhow::{anyhow, Error, Result};
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Item(char);

#[derive(Debug)]
struct Rucksack(Vec<Item>);

#[derive(Debug)]
struct Group<'a>([&'a Rucksack; 3]);

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

        unique_items[0]
            .iter()
            .flat_map(|&i| unique_items[1].iter().map(move |&j| (i, j)))
            .find(|(i, j)| i == j)
            .map(|(item, _)| item)
            .map(Item)
    }
}

impl Group<'_> {
    fn find_badge(&self) -> Option<Item> {
        let unique_items = self
            .0
            .map(|rucksack| rucksack.0.iter())
            .map(|items| items.map(|item| item.0))
            .map(|items| items.collect::<HashSet<_>>());

        unique_items[0]
            .iter()
            .flat_map(|&i| unique_items[1].iter().map(move |&j| (i, j)))
            .flat_map(|(i, j)| unique_items[2].iter().map(move |&k| (i, j, k)))
            .find(|(i, j, k)| i == j && i == k)
            .map(|(item, _, _)| item)
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
    rucksacks
        .chunks(3)
        .map(|group| [&group[0], &group[1], &group[2]])
        .map(Group)
        .flat_map(|group| group.find_badge())
        .flat_map(|badge| badge.as_priority())
        .sum()
}

fn main() -> Result<()> {
    let rucksacks = read_rucksacks()?;

    println!("Part one: {}", part_one(rucksacks.as_slice()));
    println!("Part two: {}", part_two(rucksacks.as_slice()));

    Ok(())
}
