use std::process::{Command};
use crate::executor::ExecutorRegistry;
use crate::parser::{DefaultParser, Parser, Token};
use crate::runner::{Runner, RunnerResult};
use crate::runner::RunnerError::{IoFailure, ParserFailure, TaskFailure};
use crate::task::{ProcessTask, Task, TaskResult, TaskError};

pub struct DefaultRunner {
    pub executor_map: ExecutorRegistry,
    pub parser: Box<dyn Parser>,
}

impl DefaultRunner {
    fn run_process(&self, command: String, args: Vec<String>) -> RunnerResult<Box<dyn Task<TaskResult<String>>>> {
        match ProcessTask::spawn(Command::new(&command).args(&args)) {
            Ok(child) => Ok(Box::new(child)),
            Err(TaskError::NotFound(_)) => {
                match self.executor_map.execute(String::from(&command), args) {
                    Ok(result) => {
                        Ok(Box::new(result))
                    },
                    Err(e) => {
                        Err(TaskFailure(e))
                    }
                }
            },
            Err(e) => Err(TaskFailure(e)),
        }


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

        let mut args: Vec<String> = Vec::with_capacity(directive.len() - 1);

        for token in directive[1..].iter() {
            args.push(match token {
                Token::RawData(data) => data.clone(),
                Token::SubDirective(dir) => match self.run(dir) {
                    Ok(data) => data,
                    // Err(e) => panic!("{}", format_error(e.to_string())),
                    Err(e) => return Err(e)
                }

            })
        }

        let mut child = self.run_process(command, args)?;

        // let res = format_info(match child.wait() {
        //     Ok(Ok(res)) => res,
        //     Err(e) | Ok(Err(e)) => return Err(TaskFailure(e)),
        // });

        let mut buff = String::new();

        let stdout = match child.stdout().take() {
            Some(stdout) => stdout,
            None => {
                return Err(IoFailure("child process has no stdout".to_string()));
            }
        };

        match stdout.read_to_string(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(IoFailure(err.to_string()))
        };

        let stderr = match child.stderr().take() {
            Some(stderr) => stderr,
            None => {
                return Err(IoFailure("child process has no stdout".to_string()));
            }
        };

        match stderr.read_to_string(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(IoFailure(err.to_string()))
        };

        Ok(buff.to_string()/* + "\n" + res.as_str()*/)

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
