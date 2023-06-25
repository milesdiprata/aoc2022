use std::io;

use anyhow::Result;

use day12::HeatMap;

mod day12 {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::fmt;
    use std::io::{BufRead, Stdin};

    use anyhow::{anyhow, Result};

    pub struct HeatMap {
        grid: Vec<Vec<char>>,
        len: usize,
        end: (usize, usize),
    }

    impl fmt::Debug for HeatMap {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.grid
                .iter()
                .try_for_each(|row| writeln!(fmt, "{}", row.iter().collect::<String>()))
        }
    }

    impl HeatMap {
        pub fn from_stdin(stdin: Stdin) -> Result<Self> {
            let grid = stdin
                .lock()
                .lines()
                .take_while(|line| {
                    line.as_ref()
                        .map(|line| line.is_empty())
                        .map(|empty| !empty)
                        .unwrap_or_default()
                })
                .map(|line| line.map(|line| line.chars().collect()))
                .map(|line| line.map_err(|err| anyhow!(err)))
                .collect::<Result<Vec<Vec<_>>>>()?;

            let len = grid
                .first()
                .map(|row| row.len())
                .ok_or_else(|| anyhow!("Empty grid!"))?;

            let end = (0..len)
                .flat_map(|i| (0..len).map(move |j| (i, j)))
                .find(|&(i, j)| {
                    grid.get(i)
                        .and_then(|row| row.get(j))
                        .map(|&elevation| elevation == 'E')
                        .unwrap_or_default()
                })
                .ok_or_else(|| anyhow!("Missing end position!"))?;

            Ok(Self { grid, len, end })
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn get(&self, (i, j): (usize, usize)) -> Option<char> {
            self.grid.get(i).and_then(|row| row.get(j)).copied()
        }

        pub fn find_path(&self, start: (usize, usize)) -> Option<VecDeque<(usize, usize)>> {
            // TODO(milesdiprata): Use priority-queue
            let mut open_set = HashSet::new();
            let mut came_from = HashMap::new();

            let mut g_scores = HashMap::new();
            let mut f_scores = HashMap::new();

            open_set.insert(start);

            g_scores.insert(start, 0);
            f_scores.insert(start, self.h_score(start));

            while !open_set.is_empty() {
                let current = open_set
                    .iter()
                    .find(|&coord| {
                        f_scores.get(coord)
                            == open_set.iter().flat_map(|coord| f_scores.get(coord)).min()
                    })
                    .copied()
                    .unwrap_or_else(|| unreachable!("Open-set cannot be empty"));

                if current == self.end {
                    return Self::reconstruct_path(&came_from, current);
                }

                open_set.remove(&current);

                self.neighbors(current).into_iter().for_each(|neighbor| {
                    let tentative_g_score =
                        g_scores.get(&current).copied().unwrap_or(usize::MAX) + 1;

                    if tentative_g_score < g_scores.get(&neighbor).copied().unwrap_or(usize::MAX) {
                        if !open_set.contains(&neighbor) {
                            open_set.insert(neighbor);
                        }

                        came_from.insert(neighbor, current);

                        g_scores.insert(neighbor, tentative_g_score);
                        f_scores.insert(neighbor, tentative_g_score + self.h_score(neighbor));
                    }
                });
            }

            None
        }

        fn get_elevation(&self, coord: (usize, usize)) -> Option<char> {
            self.get(coord).map(|elevation| match elevation {
                'S' => 'a',
                'E' => 'z',
                elevation => elevation,
            })
        }

        fn neighbors(&self, (i, j): (usize, usize)) -> Vec<(usize, usize)> {
            [
                (i as isize - 1, j as isize),
                (i as isize + 1, j as isize),
                (i as isize, j as isize - 1),
                (i as isize, j as isize + 1),
            ]
            .into_iter()
            .filter(|&(i, _)| i >= 0)
            .filter(|&(_, j)| j >= 0)
            .map(|(i, j)| (i as usize, j as usize))
            .filter(|&neighbor| self.get(neighbor).is_some())
            .filter(|&neighbor| {
                self.get_elevation(neighbor)
                    .map(|elevation| elevation as i8)
                    .unwrap_or_default()
                    - self
                        .get_elevation((i, j))
                        .map(|elevation| elevation as i8)
                        .unwrap_or_default()
                    <= 1
            })
            .collect()
        }

        fn h_score(&self, coord: (usize, usize)) -> usize {
            Self::euclidean_distance(coord, self.end)
        }

        fn euclidean_distance(p: (usize, usize), q: (usize, usize)) -> usize {
            ((q.0 as f64 - p.0 as f64).powi(2) + (q.1 as f64 - p.1 as f64).powi(2))
                .sqrt()
                .round() as usize
        }

        fn reconstruct_path(
            came_from: &HashMap<(usize, usize), (usize, usize)>,
            mut current: (usize, usize),
        ) -> Option<VecDeque<(usize, usize)>> {
            let mut path = VecDeque::from([current]);

            while came_from.contains_key(&current) {
                current = came_from.get(&current).copied()?;
                path.push_front(current);
            }

            Some(path)
        }
    }
}

fn part_one(heat_map: &HeatMap) -> usize {
    (0..heat_map.len())
        .flat_map(|i| (0..heat_map.len()).map(move |j| (i, j)))
        .find(|&(i, j)| {
            heat_map
                .get((i, j))
                .map(|elevation| elevation == 'S')
                .unwrap_or_default()
        })
        .and_then(|start| heat_map.find_path(start))
        .map(|path| path.len())
        .map(|path_len| path_len - 1)
        .unwrap_or_default()
}

fn part_two(heat_map: &HeatMap) -> usize {
    (0..heat_map.len())
        .flat_map(|i| (0..heat_map.len()).map(move |j| (i, j)))
        .filter(|&(i, j)| {
            heat_map
                .get((i, j))
                .map(|elevation| elevation == 'S' || elevation == 'a')
                .unwrap_or_default()
        })
        .flat_map(|start| heat_map.find_path(start))
        .map(|path| path.len())
        .map(|path_len| path_len - 1)
        .min()
        .unwrap_or_default()
}

fn main() -> Result<()> {
    let heat_map = HeatMap::from_stdin(io::stdin())?;

    println!("Part one: {}", part_one(&heat_map));
    println!("Part two: {}", part_two(&heat_map));

    Ok(())
}
