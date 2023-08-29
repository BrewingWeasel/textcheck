use std::str::Lines;

#[derive(Debug, PartialEq, Eq)]
pub struct Mistake {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

struct MultipleSpaces {
    initial: usize,
    was_last: bool,
}

struct LowerCaseI {
    char_before_last: char,
}

struct MDash {
    was_space_before_hyphen: bool,
    initial: usize,
}

struct QuotePositioning {}

struct CapitalizeAfterSentence {
    was_punc_before_whitespace: bool,
}

impl EachCharacter for LowerCaseI {
    fn check(
        &mut self,
        c: char,
        index: usize,
        last_char: char,
        _max_index: usize,
    ) -> Option<(usize, usize)> {
        if last_char == 'i'
            && self.char_before_last.is_ascii_whitespace()
            && c.is_ascii_whitespace()
        {
            self.char_before_last = last_char;
            Some((index.saturating_sub(1), index.saturating_sub(1)))
        } else {
            self.char_before_last = last_char;
            None
        }
    }

    fn new() -> Self {
        LowerCaseI {
            char_before_last: ' ',
        }
    }
}

impl EachCharacter for MDash {
    fn check(
        &mut self,
        c: char,
        index: usize,
        last_char: char,
        max_index: usize,
    ) -> Option<(usize, usize)> {
        if self.was_space_before_hyphen {
            if c.is_ascii_whitespace() {
                self.was_space_before_hyphen = false;
                return Some((self.initial, index - 1));
            } else if c != '-' {
                self.was_space_before_hyphen = false;
            }
        }

        if c == '-' && last_char.is_ascii_whitespace() {
            if index == max_index {
                return Some((index, index));
            }
            self.initial = index;
            self.was_space_before_hyphen = true;
        }
        None
    }

    fn new() -> Self {
        MDash {
            was_space_before_hyphen: false,
            initial: 0,
        }
    }
}

impl EachCharacter for CapitalizeAfterSentence {
    fn check(
        &mut self,
        c: char,
        index: usize,
        last_char: char,
        max_index: usize,
    ) -> Option<(usize, usize)> {
        if self.was_punc_before_whitespace {
            if c.is_ascii_lowercase() {
                self.was_punc_before_whitespace = false;
                return Some((index, index));
            } else if c.is_ascii_uppercase() {
                self.was_punc_before_whitespace = false;
            }
        }
        if c.is_ascii_whitespace() && ['.', '!', '?'].contains(&last_char)
            || (max_index == index) && ['.', '!', '?'].contains(&c)
        {
            self.was_punc_before_whitespace = true;
        }
        None
    }

    fn new() -> Self {
        CapitalizeAfterSentence {
            was_punc_before_whitespace: false,
        }
    }
}

impl EachCharacter for QuotePositioning {
    fn check(
        &mut self,
        c: char,
        index: usize,
        last_char: char,
        _max_index: usize,
    ) -> Option<(usize, usize)> {
        if [',', '.', '!'].contains(&c) && last_char == '"' {
            Some((index.saturating_sub(1), index))
        } else {
            None
        }
    }

    fn new() -> Self {
        QuotePositioning {}
    }
}

impl EachCharacter for MultipleSpaces {
    fn check(
        &mut self,
        c: char,
        index: usize,
        last_char: char,
        max_index: usize,
    ) -> Option<(usize, usize)> {
        if self.was_last {
            if !c.is_ascii_whitespace() || max_index == index {
                let final_index = if max_index == index {
                    index
                } else {
                    index.saturating_sub(1)
                };
                self.was_last = false;
                return Some((self.initial.saturating_sub(1), final_index));
            }
        } else if c == ' ' {
            if max_index == index {
                if last_char.is_ascii_whitespace() {
                    return Some((index.saturating_sub(1), index));
                } else {
                    return Some((index, index));
                }
            } else if last_char.is_ascii_whitespace() {
                self.was_last = true;
                self.initial = index;
            }
        }
        None
    }

    fn new() -> Self {
        MultipleSpaces {
            initial: 0,
            was_last: false,
        }
    }
}

trait EachCharacter {
    fn check(
        &mut self,
        c: char,
        index: usize,
        last_char: char,
        max_index: usize,
    ) -> Option<(usize, usize)>;
    fn new() -> Self
    where
        Self: Sized;
}

pub fn check(initial: &str) -> Vec<Mistake> {
    let mut mistakes = Vec::new();

    let mut all_chars: Vec<Box<dyn EachCharacter>> = vec![
        Box::new(MultipleSpaces::new()),
        Box::new(QuotePositioning::new()),
        Box::new(LowerCaseI::new()),
        Box::new(MDash::new()),
        Box::new(CapitalizeAfterSentence::new()),
    ];

    for (i, line) in initial.lines().enumerate() {
        let mut last_char = ' ';
        let line_length = line.len().saturating_sub(1);
        for (ind, char) in line.char_indices() {
            for catch in &mut all_chars {
                if let Some((start, end)) = catch.check(char, ind, last_char, line_length) {
                    mistakes.push(Mistake {
                        line: i,
                        start,
                        end,
                    });
                }
            }
            last_char = char;
        }
    }
    mistakes
}

pub fn display(mistake: Mistake, mut lines: Lines) {
    let mut line = lines.nth(mistake.line).unwrap().chars();
    print!(
        "\x1b[31mLine {}\x1b[0m (\x1b[33m{}-{}\x1b[0m): ",
        mistake.line, mistake.start, mistake.end,
    );
    println!(
        "{}\x1b[4:3m\x1b[58:2::240:143:104m{}\x1b[59m\x1b[4:0m{}",
        line.by_ref().take(mistake.start).collect::<String>(),
        line.by_ref()
            .take(mistake.end.wrapping_sub(mistake.start) + 1)
            .collect::<String>(),
        line.collect::<String>(),
    );
}
