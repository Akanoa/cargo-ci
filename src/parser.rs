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
pub(crate) struct GitUrl {
    pub domain: String,
    pub organisation: String,
}

fn parse_complex_string(input: &str) -> IResult<&str, String, Error> {
    let (remain, tokens) = many1(alt((alphanumeric1, tag("."), tag("_"), tag("-"))))(input)?;
    let domain = tokens.join("");
    Ok((remain, domain))
}

pub(crate) fn parse_git(input: &str) -> IResult<String, GitUrl, Error> {
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

#[cfg(test)]
mod tests {
    use crate::parse_git;
    use crate::parser::GitUrl;

    #[test]
    fn parse_git_url() {
        let project_name = "project22_zefsdfgdfg_ff4-FHGF_55";
        let organisation = "4155fd.dGHFHsf_dsgsd-4245DJFVH";
        let domain = "4155fd.dGHFHsf_dsgsd-4245DJFVH.my_gitlab.co.uk";

        let url = format!("git@{domain}:{organisation}/{project_name}.git");

        let result = parse_git(url.as_str());

        assert_eq!(
            result,
            Ok((
                format!("/{project_name}.git"),
                GitUrl {
                    domain: domain.to_string(),
                    organisation: organisation.to_string()
                }
            ))
        )
    }
}
