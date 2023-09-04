use crate::{EachCharacter, Shared};

pub struct SpaceBeforePunc {}

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
        Self {}
    }
}

pub struct MultipleSpaces {
    initial: usize,
    was_last: bool,
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
        let starts_with_bullet_point = c.is_ascii_whitespace() && shared.last_char == '-';

        if self.was_last {
            if !c.is_ascii_whitespace() {
                self.was_last = false;
                if self.initial == 1
                    && (c == '-' || starts_with_bullet_point || shared.last_indent == index)
                {
                } else {
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
                }
                return Some((index, index, "Extra whitespace at end of line"));
            } else if shared.last_char.is_ascii_whitespace() {
                self.was_last = true;
                self.initial = index;
            }
        }
        None
    }

    fn new() -> Self {
        Self {
            initial: 0,
            was_last: false,
        }
    }
}
