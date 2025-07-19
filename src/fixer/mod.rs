use crate::api::Match;

#[derive(Debug)]
pub struct Fixer {}

impl Fixer {
    pub fn all<'a>(og_text: &'a str, suggestions: &'a [Match]) -> String {
        let mut buffer: String = String::from(og_text);
        let mut overall_offset: isize = 0;

        for suggestion in suggestions {
            if suggestion.replacements().is_empty() {
                continue;
            }

            let replacement = suggestion.replacements().first().unwrap();
            let offset: usize = (overall_offset + suggestion.offset() as isize) as usize;
            buffer.replace_range(offset..offset + suggestion.length(), replacement.value());
            overall_offset += replacement.value().len() as isize - suggestion.length() as isize;
        }
        buffer
    }

    pub fn interactive<'a>(og_text: &'a str, suggestions: &'a [Match]) -> String {
        eprintln!("To implement");
        todo!();
    }
}
