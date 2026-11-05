use crate::executor::*;
use crate::executor::ExecutorError::{NotFound, IoFailure};
use crate::task::ThreadPipeOwner;

pub struct PWD {}
impl Executor for PWD {
    fn name(&self) -> &str {
        "pwd"
    }

    fn execute(&self, _args: ExecutorArgs, _pipes: ThreadPipeOwner) -> ExecutorResult<String> {
        match std::env::current_dir() {
            Ok(dir) => match dir.to_str() {
                Some(dir) => Ok(dir.to_string()),
                None => Err(NotFound(String::from("Error: current working directory not found")))
            }
            Err(e) => Err(IoFailure(format!("{} error; could not read current working directory: {}", self.name(), e)))
        }
    }

    fn dyn_clone(&self) -> Box<dyn Executor> {
        Box::new(Self{})
    }
}