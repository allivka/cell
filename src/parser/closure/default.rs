
use crate::parser::closure::*;

pub struct DefaultClosureParser { }

impl Default for DefaultClosureParser {
    fn default() -> Self {
        DefaultClosureParser { }
    }
}

impl ClosureParser for DefaultClosureParser {
    fn parse(&self, s: &str, closure_start_seq: &str, closure_end_seq: &str) -> ClosureParserResult<ClosureParserOutput> {

        if closure_start_seq.is_empty() || closure_end_seq.is_empty() {
            return Err(ClosureParserError::InvalidInput(String::from("closure delimiters must not be empty")));
        }

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

//Returns ClosureParserOutput and index vector Vec<Option<ClosureKind>> with the same lengths. kinds refer to the kinds of closure the according Element::SubElement in ClosureParserOutput has. The output vectors are always the same length or the Err will be returned
pub fn parse_closures(s: &str, closures: Vec<(String, String, ClosureKind)>) -> ClosureParserResult<(ClosureParserOutput, Vec<Option<ClosureKind>>)> {
    if closures.iter().any(|(start, end, _)| start.is_empty() || end.is_empty()) {
        return Err(ClosureParserError::InvalidInput(String::from("closure delimiters must not be empty")));
    }

    if closures.len() < 1 {
        return Err(ClosureParserError::InvalidArgument(String::from("no closures info provided to use for parsing")));
    }

    let mut active_closure: usize = 0;

    let mut output = ClosureParserOutput::new();
    let mut output_kinds = Vec::<Option<ClosureKind>>::new();

    let mut push_output = |v: ClosureParsedElement, kind: Option<ClosureKind>| {
        output.push(v);
        output_kinds.push(kind);
    };

    let mut word = String::new();
    let mut captured = String::new();
    let mut inside = false;
    let mut rest = s;

    while !rest.is_empty() {

        if inside && rest.starts_with(closures[active_closure].1.as_str()) {
            inside = false;
            push_output(ClosureParsedElement::SubElement(std::mem::take(&mut captured)), Some(closures[active_closure].2));
            rest = &rest[closures[active_closure].1.as_str().len()..];
            continue;
        }

        if !inside {

            let mut found = false;

            for i in 0..closures.len() {
                if rest.starts_with(closures[i].0.as_str()) {
                    active_closure = i;
                    found = true;
                    break;
                }
            }

            if found {
                inside = true;
                if !word.is_empty() {
                    push_output(ClosureParsedElement::UsualElement(std::mem::take(&mut word)), None);
                }
                rest = &rest[closures[active_closure].0.as_str().len()..];
                continue;
            }
        }

        let ch = rest.chars().next().unwrap();
        rest = &rest[ch.len_utf8()..];

        match (inside, ch.is_whitespace()) {
            (true, _)       => captured.push(ch),
            (false, false)  => word.push(ch),
            (false, true)   => if !word.is_empty() {
                push_output(ClosureParsedElement::UsualElement(std::mem::take(&mut word)), None);
            },
        }
    }

    if inside {
        return Err(ClosureParserError::UnterminatedClosure(closures[active_closure].1.as_str().to_string()));
    }
    if !word.is_empty() {
        push_output(ClosureParsedElement::UsualElement(word), None);
    }

    if output.len() != output_kinds.len() {
        return Err(ClosureParserError::InternalFailure(String::from(format!("error during parsing multiple closures in one request: output token length{}{}", output.len(), output_kinds.len()))));
    }

    Ok((output, output_kinds))

}


// #[deprecated]
// pub fn parse_multiple_closures(s: &str, closures: Vec<(String, String, ClosureKind)>) -> ClosureParserResult<ClosureParserOutput> {
//     if closures.iter().any(|(start, end, _)| start.is_empty() || end.is_empty()) {
//         return Err(ClosureParserError::InvalidInput(String::from("closure delimiters must not be empty")));
//     }
//
//     let mut output = ClosureParserOutput::new();
//     let mut word = String::new();
//     let mut captured = String::new();
//     let mut active_closure: Option<usize> = None;
//     let mut rest = s;
//
//     while !rest.is_empty() {
//         if let Some(idx) = active_closure {
//             let end = &closures[idx].1;
//             if rest.starts_with(end) {
//                 active_closure = None;
//                 output.push(ClosureParsedElement::SubElement(std::mem::take(
//                     &mut captured,
//                 )));
//                 rest = &rest[end.len()..];
//                 continue;
//             }
//
//             let ch = rest.chars().next().unwrap();
//             captured.push(ch);
//             rest = &rest[ch.len_utf8()..];
//             continue;
//         }
//
//         let opening_closure = closures
//             .iter()
//             .enumerate()
//             .filter(|(_, (start, _, _))| rest.starts_with(start))
//             .max_by_key(|(_, (start, _, _))| start.len())
//             .map(|(index, _)| index);
//
//         if let Some(index) = opening_closure {
//             if !word.is_empty() {
//                 output.push(ClosureParsedElement::UsualElement(std::mem::take(
//                     &mut word,
//                 )));
//             }
//             active_closure = Some(index);
//             rest = &rest[closures[index].0.len()..];
//             continue;
//         }
//
//         let ch = rest.chars().next().unwrap();
//         rest = &rest[ch.len_utf8()..];
//         match ch.is_whitespace() {
//             true if !word.is_empty() => {
//                 output.push(ClosureParsedElement::UsualElement(std::mem::take(
//                     &mut word,
//                 )));
//             }
//             false => word.push(ch),
//             _ => {}
//         }
//     }
//
//     if let Some(index) = active_closure {
//         return Err(ClosureParserError::UnterminatedClosure(
//             closures[index].1.clone(),
//         ));
//     }
//     if !word.is_empty() {
//         output.push(ClosureParsedElement::UsualElement(word));
//     }
//     Ok(output)
// }

