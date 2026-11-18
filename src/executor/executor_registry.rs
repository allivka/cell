use std::collections::HashMap;
use simply_colored::*;
use crate::executor::*;
use crate::task::*;
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

    pub fn execute(&self, s: String, args: ExecutorArgs) -> TaskResult<ThreadTask<TaskResult<String>>> {
        let executor = match self.table.get(s.as_str()) {
            Some(v) => v,
            None => {
                return Err(TaskError::ThreadFailure(format!("command {RED}[ {} ]{RESET} is not found", s)))
            }
        }.dyn_clone();

        ThreadTask::spawn(move |pipes| {
            match executor.execute(args, pipes) {
                Ok(v) => Ok(v),
                Err(e) => Err(TaskError::ExecutorFailure(e))
            }
        })

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