use crate::types::{Action, Connector, DataType, Directive, Expression, Format};
use std::str::FromStr;

impl FromStr for Directive {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "set" | "SET" => Ok(Directive::SET),
            "add" | "ADD" => Ok(Directive::ADD),
            "alias" | "ALIAS" => Ok(Directive::ALIAS),
            "merge" | "MERGE" => Ok(Directive::MERGE),
            "ignore" | "IGNORE" => Ok(Directive::IGNORE),
            "rename" | "RENAME" => Ok(Directive::RENAME),
            "filter" | "FILTER" => Ok(Directive::FILTER),
            "coerce" | "COERCE" => Ok(Directive::COERCE),
            "distinct" | "DISTINCT" => Ok(Directive::DISTINCT),
            "validate" | "VALIDATE" => Ok(Directive::VALIDATE),
            _ => Err(s.to_string()),
        }
    }
}

impl FromStr for Connector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "or" | "OR" => Ok(Connector::OR),
            "to" | "TO" => Ok(Connector::TO),
            "typed" | "TYPED" => Ok(Connector::TYPED),
            "rescue" | "RESCUE" => Ok(Connector::RESCUE),
            "default" | "DEFAULT" => Ok(Connector::DEFAULT),
            "matching" | "MATCHING" => Ok(Connector::MATCHING),
            _ => Err(s.to_string()),
        }
    }
}

impl FromStr for DataType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "float" | "FLOAT" => Ok(DataType::FLOAT),
            "string" | "STRING" => Ok(DataType::STRING),
            "integer" | "INTEGER" => Ok(DataType::INTEGER),
            "boolean" | "BOOLEAN" => Ok(DataType::BOOLEAN),
            _ => Err(s.to_string()),
        }
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "uri" | "URI" => Ok(Format::URI),
            "uuid" | "UUID" => Ok(Format::UUID),
            "date" | "DATE" => Ok(Format::DATE),
            "time" | "TIME" => Ok(Format::TIME),
            "email" | "EMAIL" => Ok(Format::EMAIL),
            "datetime" | "DATETIME" => Ok(Format::DATETIME),
            _ => Err(s.to_string()),
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "halt" | "HALT" => Ok(Action::HALT),
            "notify" | "NOTIFY" => Ok(Action::NOTIFY),
            "discard" | "DISCARD" => Ok(Action::DISCARD),
            _ => Err(s.to_string()),
        }
    }
}

impl FromStr for Expression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "equals" | "EQUALS" => Ok(Expression::EQUALS),
            "lesser" | "LESSER" => Ok(Expression::LESSER),
            "differs" | "DIFFERS" => Ok(Expression::DIFFERS),
            "greater" | "GREATER" => Ok(Expression::GREATER),
            "eqlesser" | "EQLESSER" => Ok(Expression::EQLESSER),
            "eqgreater" | "EQGREATER" => Ok(Expression::EQGREATER),
            _ => Err(s.to_string()),
        }
    }
}
