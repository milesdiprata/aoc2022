use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::io;

use anyhow::Result;

use day11::Monkey;

mod day11 {
    use std::{
        io::{BufRead, Stdin},
        str::FromStr,
    };

    use anyhow::{anyhow, Error, Result};

    #[derive(Debug)]
    enum Operation {
        Add((Option<usize>, Option<usize>)),
        Multiply((Option<usize>, Option<usize>)),
    }

    #[derive(Debug)]
    struct Test {
        condition: usize,
        test_pass_result: usize,
        test_fail_result: usize,
    }

    #[derive(Debug)]
    pub struct Monkey {
        pub items: Vec<usize>,
        operation: Operation,
        test: Test,
    }

    impl FromStr for Operation {
        type Err = Error;

        fn from_str(str: &str) -> Result<Self> {
            let mut operation = str.split_at("Operation: new = ".len()).1.split_whitespace();

            let lhs = match operation
                .next()
                .ok_or_else(|| anyhow!("Missing LHS of operation!"))?
            {
                "old" => Ok(None),
                lhs => usize::from_str(lhs)
                    .map(Some)
                    .map_err(|_| anyhow!("Invalid LHS of operation!")),
            }?;

            let operator = operation
                .next()
                .map(|operator| operator.chars())
                .and_then(|mut operator| operator.next())
                .ok_or_else(|| anyhow!("Missing LHS of operation!"))?;

            let rhs = match operation
                .next()
                .ok_or_else(|| anyhow!("Missing RHS of operation!"))?
            {
                "old" => Ok(None),
                rhs => usize::from_str(rhs)
                    .map(Some)
                    .map_err(|_| anyhow!("Invalid RHS of operation!")),
            }?;

            match operator {
                '+' => Ok(Operation::Add((lhs, rhs))),
                '*' => Ok(Operation::Multiply((lhs, rhs))),
                _ => Err(anyhow!("Invalid operator in expression!")),
            }
        }
    }

    impl Operation {
        fn evaluate(&self, old: usize) -> usize {
            match *self {
                Operation::Add((lhs, rhs)) => lhs.unwrap_or(old) + rhs.unwrap_or(old),
                Operation::Multiply((lhs, rhs)) => lhs.unwrap_or(old) * rhs.unwrap_or(old),
            }
        }
    }

    impl Test {
        fn from_lines(lines: &[&str]) -> Result<Self> {
            let mut lines = lines.iter();

            let condition = lines
                .next()
                .map(|&condition| condition.split_whitespace())
                .and_then(|condition| condition.last())
                .map(usize::from_str)
                .ok_or_else(|| anyhow!("Missing condition in test!"))??;

            let test_pass_result = lines
                .next()
                .map(|&result| result.split_whitespace())
                .and_then(|result| result.last())
                .map(usize::from_str)
                .ok_or_else(|| anyhow!("Missing true-result in test!"))??;

            let test_fail_result = lines
                .next()
                .map(|&result| result.split_whitespace())
                .and_then(|result| result.last())
                .map(usize::from_str)
                .ok_or_else(|| anyhow!("Missing false-result in test!"))??;

            Ok(Test {
                condition,
                test_pass_result,
                test_fail_result,
            })
        }

        fn evaluate(&self, item: usize) -> usize {
            if item % self.condition == 0 {
                self.test_pass_result
            } else {
                self.test_fail_result
            }
        }
    }

    impl Monkey {
        pub fn from_stdin(stdin: Stdin) -> Result<Vec<Self>> {
            const MONKEY_LINE_LEN: u8 = 6;

            let mut last_valid = true;

            stdin
                .lock()
                .lines()
                .map(|line| {
                    (
                        line.as_ref()
                            .map(|line| line.is_empty())
                            .map(|empty| !empty)
                            .unwrap_or_default(),
                        line,
                    )
                })
                .take_while(|(valid, _)| {
                    let take = *valid || last_valid;
                    last_valid = *valid;
                    take
                })
                .map(|(_, line)| line.map_err(|err| anyhow!("{err}")))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .chunks(MONKEY_LINE_LEN as usize)
                .map(Monkey::from_lines)
                .collect()
        }

