mod text_decoration {
    static RESET: &str = "\x1b[0m";
    pub fn red(text: &str) -> String {
        let red = "\x1B[31m";
        format!("{}{}{}", red, text, RESET)
    }

    pub fn green(text: &str) -> String {
        let green = "\x1B[32m";
        format!("{}{}{}", green, text, RESET)
    }

    pub fn underlined(text: &str) -> String {
        let underlined = "\x1B[4m";
        format!("{}{}{}", underlined, text, RESET)
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
    }

    pub struct NullFormatter;
    impl TestFormat for NullFormatter {
        fn new_test(&self, _title: &str) {}
        fn assertion(&self, _passed: bool, _assertion: &str) {}
    }

    pub trait TestFormat {
        fn new_test(&self, title: &str);
        fn assertion(&self, passed: bool, assertion: &str);
    }
    pub struct SpecFormatter;
    impl TestFormat for SpecFormatter {
        fn new_test(&self, title: &str) {
            println!("\n{}\n", underlined(title));
        }
        fn assertion(&self, passed: bool, assertion: &str) {
            if passed {
                println!("\t ✅ {}", green(assertion));
            } else {
                println!("\t ❌ {}", red(assertion));
            }
        }
    }
}
