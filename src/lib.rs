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

#[derive(Debug)]
struct InTable {
    intable: bool,
    last_line_was_table_start: bool,
    cur_line_table_start: bool,
    was_in_table: bool,
}

impl CheckLocked for InTable {
    fn check(
        &mut self,
        c: char,
        index: usize,
        max_index: usize,
        _shared: &Shared,
    ) -> ContinueState {
        if self.intable {
            if (index == 0 || index == max_index) && c != '|' {
                self.was_in_table = true;
                self.intable = false;
            }
        } else if index == 0 && c == '|' && !self.last_line_was_table_start && !self.was_in_table {
            self.cur_line_table_start = true;
        } else if self.cur_line_table_start && index == max_index && c == '|' {
            // If the first possible line of the table ends with a pipe, check the next line to
            // confirm that it is a table
            self.cur_line_table_start = false;
            self.last_line_was_table_start = true;
        } else if self.last_line_was_table_start {
            // If the line that would be seperating the column headers isn't only made up of valid
            // characters, we aren't in a table
            if !c.is_ascii_whitespace() && ![':', '-', '|'].contains(&c) {
                self.was_in_table = true;
                self.last_line_was_table_start = false;
            } else if index == max_index {
                self.intable = true;
                self.last_line_was_table_start = false;
            }
        } else if (index == 0 || index == max_index) && c != '|' && self.was_in_table {
            self.was_in_table = false;
        }
        if self.intable {
            ContinueState::True
        } else if self.last_line_was_table_start || self.cur_line_table_start {
            ContinueState::Possible
        } else {
            ContinueState::False
        }
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        InTable {
            intable: false,
            last_line_was_table_start: false,
            cur_line_table_start: false,
            was_in_table: false,
        }
    }
}

impl CheckLocked for InCodeBlock {
    fn check<'a>(
        &mut self,
        c: char,
        _index: usize,
        _max_index: usize,
        _shared: &Shared,
    ) -> ContinueState {
        if c == '`' {
            self.inblock = !self.inblock;
        }
        ContinueState::from_bool(self.inblock)
    }

    fn new() -> Self {
        Self { inblock: false }
    }
}

pub enum ContinueState {
    True,
    Possible,
    False,
}

impl ContinueState {
    pub fn from_bool(b: bool) -> Self {
        if b {
            ContinueState::True
        } else {
            ContinueState::False
        }
    }
}

trait CheckLocked {
    fn check(&mut self, c: char, index: usize, max_index: usize, shared: &Shared) -> ContinueState;
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

    let mut decide_to_run_checks: Vec<Box<dyn CheckLocked>> =
        vec![Box::new(InCodeBlock::new()), Box::new(InTable::new())];

    let mut shared = Shared {
        last_char: ' ',
        char_before_last: ' ',
        fake_whitespace: true,
        last_numeric: false,
        last_indent: 0,
        has_had_starter: false,
    };

    let mut in_possible_continue_state: Vec<bool> = vec![false; decide_to_run_checks.len()];
    let mut possible_mistakes: Vec<Mistake> = Vec::new();

    for (i, line) in initial.lines().enumerate() {
        shared.char_before_last = ' ';
        shared.fake_whitespace = true;
        let line_length = line.len().saturating_sub(1);
        shared.update_line();

        'chars: for (ind, char) in line.char_indices() {
            for (check_i, should_continue) in decide_to_run_checks.iter_mut().enumerate() {
                match should_continue.check(char, ind, line_length, &shared) {
                    ContinueState::True => {
                        if in_possible_continue_state[check_i] {
                            possible_mistakes.clear();
                            in_possible_continue_state[check_i] = false;
                        }
                        shared.update(ind, char);
                        continue 'chars;
                    }
                    ContinueState::Possible => {
                        in_possible_continue_state[check_i] = true;
                    }
                    ContinueState::False => {
                        if in_possible_continue_state[check_i] {
                            mistakes.append(&mut possible_mistakes);
                            in_possible_continue_state[check_i] = false;
                        }
                    }
                }
            }

            for catch in &mut all_chars {
                if let Some((start, end, name)) = catch.check(char, ind, i, &shared, line_length) {
                    let mistake = Mistake {
                        line: i,
                        start,
                        end,
                        name,
                    };
                    if in_possible_continue_state.iter().any(|x| *x) {
                        possible_mistakes.push(mistake);
                    } else {
                        mistakes.push(mistake);
                    }
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
