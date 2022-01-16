use nom::error::VerboseError;
use nom::IResult;

pub type Program = Vec<Statement>;

pub struct Statement {
    pub directive_component: DirectiveComponent,
    pub connector_components: ConnectorComponentList,
}

pub struct DirectiveComponent {
    pub directive: Directive,
    pub fields: FieldList,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Directive {
    SET,
    ADD,
    ALIAS,
    MERGE,
    IGNORE,
    RENAME,
    FILTER,
    COERCE,
    DISTINCT,
    VALIDATE,
}

pub type FieldList = Vec<Field>;

pub type Field = String;

pub type ConnectorComponentList = Vec<ConnectorComponent>;

pub struct ConnectorComponent {
    pub connector: Connector,
    pub target: TargetComponent,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Connector {
    OR,
    TO,
    TYPED,
    RESCUE,
    DEFAULT,
    MATCHING,
}

pub enum TargetComponent {
    DataValue(DataValue),
    DataType(DataType),
    Format(Format),
    Action(Action),
    Expression(Expression),
}

pub enum DataValue {
    FLOAT(f64),
    STRING(String),
    BOOLEAN(bool),
    INTEGER(isize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    FLOAT,
    STRING,
    INTEGER,
    BOOLEAN,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    URI,
    UUID,
    DATE,
    TIME,
    EMAIL,
    DATETIME,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    HALT,
    NOTIFY,
    DISCARD,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    EQUALS,
    LESSER,
    DIFFERS,
    GREATER,
    EQLESSER,
    EQGREATER,
}

pub type VResult<I, O> = IResult<I, O, VerboseError<I>>;
