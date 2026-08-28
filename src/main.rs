pub mod base;
pub mod executor;
pub mod commander;
pub mod runner;

use std::io::{stdin, stdout, Write};
use simply_colored::*;
use crate::base::*;
use crate::runner::{DefaultRunner, Runner};

fn main() -> std::io::Result<()> {

    let runner = DefaultRunner::default();

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

        if let Err(e) = stdout().write_all((output + "\n").as_bytes()) {
            pf_error(e.to_string())?;
        };

        if let Err(e) = stdout().flush() {
            pf_error(e.to_string())?;
        };
    }
}
