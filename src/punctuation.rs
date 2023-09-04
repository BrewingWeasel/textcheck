use crate::{EachCharacter, Shared};

pub struct Quotes {
    quote_level_starts: Vec<(usize, usize)>,
}

impl EachCharacter for Quotes {
    fn check<'a>(
        &mut self,
        c: char,
        index: usize,
        line: usize,
        _shared: &Shared,
        _max_index: usize,
    ) -> Option<(usize, usize, &'a str)> {
        if c == '"' {
            self.quote_level_starts.push((line, index));
        }
        None
    }

    fn new() -> Self {
        Self {
            quote_level_starts: Vec::new(),
        }
    }

    fn on_ending<'a>(&self) -> Option<(usize, usize, usize, &'a str)> {
        if self.quote_level_starts.len() % 2 == 1 {
            let (line, index) = self.quote_level_starts.last().unwrap();
            Some((*index, *index, *line, "Unmatched quote"))
        } else {
            None
        }
    }
}

pub struct QuotePositioning {}

impl EachCharacter for QuotePositioning {
    fn check<'a>(
        &mut self,
        c: char,
        index: usize,
        _line: usize,
        shared: &Shared,
        _max_index: usize,
    ) -> Option<(usize, usize, &'a str)> {
        if [',', '.', '!'].contains(&c) && shared.last_char == '"' {
            Some((
                index.saturating_sub(1),
                index,
                "The comma should go before the quotation mark",
            ))
        } else {
            None
        }
    }

    fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct MDash {
    was_space_before_hyphen: bool,
    initial: usize,
    only_space: bool,
}

impl EachCharacter for MDash {
    fn check<'a>(
        &mut self,
        c: char,
        index: usize,
        _line: usize,
        shared: &Shared,
        max_index: usize,
    ) -> Option<(usize, usize, &'a str)> {
        if self.was_space_before_hyphen && !self.only_space {
            if c.is_ascii_whitespace() {
                self.was_space_before_hyphen = false;
                return Some((
                    self.initial,
                    index - 1,
                    "Should be em dash (—) instead of hyphen (-)",
                ));
            } else if c != '-' {
                self.was_space_before_hyphen = false;
            }
        }

        if c == '-' && shared.last_char.is_ascii_whitespace() && !self.only_space {
            if index == max_index {
                return Some((index, index, "Should be em dash (—) instead of hyphen (-)"));
            }
            self.initial = index;
            self.was_space_before_hyphen = true;
        }

        if self.only_space {
            self.only_space = c.is_ascii_whitespace();
        }
        None
    }

    fn on_line_ending(&mut self) {
        self.only_space = true;
    }

    fn new() -> Self {
        Self {
            was_space_before_hyphen: false,
            only_space: true,
            initial: 0,
        }
    }
}
