mod types;

use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{char, newline, space1};
use nom::combinator::map;
use nom::combinator::value;
use nom::multi::many0;
use nom::sequence::{pair, preceded, terminated};

use super::types::*;
use types::*;

// TODO:
//  - create a `list` combinator to unify *_list combinators

pub fn statement(input: &str) -> VResult<&str, Statement> {
    map(
        terminated(
            pair(
                directive_component,
                preceded(space1, connector_component_list),
            ),
            alt((char(';'), newline)),
        ),
        |(dcomp, clist): (DirectiveComponent, ConnectorComponentList)| Statement {
            directive_component: dcomp,
            connector_components: clist,
        },
    )(input)
}

pub fn directive_component(input: &str) -> VResult<&str, DirectiveComponent> {
    map(
        pair(directive, preceded(space1, field_list)),
        |(dir, flist): (Directive, FieldList)| DirectiveComponent {
            directive: dir,
            fields: flist,
        },
    )(input)
}

pub fn connector_component_list(input: &str) -> VResult<&str, ConnectorComponentList> {
    map(
        pair(
            connector_component,
            many0(preceded(space1, connector_component)),
        ),
        |(first, rest): (ConnectorComponent, Vec<ConnectorComponent>)| {
            let mut result = vec![first];

            for item in rest {
                result.push(item);
            }

            return result;
        },
    )(input)
}

pub fn field_list(input: &str) -> VResult<&str, FieldList> {
    map(
        pair(field, many0(preceded(tag(","), field))),
        |(first, rest): (Field, Vec<Field>)| {
            let mut result = vec![first];

            for item in rest {
                result.push(item);
            }

            return result;
        },
    )(input)
}

pub fn field(input: &str) -> VResult<&str, Field> {
    map(textchars1, |result: &str| result.into())(input)
}

pub fn connector_component(input: &str) -> VResult<&str, ConnectorComponent> {
    map(
        pair(connector, preceded(space1, target_component)),
        |(con, tar): (Connector, TargetComponent)| ConnectorComponent {
            connector: con,
            target: tar,
        },
    )(input)
}

pub fn target_component(input: &str) -> VResult<&str, TargetComponent> {
    alt((
        map(format, |result: Format| TargetComponent::Format(result)),
        map(action, |result: Action| TargetComponent::Action(result)),
        map(expression, |result: Expression| {
            TargetComponent::Expression(result)
        }),
        map(data_type, |result: DataType| {
            TargetComponent::DataType(result)
        }),
        map(data_value, |result: DataValue| {
            TargetComponent::DataValue(result)
        }),
    ))(input)
}

pub fn data_value(input: &str) -> VResult<&str, DataValue> {
    alt((
        map(float, |result: Float| DataValue::Float(result)),
        map(integer, |result: Integer| DataValue::Integer(result)),
        map(boolean, |result: Boolean| DataValue::Boolean(result)),
        map(text, |result: Text| DataValue::Text(result)),
    ))(input)
}

pub fn directive(input: &str) -> VResult<&str, Directive> {
    alt((
        value(Directive::SET, tag_no_case("set")),
        value(Directive::ADD, tag_no_case("add")),
        value(Directive::ALIAS, tag_no_case("alias")),
        value(Directive::MERGE, tag_no_case("merge")),
        value(Directive::IGNORE, tag_no_case("ignore")),
        value(Directive::RENAME, tag_no_case("rename")),
        value(Directive::FILTER, tag_no_case("filter")),
        value(Directive::COERCE, tag_no_case("coerce")),
        value(Directive::DISTINCT, tag_no_case("distinct")),
        value(Directive::VALIDATE, tag_no_case("validate")),
    ))(input)
}

pub fn connector(input: &str) -> VResult<&str, Connector> {
    alt((
        value(Connector::OR, tag_no_case("or")),
        value(Connector::TO, tag_no_case("to")),
        value(Connector::TYPED, tag_no_case("typed")),
        value(Connector::RESCUE, tag_no_case("rescue")),
        value(Connector::DEFAULT, tag_no_case("default")),
        value(Connector::MATCHING, tag_no_case("matching")),
    ))(input)
}

