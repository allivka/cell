pub mod core;
pub mod executor;
pub mod runner;
pub mod parser;

use std::io::{stdin, stdout, Write};
use simply_colored::*;
use crate::runner::{DefaultRunner, Runner};
use core::*;

pub struct Cell {
    pub runner: Box<dyn Runner>,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            runner: Box::new(DefaultRunner::default()),
        }
    }
}

impl Cell {
    pub fn run(&self) -> std::io::Result<()> {

        let runner = &self.runner;

        loop {

            if let Err(e) = stdout().write_all(format!("{GREEN}{}{RESET}", char::from_u32(0x2192).unwrap().to_string() + " ").as_bytes()) {
                pf_error(e.to_string())?;

                continue;
            };

            if let Err(e) = stdout().flush() {
                pf_error(e.to_string())?;
                continue;
            };

            let mut input = String::new();

            if let Err(e) = stdin().read_line(&mut input) {
                pf_error(e.to_string())?;
                continue;
            };

            let output = match runner.run(input) {
                Ok(output) => output,
                Err(e) => {
                    pf_error(e.to_string())?;
                    continue;
                }
            };

            if let Err(e) = stdout().write_all(output.as_bytes()) {
                pf_error(e.to_string())?;
            };

            if let Err(e) = stdout().flush() {
                pf_error(e.to_string())?;
            };
        }
    }
}
