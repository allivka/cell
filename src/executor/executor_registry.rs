use std::collections::HashMap;
use simply_colored::*;
use crate::executor::*;
use crate::executor::ExecutorError::NotFound;

pub struct ExecutorRegistry<'a> {
    table: HashMap<String, &'a dyn Executor>,
}

impl<'a> ExecutorRegistry<'a> {

    pub fn new() -> ExecutorRegistry<'a> {
        ExecutorRegistry {
            table: HashMap::new(),
        }
    }

    pub fn register(&mut self, exec: &'a dyn Executor) {
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

impl<'a> Default for ExecutorRegistry<'a> {
    fn default() -> Self {
        let mut this = ExecutorRegistry::new();

        let executors = get_all_executors();
        
        for e in executors {
            this.register(e);
        }

        this
    }

}