use crate::core::*;

use crate::executor::*;
use crate::task::ThreadPipeOwner;

pub struct EXIT {}
impl Executor for EXIT {
    fn name(&self) -> &str {
        "exit"
    }

    fn execute(&self, _args: ExecutorArgs, _pipes: ThreadPipeOwner) -> ExecutorResult<String> {
        spf_info("Exiting the shell").unwrap();
        std::process::exit(0);
    }

    fn dyn_clone(&self) -> Box<dyn Executor> {
        Box::new(Self{})
    }
}