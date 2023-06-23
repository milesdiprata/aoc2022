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
        crt: Crt,
        x: isize,
    }

    #[derive(Default)]
    struct Clock {
        cycle: usize,
    }

    #[derive(Default)]
    struct Crt {
        x: u8,
    }

    impl Default for Cpu {
        fn default() -> Self {
            Self {
                clk: Default::default(),
                crt: Default::default(),
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
                .take_while(|line| {
                    line.as_ref()
                        .map(|line| line.is_empty())
                        .map(|empty| !empty)
                        .unwrap_or_default()
                })
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
            self.cycle += 1;
            self.interesting_cycle()
        }

        fn interesting_cycle(&self) -> Option<usize> {
            (self.cycle == 20 || (self.cycle > 20 && (self.cycle - 20) % 40 == 0))
                .then_some(self.cycle)
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

                self.crt.draw(self.x);

                match instr {
                    Instr::AddX(val) if i == 1 => self.x += val,
                    _ => (),
                }
            });

            sig_strength
        }
    }

    impl Crt {
        fn draw(&mut self, x: isize) {
            let pixel = match [x - 1, x, x + 1]
                .iter()
                .map(|&x| x as u8)
                .any(|x| x == self.x)
            {
                true => '#',
                _ => '.',
            };

            print!("{pixel}");

            self.x += 1;
            self.x %= 40;

            if self.x == 0 {
                println!();
            }
        }
    }
}

fn part_one(mut cpu: Cpu, program: &[Instr]) -> isize {
    cpu.exec(program)
}

fn main() -> Result<()> {
    let program = Instr::from_stdin(io::stdin())?;

    println!();
    println!("Part one: {}", part_one(Cpu::default(), program.as_slice()));

    Ok(())
}
