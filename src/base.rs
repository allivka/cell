use std::io;
use std::io::{stdout, Write, stderr};
pub use simply_colored::*;

pub fn format_custom(name: &str, style: String,  v: String) -> String {
    format!("{style}{name}: {}{RESET} {v}{style} {}{RESET}", "{", "}")
}

pub fn format_error(v: String) -> String {
    format!("{RED}{BOLD}Error: {}{RESET} {v}{RED}{BOLD} {}{RESET}", "{", "}")
}

pub fn format_warning(v: String) -> String {
    format!("{YELLOW}{BOLD}Warning: {}{RESET} {v}{YELLOW}{BOLD} {}{RESET}", "{", "}")
}

pub fn format_info(v: String) -> String {
    format!("{BLUE}{BOLD}Info: {}{RESET} {v}{BLUE}{BOLD} {}{RESET}", "{", "}")
}

pub fn format_success(v: String) -> String {
    format!("{GREEN}{BOLD}Success: {}{RESET} {v}{GREEN}{BOLD} {}{RESET}", "{", "}")
}

pub fn pf_error(v: String) -> io::Result<()> {
    stderr().write_all((format_error(v) + "\n").as_bytes())?;
    stderr().flush()?;

    Ok(())
}

pub fn pf_warning(v: String) -> io::Result<()> {
    stderr().write_all((format_warning(v) + "\n").as_bytes())?;
    stderr().flush()?;

    Ok(())
}

pub fn pf_info(v: String) -> io::Result<()> {
    stdout().write_all((format_info(v) + "\n").as_bytes())?;
    stdout().flush()?;

    Ok(())
}

pub fn pf_success(v: String) -> io::Result<()> {
    stdout().write_all((format_success(v) + "\n").as_bytes())?;
    stdout().flush()?;

    Ok(())
}

pub fn spf_error(v: &str) -> io::Result<()> {
    stderr().write_all((format_error(String::from(v)) + "\n").as_bytes())?;
    stderr().flush()?;

    Ok(())
}

pub fn spf_warning(v: &str) -> io::Result<()> {
    stderr().write_all((format_warning(String::from(v)) + "\n").as_bytes())?;
    stderr().flush()?;

    Ok(())
}

pub fn spf_info(v: &str) -> io::Result<()> {
    stdout().write_all((format_info(String::from(v)) + "\n").as_bytes())?;
    stdout().flush()?;

    Ok(())
}

pub fn spf_success(v: &str) -> io::Result<()> {
    stdout().write_all((format_success(String::from(v)) + "\n").as_bytes())?;
    stdout().flush()?;

    Ok(())
}