use std::io::{Read, Stdout};
use std::process::{Child, ChildStdout, Command, Stdio};
use crate::executor::ExecutorRegistry;
use crate::parser::{DefaultParser, Parser, Token};
use crate::runner::{Runner, RunnerError, RunnerResult};
use crate::runner::RunnerError::{ExecutorFailure, IoFailure, ParserFailure};
use crate::core::*;

pub struct DefaultRunner {
    pub executor_map: ExecutorRegistry,
    pub parser: Box<dyn Parser>,
}

impl DefaultRunner {
    fn run_process(&self, command: String, args: Vec<String>) -> RunnerResult<Child> {
        Ok(match Command::new(&command).stdout(Stdio::piped()).args(&args).spawn() {
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
        })

    }
}

impl Runner for DefaultRunner {

    fn name(&self) -> &str {
        "default runner"
    }



    fn run(&self, input: &String) -> RunnerResult<String> {

        let directive = match self.parser.parse(&input) {
            Ok(directive) => directive,
            Err(e) => return Err(ParserFailure(e))
        };

        if directive.len() < 1 {
            return Ok(String::new());
        }

        let command = match &directive[0] {
            Token::RawData(data) => data.clone(),
            Token::SubDirective(dir) => self.run(dir)?,
        };

        let args: Vec<String> = directive[1..].iter().map(|token| -> String {
            match token {
                Token::RawData(data) => data.clone(),
                Token::SubDirective(dir) => match self.run(dir) {
                        Ok(data) => data,
                        Err(e) => panic!("{}", format_error(e.to_string())),
                }

            }
        }).collect();


        let mut child = self.run_process(command, args)?;

        let mut stdout = match child.stdout.take() {
            Some(stdout) => stdout,
            None => {
                return Err(RunnerError::IoFailure("child process has no stdout".to_string()));
            }
        };

        let mut buff = String::new();

        match stdout.read_to_string(&mut buff) {
            Ok(_) => Ok(buff.trim().to_string()),
            Err(err) => Err(IoFailure(err.to_string()))
        }

    }
}

impl Default for DefaultRunner {
    fn default() -> Self {
        DefaultRunner {
            executor_map: ExecutorRegistry::default(),
            parser: Box::new(DefaultParser::default()),
        }
    }
}
