use nom::error::VerboseError;
use nom::IResult;

pub type Program<'a> = Vec<Statement<'a>>;

#[derive(Debug, PartialEq)]
pub struct Statement<'a> {
    pub directive_component: DirectiveComponent<'a>,
    pub connector_components: ConnectorComponentList,
}

#[derive(Debug, PartialEq)]
pub struct DirectiveComponent<'a> {
    pub directive: Directive,
    pub fields: FieldList<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

pub type FieldList<'a> = Vec<Field<'a>>;

pub type Field<'a> = &'a str;

pub type ConnectorComponentList = Vec<ConnectorComponent>;

#[derive(Debug, PartialEq)]
pub struct ConnectorComponent {
    pub connector: Connector,
    pub target: TargetComponent,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Connector {
    OR,
    TO,
    TYPED,
    RESCUE,
    DEFAULT,
    MATCHING,
}

#[derive(Debug, PartialEq)]
pub enum TargetComponent {
    DataValue(DataValue),
    DataType(DataType),
    Format(Format),
    Action(Action),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum DataValue {
    Float(Float),
    Text(Text),
    Boolean(Boolean),
    Integer(Integer),
}

pub type Float = f64;
pub type Text = String;
pub type Boolean = bool;
pub type Integer = isize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType {
    FLOAT,
    STRING,
    INTEGER,
    BOOLEAN,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Format {
    URI,
    UUID,
    DATE,
    TIME,
    EMAIL,
    DATETIME,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    HALT,
    NOTIFY,
    DISCARD,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    EQUALS,
    LESSER,
    DIFFERS,
    GREATER,
    EQLESSER,
    EQGREATER,
}

pub type VResult<I, O> = IResult<I, O, VerboseError<I>>;
