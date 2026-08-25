use std::io::{stdin, stdout, Write, stderr};
use std::process::{Command};
use simply_colored::*;


fn main(){

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
        let command = parts[0];

        match command {
            "exit" => break,
            _ => ()
        }

        let mut child = match Command::new(command).args(&parts[1..]).spawn() {
            Ok(child) => child,
            Err(e) => {
                stderr().write_fmt(format_args!("{:?}\n", e));
            continue;
            },
        };

        if let Err(e) = child.wait() {
            stderr().write_fmt(format_args!("{:?}\n", e));
        };
    }

}
