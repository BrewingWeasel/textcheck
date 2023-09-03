use std::str::Lines;
mod full_word;

#[derive(Debug, PartialEq, Eq)]
pub struct Mistake<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub name: &'a str,
}

struct MultipleSpaces {
    initial: usize,
    was_last: bool,
    last_line_indented: bool,
}

struct LowerCaseI {}

#[derive(Debug)]
struct MDash {
    was_space_before_hyphen: bool,
    initial: usize,
    only_space: bool,
}

struct QuotePositioning {}

struct CapitalizeAfterSentence {
    was_punc_before_whitespace: bool,
}

struct SpaceBeforePunc {}

struct Quotes {
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
            self.quote_level_starts.push((line, index))
        }
        None
    }

    fn new() -> Self {
        Quotes {
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
        LowerCaseI {}
    }
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
        MDash {
            was_space_before_hyphen: false,
            only_space: true,
            initial: 0,
        }
    }
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
        CapitalizeAfterSentence {
            was_punc_before_whitespace: false,
        }
    }
}

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
        QuotePositioning {}
    }
}

impl EachCharacter for SpaceBeforePunc {
    fn check<'a>(
        &mut self,
        c: char,
        index: usize,
        _line: usize,
        shared: &Shared,
        _max_index: usize,
    ) -> Option<(usize, usize, &'a str)> {
        if [',', '.', '!', '?', ';', ':'].contains(&c) && shared.last_char.is_ascii_whitespace() {
            Some((
                index.saturating_sub(1),
                index.saturating_sub(1),
                "There shouldn't be a space before punctuation",
            ))
        } else {
            None
        }
    }

    fn new() -> Self {
        SpaceBeforePunc {}
    }
}

impl EachCharacter for MultipleSpaces {
    fn check<'a>(
        &mut self,
        c: char,
        index: usize,
        _line: usize,
        shared: &Shared,
        max_index: usize,
    ) -> Option<(usize, usize, &'a str)> {
        if c.is_ascii_whitespace() && shared.last_char == '-' {
            self.last_line_indented = true;
        }
        if self.was_last {
            if !c.is_ascii_whitespace() {
                self.was_last = false;
                if self.initial == 1 && (c == '-' || self.last_line_indented) {
                    self.last_line_indented = true;
                } else {
                    self.last_line_indented = false;
                    return Some((
                        self.initial.saturating_sub(1),
                        index.saturating_sub(1),
                        "Multiple spaces used instead of one",
                    ));
                }
            }
            if max_index == index {
                self.was_last = false;
                return Some((
                    self.initial.saturating_sub(1),
                    index,
                    "Extra whitespace at end of line",
                ));
            }
        } else if c.is_ascii_whitespace() {
            if max_index == index {
                if shared.last_char.is_ascii_whitespace() {
                    return Some((
                        index.saturating_sub(1),
                        index,
                        "Extra whitespace at end of line",
                    ));
                } else {
                    return Some((index, index, "Extra whitespace at end of line"));
                }
            } else if shared.last_char.is_ascii_whitespace() {
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
            last_line_indented: false,
        }
    }
}

struct InCodeBlock {
    inblock: bool,
}

impl CheckLocked for InCodeBlock {
    fn check<'a>(&mut self, c: char, shared: &Shared) -> bool {
        if c == '`' && shared.last_char == '`' && shared.char_before_last == '`' {
            self.inblock = !self.inblock;
        }
        self.inblock
    }

    fn new() -> Self {
        InCodeBlock { inblock: false }
    }
}

trait CheckLocked {
    fn check<'a>(&mut self, c: char, shared: &Shared) -> bool;
    fn new() -> Self
    where
        Self: Sized;
}

trait EachCharacter {
    fn check<'a>(
        &mut self,
        c: char,
        index: usize,
        line: usize,
        shared: &Shared,
        max_index: usize,
    ) -> Option<(usize, usize, &'a str)>;
    fn on_ending<'a>(&self) -> Option<(usize, usize, usize, &'a str)> {
        None
    }
    fn on_line_ending(&mut self) {}
    fn new() -> Self
    where
        Self: Sized;
}

pub struct Shared {
    last_char: char,
    char_before_last: char,
}

impl Shared {
    fn update(&mut self, _index: usize, c: char) {
        self.char_before_last = self.last_char;
        self.last_char = c;
    }
}

pub fn check(initial: &str) -> Vec<Mistake> {
    let mut mistakes = Vec::new();

    let mut all_chars: Vec<Box<dyn EachCharacter>> = vec![
        Box::new(MultipleSpaces::new()),
        Box::new(QuotePositioning::new()),
        Box::new(LowerCaseI::new()),
        Box::new(MDash::new()),
        Box::new(CapitalizeAfterSentence::new()),
        Box::new(full_word::WordCapitalization::new()),
        Box::new(SpaceBeforePunc::new()),
        Box::new(Quotes::new()),
    ];

    let mut decide_to_run_checks: Vec<Box<dyn CheckLocked>> = vec![Box::new(InCodeBlock::new())];

    let mut shared = Shared {
        last_char: ' ',
        char_before_last: ' ',
    };

    for (i, line) in initial.lines().enumerate() {
        shared.char_before_last = ' ';
        let line_length = line.len().saturating_sub(1);
        'chars: for (ind, char) in line.char_indices() {
            for should_continue in &mut decide_to_run_checks {
                if should_continue.check(char, &shared) {
                    shared.update(ind, char);
                    continue 'chars;
                }
            }
            for catch in &mut all_chars {
                if let Some((start, end, name)) = catch.check(char, ind, i, &shared, line_length) {
                    mistakes.push(Mistake {
                        line: i,
                        start,
                        end,
                        name,
                    });
                }
            }
            shared.update(ind, char);
        }
        for catch in &mut all_chars {
            catch.on_line_ending();
        }
    }
    for catch in &mut all_chars {
        if let Some((start, end, line, name)) = catch.on_ending() {
            mistakes.push(Mistake {
                line,
                start,
                end,
                name,
            });
        }
    }
    mistakes
}

pub fn display(mistake: Mistake, mut lines: Lines) {
    let mut line = lines.nth(mistake.line).unwrap().chars();
    print!(
        "\x1b[31mLine {}\x1b[0m (\x1b[33m{}-{}\x1b[0m) {}: ",
        mistake.line, mistake.start, mistake.end, mistake.name
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
