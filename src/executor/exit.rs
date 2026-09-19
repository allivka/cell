use crate::base::*;

use crate::executor::*;

pub struct EXIT {}
impl Executor for EXIT {
    fn name(&self) -> &str {
        "exit"
    }

    fn execute(&self, _args: &ExecutorArgs) -> ExecutorResult<String> {
        spf_info("Exiting the shell").unwrap();
        std::process::exit(0);
    }
}