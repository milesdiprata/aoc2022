use std::io;

use anyhow::Result;

use day13::DistressSignal;

mod day13 {
    use std::{
        cmp::Ordering,
        fmt,
        io::{BufRead, Stdin},
        str::FromStr,
    };

    use anyhow::{anyhow, Error, Result};

    #[derive(Clone, PartialEq)]
    enum PacketData {
        Integer(u8),
        List(Vec<PacketData>),
    }

    #[derive(Clone, PartialEq)]
    pub struct Packet {
        data: Vec<PacketData>,
    }

    #[derive(Clone)]
    pub struct DistressSignal {
        packets: Vec<(Packet, Packet)>,
    }

    fn parse<T>(str: &str) -> Result<Vec<T>>
    where
        T: FromStr + FromStr<Err = Error>,
    {
        let mut data = vec![];

        let mut data_start = None;
        let mut list_depth = 0;

        for (idx, char) in str.char_indices() {
            if char.is_ascii_digit() {
                if list_depth == 0 && data_start.is_none() {
                    data_start = Some(idx);
                }
            } else if char == ',' {
                if list_depth == 0 && data_start.is_some() {
                    data.push(str[data_start.unwrap_or(idx)..idx].parse()?);
                    data_start = None;
                }
            } else if char == '[' {
                list_depth += 1;
                if list_depth == 1 && data_start.is_none() {
                    data_start = Some(idx);
                }
            } else if char == ']' {
                list_depth -= 1;

                if list_depth == 0 {
                    data.push(str[data_start.unwrap_or(idx)..=idx].parse()?);
                    data_start = None;
                }
            } else {
            }
        }

        if let Some(start) = data_start {
            data.push(str[start..].parse()?);
        }

        Ok(data)
    }

    impl fmt::Debug for PacketData {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Integer(int) => write!(fmt, "{int:?}"),
                Self::List(list) => fmt.debug_list().entries(list.iter()).finish(),
            }
        }
    }

    impl fmt::Debug for Packet {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_list().entries(self.data.iter()).finish()
        }
    }

    impl fmt::Debug for DistressSignal {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut sep = false;

            for (first, second) in self.packets.iter() {
                if sep {
                    writeln!(fmt, "\n")?;
                }

                writeln!(fmt, "{first:?}")?;
                write!(fmt, "{second:?}")?;

                sep = true;
            }

            Ok(())
        }
    }

    impl FromStr for PacketData {
        type Err = Error;

        fn from_str(str: &str) -> Result<Self> {
            if str.starts_with('[') && str.ends_with(']') {
                parse(&str[1..str.len() - 1]).map(Self::List)
            } else if let Ok(int) = str
                .chars()
                .take_while(|char| char.is_ascii_digit())
                .collect::<String>()
                .parse()
            {
                Ok(Self::Integer(int))
            } else {
                Err(anyhow!("Unknown packet data format!"))
            }
        }
    }

    impl FromStr for Packet {
        type Err = Error;

        fn from_str(str: &str) -> Result<Self> {
            if str.starts_with('[') && str.ends_with(']') {
                parse(&str[1..str.len() - 1]).map(|data| Self { data })
            } else {
                Err(anyhow!("Packet is not a list!"))
            }
        }
    }

    impl PartialOrd for PacketData {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match (self, other) {
                (&Self::Integer(i), &Self::Integer(j)) => i < j,
                (Self::List(i), Self::List(j)) => {
                    i.iter().zip(j.iter()).any(|(i, j)| i < j)
                        || (i.iter().zip(j.iter()).all(|(i, j)| i == j) && i.len() < j.len())
                }
                (&Self::Integer(i), _) => &Self::List(vec![Self::Integer(i)]) < other,
                (_, &Self::Integer(j)) => self < &Self::List(vec![Self::Integer(j)]),
            }
            .then_some(Ordering::Less)
        }
    }

    impl PartialOrd for Packet {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            (self
                .data
                .iter()
                .zip(other.data.iter())
                .any(|(left, right)| left < right)
                || (self
                    .data
                    .iter()
                    .zip(other.data.iter())
                    .all(|(left, right)| left == right)
                    && self.data.len() < other.data.len()))
            .then_some(Ordering::Less)
        }
    }

    impl DistressSignal {
        pub fn from_stdin(stdin: Stdin) -> Result<Self> {
            const PACKET_PAIR_LINE_LEN: u8 = 2;

            let mut last_valid = true;

            let packets = stdin
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
                .chunks(PACKET_PAIR_LINE_LEN as usize)
                .map(|packets| {
                    packets
                        .iter()
                        .map(|packet| packet.as_str())
                        .map(Packet::from_str)
                        .collect::<Result<Vec<_>>>()
                })
                .collect::<Result<Vec<Vec<_>>>>()?
                .into_iter()
                .map(|packets| packets.into_iter())
                .map(|mut packets| (packets.next().unwrap(), packets.next().unwrap()))
                .collect();

            Ok(Self { packets })
        }

        pub fn packets(&self) -> &[(Packet, Packet)] {
            self.packets.as_slice()
        }
    }
}

fn part_one(signal: &DistressSignal) -> usize {
    signal
        .packets()
        .iter()
        .enumerate()
        .map(|(idx, packets)| (idx + 1, packets))
        .filter(|&(_, (i, j))| i < j)
        .map(|(idx, _)| idx)
        .sum()
}

fn main() -> Result<()> {
    let signal = DistressSignal::from_stdin(io::stdin())?;

    println!("{signal:?}");

    println!("Part one: {}", part_one(&signal));

    Ok(())
}
