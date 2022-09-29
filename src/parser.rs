use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Error {
    Nom,
}

impl nom::error::ParseError<&str> for Error {
    fn from_error_kind(_input: &str, _kind: ErrorKind) -> Self {
        Error::Nom
    }

    fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GitUrl {
    pub domain: String,
    pub organisation: String,
}

fn parse_complex_string(input: &str) -> IResult<&str, String, Error> {
    let (remain, tokens) = many1(alt((alphanumeric1, tag("."), tag("_"), tag("-"))))(input)?;
    let domain = tokens.join("");
    Ok((remain, domain))
}

pub fn parse_git(input: &str) -> IResult<String, GitUrl, Error> {
    let (remain, (_, domain, _, organisation)) = tuple((
        tag("git@"),
        parse_complex_string,
        tag(":"),
        parse_complex_string,
    ))(input)?;

    Ok((
        remain.to_string(),
        GitUrl {
            domain,
            organisation,
        },
    ))
}
