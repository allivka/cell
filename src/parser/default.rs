use crate::parser::closure::{Closure, ClosureKind, ClosureParsedElement};
use crate::parser::parser::*;
use crate::parser::ParserError::ClosureParserFailure;

pub fn get_default_closures() -> Vec<Closure> {
    vec![
        Closure::new("\"", "\"", ClosureKind::Text),
    ]
}

pub struct DefaultParser {
    pub closures: Vec<Closure>,
}

impl Default for DefaultParser {
    fn default() -> Self {
        DefaultParser {
            closures: get_default_closures(),
        }
    }
}

impl Parser for DefaultParser {
    fn name(&self) -> &str {
        "default parser"
    }

    fn parse(&self, s: &String) -> ParserResult<Directive> {

        let mut directive: Directive = Vec::new();

        for closure in &self.closures {

            //TODO: add proper closure parsing one after another

            let parsed = match closure.parse(s.as_str()) {
                Ok(parsed) => parsed,
                Err(e) => return Err(ClosureParserFailure(e)),
            };


            for el in parsed { match el {
                ClosureParsedElement::UsualElement(data) => {
                    directive.push(Token::RawData(data));
                },

                ClosureParsedElement::SubElement(data) => { match closure.kind {

                    ClosureKind::Text => {
                        directive.push(Token::RawData(data));
                    },

                    ClosureKind::Directive => {
                        //TODO: implement directive recursive parsing
                    }
                }}
            } }

        }


        Ok(directive)
    }
    
     // fn parse(&self, s: String) -> ParserResult<Directive> {
     //     let parts = s.trim().split_whitespace().collect::<Vec<_>>();
     //
     //     if parts.len() < 1 {
     //         return Ok(vec![]);
     //     }
     //
     //     let mut directive = Directive::new();
     //     let mut substring: String = String::new();
     //
     //     let mut captured = false;
     //
     //     for part in parts {
     //         if !captured && part.starts_with(DEFAULT_CAPTURE_CHAR) {
     //
     //             if part.starts_with(DEFAULT_CAPTURE_CHAR) {
     //                 directive.push(Token::RawData(part.trim_matches(DEFAULT_CAPTURE_CHAR).to_string()));
     //                 continue
     //             }
     //
     //             captured = true;
     //             substring += part.trim_start_matches(DEFAULT_CAPTURE_CHAR);
     //             continue;
     //
     //         }
     //
     //         if !captured && part.ends_with(DEFAULT_CAPTURE_CHAR) {
     //             captured = true;
     //             directive.push(Token::RawData(part.trim_end_matches(DEFAULT_CAPTURE_CHAR).to_string()));
     //             continue;
     //         }
     //
     //
     //         if !captured {
     //             directive.push(Token::RawData(part.to_string()));
     //             continue;
     //         }
     //
     //         if captured && part.starts_with(DEFAULT_CAPTURE_CHAR) {
     //             captured = false;
     //             directive.push(Token::RawData(substring));
     //             substring = String::new();
     //
     //             if part.ends_with(DEFAULT_CAPTURE_CHAR) {
     //                 captured = true;
     //                 directive.push(Token::RawData(part.trim_matches(DEFAULT_CAPTURE_CHAR).to_string()));
     //             }
     //
     //             continue
     //
     //         }
     //
     //         if captured && part.ends_with(DEFAULT_CAPTURE_CHAR) {
     //             captured = false;
     //             substring += part.trim_start_matches(DEFAULT_CAPTURE_CHAR);
     //             directive.push(Token::RawData(substring));
     //             substring = String::new();
     //
     //             continue
     //         }
     //
     //     }
     //
     //     if captured {
     //         directive.push(Token::RawData(substring));
     //     }
     //
     //     Ok(directive)
     // }
}