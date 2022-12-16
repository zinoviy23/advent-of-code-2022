use crate::packets::PacketElement::{Integer, List};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{map_res, recognize};
use nom::error::Error;
use nom::multi::{many1, separated_list0};
use nom::{Finish, IResult};
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketElement {
    Integer(u32),
    List(Vec<PacketElement>),
}

impl FromStr for PacketElement {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s).finish() {
            Ok((_remaining, packet)) => Ok(packet),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl PartialOrd for PacketElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Integer(i1), Integer(i2)) => i1.partial_cmp(i2),
            (List(elements1), List(elements2)) => elements1.partial_cmp(elements2),
            (Integer(i1), List(elements2)) => vec![Integer(*i1)].partial_cmp(elements2),
            (List(elements1), Integer(i2)) => elements1.partial_cmp(&vec![Integer(*i2)]),
        }
    }
}

impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_integer(input: &str) -> IResult<&str, PacketElement> {
    map_res(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse::<u32>().map(|i| Integer(i))
    })(input)
}

fn parse(input: &str) -> IResult<&str, PacketElement> {
    parse_list(input)
}

fn parse_list(input: &str) -> IResult<&str, PacketElement> {
    let (input, _) = char('[')(input)?;

    let (input, result) = separated_list0(tag(","), alt((parse_integer, parse_list)))(input)?;

    let (input, _) = char(']')(input)?;
    Ok((input, List(result)))
}

#[cfg(test)]
mod tests {
    use crate::packets::PacketElement::{Integer, List};
    use crate::packets::{parse_integer, parse_list, PacketElement};
    use std::cmp::Ordering::Greater;

    #[test]
    fn test_integer() {
        let result = parse_integer("123");
        assert_eq!(result, Ok(("", Integer(123))))
    }

    #[test]
    fn test_integer_list_parse() {
        let result = parse_list("[1,2,3]");
        assert_eq!(
            result,
            Ok(("", List(vec![Integer(1), Integer(2), Integer(3)])))
        );
    }

    #[test]
    fn test_list_of_empty_list() {
        let result = parse_list("[[]]");
        assert_eq!(result, Ok(("", List(vec![List(vec![])]))));
    }

    #[test]
    fn test_cmp() {
        let first: PacketElement = "[9]".parse().unwrap();
        let second: PacketElement = "[[8,7,6]]".parse().unwrap();
        assert_eq!(first.partial_cmp(&second), Some(Greater))
    }

    #[test]
    fn test_cmp_list_with_integers() {
        let first: PacketElement = "[7,7,7,7]".parse().unwrap();
        let second: PacketElement = "[[6]]".parse().unwrap();

        assert_eq!(first.cmp(&second), Greater);
    }
}
