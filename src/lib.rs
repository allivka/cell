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
    pub entrance_seq: String,
    pub opened_closure_seq: String,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            runner: Box::new(DefaultRunner::default()),
            entrance_seq: format!("{GREEN}{}{RESET}",  char::from_u32(0x2192).unwrap().to_string() + " "),
            opened_closure_seq: format!("{BLUE}{}{RESET}",  ">>> "),
        }
    }
}

impl Cell {
    pub fn run(&self) -> std::io::Result<()> {

        let runner = &self.runner;

        let mut input_buffer = String::new();
        let mut should_clear_input_buffer = true;
        let mut seq: u8 = 0;
        let print_seq = |seq: &u8| -> std::io::Result<()> {

            if seq % 2 == 0 {
                stdout().write_all(self.entrance_seq.as_bytes())
            } else {
                stdout().write_all(self.opened_closure_seq.as_bytes())
            }

        };

        loop {

            if let Err(e) = print_seq(&seq) {
                pf_error(e.to_string())?;

                continue;
            };

            if let Err(e) = stdout().flush() {
                pf_error(e.to_string())?;
                continue;
            };

            if should_clear_input_buffer {
                input_buffer.clear();
            }

            if let Err(e) = stdin().read_line(&mut input_buffer) {
                pf_error(e.to_string())?;
                continue;
            };


            let output = match runner.run(&input_buffer) {
                Ok(output) => output,
                Err(e) => {
                    let mut f = false;
                    match &e {
                        ParserFailure(e) => match &e {
                            ClosureParserFailure(e) => match &e {
                                ClosureParserError::UnterminatedClosure(e) => {

                                    f = false;
                                },
                                _ => f = true
                            },
                            _ => f = true

                        },
                        _ => f = true
                    }

                    if f {
                        pf_error(e.to_string())?;
                        should_clear_input_buffer = true;
                        seq = 0;

                    } else {
                        should_clear_input_buffer = false;
                        seq = 1;
                    }

                    continue;
                }
            };

            should_clear_input_buffer = true;
            seq = 0;

            if let Err(e) = stdout().write_all(output.as_bytes()) {
                pf_error(e.to_string())?;
            };

            if let Err(e) = stdout().flush() {
                pf_error(e.to_string())?;
            };
        }
    }
}
