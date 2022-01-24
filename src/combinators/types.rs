use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::character::is_alphanumeric;
use nom::combinator::{map, opt, recognize};
use nom::error::{context, ErrorKind};
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::{AsChar, InputTakeAtPosition};

use crate::combinators::contexts::Contexts;
use crate::types::{Boolean, Float, Integer, Text, VResult};

pub fn text(input: &str) -> VResult<&str, Text> {
    context(Contexts::Text.into(), map(textchars1, Text::from))(input)
}

pub fn boolean(input: &str) -> VResult<&str, Boolean> {
    context(
        Contexts::Boolean.into(),
        map(alt((tag("true"), tag("false"))), |result: &str| {
            result.parse().unwrap()
        }),
    )(input)
}

pub fn float(input: &str) -> VResult<&str, Float> {
    context(
        Contexts::Float.into(),
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
        ),
    )(input)
}

pub fn integer(input: &str) -> VResult<&str, Integer> {
    context(
        Contexts::Integer.into(),
        map(decimal, |number: &str| {
            number.replace("_", "").parse().unwrap()
        }),
    )(input)
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
