use dftp::Parser;

#[test]
fn from_string_test() {
    assert_eq!(
        Parser::from(String::new()),
        Parser {
            source: String::new(),
            program: None
        }
    );
}

#[test]
fn new_test() {
    assert_eq!(
        Parser::new(),
        Parser {
            source: String::new(),
            program: None
        }
    )
}
