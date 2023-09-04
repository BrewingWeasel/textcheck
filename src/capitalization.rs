use crate::{EachCharacter, Shared};

pub struct LowerCaseI {}

impl EachCharacter for LowerCaseI {
    fn check<'a>(
        &mut self,
        c: char,
        index: usize,
        _line: usize,
        shared: &Shared,
        _max_index: usize,
    ) -> Option<(usize, usize, &'a str)> {
        if shared.last_char == 'i'
            && shared.char_before_last.is_ascii_whitespace()
            && (c.is_ascii_whitespace() || c == '\'')
        {
            Some((
                index.saturating_sub(1),
                index.saturating_sub(1),
                "'i' should be uppercase",
            ))
        } else {
            None
        }
    }

    fn new() -> Self {
        Self {}
    }
}

pub struct CapitalizeAfterSentence {
    was_punc_before_whitespace: bool,
}

impl EachCharacter for CapitalizeAfterSentence {
    fn check<'a>(
        &mut self,
        c: char,
        index: usize,
        _line: usize,
        shared: &Shared,
        max_index: usize,
    ) -> Option<(usize, usize, &'a str)> {
        if self.was_punc_before_whitespace {
            if c.is_ascii_lowercase() {
                self.was_punc_before_whitespace = false;
                return Some((
                    index,
                    index,
                    "The first letter after a sentence should be capitalized",
                ));
            } else if c.is_ascii_uppercase() {
                self.was_punc_before_whitespace = false;
            }
        }
        if c.is_ascii_whitespace() && ['.', '!', '?'].contains(&shared.last_char)
            || (max_index == index) && ['.', '!', '?'].contains(&c)
        {
            self.was_punc_before_whitespace = true;
        }
        None
    }

    fn new() -> Self {
        Self {
            was_punc_before_whitespace: false,
        }
    }
}
