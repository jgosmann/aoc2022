use std::cmp::Ordering;

use super::{base::AocSolver, error::InputParseError};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Data {
    Int(u8),
    List(Vec<Data>),
}

type ParseResult<'a, T> = Result<(T, &'a str), InputParseError>;

fn parse_pair(input: &str) -> ParseResult<(Data, Data)> {
    let (packet0, input) = parse_packet(input)?;
    let (packet1, input) = parse_packet(input)?;
    let input = skip_optional_char(input, '\n');
    Ok(((packet0, packet1), input))
}

fn parse_packet(input: &str) -> ParseResult<Data> {
    let (list, input) = parse_list(input)?;
    let input = skip_optional_char(input, '\n');
    Ok((Data::List(list), input))
}

fn parse_data(input: &str) -> ParseResult<Data> {
    if let Ok((list, input)) = parse_list(input) {
        Ok((Data::List(list), input))
    } else if let Ok((int, input)) = parse_int(input) {
        Ok((Data::Int(int), input))
    } else {
        Err(InputParseError::new("expected data".into()))
    }
}

fn parse_list(input: &str) -> ParseResult<Vec<Data>> {
    let mut list = vec![];
    let (_, mut input) = skip_char(input, '[')?;
    if let Ok((data, new_input)) = parse_data(input) {
        input = new_input;
        list.push(data);
        while let Ok(((), new_input)) = skip_char(input, ',') {
            input = new_input;
            let (data, new_input) = parse_data(input)?;
            input = new_input;
            list.push(data);
        }
    }
    let (_, input) = skip_char(input, ']')?;
    Ok((list, input))
}

fn parse_int(input: &str) -> ParseResult<u8> {
    if let Some((prefix, _)) = input.split_once(|c: char| !c.is_digit(10)) {
        Ok((prefix.parse()?, &input[prefix.len()..]))
    } else {
        Ok((input.parse()?, &input[input.len()..]))
    }
}

fn skip_char(input: &str, c: char) -> ParseResult<()> {
    if input.chars().nth(0) == Some(c) {
        Ok(((), &input[1..]))
    } else {
        Err(InputParseError::new(format!("expected {}", c)))
    }
}

fn skip_optional_char(input: &str, c: char) -> &str {
    if let Ok((_, input)) = skip_char(input, c) {
        input
    } else {
        input
    }
}

fn has_right_order(left: &Data, right: &Data) -> Ordering {
    match (left, right) {
        (Data::Int(left_value), Data::Int(right_value)) => {
            if left_value == right_value {
                Ordering::Equal
            } else if left_value < right_value {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        (Data::List(left_value), Data::List(right_value)) => {
            for i in 0..left_value.len().min(right_value.len()) {
                match has_right_order(&left_value[i], &right_value[i]) {
                    Ordering::Equal => {}
                    result => return result,
                }
            }
            has_right_order(
                &Data::Int(left_value.len() as u8),
                &Data::Int(right_value.len() as u8),
            )
        }
        (Data::Int(left_value), right) => {
            has_right_order(&Data::List(vec![Data::Int(*left_value)]), right)
        }
        (left, Data::Int(right_value)) => {
            has_right_order(left, &Data::List(vec![Data::Int(*right_value)]))
        }
    }
}

pub struct Solver {
    packet_pairs: Vec<(Data, Data)>,
}

impl<'a> AocSolver<'a, usize, usize> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut packet_pairs = vec![];
        let mut input = input;
        while let Ok((packet_pair, new_input)) = parse_pair(input) {
            input = new_input;
            packet_pairs.push(packet_pair);
        }
        Ok(Self { packet_pairs })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        Ok(self
            .packet_pairs
            .iter()
            .map(|pair| has_right_order(&pair.0, &pair.1))
            .enumerate()
            .filter_map(|(i, ordering)| match ordering {
                Ordering::Less => Some(i + 1),
                _ => None,
            })
            .sum())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        let mut packets = Vec::with_capacity(2 * self.packet_pairs.len() + 2);
        for (p0, p1) in self.packet_pairs.iter() {
            packets.push(p0);
            packets.push(p1);
        }
        let divider0 = Data::List(vec![Data::List(vec![Data::Int(2)])]);
        let divider1 = Data::List(vec![Data::List(vec![Data::Int(6)])]);
        packets.push(&divider0);
        packets.push(&divider1);

        packets.sort_by(|&a, &b| has_right_order(a, b));

        Ok(Some(
            packets
                .iter()
                .enumerate()
                .filter_map(|(i, &p)| {
                    if *p == divider0 || *p == divider1 {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .product(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day13");
        test_example_input::<Solver, _, _>(input, 13, Some(140));
    }
}
