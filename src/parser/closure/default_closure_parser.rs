
use crate::parser::closure::*;

pub struct DefaultClosureParser { }

impl Default for DefaultClosureParser {
    fn default() -> Self {
        DefaultClosureParser { }
    }
}

impl ClosureParser for DefaultClosureParser {
    fn parse(&self, s: &str, closure_start_seq: &str, closure_end_seq: &str) -> ClosureParserResult<ClosureParserOutput> {
        let mut output = ClosureParserOutput::new();
        let mut word = String::new();
        let mut captured = String::new();
        let mut inside = false;
        let mut rest = s;

        while !rest.is_empty() {
            if inside && rest.starts_with(closure_end_seq) {
                inside = false;
                output.push(ClosureParsedElement::SubElement(std::mem::take(&mut captured)));
                rest = &rest[closure_end_seq.len()..];
                continue;
            }
            if !inside && rest.starts_with(closure_start_seq) {
                inside = true;
                if !word.is_empty() {
                    output.push(ClosureParsedElement::UsualElement(std::mem::take(&mut word)));
                }
                rest = &rest[closure_start_seq.len()..];
                continue;
            }

            let ch = rest.chars().next().unwrap();
            rest = &rest[ch.len_utf8()..];

            match (inside, ch.is_whitespace()) {
                (true, _)       => captured.push(ch),
                (false, false)  => word.push(ch),
                (false, true)   => if !word.is_empty() {
                    output.push(ClosureParsedElement::UsualElement(std::mem::take(&mut word)));
                },
            }
        }

        if inside {
            return Err(ClosureParserError::UnterminatedClosure(closure_end_seq.to_string()));
        }
        if !word.is_empty() {
            output.push(ClosureParsedElement::UsualElement(word));
        }
        Ok(output)
    }
}