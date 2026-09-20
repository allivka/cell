use crate::parser::{Directive, ParserResult, Token};


#[derive(derive_more::Debug, Clone)]
pub enum ClosureParserError {
    #[debug("ClosureParserError::Custom -> {}", _0)]
    Custom(String),

    #[debug("ClosureParserError::InvalidInput -> {}", _0)]
    InvalidInput(String),

    #[debug("ClosureParserError::UnterminatedClosure -> {}", _0)]
    UnterminatedClosure(String)

}

impl std::fmt::Display for ClosureParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type ClosureParserOutput = Vec<ClosureParsedElement>;
pub type ClosureParserResult<T> = Result<T, ClosureParserError>;

pub enum ClosureParsedElement {
    UsualElement(String),
    SubElement(String)
}

pub trait ClosureParser {
    fn parse(&self, s: &str, closure_start_seq: &str, closure_end_seq: &str) -> ClosureParserResult<ClosureParserOutput>;
}