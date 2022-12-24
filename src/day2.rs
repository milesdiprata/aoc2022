use anyhow::{anyhow, Error, Result};
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Loss(Shape),
    Draw(Shape),
    Win(Shape),
}

struct Round {
    player: Shape,
    opponent: Shape,
}

impl Shape {
    fn from_opponent_strategy(shape: char) -> Option<Self> {
        match shape {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn from_player_strategy(shape: char) -> Option<Self> {
        match shape {
            'X' => Some(Self::Rock),
            'Y' => Some(Self::Paper),
            'Z' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn into_score(self) -> u8 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl RoundResult {
    fn from_round(round: &Round) -> Self {
        match round {
            Round {
                opponent: Shape::Rock,
                player: Shape::Rock,
            } => Self::Draw(round.player.clone()),
            Round {
                opponent: Shape::Rock,
                player: Shape::Paper,
            } => Self::Win(round.player.clone()),
            Round {
                opponent: Shape::Rock,
                player: Shape::Scissors,
            } => Self::Loss(round.player.clone()),
            Round {
                opponent: Shape::Paper,
                player: Shape::Rock,
            } => Self::Loss(round.player.clone()),
            Round {
                opponent: Shape::Paper,
                player: Shape::Paper,
            } => Self::Draw(round.player.clone()),
            Round {
                opponent: Shape::Paper,
                player: Shape::Scissors,
            } => Self::Win(round.player.clone()),
            Round {
                opponent: Shape::Scissors,
                player: Shape::Rock,
            } => Self::Win(round.player.clone()),
            Round {
                opponent: Shape::Scissors,
                player: Shape::Paper,
            } => Self::Loss(round.player.clone()),
            Round {
                opponent: Shape::Scissors,
                player: Shape::Scissors,
            } => Self::Draw(round.player.clone()),
        }
    }

    fn into_score(self) -> u8 {
        const PLAYER_LOSS_SCORE: u8 = 0;
        const PLAYER_DRAW_SCORE: u8 = 3;
        const PLAYER_WIN_SCORE: u8 = 6;

        match self {
            Self::Loss(shape) => shape.into_score() + PLAYER_LOSS_SCORE,
            Self::Draw(shape) => shape.into_score() + PLAYER_DRAW_SCORE,
            Self::Win(shape) => shape.into_score() + PLAYER_WIN_SCORE,
        }
    }
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        let mut split = str.split_whitespace();

        let opponent_shape = Shape::from_opponent_strategy(
            split
                .next()
                .map(|str| str.chars().next())
                .ok_or_else(|| anyhow!("Missing opponent strategy!"))?
                .ok_or_else(|| anyhow!("Missing opponent strategy character!"))?,
        )
        .ok_or_else(|| anyhow!("Invalid opponent strategy!"))?;

        let player_shape = Shape::from_player_strategy(
            split
                .next()
                .map(|str| str.chars().next())
                .ok_or_else(|| anyhow!("Missing player strategy!"))?
                .ok_or_else(|| anyhow!("Missing opponent strategy character!"))?,
        )
        .ok_or_else(|| anyhow!("Invalid player strategy!"))?;

        Ok(Self {
            opponent: opponent_shape,
            player: player_shape,
        })
    }
}

fn read_strategy_guide() -> Result<Vec<Round>> {
    let mut lines = io::stdin().lock().lines();
    let mut strategy_guide = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        strategy_guide.push(line.parse()?);
    }

    Ok(strategy_guide)
}

fn part_one(strategy_guide: &[Round]) -> usize {
    strategy_guide
        .iter()
        .map(RoundResult::from_round)
        .map(|result| result.into_score())
        .map(|score| score as usize)
        .sum()
}

fn main() -> Result<()> {
    let strategy_guide = read_strategy_guide()?;

    println!("Part one: {}", part_one(strategy_guide.as_slice()));

    Ok(())
}
