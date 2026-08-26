use crate::base::*;

pub type ExecutorError = String;

pub type ExecutorResult<T> = Result<T, ExecutorError>;

pub type ExecutorArgs = Vec<String>;

pub trait Executor {

    fn name(&self) -> &str {
        "Unnamed executor"
    }

    fn execute(&self, _args: &ExecutorArgs) -> ExecutorResult<String> {
        Ok(format!("{} is not implemented", self.name()))
    }
}

pub struct UnnamedExecutor {}
impl Executor for UnnamedExecutor {}

pub struct ExecutorPWD {}
impl Executor for ExecutorPWD {
    fn name(&self) -> &str {
        "pwd"
    }

    fn execute(&self, _args: &ExecutorArgs) -> ExecutorResult<String> {
        match std::env::current_dir() {
            Ok(dir) => match dir.to_str() {
                Some(dir) => Ok(dir.to_string()),
                None => Err(String::from("Error: current working directory not found"))
            }
            Err(e) => Err(format!("{} error; could not read current working directory: {}", self.name(), e))
        }
    }
}

pub struct ExecutorExit {}
impl Executor for ExecutorExit {
    fn name(&self) -> &str {
        "exit"
    }

    fn execute(&self, _args: &ExecutorArgs) -> ExecutorResult<String> {
        spf_info("Exiting the shell").unwrap();
        std::process::exit(0);
    }
}