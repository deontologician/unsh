use anyhow::{anyhow, Context, Result};
use multi::separated_list1;
use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, char, multispace0},
    combinator::{map, value},
    error::{ParseError, VerboseError},
    multi::{self, many1},
    sequence::{delimited, pair, preceded, separated_pair},
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Path<'a>(pub Vec<Segment<'a>>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment<'a> {
    Atom(&'a str),
    Multiple(Vec<Path<'a>>),
    Wildcard,
}

type IResult<'a, T> = nom::IResult<&'a str, T, VerboseError<&'a str>>;

pub fn import<'a>(i: &'a str) -> Result<Path<'a>> {
    separated_pair(tag(":use"), multispace0, path)(i)
        .map_err(|e| anyhow!("Failed to parse path: {:#?}", e))
        .map(|(_, (_, path))| path)
}

pub fn path<'a>(i: &'a str) -> IResult<'a, Path<'a>> {
    map(separated_list1(tag("::"), segment), Path)(i)
}

pub fn segment<'a>(i: &'a str) -> IResult<'a, Segment<'a>> {
    alt((value(Segment::Wildcard, tag("*")), atom, multiple))(i)
}

pub fn atom<'a>(i: &'a str) -> IResult<'a, Segment<'a>> {
    map(complete::alphanumeric1, Segment::Atom)(i)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> nom::IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn multiple<'a>(i: &'a str) -> IResult<'a, Segment<'a>> {
    map(
        delimited(
            ws(char('{')),
            separated_list1(ws(char(',')), path),
            ws(char('}')),
        ),
        Segment::Multiple,
    )(i)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        let input_str = "usr::{bin::{less,tail}, local::bin::*}";
        let expected = Path(vec![
            Segment::Atom("usr"),
            Segment::Multiple(vec![
                Path(vec![
                    Segment::Atom("bin"),
                    Segment::Multiple(vec![
                        Path(vec![Segment::Atom("less")]),
                        Path(vec![Segment::Atom("tail")]),
                    ]),
                ]),
                Path(vec![
                    Segment::Atom("local"),
                    Segment::Atom("bin"),
                    Segment::Wildcard,
                ]),
            ]),
        ]);
        let result = import(input_str).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_2() {
        let input_str = "usr::bin::{env, ls};";
        let expected = Path(vec![
            Segment::Atom("usr"),
            Segment::Atom("bin"),
            Segment::Multiple(vec![
                Path(vec![Segment::Atom("env")]),
                Path(vec![Segment::Atom("ls")]),
            ]),
        ]);
        let result = import(input_str).unwrap();
        assert_eq!(result, expected)
    }
}
