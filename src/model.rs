pub mod model {
    pub struct Test {
        name: String,
        assertions: Vec<Assertion>,
        pass: bool,
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

        pub fn build(&self) -> &Test {
            &self.test
        }
    }
}
