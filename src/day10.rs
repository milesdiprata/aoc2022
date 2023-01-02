use std::io;

use anyhow::Result;

use day10::{Cpu, Instr};

mod day10 {
    use std::{
        io::{BufRead, Stdin},
        str::FromStr,
    };

    use anyhow::{anyhow, Error, Result};

    #[derive(Clone, Copy)]
    pub enum Instr {
        Noop,
        AddX(isize),
    }

    pub struct Cpu {
        clk: Clock,
        x: isize,
    }

    #[derive(Default)]
    struct Clock(usize);

    impl Default for Cpu {
        fn default() -> Self {
            Self {
                clk: Default::default(),
                x: 1,
            }
        }
    }

    impl FromStr for Instr {
        type Err = Error;

        fn from_str(str: &str) -> Result<Self> {
            let mut split = str.split_whitespace();

            split
                .next()
                .ok_or_else(|| anyhow!("Missing Instr!"))
                .map(|instr| match instr {
                    "noop" => Ok(Self::Noop),
                    "addx" => Ok(Self::AddX(
                        split
                            .next()
                            .ok_or_else(|| anyhow!("Missing value!"))?
                            .parse()?,
                    )),
                    _ => Err(anyhow!("Unknown Instr!")),
                })?
        }
    }

    impl Instr {
        pub fn from_stdin(stdin: Stdin) -> Result<Vec<Self>> {
            stdin
                .lock()
                .lines()
                .take_while(|line| line.is_ok() && !line.as_ref().unwrap().is_empty())
                .flatten()
                .map(|line| line.parse())
                .collect()
        }

        fn as_cycle_len(&self) -> usize {
            match self {
                Instr::Noop => 1,
                Instr::AddX(_) => 2,
            }
        }
    }

    impl Clock {
        fn tick(&mut self) -> Option<usize> {
            self.0 += 1;

            match self.0 == 20 || (self.0 > 20 && (self.0 - 20) % 40 == 0) {
                true => Some(self.0),
                _ => None,
            }
        }
    }

    impl Cpu {
        pub fn exec(&mut self, program: &[Instr]) -> isize {
            program
                .iter()
                .flat_map(|&instr| self.handle_instr(instr))
                .sum()
        }

        fn handle_instr(&mut self, instr: Instr) -> Option<isize> {
            let mut sig_strength = None;

            (0..instr.as_cycle_len()).for_each(|i| {
                if let Some(cycle) = self.clk.tick() {
                    sig_strength = Some(cycle as isize * self.x);
                }

                match instr {
                    Instr::Noop => (),
                    Instr::AddX(val) => {
                        if i == 1 {
                            self.x += val
                        }
                    }
                }
            });

            sig_strength
        }
    }
}

fn part_one(mut cpu: Cpu, program: &[Instr]) -> isize {
    cpu.exec(program)
}

fn main() -> Result<()> {
    let program = Instr::from_stdin(io::stdin())?;

    println!("Part one: {}", part_one(Cpu::default(), program.as_slice()));

    Ok(())
}
