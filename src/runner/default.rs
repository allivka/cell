use std::process::Command;
use crate::executor_map::ExecutorMap;
use crate::runner::{Runner, RunnerResult};
use crate::runner::RunnerError::{ExecutorFailure, InvalidInput, IoFailure};

pub struct DefaultRunner<'a> {
    executor_map: ExecutorMap<'a>
}

impl Runner for DefaultRunner<'_> {

    fn name(&self) -> &str {
        "default runner"
    }

    fn run(&self, input: String) -> RunnerResult<String> {
        let parts = input.trim().split_whitespace().collect::<Vec<_>>();

        if parts.len() < 1 {
            return Err(InvalidInput("no parts were parsed from the input".to_string()));
        }

        let command = parts[0];

        let args = &parts[1..];

        let child = match Command::new(command).args(args).spawn() {
            Ok(child) => child,
            Err(e) => {
                return match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        match self.executor_map.execute(String::from(command), &args.iter().map(|v| String::from(*v)).collect()) {
                            Ok(result) => {
                                Ok(result)
                            },
                            Err(err) => {
                                Err(ExecutorFailure(err))
                            }
                        }
                    },
                    r => {
                        Err(IoFailure(r.to_string()))
                    }
                }
            },
        };

        let out = match child.wait_with_output() {
            Ok(output) => output,
            Err(err) => {
                return Err(IoFailure(err.to_string()));
            }
        };

        match String::from_utf8(out.stdout) {
            Ok(v) => Ok(v),
            Err(err) => Err(IoFailure(err.to_string())),        }

    }
}

impl<'a> Default for DefaultRunner<'a> {
    fn default() -> Self {
        DefaultRunner {
            executor_map: ExecutorMap::default()
        }
    }
}
