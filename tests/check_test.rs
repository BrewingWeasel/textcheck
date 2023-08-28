#[test]
fn single_line_double_space() {
    assert_eq!(
        textcheck::check("oh  no"),
        vec![textcheck::Mistake {
            line: 0,
            start: 2,
            end: 3,
        }]
    );
}

#[test]
fn single_line_triple_space() {
    assert_eq!(
        textcheck::check("oh   no"),
        vec![textcheck::Mistake {
            line: 0,
            start: 2,
            end: 4
        }]
    );
}
#[test]
fn single_line_triple_space_ending() {
    assert_eq!(
        textcheck::check("oh no   "),
        vec![textcheck::Mistake {
            line: 0,
            start: 5,
            end: 7
        }]
    );
}

#[test]
fn single_line_space_ending_and_beginning() {
    assert_eq!(
        textcheck::check(" oh no "),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 0,
                end: 0
            },
            textcheck::Mistake {
                line: 0,
                start: 6,
                end: 6
            },
        ]
    );
}

#[test]
fn single_line_double_space_ending_and_beginning() {
    assert_eq!(
        textcheck::check("  oh no  "),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 0,
                end: 1
            },
            textcheck::Mistake {
                line: 0,
                start: 7,
                end: 8
            },
        ]
    );
}

#[test]
fn space_ending_and_beginning_and_two_blank_lines() {
    assert_eq!(
        textcheck::check(" oh no \n\n"),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 0,
                end: 0
            },
            textcheck::Mistake {
                line: 0,
                start: 6,
                end: 6
            },
        ]
    );
}

#[test]
fn space_before_blank_line() {
    assert_eq!(
        textcheck::check(" \n"),
        vec![textcheck::Mistake {
            line: 0,
            start: 0,
            end: 0
        },]
    );
}

#[test]
fn space_before_blank_lines() {
    assert_eq!(
        textcheck::check(" \n \n \n "),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 0,
                end: 0
            },
            textcheck::Mistake {
                line: 1,
                start: 0,
                end: 0
            },
            textcheck::Mistake {
                line: 2,
                start: 0,
                end: 0
            },
            textcheck::Mistake {
                line: 3,
                start: 0,
                end: 0
            },
        ]
    );
}

#[test]
fn space_before_misplaced_quote() {
    assert_eq!(
        textcheck::check(" \","),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 0,
                end: 0
            },
            textcheck::Mistake {
                line: 0,
                start: 1,
                end: 2
            },
        ]
    );
}

#[test]
fn two_misplaced_quotes() {
    assert_eq!(
        textcheck::check("\",\","),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 0,
                end: 1
            },
            textcheck::Mistake {
                line: 0,
                start: 2,
                end: 3
            },
        ]
    );
}

#[test]
fn lowercase_i() {
    assert_eq!(
        textcheck::check("i say uh oh"),
        vec![textcheck::Mistake {
            line: 0,
            start: 0,
            end: 0
        },]
    );
}

#[test]
fn lowercase_i_then_two_spaces() {
    assert_eq!(
        textcheck::check("i  say uh oh"),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 0,
                end: 0
            },
            textcheck::Mistake {
                line: 0,
                start: 1,
                end: 2
            },
        ]
    );
}

#[test]
fn empty() {
    assert_eq!(textcheck::check(""), Vec::new());
}

#[test]
fn large_text_no_errors() {
    assert_eq!(textcheck::check("
I'd just like to interject for a moment. What you're referring to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating system unto itself, but rather another free component of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX. Many computer users run a modified version of the GNU system every day, without realizing it. Through a peculiar turn of events, the version of GNU which is widely used today is often called “Linux,” and many of its users are not aware that it is basically the GNU system, developed by the GNU Project. There really is a Linux, and these people are using it, but it is just a part of the system they use.

Linux is the kernel: the program in the system that allocates the machine's resources to the other programs that you run. The kernel is an essential part of an operating system, but useless by itself; it can only function in the context of a complete operating system. Linux is normally used in combination with the GNU operating system: the whole system is basically GNU with Linux added, or GNU/Linux. All the so-called “Linux” distributions are really distributions of GNU/Linux.
"), Vec::new());
}

#[test]
fn correctly_placed_quote() {
    assert_eq!(textcheck::check(",\""), Vec::new());
}
