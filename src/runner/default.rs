use std::process::Command;
use crate::executor::ExecutorMap;
use crate::parser::{DefaultParser, Parser, Token};
use crate::runner::{Runner, RunnerResult};
use crate::runner::RunnerError::{Custom, ExecutorFailure, IoFailure, ParserFailure};
use crate::core::*;

pub struct DefaultRunner<'a> {
    executor_map: ExecutorMap<'a>,
    parser: &'a dyn Parser
}

impl Runner for DefaultRunner<'_> {

    fn name(&self) -> &str {
        "default runner"
    }

    fn run(&self, input: String) -> RunnerResult<String> {

        let directive = match self.parser.parse(input) {
            Ok(directive) => directive,
            Err(e) => return Err(ParserFailure(e))
        };

        if directive.len() < 1 {
            return Ok(String::new());
        }

        let command = match &directive[0] {
            Token::RawData(data) => data.clone(),
            Token::SubDirective(dir) => return Err(Custom(format!("{RED}{BOLD}Subdirectives are not implemented yet!{RESET}")))
        };

        let args: Vec<String> = directive[1..].iter().map(|token| -> String {
            match token {
                Token::RawData(data) => data.clone(),
                Token::SubDirective(dir) => String::new()
            }
        }).collect();

        // print!("{command} {}", args.join(" "));

        let child = match Command::new(&command).args(&args).spawn() {
            Ok(child) => child,
            Err(e) => {
                return match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        match self.executor_map.execute(String::from(&command), &args) {
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
            Err(err) => Err(IoFailure(err.to_string()))
        }

    }
}

impl<'a> Default for DefaultRunner<'a> {
    fn default() -> Self {
        DefaultRunner {
            executor_map: ExecutorMap::default(),
            parser: &DefaultParser{}
        }
    }
}
