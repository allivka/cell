
use derive_more::Debug;

#[derive(Debug, Clone)]
pub enum ParserError {
    #[debug("ParserError::Custom -> {}", _0)]
    Custom(String),

    #[debug("ParserError::InvalidInput -> {}", _0)]
    InvalidInput(String),
    
    
    
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

pub type Directive = Vec<Token>;

pub enum Token {
    RawData(String),
    SubDirective(Directive)
}

pub trait Parser {
    
    fn name(&self) -> &str;
    
    fn parse(&self, s: String) -> ParserResult<Directive>;
    
    // fn parse_mut(&mut self, s: String) -> ParserResult<Directive> {
    //     self.parse(s)
    // }
}