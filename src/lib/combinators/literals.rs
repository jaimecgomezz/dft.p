use nom::bytes::complete::tag_no_case;
use nom::combinator::map;

use crate::types::{Action, DataType, Directive, Expression, Format, VResult};

macro_rules! literal {
    ($name:ident, $result:ty) => {
        pub fn $name(input: &str) -> VResult<&str, $result> {
            map(tag_no_case(stringify!($name)), |result: &str| result.into())(input)
        }
    };
}

// Connector
literal!(or, Directive);
literal!(to, Directive);
literal!(typed, Directive);
literal!(rescue, Directive);
literal!(default, Directive);
literal!(matching, Directive);

// DataType
literal!(float, DataType);
literal!(string, DataType);
literal!(integer, DataType);
literal!(boolean, DataType);

// Format
literal!(uri, Format);
literal!(uuid, Format);
literal!(date, Format);
literal!(time, Format);
literal!(email, Format);
literal!(datetime, Format);

// Action
literal!(halt, Action);
literal!(notify, Action);
literal!(discard, Action);

// Expression
literal!(equals, Expression);
literal!(lesser, Expression);
literal!(differs, Expression);
literal!(greater, Expression);
literal!(eqlesser, Expression);
literal!(eqgreater, Expression);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Example {
        A,
    }

    impl From<&str> for Example {
        fn from(s: &str) -> Self {
            match s {
                "a" | "A" => Example::A,
                _ => panic!("Error"),
            }
        }
    }

    literal!(a, Example);

    #[test]
    fn test_macro() {
        assert_eq!(a("a"), Ok(("", Example::A)));
        assert_eq!(a("A"), Ok(("", Example::A)));
        assert!(a("b").is_err());
        assert!(a("B").is_err());
    }
}
