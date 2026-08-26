pub mod base;
pub mod executor;
pub mod commander;

use std::io::{stdin, stdout, Write};
use std::process::{Command};
use simply_colored::*;
use crate::commander::Commander;
use crate::base::*;

fn main() -> std::io::Result<()> {

    let commander = Commander::default();

    loop {

        if let Err(e) = stdout().write(format!("{GREEN}{}{RESET}", char::from_u32(0x2192).unwrap().to_string() + " ").as_bytes()) {
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

        let parts = input.trim().split_whitespace().collect::<Vec<_>>();

        if parts.len() < 1 {
            continue;
        }

        let command = parts[0];

        match command {
            "exit" => break,
            _ => ()
        }

        let args = &parts[1..];

        let mut child = match Command::new(command).args(args).spawn() {
            Ok(child) => child,
            Err(e) => {

                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        match commander.execute(String::from(command), &args.iter().map(|v| String::from(*v)).collect()) {
                            Ok(result) => {
                                if let Err(e) = stdout().write((result + "\n").as_bytes()) {
                                    pf_error(e.to_string())?;
                                    continue;
                                }

                                if let Err(e) = stdout().flush() {
                                    pf_error(e.to_string())?;
                                    continue;
                                }
                            },
                            Err(err) => {
                                pf_error(err)?;
                                continue;
                            }
                        }
                    },
                    _ => {
                        pf_error(e.to_string())?;
                        continue;
                    }
                }

                continue
            },
        };

        if let Err(e) = child.wait() {
            pf_error(e.to_string())?;
        };
    }

    Ok(())
}
