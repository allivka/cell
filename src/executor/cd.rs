use std::env;
use std::path::Path;
use crate::executor::{Executor, ExecutorArgs, ExecutorResult};
use crate::executor::ExecutorError::IoFailure;

pub struct CD {}

impl Executor for CD {
    fn name(&self) -> &str {
        "cd"
    }

    fn execute(&self, args: &ExecutorArgs) -> ExecutorResult<String> {

        if !args.is_empty() {
            if let Err(e) = env::set_current_dir(Path::new(args[0].as_str())) {
                return Err(IoFailure(e.to_string()));
            }
        }

        Ok(String::from(""))
    }
}