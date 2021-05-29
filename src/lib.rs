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
    use crate::text_decoration::*;

    pub struct DotFormatter;
    impl TestFormat for DotFormatter {
        fn new_test(&self, _title: &str) {}
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
        fn summerise(&self, plan: Option<(i32, i32)>) {
            if let Some(expected) = plan {
                print!("Ran {} tests", expected.1);
            } else {
                print!("Test suite ended without report");
            }
        }
    }

    pub struct NullFormatter;
    impl TestFormat for NullFormatter {
        fn new_test(&self, _title: &str) {}
        fn assertion(&self, _passed: bool, _assertion: &str) {}
        fn log_output(&self, _output: &str) {}
        fn summerise(&self, _plan: Option<(i32, i32)>) {}
    }

    pub trait TestFormat {
        fn new_test(&self, title: &str);
        fn assertion(&self, passed: bool, assertion: &str);
        fn log_output(&self, output: &str);
        fn summerise(&self, plan: Option<(i32, i32)>);
    }
    pub struct SpecFormatter;
    impl TestFormat for SpecFormatter {
        fn new_test(&self, title: &str) {
            println!("\n{}\n", underlined(title));
        }
        fn assertion(&self, passed: bool, assertion: &str) {
            if passed {
                print!("✅ ");
            //println!("  ✅ {}", green(assertion));
            } else {
                println!("  ❌ {}", red(assertion));
            }
        }
        fn log_output(&self, output: &str) {
            print!("  {}\n", yellow(output));
        }
        fn summerise(&self, plan: Option<(i32, i32)>) {
            if let Some(expected) = plan {
                print!("Ran {} tests\n", expected.1);
            } else {
                print!("Test suite ended without report");
            }
        }
    }
}
