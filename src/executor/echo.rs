
use crate::executor::executor::*;
pub struct ECHO {}

impl Executor for ECHO {
    fn name(&self) -> &str {
        "echo"
    }
    
    fn execute(&self, args: &ExecutorArgs) -> ExecutorResult<String> {
        Ok(args.join(" "))
    }
}