use dftp::combinators::types::*;
use dftp::combinators::*;
use dftp::types::*;

#[test]
fn program_test() {
    assert_eq!(
        program("SET fa TO true;"),
        Ok((
            "",
            vec![Statement {
                directive_component: DirectiveComponent {
                    directive: Directive::SET,
                    fields: vec!["fa"]
                },
                connector_components: vec![ConnectorComponent {
                    connector: Connector::TO,
                    target: TargetComponent::DataValue(DataValue::Boolean(true))
                }]
            }]
        ))
    );
    assert_eq!(
        program("SET fa TO true \t ;"),
        Ok((
            "",
            vec![Statement {
                directive_component: DirectiveComponent {
                    directive: Directive::SET,
                    fields: vec!["fa"]
                },
                connector_components: vec![ConnectorComponent {
                    connector: Connector::TO,
                    target: TargetComponent::DataValue(DataValue::Boolean(true))
                }]
            }]
        ))
    );
    assert_eq!(
        program("SET fa TO true; SET fb TO false;"),
        Ok((
            "",
            vec![
                Statement {
                    directive_component: DirectiveComponent {
                        directive: Directive::SET,
                        fields: vec!["fa"]
                    },
                    connector_components: vec![ConnectorComponent {
                        connector: Connector::TO,
                        target: TargetComponent::DataValue(DataValue::Boolean(true))
                    },]
                },
                Statement {
                    directive_component: DirectiveComponent {
                        directive: Directive::SET,
                        fields: vec!["fb"]
                    },
                    connector_components: vec![ConnectorComponent {
                        connector: Connector::TO,
                        target: TargetComponent::DataValue(DataValue::Boolean(false))
                    },]
                }
            ]
        ))
    );
    assert_eq!(
        program(
            "SET fa TO true;
                 SET fb TO false;"
        ),
        Ok((
            "",
            vec![
                Statement {
                    directive_component: DirectiveComponent {
                        directive: Directive::SET,
                        fields: vec!["fa"]
                    },
                    connector_components: vec![ConnectorComponent {
                        connector: Connector::TO,
                        target: TargetComponent::DataValue(DataValue::Boolean(true))
                    },]
                },
                Statement {
                    directive_component: DirectiveComponent {
                        directive: Directive::SET,
                        fields: vec!["fb"]
                    },
                    connector_components: vec![ConnectorComponent {
                        connector: Connector::TO,
                        target: TargetComponent::DataValue(DataValue::Boolean(false))
                    },]
                }
            ]
        ))
    );
}

#[test]
fn statement_test() {
    assert_eq!(
        statement("SET fa TO true;"),
        Ok((
            "",
            Statement {
                directive_component: DirectiveComponent {
                    directive: Directive::SET,
                    fields: vec!["fa"]
                },
                connector_components: vec![ConnectorComponent {
                    connector: Connector::TO,
                    target: TargetComponent::DataValue(DataValue::Boolean(true))
                }]
            }
        ))
    );
    assert_eq!(
        statement("SET fa TO true;"),
        Ok((
            "",
            Statement {
                directive_component: DirectiveComponent {
                    directive: Directive::SET,
                    fields: vec!["fa"]
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
                    fields: vec!["fa"]
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
                fields: vec!["fa"]
            }
        ))
    );
    assert_eq!(
        directive_component("SET  \t\t\t  fa"),
        Ok((
            "",
            DirectiveComponent {
                directive: Directive::SET,
                fields: vec!["fa"]
            }
        ))
    );
    assert_eq!(
        directive_component("SET fa,fb"),
        Ok((
            "",
            DirectiveComponent {
                directive: Directive::SET,
                fields: vec!["fa", "fb"]
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
    assert_eq!(field_list("fa"), Ok(("", vec!["fa"])));
    assert_eq!(field_list("fa,fb"), Ok(("", vec!["fa", "fb"])));
    assert_eq!(field_list("fa, fb"), Ok(("", vec!["fa", "fb"])));
    assert_eq!(field_list("fa ,fb"), Ok((" ,fb", vec!["fa"])));
}

#[test]
fn field_test() {
    assert!(field("").is_err());
    assert_eq!(field("fiel?"), Ok(("?", "fiel")));
    assert_eq!(field("field"), Ok(("", "field")));
    assert_eq!(field("nested.field"), Ok(("", "nested.field")));
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

#[test]
fn text_test() {
    assert_eq!(text("aA1."), Ok(("", "aA1.".to_string())));
    assert_eq!(text("aA1.>"), Ok((">", "aA1.".to_string())));
    assert!(text(">").is_err());
}

#[test]
fn boolean_test() {
    assert!(boolean("1").is_err());
    assert!(boolean("t").is_err());
    assert!(boolean("TRUE").is_err());
    assert_eq!(boolean("true"), Ok(("", true)));
    assert!(boolean("0").is_err());
    assert!(boolean("f").is_err());
    assert!(boolean("FALSE").is_err());
    assert_eq!(boolean("false"), Ok(("", false)));
}

#[test]
fn float_test() {
    assert_eq!(float(".1"), Ok(("", 0.1)));
    assert_eq!(float("-.1"), Ok(("", -0.1)));
    assert_eq!(float(".42e42"), Ok(("", 0.42e42)));
    assert_eq!(float("-.42e42"), Ok(("", -0.42e42)));
    assert_eq!(float(".42e-42"), Ok(("", 0.42e-42)));
    assert_eq!(float("-.42e-42"), Ok(("", -0.42e-42)));
    assert_eq!(float("1.1"), Ok(("", 1.1)));
    assert_eq!(float("-1.1"), Ok(("", -1.1)));
    assert_eq!(float("1.1e42"), Ok(("", 1.1e42)));
    assert_eq!(float("1.1e-42"), Ok(("", 1.1e-42)));
    assert_eq!(float("-1.1e42"), Ok(("", -1.1e42)));
    assert_eq!(float("-1.1e-42"), Ok(("", -1.1e-42)));
    assert_eq!(float("42e42"), Ok(("", 42e42)));
    assert_eq!(float("42e-42"), Ok(("", 42e-42)));
    assert_eq!(float("-42e42"), Ok(("", -42e42)));
    assert_eq!(float("-42e-42"), Ok(("", -42e-42)));
    assert_eq!(float("42.42e42"), Ok(("", 42.42e42)));
    assert_eq!(float("42.42e-42"), Ok(("", 42.42e-42)));
    assert_eq!(float("-42.42e42"), Ok(("", -42.42e42)));
    assert_eq!(float("-42.42e-42"), Ok(("", -42.42e-42)));
}

#[test]
fn integer_test() {
    assert_eq!(integer("1"), Ok(("", 1)));
    assert_eq!(integer("1_000"), Ok(("", 1_000)));
    assert_eq!(integer("1_0_0___0"), Ok(("", 1_000)));
}
