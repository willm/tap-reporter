use std::io::prelude::*;
use std::io::{self};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut parser = TapParser::new(&lines.next());
    for line in lines {
        parser.line(&Some(line));
    }
    Ok(())
}

struct Test {
    name: String,
    assertion: String,
    pass: bool,
}

struct TestBuilder {
    test: Test,
}

impl TestBuilder {
    fn new() -> Self {
        TestBuilder {
            test: Test {
                name: String::from(""),
                assertion: String::from(""),
                pass: false,
            },
        }
    }

    fn with_name(&mut self, name: &str) -> &Self {
        self.test.name = String::from(name);
        self
    }

    fn with_result(&mut self, pass: bool, assertion: String) -> &Self {
        self.test.pass = pass;
        self.test.assertion = assertion;
        self
    }

    fn build(&self) -> &Test {
        &self.test
    }
}

struct TapParser {
    tests: Vec<TestBuilder>,
}

impl TapParser {
    fn new(input: &Option<io::Result<String>>) -> Self {
        let tap_header = "TAP version 13";
        if let Some(first_line) = input {
            if let Ok(line) = first_line {
                if tap_header == line {
                    return TapParser { tests: vec![] };
                }
            }
        }
        panic!("Invalid Tap input, must start with '{}'", tap_header);
    }

    fn line(&mut self, input: &Option<io::Result<String>>) {
        if let Some(first_line) = input {
            if let Ok(line) = first_line {
                if line.starts_with("# ") {
                    if let Some(test_name) = line.get(2..) {
                        let mut builder = TestBuilder::new();
                        builder.with_name(test_name.clone());

                        self.tests.push(builder);
                    }
                }
                if let Some(builder) = self.tests.last_mut() {
                    if line.starts_with("ok ") {
                        if let Some(msg) = take_line_from_word(&line, 2) {
                            builder.with_result(true, msg.to_string());
                        }
                    }
                    if line.starts_with("not ok ") {
                        if let Some(msg) = take_line_from_word(&line, 3) {
                            builder.with_result(false, msg.to_string());
                        }
                    }
                }
            }
        }
    }
}
fn take_line_from_word(line: &str, word: usize) -> Option<String> {
    match line.split(" ").collect::<Vec<&str>>().get(word..) {
        Some(msg) => Some(msg.join(" ")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TAP_HEADER: &str = "TAP version 13";

    #[test]
    fn test_a_valid_header_line() {
        TapParser::new(&Some(Ok(String::from(TAP_HEADER))));
    }

    #[test]
    fn test_a_single_passing_test() {
        let mut parser = TapParser::new(&Some(Ok(String::from(TAP_HEADER))));
        parser.line(&Some(Ok(String::from("# the happy path"))));
        parser.line(&Some(Ok(String::from("ok 1 should be equal"))));

        assert_eq!(parser.tests.len(), 1);
        let test = parser.tests.last().unwrap().build();
        assert_eq!(test.name, "the happy path");
        assert_eq!(test.assertion, "should be equal");
        assert_eq!(test.pass, true);
    }

    #[test]
    fn test_a_single_failing_test() {
        let mut parser = TapParser::new(&Some(Ok(String::from(TAP_HEADER))));
        parser.line(&Some(Ok(String::from("# the happy path"))));
        parser.line(&Some(Ok(String::from("not ok 2 should be equivalent"))));

        assert_eq!(parser.tests.len(), 1);
        let test = parser.tests.last().unwrap().build();
        assert_eq!(test.name, "the happy path");
        assert_eq!(test.assertion, "should be equivalent");
        assert_eq!(test.pass, false);
    }

    #[test]
    #[should_panic]
    fn test_an_invalid_header_line() {
        TapParser::new(&mut Some(Ok(String::from("invalid"))));
    }
}
