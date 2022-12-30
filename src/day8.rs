use std::io;

use day8::Grid;

use anyhow::Result;

mod day8 {
    use std::collections::HashMap;
    use std::io::{BufRead, Stdin};
    use std::slice::Iter;

    use anyhow::{anyhow, Result};

    #[derive(Debug)]
    pub struct Grid {
        trees: HashMap<(usize, usize), Tree>,
        len: usize,
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct Tree {
        height: u8,
        coords: (usize, usize),
    }

    #[derive(Clone, Copy, Debug)]
    enum Direction {
        Up,
        Left,
        Right,
        Down,
    }

    impl Direction {
        fn iter() -> Iter<'static, Self> {
            const DIRECTIONS: [Direction; 4] = [
                Direction::Up,
                Direction::Left,
                Direction::Right,
                Direction::Down,
            ];

            DIRECTIONS.iter()
        }

        fn as_offset(&self) -> (isize, isize) {
            match self {
                Direction::Up => (-1, 0),
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
                Direction::Down => (1, 0),
            }
        }
    }

    impl Tree {
        fn new(height: u8, coords: (usize, usize)) -> Result<Self> {
            match height {
                0..=9 => Ok(Self { height, coords }),
                _ => Err(anyhow!("Invalid tree height!")),
            }
        }
    }

    impl Grid {
        pub fn from_stdin(stdin: Stdin) -> Result<Self> {
            let mut lines = stdin.lock().lines();
            let mut trees = HashMap::new();
            let mut i = 0;

            while let Some(Ok(line)) = lines.next() {
                if line.is_empty() {
                    break;
                }

                trees.extend(
                    line.chars()
                        .map(|char| char.to_digit(10))
                        .map(|height| height.map(|height| height as u8))
                        .map(|height| height.ok_or_else(|| anyhow!("Invalid height!")))
                        .collect::<Result<Vec<_>>>()?
                        .into_iter()
                        .enumerate()
                        .map(|(j, height)| Tree::new(height, (i, j)))
                        .collect::<Result<Vec<_>>>()?
                        .into_iter()
                        .map(|tree| (tree.coords, tree)),
                );

                i += 1;
            }

            Grid::from_trees(trees)
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_visible(&self, (i, j): (usize, usize)) -> Option<bool> {
            if [i, j]
                .iter()
                .any(|&coord| coord == 0 || coord == self.len - 1)
            {
                return Some(true);
            }

            let height = self.trees.get(&(i, j)).map(|tree| tree.height)?;
            let is_visible = Direction::iter()
                .map(|&dir| dir.as_offset())
                .map(|(i_dir, j_dir)| {
                    (1..self.len)
                        .map(|offset| offset as isize)
                        .map(move |offset| (offset * i_dir, offset * j_dir))
                        .map(|(i_offset, j_offset)| (i as isize + i_offset, j as isize + j_offset))
                        .map(|(i, j)| (i as usize, j as usize))
                        .flat_map(|coords| self.trees.get(&coords))
                        .collect::<Vec<_>>()
                })
                .any(|trees| trees.iter().all(|&tree| tree.height < height));

            Some(is_visible)
        }

        pub fn get_scenic_score(&self, (i, j): (usize, usize)) -> Option<usize> {
            if [i, j]
                .iter()
                .any(|&coord| coord == 0 || coord == self.len - 1)
            {
                return Some(0);
            }

            let height = self.trees.get(&(i, j)).map(|tree| tree.height)?;
            let scenic_score = Direction::iter()
                .map(|&dir| dir.as_offset())
                .map(|(i_dir, j_dir)| {
                    (1..self.len)
                        .map(|offset| offset as isize)
                        .map(move |offset| (offset * i_dir, offset * j_dir))
                        .map(|(i_offset, j_offset)| (i as isize + i_offset, j as isize + j_offset))
                        .map(|(i, j)| (i as usize, j as usize))
                        .flat_map(|coords| self.trees.get(&coords))
                        .collect::<Vec<_>>()
                })
                .map(|trees| {
                    trees
                        .iter()
                        .take_while(|&&tree| tree.height < height)
                        .count()
                        + match trees.iter().find(|&&tree| tree.height >= height) {
                            Some(_) => 1,
                            None => 0,
                        }
                })
                .product();

            Some(scenic_score)
        }

        fn from_trees(trees: HashMap<(usize, usize), Tree>) -> Result<Self> {
            let max_coord = trees
                .keys()
                .map(|&(i, _)| i)
                .max()
                .ok_or_else(|| anyhow!("No trees in grid!"))?;

            match max_coord == trees.keys().map(|&(_, j)| j).max().unwrap() {
                true => Ok(Grid {
                    trees,
                    len: max_coord + 1,
                }),
                false => Err(anyhow!("Uneven grid!")),
            }
        }
    }
}

fn part_one(grid: &Grid) -> usize {
    (0..grid.len())
        .flat_map(|i| (0..grid.len()).map(move |j| (i, j)))
        .flat_map(|coords| grid.is_visible(coords))
        .filter(|&is_visible| is_visible)
        .count()
}

fn part_two(grid: &Grid) -> usize {
    (0..grid.len())
        .flat_map(|i| (0..grid.len()).map(move |j| (i, j)))
        .flat_map(|coords| grid.get_scenic_score(coords))
        .max()
        .unwrap_or_default()
}

fn main() -> Result<()> {
    let grid = Grid::from_stdin(io::stdin())?;

    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));

    Ok(())
}
