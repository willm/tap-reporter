pub mod model {
    pub struct Test {
        name: String,
        assertions: Vec<Assertion>,
        pass: bool,
        log: Vec<String>,
    }

    impl Test {
        pub fn name(&self) -> String {
            self.name.clone()
        }

        pub fn assertions(&self) -> &Vec<Assertion> {
            &self.assertions
        }

        pub fn pass(&self) -> bool {
            self.pass
        }

        pub fn log(&self) -> String {
            self.log.join("\n")
        }
    }

    pub struct Assertion {
        message: String,
        pass: bool,
    }

    impl Assertion {
        pub fn message(&self) -> String {
            self.message.clone()
        }
        pub fn pass(&self) -> bool {
            self.pass
        }
    }

    pub struct TestBuilder {
        test: Test,
    }

    impl TestBuilder {
        pub fn new() -> Self {
            TestBuilder {
                test: Test {
                    name: String::from(""),
                    assertions: vec![],
                    pass: true,
                    log: vec![],
                },
            }
        }

        pub fn with_name(&mut self, name: &str) -> &Self {
            self.test.name = String::from(name);
            self
        }

        pub fn with_assertion(&mut self, pass: bool, assertion: String) -> &Self {
            self.test.assertions.push(Assertion {
                pass,
                message: assertion,
            });
            self.test.pass = self.test.pass && pass;
            self
        }

        pub fn with_log(&mut self, log_line: &str) {
            self.test.log.push(log_line.to_string());
        }

        pub fn build(&self) -> &Test {
            &self.test
        }
    }
}
