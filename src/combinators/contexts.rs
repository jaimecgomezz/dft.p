#[derive(Debug)]
pub enum Contexts {
    Program,
    Statement,
    DirectiveComponent,
    ConnectorComponentList,
    FieldList,
    Field,
    ConnectorComponent,
    TargetComponent,
    DataValue,
    Directive,
    Connector,
    DataType,
    Format,
    Action,
    Expression,
    Text,
    Boolean,
    Float,
    Integer,
}

impl Into<&str> for Contexts {
    fn into(self) -> &'static str {
        match self {
            Contexts::Program => "Program",
            Contexts::Statement => "Statement",
            Contexts::DirectiveComponent => "DirectiveComponent",
            Contexts::ConnectorComponentList => "ConnectorComponentList",
            Contexts::FieldList => "FieldList",
            Contexts::Field => "Field",
            Contexts::ConnectorComponent => "ConnectorComponent",
            Contexts::TargetComponent => "TargetComponent",
            Contexts::DataValue => "DataValue",
            Contexts::Directive => "Directive",
            Contexts::Connector => "Connector",
            Contexts::DataType => "DataType",
            Contexts::Format => "Format",
            Contexts::Action => "Action",
            Contexts::Expression => "Expression",
            Contexts::Text => "Text",
            Contexts::Boolean => "Boolean",
            Contexts::Float => "Float",
            Contexts::Integer => "Integer",
        }
    }
}
