pub mod types;

use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{char, multispace0, space0, space1};
use nom::combinator::map;
use nom::combinator::value;
use nom::multi::many0;
use nom::sequence::{pair, preceded, terminated};

use super::types::*;
use types::*;

pub fn program(input: &str) -> VResult<&str, Program> {
    map(
        pair(statement, many0(preceded(multispace0, statement))),
        |(first, rest): (Statement, Program)| {
            let mut result = vec![first];

            for item in rest {
                result.push(item);
            }

            result
        },
    )(input)
}

pub fn statement(input: &str) -> VResult<&str, Statement> {
    map(
        terminated(
            pair(
                directive_component,
                preceded(space1, connector_component_list),
            ),
            preceded(space0, char(';')),
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
        |(first, rest): (ConnectorComponent, ConnectorComponentList)| {
            let mut result = vec![first];

            for item in rest {
                result.push(item);
            }

            result
        },
    )(input)
}

pub fn field_list(input: &str) -> VResult<&str, FieldList> {
    map(
        pair(field, many0(preceded(tag(","), preceded(space0, field)))),
        |(first, rest): (Field, FieldList)| {
            let mut result = vec![first];

            for item in rest {
                result.push(item);
            }

            result
        },
    )(input)
}

pub fn field(input: &str) -> VResult<&str, Field> {
    textchars1(input)
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
        map(format, TargetComponent::Format),
        map(action, TargetComponent::Action),
        map(expression, TargetComponent::Expression),
        map(data_type, TargetComponent::DataType),
        map(data_value, TargetComponent::DataValue),
    ))(input)
}

pub fn data_value(input: &str) -> VResult<&str, DataValue> {
    alt((
        map(float, DataValue::Float),
        map(integer, DataValue::Integer),
        map(boolean, DataValue::Boolean),
        map(text, DataValue::Text),
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
