
use derive_more::Debug;

#[derive(Debug, Clone)]
pub enum ParserError {
    #[debug("ParserError::Custom -> {}", _0)]
    Custom(String),

    #[debug("ParserError::InvalidInput -> {}", _0)]
    InvalidInput(String),
    
    
    
}

pub type ParserResult<T> = Result<T, ParserError>;

pub type Directive = Vec<Token>;

pub enum Token {
    RawData(String),
    SubDirective(Directive)
}

pub trait Parser {
    fn parse(s: String) -> ParserResult<Directive>;
}