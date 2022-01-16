pub type Program = Vec<Statement>;

pub struct Statement {
    pub directive_component: DirectiveComponent,
    pub connector_components: ConnectorComponentList,
}

pub struct DirectiveComponent {
    pub directive: Directive,
    pub fields: FieldList,
}

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

pub enum DataType {
    FLOAT,
    STRING,
    INTEGER,
    BOOLEAN,
}

pub enum Format {
    URI,
    UUID,
    DATE,
    TIME,
    EMAIL,
    DATETIME,
}

pub enum Action {
    HALT,
    NOTIFY,
    DISCARD,
}

pub enum Expression {
    EQUALS,
    LESSER,
    DIFFERS,
    GREATER,
    EQLESSER,
    EQGREATER,
}
