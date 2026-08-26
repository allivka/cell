pub mod base;
pub mod executor;
pub mod commander;

use std::io::{stdin, stdout, Write, stderr};
use std::process::{Command};
use simply_colored::*;
use crate::commander::Commander;

fn main(){

    let commander = Commander::default();

    loop {

        if let Err(e) = stdout().write(format!("{GREEN}{}{RESET}", char::from_u32(0x2192).unwrap().to_string() + " ").as_bytes()) {
            stderr().write_fmt(format_args!("{:?}\n", e));
            continue;
        };

        if let Err(e) = stdout().flush() {
            stderr().write_fmt(format_args!("{:?}\n", e));
            continue;
        };

        let mut input = String::new();

        if let Err(e) = stdin().read_line(&mut input) {
            stderr().write_fmt(format_args!("{:?}\n", e));
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
                                    stderr().write_fmt(format_args!("{:?}\n", e));
                                    continue;
                                }

                                if let Err(e) = stdout().flush() {
                                    stderr().write_fmt(format_args!("{:?}\n", e));
                                    continue;
                                }
                            },
                            Err(err) => {
                                stderr().write_fmt(format_args!("{:?}\n", err));
                                continue;
                            }
                        }
                    },
                    _ => {
                        stderr().write_fmt(format_args!("{:?}\n", e));
                        continue;
                    }
                }

                continue
            },
        };

        if let Err(e) = child.wait() {
            stderr().write_fmt(format_args!("{:?}\n", e));
        };
    }

}
