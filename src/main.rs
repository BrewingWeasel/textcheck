use std::str::Lines;

fn main() {
    // let value = "Hello,  world!\n HELLO    WOoorlLLDDD  HELL";
    let value = "heloo    world";
    let errors = check(value);
    for error in errors {
        display(error, value.lines())
    }
}

struct Mistake {
    line: usize,
    start: usize,
    end: usize,
}

struct MultipleSpaces {
    initial: usize,
    was_last: bool,
}

impl MultipleSpaces {
    fn check(&mut self, c: char, index: usize, last_char: char) -> Option<usize> {
        if self.was_last {
            if c != ' ' {
                self.was_last = false;
                return Some(self.initial);
            }
        } else if c == ' ' && last_char == ' ' {
            self.was_last = true;
            self.initial = index;
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

trait ErrorVal {
    fn check(&mut self, c: char, index: usize, last_char: char) -> Option<Mistake>;
    fn new() -> Self;
}

fn check(initial: &str) -> Vec<Mistake> {
    let mut mistakes = Vec::new();

    let mut catching = vec![MultipleSpaces::new()];

    for (i, line) in initial.lines().enumerate() {
        let mut last_char = ' ';
        for (ind, char) in line.char_indices() {
            for catch in &mut catching {
                if let Some(start) = catch.check(char, ind, last_char) {
                    mistakes.push(Mistake {
                        line: i,
                        start,
                        end: ind,
                    });
                }
            }
            last_char = char;
        }
    }
    mistakes
}

fn display(mistake: Mistake, mut lines: Lines) {
    let mut line = lines.nth(mistake.line).unwrap().chars();
    println!(
        "{}\x1b[4:3m\x1b[58:2::240:143:104m{}\x1b[59m\x1b[4:0m{}",
        line.by_ref()
            .take(mistake.start.saturating_sub(1))
            .collect::<String>(),
        line.by_ref()
            .take(mistake.end - mistake.start + 1)
            .collect::<String>(),
        line.collect::<String>(),
    );
}
