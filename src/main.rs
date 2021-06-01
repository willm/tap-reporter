use std::io;
use std::io::prelude::*;

use tap_reporter::formatters::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut parser = TapParser::new(&lines.next(), SpecFormatter {});
    for line in lines {
        parser.line(&Some(line));
    }
    parser.finalise();
    Ok(())
}

struct Test {
    name: String,
    assertions: Vec<Assertion>,
    pass: bool,
}

#[derive(Clone)]
struct Assertion {
    message: String,
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
                assertions: vec![],
                pass: false,
            },
        }
    }

    fn with_name(&mut self, name: &str) -> &Self {
        self.test.name = String::from(name);
        self
    }

    fn with_assertion(&mut self, pass: bool, assertion: String) -> &Self {
        self.test.assertions.push(Assertion {
            pass,
            message: assertion,
        });
        self
    }

    fn build(&self) -> &Test {
        &self.test
    }
}

struct TapParser<T>
where
    T: TestFormat,
{
    tests: Vec<TestBuilder>,
    plan: Option<(i32, i32)>,
    formatter: T,
}

impl<T: TestFormat> TapParser<T> {
    fn new(input: &Option<io::Result<String>>, formatter: T) -> Self {
        let tap_header = "TAP version 13";
        if let Some(first_line) = input {
            if let Ok(line) = first_line {
                if tap_header == line {
                    return TapParser {
                        tests: vec![],
                        formatter,
                        plan: None,
                    };
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
                        self.formatter.new_test(test_name);
                        builder.with_name(test_name.clone());

                        self.tests.push(builder);
                    }
                } else if let Some(builder) = self.tests.last_mut() {
                    if line.starts_with("ok ") {
                        let assertion = take_line_from_word(&line, 2);
                        self.formatter.assertion(true, &assertion);
                        builder.with_assertion(true, assertion);
                    } else if line.starts_with("not ok ") {
                        let assertion = take_line_from_word(&line, 3);
                        self.formatter.assertion(false, &assertion);
                        builder.with_assertion(false, assertion);
                    } else if let Some(plan) = parse_test_plan(line) {
                        self.plan = Some(plan.clone());
                    } else {
                        self.formatter.log_output(line);
                    }
                }
            }
        }
    }

    fn finalise(&self) {
        self.formatter.summerise(self.plan);
    }
}

fn parse_test_plan(line: &str) -> Option<(i32, i32)> {
    // converts 1..68 to (1,68)
    let numbers = line
        .split("..")
        .map(|x| x.parse::<i32>())
        .flatten()
        .collect::<Vec<i32>>();
    if numbers.len() != 2 {
        return None;
    }
    Some((numbers[0], numbers[1]))
}

fn take_line_from_word(line: &str, word: usize) -> String {
    match line.split(" ").collect::<Vec<&str>>().get(word..) {
        Some(msg) => msg.join(" "),
        _ => String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TAP_HEADER: &str = "TAP version 13";

    #[test]
    fn test_a_valid_header_line() {
        TapParser::new(&Some(Ok(String::from(TAP_HEADER))), NullFormatter);
    }

    #[test]
    fn test_a_single_passing_test() {
        let mut parser = TapParser::new(&Some(Ok(String::from(TAP_HEADER))), NullFormatter);
        parser.line(&Some(Ok(String::from("# the happy path"))));
        parser.line(&Some(Ok(String::from("ok 1 should be equal"))));

        assert_eq!(parser.tests.len(), 1);
        let test = parser.tests.last().unwrap().build();
        assert_eq!(test.name, "the happy path");
        let assertion = &test.assertions[0];
        assert_eq!(assertion.message, "should be equal");
        assert_eq!(assertion.pass, true);
    }

    #[test]
    fn test_a_test_with_multiple_assertions() {
        let mut parser = TapParser::new(&Some(Ok(String::from(TAP_HEADER))), NullFormatter);
        parser.line(&Some(Ok(String::from("# the happy path"))));
        parser.line(&Some(Ok(String::from("ok 1 should be equal"))));
        parser.line(&Some(Ok(String::from("not ok 2 should work"))));

        assert_eq!(parser.tests.len(), 1);
        let test = parser.tests.last().unwrap().build();
        assert_eq!(test.name, "the happy path");
        assert_eq!(test.assertions.len(), 2);

        let assertion = &test.assertions[0];
        assert_eq!(assertion.message, "should be equal");
        assert_eq!(assertion.pass, true);

        let assertion = &test.assertions[1];
        assert_eq!(assertion.message, "should work");
        assert_eq!(assertion.pass, false);
    }

    #[test]
    fn test_a_single_failing_test() {
        let mut parser = TapParser::new(&Some(Ok(String::from(TAP_HEADER))), NullFormatter);
        parser.line(&Some(Ok(String::from("# the happy path"))));
        parser.line(&Some(Ok(String::from("not ok 2 should be equivalent"))));

        assert_eq!(parser.tests.len(), 1);
        let test = parser.tests.last().unwrap().build();
        assert_eq!(test.name, "the happy path");
        assert_eq!(test.name, "the happy path");
        let assertion = &test.assertions[0];
        assert_eq!(assertion.message, "should be equivalent");
        assert_eq!(assertion.pass, false);
    }

    #[test]
    fn test_finalising_without_a_plan_line() {
        let parser = TapParser::new(&Some(Ok(String::from(TAP_HEADER))), NullFormatter);
        parser.finalise();

        assert_eq!(parser.plan, None);
    }

    #[test]
    fn test_finalising_with_a_plan_line() {
        let mut parser = TapParser::new(&Some(Ok(String::from(TAP_HEADER))), NullFormatter);
        parser.line(&Some(Ok(String::from("# the happy path"))));
        parser.line(&Some(Ok(String::from("not ok 2 should be equivalent"))));
        parser.line(&Some(Ok(String::from("1..1"))));
        parser.finalise();

        assert_eq!(parser.plan, Some((1, 1)));
    }

    #[test]
    #[should_panic]
    fn test_an_invalid_header_line() {
        TapParser::new(&mut Some(Ok(String::from("invalid"))), NullFormatter);
    }
}
