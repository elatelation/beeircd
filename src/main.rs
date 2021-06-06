#[macro_use]
use nom::{self, IResult, sequence::preceded};
use nom::bytes::complete::{tag, take_until};

enum Verb {
    Unknown(Vec<u8>),
}

trait Message {
    fn source(&self) -> Option<&[u8]>;
    fn verb(&self) -> Verb;
    fn params(&self) -> &[&[u8]];
    fn to_bytes(&self) -> Vec<u8>;
}

struct Raw {
    source: Vec<u8>,
    verb: Vec<u8>,
    params: Vec<Vec<u8>>,
}

fn source_parser(input: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(tag(":"), take_until(" "))(input)
}

fn parse(input: &[u8]) -> nom::IResult<&[u8], Raw> {
    unimplemented!()
}

fn main() {
    println!(
        "{:?}",
        source_parser(b":irc.example.org ")
            .map(|(_, bs)| unsafe { std::str::from_utf8_unchecked(bs) })
    );
}