        pub fn take_turn(&mut self) -> Vec<(usize, usize)> {
            self.evaluate();
            self.inspect();
            self.throw()
        }

        pub fn catch(&mut self, item: usize) {
            self.items.push(item)
        }

        fn from_lines(lines: &[String]) -> Result<Self> {
            let mut lines = lines.iter();

            let id = lines
                .next()
                .map(|id| id.chars())
                .and_then(|mut id| id.nth("Monkey ".len()))
                .map(|id| id.to_digit(10))
                .map(|id| id.ok_or_else(|| anyhow!("Invalid monkey ID!")))
                .map(|id| id.map(|id| id as usize))
                .ok_or_else(|| anyhow!("Missing id for monkey!"))??;

            let items = lines
                .next()
                .map(|items| items.trim_start())
                .map(|items| items.split_at("Starting items: ".len()))
                .map(|items| items.1)
                .map(|items| items.split(", "))
                .map(|items| items.map(usize::from_str))
                .map(|items| items.map(|item| item.map_err(|err| anyhow!(err))))
                .map(|items| items.collect::<Result<_>>())
                .ok_or_else(|| anyhow!("Missing starting items for monkey {id}!"))??;

            let operation = lines
                .next()
                .map(|operation| operation.trim_start())
                .map(Operation::from_str)
                .ok_or_else(|| anyhow!("Missing operation for monkey {id}!"))??;

            let test = [lines.next(), lines.next(), lines.next()]
                .into_iter()
                .map(|test| test.map(|test| test.trim_start()))
                .collect::<Option<Vec<_>>>()
                .as_deref()
                .map(Test::from_lines)
                .ok_or_else(|| anyhow!("Missing test for monkey {id}!"))??;

            Ok(Self {
                items,
                operation,
                test,
            })
        }

        fn evaluate(&mut self) {
            self.items
                .iter_mut()
                .for_each(|item| *item = self.operation.evaluate(*item));
        }

        fn inspect(&mut self) {
            self.items.iter_mut().for_each(|item| *item /= 3);
        }

        fn throw(&mut self) -> Vec<(usize, usize)> {
            let thrown_items = self
                .items
                .iter()
                .cloned()
                .map(|item| (self.test.evaluate(item), item))
                .collect();

            self.items = vec![];

            thrown_items
        }
    }
}

fn part_one(monkeys: &[RefCell<Monkey>]) -> usize {
    const ROUND_LEN: u8 = 20;
    const ACTIVE_MONKEY_LEN: u8 = 2;

    let mut inspection_lens = (0..monkeys.len()).map(|_| 0usize).collect::<Vec<_>>();

    (0..ROUND_LEN).for_each(|i| {
        monkeys.iter().enumerate().for_each(|(sender, monkey)| {
            monkey
                .borrow_mut()
                .take_turn()
                .into_iter()
                .for_each(|(recipient, thrown_item)| {
                    *inspection_lens
                        .get_mut(sender)
                        .unwrap_or_else(|| unreachable!()) += 1;

                    monkeys
                        .get(recipient)
                        .unwrap_or_else(|| unreachable!())
                        .borrow_mut()
                        .catch(thrown_item);
                })
        });

        println!("after round {}", i + 1);
        println!(
            "{:?}",
            monkeys
                .iter()
                .map(|monkey| monkey.borrow().items.clone())
                .collect::<Vec<_>>()
        );
        println!();
    });

    inspection_lens
        .into_iter()
        .collect::<BinaryHeap<_>>()
        .into_iter()
        .take(ACTIVE_MONKEY_LEN as usize)
        .product()
}

fn main() -> Result<()> {
    let monkeys = Monkey::from_stdin(io::stdin())?
        .into_iter()
        .map(RefCell::new)
        .collect::<Vec<_>>();

    println!("Part one: {}", part_one(monkeys.as_slice()));

    Ok(())
}
