pub mod model;

mod text_decoration {
    static RESET: &str = "\x1b[0m";
    pub fn red(text: &str) -> String {
        wrap(text, "\x1B[31m")
    }

    pub fn green(text: &str) -> String {
        wrap(text, "\x1B[32m")
    }

    pub fn yellow(text: &str) -> String {
        wrap(text, "\x1B[33m")
    }

    pub fn underlined(text: &str) -> String {
        wrap(text, "\x1B[4m")
    }

    fn wrap(text: &str, decoration: &str) -> String {
        format!("{}{}{}", decoration, text, RESET)
    }
}

pub mod formatters {

    use crate::{
        model::model::{Assertion, Test},
        text_decoration::*,
    };

    pub struct DotFormatter;
    impl TestFormat for DotFormatter {
        fn new_test(&mut self, _title: &str) {}
        fn assertion(&self, passed: bool, _assertion: &str) {
            if passed {
                print!(".");
            } else {
                print!("x");
            }
        }
        fn log_output(&self, output: &str) {
            print!("{}", output);
        }
        fn summerise(&self, plan: Option<(i32, i32)>, _tests: Vec<&Test>) {
            if let Some(expected) = plan {
                print!("Ran {} tests", expected.1);
            } else {
                print!("Test suite ended without report");
            }
        }
    }

    pub struct NullFormatter;
    impl TestFormat for NullFormatter {
        fn new_test(&mut self, _title: &str) {}
        fn assertion(&self, _passed: bool, _assertion: &str) {}
        fn log_output(&self, _output: &str) {}
        fn summerise(&self, _plan: Option<(i32, i32)>, _tests: Vec<&Test>) {}
    }

    pub trait TestFormat {
        fn new_test(&mut self, title: &str);
        fn assertion(&self, passed: bool, assertion: &str);
        fn log_output(&self, output: &str);
        fn summerise(&self, plan: Option<(i32, i32)>, tests: Vec<&Test>);
    }
    pub struct SpecFormatter {
        current_test: String,
    }
    impl SpecFormatter {
        pub fn new() -> Self {
            SpecFormatter {
                current_test: String::from(""),
            }
        }
    }
    impl TestFormat for SpecFormatter {
        fn new_test(&mut self, title: &str) {
            self.current_test = title.to_string();
        }

        fn assertion(&self, passed: bool, _assertion: &str) {
            if passed {
                print!("{}", green("."));
            } else {
                print!("{}", red("x"));
            }
        }
        fn log_output(&self, _output: &str) {}
        fn summerise(&self, plan: Option<(i32, i32)>, tests: Vec<&Test>) {
            if let Some(expected) = plan {
                let total_assertions = tests
                    .iter()
                    .flat_map(|&test| test.assertions())
                    .collect::<Vec<&Assertion>>()
                    .len();

                let failed_tests = tests
                    .into_iter()
                    .filter(|&test| !test.pass())
                    .collect::<Vec<&Test>>();

                println!("\nRan {} of {} tests\n", expected.1, total_assertions);
                for test in failed_tests {
                    println!("\n{}\n", underlined(&test.name()));
                    println!("{}", yellow(&test.log()));
                    for assertion in test.assertions() {
                        if assertion.pass() {
                            println!("  ✅ {}", green(&assertion.message()));
                        } else {
                            println!("  ❌ {}", red(&assertion.message()));
                        }
                    }
                }
            } else {
                print!("Test suite ended without report");
            }
        }
    }
}
