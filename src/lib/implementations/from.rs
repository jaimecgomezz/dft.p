use crate::types::{Action, Connector, DataType, Directive, Expression, Format};

impl From<&str> for Directive {
    fn from(s: &str) -> Self {
        match s {
            "set" | "SET" => Directive::SET,
            "add" | "ADD" => Directive::ADD,
            "alias" | "ALIAS" => Directive::ALIAS,
            "merge" | "MERGE" => Directive::MERGE,
            "ignore" | "IGNORE" => Directive::IGNORE,
            "rename" | "RENAME" => Directive::RENAME,
            "filter" | "FILTER" => Directive::FILTER,
            "coerce" | "COERCE" => Directive::COERCE,
            "distinct" | "DISTINCT" => Directive::DISTINCT,
            "validate" | "VALIDATE" => Directive::VALIDATE,
            _ => panic!("Not implemented: {s}"),
        }
    }
}

impl From<&str> for Connector {
    fn from(s: &str) -> Self {
        match s {
            "or" | "OR" => Connector::OR,
            "to" | "TO" => Connector::TO,
            "typed" | "TYPED" => Connector::TYPED,
            "rescue" | "RESCUE" => Connector::RESCUE,
            "default" | "DEFAULT" => Connector::DEFAULT,
            "matching" | "MATCHING" => Connector::MATCHING,
            _ => panic!("Not implemented: {s}"),
        }
    }
}

impl From<&str> for DataType {
    fn from(s: &str) -> Self {
        match s {
            "float" | "FLOAT" => DataType::FLOAT,
            "string" | "STRING" => DataType::STRING,
            "integer" | "INTEGER" => DataType::INTEGER,
            "boolean" | "BOOLEAN" => DataType::BOOLEAN,
            _ => panic!("Not implemented: {s}"),
        }
    }
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s {
            "uri" | "URI" => Format::URI,
            "uuid" | "UUID" => Format::UUID,
            "date" | "DATE" => Format::DATE,
            "time" | "TIME" => Format::TIME,
            "email" | "EMAIL" => Format::EMAIL,
            "datetime" | "DATETIME" => Format::DATETIME,
            _ => panic!("Not implemented: {s}"),
        }
    }
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s {
            "halt" | "HALT" => Action::HALT,
            "notify" | "NOTIFY" => Action::NOTIFY,
            "discard" | "DISCARD" => Action::DISCARD,
            _ => panic!("Not implemented: {s}"),
        }
    }
}

impl From<&str> for Expression {
    fn from(s: &str) -> Self {
        match s {
            "equals" | "EQUALS" => Expression::EQUALS,
            "lesser" | "LESSER" => Expression::LESSER,
            "differs" | "DIFFERS" => Expression::DIFFERS,
            "greater" | "GREATER" => Expression::GREATER,
            "eqlesser" | "EQLESSER" => Expression::EQLESSER,
            "eqgreater" | "EQGREATER" => Expression::EQGREATER,
            _ => panic!("Not implemented: {s}"),
        }
    }
}
