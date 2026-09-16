use std::collections::HashMap;
use simply_colored::*;
use crate::executor::*;
use crate::executor::ExecutorError::NotFound;

pub struct ExecutorMap<'a> {
    table: HashMap<String, &'a dyn Executor>,
}

impl<'a> ExecutorMap<'a> {

    pub fn new() -> ExecutorMap<'a> {
        ExecutorMap {
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

impl<'a> Default for ExecutorMap<'a> {
    fn default() -> Self {
        let mut this = ExecutorMap::new();

        this.add(&ExecutorPWD{});
        this.add(&ExecutorExit{});

        this
    }

}