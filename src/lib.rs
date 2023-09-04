use std::str::Lines;
mod capitalization;
mod full_word;
mod punctuation;
mod spacing;

#[derive(Debug, PartialEq, Eq)]
pub struct Mistake<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub name: &'a str,
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
        Self { inblock: false }
    }
}

trait CheckLocked {
    fn check(&mut self, c: char, shared: &Shared) -> bool;
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
    fake_whitespace: bool,
    last_numeric: bool,
    has_had_starter: bool,
    last_indent: usize,
}

impl Shared {
    fn update(&mut self, index: usize, c: char) {
        self.char_before_last = self.last_char;
        self.last_char = c;
        if self.fake_whitespace {
            if self.last_numeric {
                if !c.is_numeric() {
                    self.last_numeric = false;
                    if c != '.' {
                        self.fake_whitespace = false;
                        self.last_indent = index;
                    }
                }
            } else if c.is_numeric() && index == 0 {
                self.last_numeric = true;
            } else if !self.has_had_starter && (c == '-' || c == '*') {
                self.has_had_starter = true;
            } else if !c.is_ascii_whitespace() {
                self.fake_whitespace = false;
                self.last_indent = index;
            }
        }
    }
    fn update_line(&mut self) {
        self.has_had_starter = false;
    }
}

pub fn check(initial: &str) -> Vec<Mistake> {
    let mut mistakes = Vec::new();

    let mut all_chars: Vec<Box<dyn EachCharacter>> = vec![
        Box::new(spacing::MultipleSpaces::new()),
        Box::new(punctuation::QuotePositioning::new()),
        Box::new(capitalization::LowerCaseI::new()),
        Box::new(punctuation::MDash::new()),
        Box::new(capitalization::CapitalizeAfterSentence::new()),
        Box::new(full_word::WordCapitalization::new()),
        Box::new(spacing::SpaceBeforePunc::new()),
        Box::new(punctuation::Quotes::new()),
    ];

    let mut decide_to_run_checks: Vec<Box<dyn CheckLocked>> = vec![Box::new(InCodeBlock::new())];

    let mut shared = Shared {
        last_char: ' ',
        char_before_last: ' ',
        fake_whitespace: true,
        last_numeric: false,
        last_indent: 0,
        has_had_starter: false,
    };

    for (i, line) in initial.lines().enumerate() {
        shared.char_before_last = ' ';
        shared.fake_whitespace = true;
        let line_length = line.len().saturating_sub(1);
        shared.update_line();

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

pub fn display(mistake: &Mistake, mut lines: Lines) {
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
