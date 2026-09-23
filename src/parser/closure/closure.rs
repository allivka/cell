use crate::parser::closure::{ClosureParser, ClosureParserOutput, ClosureParserResult, DefaultClosureParser};

pub const DEFAULT_CLOSURE_START: &str = "\"";
pub const DEFAULT_CLOSURE_END: &str = "\"";

#[derive(Copy, Clone)]
pub enum ClosureKind {
    Text,
    Directive
}

pub struct Closure {
    pub closure_parser: Box<dyn ClosureParser>,
    pub start_seq: String,
    pub end_seq: String,
    pub kind: ClosureKind
}

impl Default for Closure {
    fn default() -> Self {
        Closure {
            closure_parser: Box::new(DefaultClosureParser::default()),
            start_seq: DEFAULT_CLOSURE_START.to_string(),
            end_seq: DEFAULT_CLOSURE_END.to_string(),
            kind: ClosureKind::Text,
        }
    }
}

impl Closure {
    pub fn new(start_seq: &str, end_seq: &str, kind: ClosureKind) -> Closure {
        Closure {
            start_seq: start_seq.to_string(),
            end_seq: end_seq.to_string(),
            kind,
            ..Closure::default()
        }
    }
    
    pub fn parse(&self, s: &str) -> ClosureParserResult<ClosureParserOutput> {
        self.closure_parser.parse(s, &self.start_seq, &self.end_seq)
    }
}