use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::character::is_alphanumeric;
use nom::combinator::{map, opt, recognize};
use nom::error::ErrorKind;
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::{AsChar, InputTakeAtPosition};

use crate::types::{Boolean, Float, Integer, Text, VResult};

pub fn text(input: &str) -> VResult<&str, Text> {
    map(textchars1, |result: &str| Text::from(result))(input)
}

pub fn boolean(input: &str) -> VResult<&str, Boolean> {
    map(alt((tag("true"), tag("false"))), |result: &str| {
        result.parse().unwrap()
    })(input)
}

pub fn float(input: &str) -> VResult<&str, Float> {
    map(
        pair(
            opt(one_of("+-")),
            alt((
                recognize(tuple((
                    char('.'),
                    decimal,
                    opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
                ))),
                recognize(tuple((
                    decimal,
                    opt(preceded(char('.'), decimal)),
                    one_of("eE"),
                    opt(one_of("+-")),
                    decimal,
                ))),
                recognize(tuple((decimal, char('.'), opt(decimal)))),
            )),
        ),
        |(sign, number): (Option<char>, &str)| match sign {
            Some(s) => format!("{s}{number}").as_str().parse().unwrap(),
            None => number.parse().unwrap(),
        },
    )(input)
}

pub fn integer(input: &str) -> VResult<&str, Integer> {
    map(decimal, |number: &str| {
        number.replace("_", "").parse().unwrap()
    })(input)
}

pub fn textchars1<T>(input: T) -> VResult<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| is_textchar(item.as_char() as u8),
        ErrorKind::AlphaNumeric,
    )
}

fn is_textchar(input: u8) -> bool {
    !(is_alphanumeric(input) || input == b'.')
}

fn decimal(input: &str) -> VResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_test() {
        assert_eq!(text("aA1."), Ok(("", "aA1.".to_string())));
        assert_eq!(text("aA1.>"), Ok((">", "aA1.".to_string())));
        assert!(text(">").is_err());
    }

    #[test]
    fn boolean_test() {
        assert!(boolean("1").is_err());
        assert!(boolean("t").is_err());
        assert!(boolean("TRUE").is_err());
        assert_eq!(boolean("true"), Ok(("", true)));
        assert!(boolean("0").is_err());
        assert!(boolean("f").is_err());
        assert!(boolean("FALSE").is_err());
        assert_eq!(boolean("false"), Ok(("", false)));
    }

    #[test]
    fn float_test() {
        assert_eq!(float(".1"), Ok(("", 0.1)));
        assert_eq!(float("-.1"), Ok(("", -0.1)));
        assert_eq!(float(".42e42"), Ok(("", 0.42e42)));
        assert_eq!(float("-.42e42"), Ok(("", -0.42e42)));
        assert_eq!(float(".42e-42"), Ok(("", 0.42e-42)));
        assert_eq!(float("-.42e-42"), Ok(("", -0.42e-42)));
        assert_eq!(float("1.1"), Ok(("", 1.1)));
        assert_eq!(float("-1.1"), Ok(("", -1.1)));
        assert_eq!(float("1.1e42"), Ok(("", 1.1e42)));
        assert_eq!(float("1.1e-42"), Ok(("", 1.1e-42)));
        assert_eq!(float("-1.1e42"), Ok(("", -1.1e42)));
        assert_eq!(float("-1.1e-42"), Ok(("", -1.1e-42)));
        assert_eq!(float("42e42"), Ok(("", 42e42)));
        assert_eq!(float("42e-42"), Ok(("", 42e-42)));
        assert_eq!(float("-42e42"), Ok(("", -42e42)));
        assert_eq!(float("-42e-42"), Ok(("", -42e-42)));
        assert_eq!(float("42.42e42"), Ok(("", 42.42e42)));
        assert_eq!(float("42.42e-42"), Ok(("", 42.42e-42)));
        assert_eq!(float("-42.42e42"), Ok(("", -42.42e42)));
        assert_eq!(float("-42.42e-42"), Ok(("", -42.42e-42)));
    }

    #[test]
    fn integer_test() {
        assert_eq!(integer("1"), Ok(("", 1)));
        assert_eq!(integer("1_000"), Ok(("", 1_000)));
        assert_eq!(integer("1_0_0___0"), Ok(("", 1_000)));
    }
}
