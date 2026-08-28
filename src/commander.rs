use std::collections::HashMap;
use simply_colored::*;
use crate::executor::*;
use crate::executor::ExecutorError::NotFound;

pub struct Commander<'a> {
    table: HashMap<String, &'a dyn Executor>,
}

impl<'a> Commander<'a> {

    pub fn new() -> Commander<'a> {
        Commander {
            table: HashMap::new(),
        }
    }

    pub fn add(&mut self, exec: &'a dyn Executor) {
        self.table.insert(String::from(exec.name()), exec);
    }

    pub fn execute(&self, s: String, args: &ExecutorArgs) -> ExecutorResult<String> {
        let executor = match self.table.get(s.as_str()) {
            Some(v) => v,
            None => {
                return Err(NotFound(format!("command {RED}[ {} ]{RESET} is not found", s)))
            }
        };

        executor.execute(args)
    }
}

impl<'a> Default for Commander<'a> {
    fn default() -> Self {
        let mut this = Commander::new();

        this.add(&ExecutorPWD{});
        this.add(&ExecutorExit{});

        this
    }

}