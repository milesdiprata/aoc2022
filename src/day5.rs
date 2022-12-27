use anyhow::{anyhow, Result};
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Crate(char);

#[derive(Debug, Clone)]
struct Stack(Vec<Crate>);

#[derive(Debug)]
struct CrateMover9000;

#[derive(Debug)]
struct CrateMover9001;

#[derive(Debug)]
struct Rearrangement {
    stack_len: usize,
    source: usize,
    destination: usize,
}

impl CrateMover9000 {
    fn rearrange(stacks: &mut [Stack], rearrangement: &Rearrangement) -> Option<()> {
        (0..rearrangement.stack_len)
            .map(|_| {
                stacks
                    .get_mut(rearrangement.source - 1)
                    .map(|stack| stack.0.pop())
            })
            .collect::<Option<Option<Vec<_>>>>()??
            .into_iter()
            .map(|item| {
                stacks
                    .get_mut(rearrangement.destination - 1)
                    .map(|stack| stack.0.push(item))
            })
            .collect()
    }
}

impl CrateMover9001 {
    fn rearrange(stacks: &mut [Stack], rearrangement: &Rearrangement) -> Option<()> {
        let source_len = stacks.get(rearrangement.source - 1)?.0.len();

        stacks
            .get_mut(rearrangement.source - 1)?
            .0
            .drain(source_len - rearrangement.stack_len..)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|item| {
                stacks
                    .get_mut(rearrangement.destination - 1)
                    .map(|stack| stack.0.push(item))
            })
            .collect()
    }
}

fn read_stacks() -> Result<Vec<Stack>> {
    let mut lines = io::stdin().lock().lines();
    let mut rows = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut row = Vec::new();

        line.char_indices().for_each(|(idx, char)| {
            if ('A'..='Z').contains(&char) {
                row.push(Some(char));
            } else if idx % 2 != 0 && (idx + 1) % 4 != 0 {
                row.push(None);
            }
        });

        if row.iter().any(|char| char.is_some()) {
            rows.push(row);
        }
    }

    let stacks = (0..rows
        .first()
        .ok_or_else(|| anyhow!("No stacks given!"))?
        .len())
        .rev()
        .map(|i| {
            rows.iter()
                .rev()
                .filter_map(|inner| inner[i])
                .map(Crate)
                .collect()
        })
        .rev()
        .map(Stack)
        .collect();

    Ok(stacks)
}

fn read_rearrangements() -> Result<Vec<Rearrangement>> {
    let mut lines = io::stdin().lock().lines();
    let mut rearrangements = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut split = line.split_whitespace().flat_map(|str| str.parse::<usize>());

        let stack_len = split
            .next()
            .ok_or_else(|| anyhow!("No stack length given!"))?;

        let source = split
            .next()
            .ok_or_else(|| anyhow!("No source stack given!"))?;

        let destination = split
            .next()
            .ok_or_else(|| anyhow!("No destination stack given!"))?;

        rearrangements.push(Rearrangement {
            stack_len,
            source,
            destination,
        })
    }

    Ok(rearrangements)
}

fn top_crates(stacks: &[Stack]) -> Option<String> {
    stacks
        .iter()
        .map(|stack| stack.0.last())
        .map(|item| item.map(|item| item.0))
        .collect()
}

fn part_one(stacks: &mut [Stack], rearrangements: &[Rearrangement]) -> Option<String> {
    rearrangements
        .iter()
        .map(|rearrangement| CrateMover9000::rearrange(stacks, rearrangement))
        .collect::<Option<_>>()?;

    top_crates(stacks)
}

fn part_two(stacks: &mut [Stack], rearrangements: &[Rearrangement]) -> Option<String> {
    rearrangements
        .iter()
        .map(|rearrangement| CrateMover9001::rearrange(stacks, rearrangement))
        .collect::<Option<_>>()?;

    top_crates(stacks)
}

fn main() -> Result<()> {
    let mut stacks = read_stacks()?;
    let rearrangements = read_rearrangements()?;

    println!(
        "Part one: {}",
        part_one(stacks.clone().as_mut_slice(), rearrangements.as_slice())
            .ok_or_else(|| anyhow!("No stacks given!"))?
    );

    println!(
        "Part two: {}",
        part_two(stacks.as_mut_slice(), rearrangements.as_slice())
            .ok_or_else(|| anyhow!("No stacks given!"))?
    );

    Ok(())
}
