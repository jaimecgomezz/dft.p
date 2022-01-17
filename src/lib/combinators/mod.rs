mod literals;
mod types;

use nom::branch::alt;
use nom::combinator::map;

use crate::types::{Boolean, DataValue, Float, Integer, Text, VResult};
use types::*;

pub fn data_value(input: &str) -> VResult<&str, DataValue> {
    alt((
        map(float, |result: Float| DataValue::Float(result)),
        map(integer, |result: Integer| DataValue::Integer(result)),
        map(boolean, |result: Boolean| DataValue::Boolean(result)),
        map(text, |result: Text| DataValue::Text(result)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_value_test() {
        assert_eq!(
            data_value("text"),
            Ok(("", DataValue::Text("text".to_string())))
        );
        assert_eq!(data_value("true"), Ok(("", DataValue::Boolean(true))));
        assert_eq!(data_value("false"), Ok(("", DataValue::Boolean(false))));
        assert_eq!(data_value("1.0"), Ok(("", DataValue::Float(1.0))));
        assert_eq!(data_value("-1.0"), Ok(("", DataValue::Float(-1.0))));
        assert_eq!(data_value("1"), Ok(("", DataValue::Integer(1))));
    }
}
