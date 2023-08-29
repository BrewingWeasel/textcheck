use crate::EachCharacter;

// TODO: more things, countries, languages etc
const WORDS_TO_CAPITALIZE: [&str; 19] = [
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];

pub struct WordCapitalization {
    word: String,
}

impl EachCharacter for WordCapitalization {
    fn check<'a>(
        &mut self,
        c: char,
        index: usize,
        _last_char: char,
        max_index: usize,
    ) -> Option<(usize, usize, &'a str)> {
        if c.is_ascii_whitespace() || max_index == index || c.is_ascii_punctuation() {
            if WORDS_TO_CAPITALIZE.contains(&self.word.as_str()) {
                let start = index - self.word.len();
                self.word = String::new();
                return Some((
                    start,
                    index - 1,
                    "Days of the week and months should be capitalized",
                ));
            }
            self.word = String::new();
        } else {
            self.word.push(c)
        }
        return None;
    }

    fn new() -> Self {
        WordCapitalization {
            word: String::new(),
        }
    }
}
