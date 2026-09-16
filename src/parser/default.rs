use crate::parser::parser::*;

pub const DEFAULT_CAPTURE_CHAR: char = '~';

pub struct DefaultParser {}

impl Parser for DefaultParser {
     fn parse(s: String) -> ParserResult<Directive> {
         let parts = s.trim().split_whitespace().collect::<Vec<_>>();

         if parts.len() < 1 {
             return Ok(vec![]);
         }

         let mut directive = Directive::new();
         let mut substring: String = String::new();

         let mut captured = false;

         for part in parts {
             if !captured && part.starts_with(DEFAULT_CAPTURE_CHAR) {

                 if part.starts_with(DEFAULT_CAPTURE_CHAR) {
                     directive.push(Token::RawData(part.trim_matches(DEFAULT_CAPTURE_CHAR).to_string()));
                     continue
                 }

                 captured = true;
                 substring += part.trim_start_matches(DEFAULT_CAPTURE_CHAR);
                 continue;

             }

             if !captured && part.ends_with(DEFAULT_CAPTURE_CHAR) {
                 captured = true;
                 directive.push(Token::RawData(part.trim_end_matches(DEFAULT_CAPTURE_CHAR).to_string()));
                 continue;
             }


             if !captured {
                 directive.push(Token::RawData(part.to_string()));
                 continue;
             }

             if captured && part.starts_with(DEFAULT_CAPTURE_CHAR) {
                 captured = false;
                 directive.push(Token::RawData(substring));
                 substring = String::new();

                 if part.ends_with(DEFAULT_CAPTURE_CHAR) {
                     captured = true;
                     directive.push(Token::RawData(part.trim_matches(DEFAULT_CAPTURE_CHAR).to_string()));
                 }

                 continue

             }

             if captured && part.ends_with(DEFAULT_CAPTURE_CHAR) {
                 captured = false;
                 substring += part.trim_start_matches(DEFAULT_CAPTURE_CHAR);
                 directive.push(Token::RawData(substring));
                 substring = String::new();

                 continue
             }

         }

         Ok(vec![])
     }
}