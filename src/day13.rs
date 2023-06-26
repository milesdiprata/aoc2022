use std::io;

use anyhow::Result;

use day13::DistressSignal;

mod day13 {
    use std::{
        fmt,
        io::{BufRead, Stdin},
        str::FromStr,
    };

    use anyhow::{anyhow, Error, Result};

    enum PacketData {
        Integer(u8),
        List(Vec<PacketData>),
    }

    struct Packet {
        data: Vec<PacketData>,
    }

    pub struct DistressSignal {
        packets: Vec<(Packet, Packet)>,
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
            println!("Parsing {str:?}");
            if str.starts_with('[') && str.ends_with(']') {
                let mut stripped = &str[1..str.len() - 1];
                let mut list = vec![];

                while let Some((lhs, rhs)) = stripped.split_once(',') {
                    println!("split-once {lhs:?} {rhs:?}");

                    list.push(lhs.parse()?);

                    if rhs.starts_with('[') && rhs.ends_with(']') {
                        list.push(rhs.parse()?);
                        stripped = &stripped[lhs.len() + rhs.len() + 1..];
                    } else {
                        stripped = rhs;
                    }
                }

                if !stripped.is_empty() {
                    list.push(stripped.parse()?);
                }

                Ok(Self::List(list))
            } else if let Some(int) = str
                .chars()
                .next()
                .and_then(|int| int.to_digit(10))
                .map(|int| int as u8)
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
                let stripped = &str[1..str.len() - 1];
                let chars = stripped.char_indices();

                let mut data = vec![];
                let mut stack = 0;

                let mut open_idx = None;

                for (idx, char) in chars {
                    if char == '[' {
                        stack += 1;
                    } else if char == ']' {
                        stack -= 1;
                    } else if char == ',' {
                        continue;
                    } else {
                    }

                    if stack == 1 && open_idx.is_none() {
                        open_idx = Some(idx);
                    } else if stack == 0 {
                        data.push(stripped[open_idx.unwrap_or(idx)..=idx].parse()?);
                        open_idx = None;
                    } else {
                    }
                }

                Ok(Self { data })
            } else {
                Err(anyhow!("Packet is not a list!"))
            }
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
    }
}

fn main() -> Result<()> {
    let signal = DistressSignal::from_stdin(io::stdin())?;

    println!("{signal:?}");

    Ok(())
}
