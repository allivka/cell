use std::collections::HashMap;
use simply_colored::*;
use crate::executor::*;
use crate::executor::ExecutorError::NotFound;

pub struct ExecutorRegistry {
    pub table: HashMap<String, Box<dyn Executor>>,
}

impl<'a> ExecutorRegistry {

    pub fn new() -> ExecutorRegistry {
        ExecutorRegistry {
            table: HashMap::new(),
        }
    }

    pub fn register(&mut self, exec: Box<dyn Executor>) {
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

impl<'a> Default for ExecutorRegistry {
    fn default() -> Self {
        let mut this = ExecutorRegistry::new();

        let executors = get_all_executors();
        
        for e in executors {
            this.register(e);
        }

        this
    }

}