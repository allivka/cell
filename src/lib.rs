pub mod core;
pub mod executor;
pub mod runner;
pub mod parser;

use std::io::{stdin, stdout, Write};
use simply_colored::*;
use crate::runner::{DefaultRunner, Runner, RunnerError};
use core::*;
use crate::parser::closure::ClosureParserError;
use crate::parser::ParserError::ClosureParserFailure;
use crate::runner::RunnerError::ParserFailure;

pub struct Cell {
    pub runner: Box<dyn Runner>,
    pub usual_prompt: String,
    pub unterminated_closure_prompt: String,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            runner: Box::new(DefaultRunner::default()),
            usual_prompt: format!("{GREEN}{}{RESET}",  char::from_u32(0x2192).unwrap().to_string() + " "),
            unterminated_closure_prompt: format!("{BLUE}{}{RESET}",  ">>> "),
        }
    }
}

impl Cell {
    pub fn run(&self) -> std::io::Result<()> {

        let runner = &self.runner;

        let mut input_buffer = String::new();
        let mut closed_closure = true;

        let print_prompt = |closed: bool| -> std::io::Result<()> {

            if closed {
                stdout().write_all(self.usual_prompt.as_bytes())
            } else {
                stdout().write_all(self.unterminated_closure_prompt.as_bytes())
            }

        };

        loop {

            if let Err(e) = print_prompt(closed_closure) {
                pf_error(e.to_string())?;

                continue;
            };

            if let Err(e) = stdout().flush() {
                pf_error(e.to_string())?;
                continue;
            };

            if closed_closure {
                input_buffer.clear();
            }

            if let Err(e) = stdin().read_line(&mut input_buffer) {
                pf_error(e.to_string())?;
                continue;
            };


            let output = match runner.run(&input_buffer) {
                Ok(output) => output,
                Err(e) => {
                    match e {
                        ParserFailure(ClosureParserFailure(ClosureParserError::UnterminatedClosure(_))) => {
                            closed_closure = false;
                        },
                        _ => {
                            pf_error(e.to_string())?;
                            closed_closure = true;
                        }
                    }

                    continue;
                }
            };

            closed_closure = true;

            if let Err(e) = stdout().write_all(output.as_bytes()) {
                pf_error(e.to_string())?;
            };

            if let Err(e) = stdout().flush() {
                pf_error(e.to_string())?;
            };
        }
    }
}
