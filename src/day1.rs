use anyhow::{anyhow, Result};
use std::{
    collections::BinaryHeap,
    io::{self, BufRead},
};

fn read_cals() -> Result<Vec<Vec<usize>>> {
    let mut lines = io::stdin().lock().lines();
    let mut elf_cals = vec![];
    let mut cals = vec![];
    let mut last_empty = false;

    while let Some(Ok(line)) = lines.next() {
        let empty = line.is_empty();

        if empty {
            if last_empty {
                break;
            } else {
                elf_cals.push(cals);
                cals = vec![];
            }

            last_empty = true;
        } else {
            cals.push(line.parse::<usize>()?);
            last_empty = false;
        }
    }

    Ok(elf_cals)
}

fn part_one(food_cals: &[Vec<usize>]) -> Option<usize> {
    food_cals
        .iter()
        .map(|food_cal| food_cal.iter().sum::<usize>())
        .max()
}

fn part_two(food_cals: &[Vec<usize>]) -> Option<usize> {
    const MAX_CALS_LEN: usize = 3;

    let mut cals = food_cals
        .iter()
        .map(|food_cal| food_cal.iter().sum::<usize>())
        .collect::<BinaryHeap<_>>();

    (0..MAX_CALS_LEN).map(move |_| cals.pop()).sum()
}

fn main() -> Result<()> {
    let food_cals = read_cals()?;

    println!(
        "Part one: {}",
        part_one(food_cals.as_slice()).ok_or_else(|| anyhow!("No calories given!"))?
    );
    println!(
        "Part two: {}",
        part_two(food_cals.as_slice()).ok_or_else(|| anyhow!("No calories given!"))?
    );

    Ok(())
}