pub fn data_type(input: &str) -> VResult<&str, DataType> {
    alt((
        value(DataType::FLOAT, tag_no_case("float")),
        value(DataType::STRING, tag_no_case("string")),
        value(DataType::INTEGER, tag_no_case("integer")),
        value(DataType::BOOLEAN, tag_no_case("boolean")),
    ))(input)
}

pub fn format(input: &str) -> VResult<&str, Format> {
    alt((
        value(Format::URI, tag_no_case("uri")),
        value(Format::UUID, tag_no_case("uuid")),
        value(Format::DATE, tag_no_case("date")),
        value(Format::TIME, tag_no_case("time")),
        value(Format::EMAIL, tag_no_case("email")),
        value(Format::DATETIME, tag_no_case("datetime")),
    ))(input)
}

pub fn action(input: &str) -> VResult<&str, Action> {
    alt((
        value(Action::HALT, tag_no_case("halt")),
        value(Action::NOTIFY, tag_no_case("notify")),
        value(Action::DISCARD, tag_no_case("discard")),
    ))(input)
}

pub fn expression(input: &str) -> VResult<&str, Expression> {
    alt((
        value(Expression::EQUALS, tag_no_case("equals")),
        value(Expression::LESSER, tag_no_case("lesser")),
        value(Expression::DIFFERS, tag_no_case("differs")),
        value(Expression::GREATER, tag_no_case("greater")),
        value(Expression::EQLESSER, tag_no_case("eqlesser")),
        value(Expression::EQGREATER, tag_no_case("eqgreater")),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statement_test() {
        assert_eq!(
            statement("SET fa TO true;"),
            Ok((
                "",
                Statement {
                    directive_component: DirectiveComponent {
                        directive: Directive::SET,
                        fields: vec!["fa".to_string()]
                    },
                    connector_components: vec![ConnectorComponent {
                        connector: Connector::TO,
                        target: TargetComponent::DataValue(DataValue::Boolean(true))
                    }]
                }
            ))
        );
        assert_eq!(
            statement("SET fa TO true\n"),
            Ok((
                "",
                Statement {
                    directive_component: DirectiveComponent {
                        directive: Directive::SET,
                        fields: vec!["fa".to_string()]
                    },
                    connector_components: vec![ConnectorComponent {
                        connector: Connector::TO,
                        target: TargetComponent::DataValue(DataValue::Boolean(true))
                    }]
                }
            ))
        );
        assert_eq!(
            statement("SET fa  \t\t\t  TO true;"),
            Ok((
                "",
                Statement {
                    directive_component: DirectiveComponent {
                        directive: Directive::SET,
                        fields: vec!["fa".to_string()]
                    },
                    connector_components: vec![ConnectorComponent {
                        connector: Connector::TO,
                        target: TargetComponent::DataValue(DataValue::Boolean(true))
                    }]
                }
            ))
        );
    }

    #[test]
    fn directive_component_test() {
        assert!(directive_component("SET ").is_err());
        assert_eq!(
            directive_component("SET fa"),
            Ok((
                "",
                DirectiveComponent {
                    directive: Directive::SET,
                    fields: vec!["fa".to_string()]
                }
            ))
        );
        assert_eq!(
            directive_component("SET  \t\t\t  fa"),
            Ok((
                "",
                DirectiveComponent {
                    directive: Directive::SET,
                    fields: vec!["fa".to_string()]
                }
            ))
        );
        assert_eq!(
            directive_component("SET fa,fb"),
            Ok((
                "",
                DirectiveComponent {
                    directive: Directive::SET,
                    fields: vec!["fa".to_string(), "fb".to_string()]
                }
            ))
        )
    }

    #[test]
    fn connector_component_list_test() {
        assert_eq!(
            connector_component_list("MATCHING true"),
            Ok((
                "",
                vec![ConnectorComponent {
                    connector: Connector::MATCHING,
                    target: TargetComponent::DataValue(DataValue::Boolean(true))
                }]
            ))
        );
        assert_eq!(
            connector_component_list("MATCHING true MATCHING false"),
            Ok((
                "",
                vec![
                    ConnectorComponent {
                        connector: Connector::MATCHING,
                        target: TargetComponent::DataValue(DataValue::Boolean(true))
                    },
                    ConnectorComponent {
                        connector: Connector::MATCHING,
                        target: TargetComponent::DataValue(DataValue::Boolean(false))
                    }
                ]
            ))
        );
        assert_eq!(
            connector_component_list("MATCHING true \t\t\t  MATCHING false"),
            Ok((
                "",
                vec![
                    ConnectorComponent {
                        connector: Connector::MATCHING,
                        target: TargetComponent::DataValue(DataValue::Boolean(true))
                    },
                    ConnectorComponent {
                        connector: Connector::MATCHING,
                        target: TargetComponent::DataValue(DataValue::Boolean(false))
                    }
                ]
            ))
        );
        assert_eq!(
            connector_component_list("MATCHING true,MATCHING false"),
            Ok((
                ",MATCHING false",
                vec![ConnectorComponent {
                    connector: Connector::MATCHING,
                    target: TargetComponent::DataValue(DataValue::Boolean(true))
                }]
            ))
        );
    }

    #[test]
    fn field_list_test() {
        assert_eq!(field_list("fa"), Ok(("", vec!["fa".to_string()])));
        assert_eq!(
            field_list("fa,fb"),
            Ok(("", vec!["fa".to_string(), "fb".to_string()]))
        );
        assert_eq!(field_list("fa ,fb"), Ok((" ,fb", vec!["fa".to_string()])));
        assert_eq!(field_list("fa, fb"), Ok((", fb", vec!["fa".to_string()])));
    }

    #[test]
    fn field_test() {
        assert!(field("").is_err());
        assert_eq!(field("fiel?"), Ok(("?", "fiel".to_string())));
        assert_eq!(field("field"), Ok(("", "field".to_string())));
        assert_eq!(field("nested.field"), Ok(("", "nested.field".to_string())));
    }

    #[test]
    fn connector_component_test() {
        assert!(connector_component("MATCHING\ntrue").is_err());
        assert_eq!(
            connector_component("MATCHING true"),
            Ok((
                "",
                ConnectorComponent {
                    connector: Connector::MATCHING,
                    target: TargetComponent::DataValue(DataValue::Boolean(true))
                }
            ))
        );
        assert_eq!(
            connector_component("MATCHING     true"),
            Ok((
                "",
                ConnectorComponent {
                    connector: Connector::MATCHING,
                    target: TargetComponent::DataValue(DataValue::Boolean(true))
                }
            ))
        );
        assert_eq!(
            connector_component("MATCHING    \t\t\t   true"),
            Ok((
                "",
                ConnectorComponent {
                    connector: Connector::MATCHING,
                    target: TargetComponent::DataValue(DataValue::Boolean(true))
                }
            ))
        );
    }

    #[test]
    fn target_component_test() {
        assert_eq!(
            target_component("UUID"),
            Ok(("", TargetComponent::Format(Format::UUID)))
        );
        assert_eq!(
            target_component("HALT"),
            Ok(("", TargetComponent::Action(Action::HALT)))
        );
        assert_eq!(
            target_component("EQUALS"),
            Ok(("", TargetComponent::Expression(Expression::EQUALS)))
        );
        assert_eq!(
            target_component("BOOLEAN"),
            Ok(("", TargetComponent::DataType(DataType::BOOLEAN)))
        );
        assert_eq!(
            target_component("1.0"),
            Ok(("", TargetComponent::DataValue(DataValue::Float(1.0))))
        );
    }

    #[test]
    fn data_value_test() {
        assert_eq!(data_value("1"), Ok(("", DataValue::Integer(1))));
        assert_eq!(data_value("1.0"), Ok(("", DataValue::Float(1.0))));
        assert_eq!(data_value("-1.0"), Ok(("", DataValue::Float(-1.0))));
        assert_eq!(data_value("true"), Ok(("", DataValue::Boolean(true))));
        assert_eq!(data_value("false"), Ok(("", DataValue::Boolean(false))));
        assert_eq!(data_value("a"), Ok(("", DataValue::Text("a".to_string()))));
    }
}
