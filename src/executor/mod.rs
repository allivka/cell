mod executor;
pub use executor::*;

mod exit;
pub use exit::*;

mod pwd;
pub use pwd::*;

pub mod echo;
pub use echo::*;

pub fn get_all_executors<'a>() -> Vec<&'a dyn Executor> {
    vec![
        &EXIT {},
        &PWD {},
        &ECHO {}
    ]
}
