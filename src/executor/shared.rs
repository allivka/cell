
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