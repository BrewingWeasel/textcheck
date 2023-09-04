#[test]
fn single_line_double_space() {
    assert_eq!(
        textcheck::check("oh  no"),
        vec![textcheck::Mistake {
            line: 0,
            start: 2,
            end: 3,
            name: "Multiple spaces used instead of one"
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
            end: 4,
            name: "Multiple spaces used instead of one",
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
            end: 7,
            name: "Extra whitespace at end of line",
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
                end: 0,
                name: "Multiple spaces used instead of one",
            },
            textcheck::Mistake {
                line: 0,
                start: 6,
                end: 6,
                name: "Extra whitespace at end of line",
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
                end: 1,
                name: "Multiple spaces used instead of one",
            },
            textcheck::Mistake {
                line: 0,
                start: 7,
                end: 8,
                name: "Extra whitespace at end of line",
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
                end: 0,
                name: "Multiple spaces used instead of one",
            },
            textcheck::Mistake {
                line: 0,
                start: 6,
                end: 6,
                name: "Extra whitespace at end of line",
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
            end: 0,
            name: "Extra whitespace at end of line",
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
                end: 0,
                name: "Extra whitespace at end of line",
            },
            textcheck::Mistake {
                line: 1,
                start: 0,
                end: 0,
                name: "Extra whitespace at end of line",
            },
            textcheck::Mistake {
                line: 2,
                start: 0,
                end: 0,
                name: "Extra whitespace at end of line",
            },
            textcheck::Mistake {
                line: 3,
                start: 0,
                end: 0,
                name: "Extra whitespace at end of line",
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
                end: 0,
                name: "Multiple spaces used instead of one",
            },
            textcheck::Mistake {
                line: 0,
                start: 1,
                end: 2,
                name: "The comma should go before the quotation mark",
            },
            textcheck::Mistake {
                line: 0,
                start: 1,
                end: 1,
                name: "Unmatched quote"
            }
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
                end: 1,
                name: "The comma should go before the quotation mark",
            },
            textcheck::Mistake {
                line: 0,
                start: 2,
                end: 3,
                name: "The comma should go before the quotation mark",
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
            end: 0,
            name: "'i' should be uppercase",
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
                end: 0,
                name: "'i' should be uppercase",
            },
            textcheck::Mistake {
                line: 0,
                start: 1,
                end: 2,
                name: "Multiple spaces used instead of one",
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
    assert_eq!(textcheck::check("\"d,\""), Vec::new());
}

#[test]
fn hypen() {
    assert_eq!(
        textcheck::check("That's true - sort of."),
        vec![textcheck::Mistake {
            line: 0,
            start: 12,
            end: 12,
            name: "Should be em dash (—) instead of hyphen (-)",
        },]
    );
}

#[test]
fn twohypens() {
    assert_eq!(
        textcheck::check("That's true -- sort of."),
        vec![textcheck::Mistake {
            line: 0,
            start: 12,
            end: 13,
            name: "Should be em dash (—) instead of hyphen (-)",
        },]
    );
}

#[test]
fn doublespace_on_both_sides_of_triple_hyphen() {
    assert_eq!(
        textcheck::check("That's true  ---  sort of."),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 11,
                end: 12,
                name: "Multiple spaces used instead of one",
            },
            textcheck::Mistake {
                line: 0,
                start: 13,
                end: 15,
                name: "Should be em dash (—) instead of hyphen (-)",
            },
            textcheck::Mistake {
                line: 0,
                start: 16,
                end: 17,
                name: "Multiple spaces used instead of one",
            },
        ]
    );
}

#[test]
fn hyphenated_word() {
    assert_eq!(textcheck::check("super-awesome"), Vec::new());
}

#[test]
fn hypen_then_lowercase_i() {
    assert_eq!(
        textcheck::check("That's true - i think."),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 12,
                end: 12,
                name: "Should be em dash (—) instead of hyphen (-)",
            },
            textcheck::Mistake {
                line: 0,
                start: 14,
                end: 14,
                name: "'i' should be uppercase",
            },
        ]
    );
}

#[test]
fn hypen_then_doublespace_lowercase_i() {
    assert_eq!(
        textcheck::check("That's true -  i think."),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 12,
                end: 12,
                name: "Should be em dash (—) instead of hyphen (-)",
            },
            textcheck::Mistake {
                line: 0,
                start: 13,
                end: 14,
                name: "Multiple spaces used instead of one",
            },
            textcheck::Mistake {
                line: 0,
                start: 15,
                end: 15,
                name: "'i' should be uppercase",
            },
        ]
    );
}

#[test]
fn hypen_end_of_line() {
    assert_eq!(
        textcheck::check("That's true -\nsort of."),
        vec![textcheck::Mistake {
            line: 0,
            start: 12,
            end: 12,
            name: "Should be em dash (—) instead of hyphen (-)",
        },]
    );
}

#[test]
fn capitalize_first_letter() {
    assert_eq!(
        textcheck::check("Hi! how are you?"),
        vec![textcheck::Mistake {
            line: 0,
            start: 4,
            end: 4,
            name: "The first letter after a sentence should be capitalized",
        },]
    );
}

#[test]
fn capitalize_first_letter_after_three_spaces() {
    assert_eq!(
        textcheck::check("Hi!   how are you?"),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 3,
                end: 5,
                name: "Multiple spaces used instead of one",
            },
            textcheck::Mistake {
                line: 0,
                start: 6,
                end: 6,
                name: "The first letter after a sentence should be capitalized",
            },
        ]
    );
}

#[test]
fn capitalize_first_letter_twice() {
    assert_eq!(
        textcheck::check("Hi! how are you? great"),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 4,
                end: 4,
                name: "The first letter after a sentence should be capitalized",
            },
            textcheck::Mistake {
                line: 0,
                start: 17,
                end: 17,
                name: "The first letter after a sentence should be capitalized",
            },
        ]
    );
}

#[test]
fn no_capitalized_needed() {
    assert_eq!(textcheck::check("Hi."), Vec::new(),);
}

#[test]
fn no_capitalized_needed_two_sents() {
    assert_eq!(textcheck::check("Hi. That's true."), Vec::new(),);
}

#[test]
fn capitalize_first_letter_new_line() {
    assert_eq!(
        textcheck::check("How are you?\nbad"),
        vec![textcheck::Mistake {
            line: 1,
            start: 0,
            end: 0,
            name: "The first letter after a sentence should be capitalized",
        },]
    );
}

#[test]
fn capitalize_first_letter_new_line_after_space() {
    assert_eq!(
        textcheck::check("How are you? \nbad"),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 12,
                end: 12,
                name: "Extra whitespace at end of line",
            },
            textcheck::Mistake {
                line: 1,
                start: 0,
                end: 0,
                name: "The first letter after a sentence should be capitalized",
            },
        ]
    );
}

#[test]
fn i_then_apostrophe() {
    assert_eq!(
        textcheck::check("i'd do that."),
        vec![textcheck::Mistake {
            line: 0,
            start: 0,
            end: 0,
            name: "'i' should be uppercase",
        },]
    );
}

#[test]
fn i_then_apostrophe_2() {
    assert_eq!(
        textcheck::check("That's true, i'd agree."),
        vec![textcheck::Mistake {
            line: 0,
            start: 13,
            end: 13,
            name: "'i' should be uppercase",
        },]
    );
}

#[test]
fn i_then_apostrophe_no_space_before() {
    assert_eq!(textcheck::check("randomi's"), Vec::new());
}

#[test]
fn capitalize_weekday() {
    assert_eq!(
        textcheck::check("Today is friday."),
        vec![textcheck::Mistake {
            line: 0,
            start: 9,
            end: 14,
            name: "Days of the week and months should be capitalized",
        },]
    );
}

#[test]
fn capitalize_weekday_after_double_space() {
    assert_eq!(
        textcheck::check("Today is  friday."),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 8,
                end: 9,
                name: "Multiple spaces used instead of one",
            },
            textcheck::Mistake {
                line: 0,
                start: 10,
                end: 15,
                name: "Days of the week and months should be capitalized",
            },
        ]
    );
}

#[test]
fn capitalize_weekday_before_space() {
    assert_eq!(
        textcheck::check("Today is  friday I think."),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 8,
                end: 9,
                name: "Multiple spaces used instead of one",
            },
            textcheck::Mistake {
                line: 0,
                start: 10,
                end: 15,
                name: "Days of the week and months should be capitalized",
            },
        ]
    );
}

#[test]
fn capitalize_weekday_after_new_line_one_sentence() {
    assert_eq!(
        textcheck::check("Today is\nfriday I think."),
        vec![textcheck::Mistake {
            line: 1,
            start: 0,
            end: 5,
            name: "Days of the week and months should be capitalized",
        },]
    );
}

#[test]
fn already_capitalized() {
    assert_eq!(textcheck::check("Monday is the day today."), Vec::new());
}

#[test]
fn punctuation_after_space() {
    assert_eq!(
        textcheck::check("hi ."),
        vec![textcheck::Mistake {
            line: 0,
            start: 2,
            end: 2,
            name: "There shouldn't be a space before punctuation",
        },]
    );
}

#[test]
fn punctuation_after_tab() {
    assert_eq!(
        textcheck::check("hi\t."),
        vec![textcheck::Mistake {
            line: 0,
            start: 2,
            end: 2,
            name: "There shouldn't be a space before punctuation",
        },]
    );
}

#[test]
fn punctuation_after_space2() {
    assert_eq!(
        textcheck::check("hi !"),
        vec![textcheck::Mistake {
            line: 0,
            start: 2,
            end: 2,
            name: "There shouldn't be a space before punctuation",
        },]
    );
}

#[test]
fn unmatched_quote() {
    assert_eq!(
        textcheck::check("\"hi!"),
        vec![textcheck::Mistake {
            line: 0,
            start: 0,
            end: 0,
            name: "Unmatched quote",
        },]
    );
}

#[test]
fn unmatched_quote_multiple_lines() {
    assert_eq!(
        textcheck::check("\"hi!\n\n\n"),
        vec![textcheck::Mistake {
            line: 0,
            start: 0,
            end: 0,
            name: "Unmatched quote",
        },]
    );
}

#[test]
fn matched_quotes_multiple_lines() {
    assert_eq!(
        textcheck::check("\"Hi!\"\n\"Hello!\n\"\"Stuff.\""),
        Vec::new(),
    );
}

#[test]
fn unmatched_quotes_multiple_lines() {
    assert_eq!(
        textcheck::check("\"Hi!\"\n\"Hello!\n\"Stuff.\""),
        vec![textcheck::Mistake {
            line: 2,
            start: 7,
            end: 7,
            name: "Unmatched quote",
        }],
    );
}

#[test]
fn matched_quote() {
    assert_eq!(textcheck::check("\"hi!\""), Vec::new());
}

#[test]
fn matched_quote_two_lines() {
    assert_eq!(textcheck::check("\"hi!\nHow are you?\""), Vec::new());
}

#[test]
fn markdown_list() {
    assert_eq!(
        textcheck::check(
            "- one
- two
- three
"
        ),
        Vec::new()
    );
}

#[test]
fn markdown_list_indented_part() {
    assert_eq!(
        textcheck::check(
            "- one
- two
  - 2
  - 2.5
- three
  - 3
"
        ),
        Vec::new()
    );
}

#[test]
fn markdown_list_indented_double_space() {
    assert_eq!(
        textcheck::check(
            "- one
-  two
  - 2
  - 2.5
-  three
  - 3
"
        ),
        vec![
            textcheck::Mistake {
                line: 1,
                start: 1,
                end: 2,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 4,
                start: 1,
                end: 2,
                name: "Multiple spaces used instead of one"
            }
        ]
    );
}

#[test]
fn markdown_codeblock() {
    assert_eq!(
        textcheck::check(
            "For example:
```python
now    anything  should     be   ok
```
"
        ),
        Vec::new()
    );
}

#[test]
fn markdown_codeblock_single_line() {
    assert_eq!(
        textcheck::check(
            "For example:
```now    anything  should     be   ok```
"
        ),
        Vec::new()
    );
}

#[test]
fn markdown_codeblock_single_line_start_with() {
    assert_eq!(
        textcheck::check("```now    anything  should     be   ok```"),
        Vec::new()
    );
}

#[test]
fn markdown_codeblock_error_after() {
    assert_eq!(
        textcheck::check(
            "For example:
```now    anything  should     be   ok```
but  not here.
"
        ),
        vec![textcheck::Mistake {
            line: 2,
            start: 3,
            end: 4,
            name: "Multiple spaces used instead of one"
        }]
    );
}

#[test]
fn two_markdown_codeblocks_error_between() {
    assert_eq!(
        textcheck::check(
            "For example:
```now    anything  should     be   ok```
but  not here. ```but    again    here```
"
        ),
        vec![textcheck::Mistake {
            line: 2,
            start: 3,
            end: 4,
            name: "Multiple spaces used instead of one"
        }]
    );
}

#[test]
fn empty_code_block() {
    assert_eq!(textcheck::check("``````"), Vec::new());
}

#[test]
fn two_backticks() {
    assert_eq!(
        textcheck::check("``propblem  here``"),
        vec![textcheck::Mistake {
            line: 0,
            start: 10,
            end: 11,
            name: "Multiple spaces used instead of one"
        }]
    );
}

#[test]
fn markdown_list_indented_extra_lines() {
    assert_eq!(
        textcheck::check(
            "- one
- two
  yes
  - 2
    two
  - 2.5
    yes
- three
  - 3
    yes
"
        ),
        Vec::new()
    );
}

#[test]
fn markdown_list_indented_extra_lines_bold_beginning() {
    assert_eq!(
        textcheck::check(
            "- **hello**
  there
  there again"
        ),
        Vec::new()
    );
}

#[test]
fn markdown_list_indented_extra_lines_wrong_indent() {
    assert_eq!(
        textcheck::check(
            "- one
- two
   yes
  - 2
    two
  - 2.5
   yes
- three
  - 3
    yes
"
        ),
        vec![
            textcheck::Mistake {
                line: 2,
                start: 0,
                end: 2,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 6,
                start: 0,
                end: 2,
                name: "Multiple spaces used instead of one"
            }
        ]
    );
}

#[test]
fn markdown_list_indented_extra_lines_wrong_indent_italic() {
    assert_eq!(
        textcheck::check(
            "- *one*
- *two*
   yes
  - 2
    two
  - 2.5
   *yes*
- three
  - 3
    yes
"
        ),
        vec![
            textcheck::Mistake {
                line: 2,
                start: 0,
                end: 2,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 6,
                start: 0,
                end: 2,
                name: "Multiple spaces used instead of one"
            }
        ]
    );
}

#[test]
fn markdown_table() {
    assert_eq!(
        textcheck::check(
            "random text here
| 1            | 2      | 3     |
| :----------- | :----: | ----: |
| one          | two    | three |"
        ),
        Vec::new()
    );
}

#[test]
fn markdown_table2() {
    assert_eq!(
        textcheck::check(
            "random text here
| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |"
        ),
        Vec::new()
    );
}

#[test]
fn markdown_table3() {
    assert_eq!(
        textcheck::check(
            "| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |"
        ),
        Vec::new()
    );
}

#[test]
fn markdown_table_then_invalid() {
    assert_eq!(
        textcheck::check(
            "| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |
oh  no an error"
        ),
        vec![textcheck::Mistake {
            line: 6,
            start: 2,
            end: 3,
            name: "Multiple spaces used instead of one"
        }]
    );
}

#[test]
fn markdown_table_invalid_both_sides() {
    assert_eq!(
        textcheck::check(
            "\"error here\",
| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |
oh  no an error"
        ),
        vec![
            textcheck::Mistake {
                line: 0,
                start: 11,
                end: 12,
                name: "The comma should go before the quotation mark"
            },
            textcheck::Mistake {
                line: 7,
                start: 2,
                end: 3,
                name: "Multiple spaces used instead of one"
            }
        ]
    );
}

#[test]
fn markdown_table_invalid() {
    assert_eq!(
        textcheck::check(
            "random text here
| 1            | 2      | 3     |
| :----dd----- | :----: | ----: |
| one          | two    | three |"
        ),
        vec![
            textcheck::Mistake {
                line: 1,
                start: 3,
                end: 14,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 1,
                start: 18,
                end: 23,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 1,
                start: 27,
                end: 31,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 2,
                start: 1,
                end: 1,
                name: "There shouldn't be a space before punctuation"
            },
            textcheck::Mistake {
                line: 2,
                start: 16,
                end: 16,
                name: "There shouldn't be a space before punctuation"
            },
            textcheck::Mistake {
                line: 3,
                start: 5,
                end: 14,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 3,
                start: 20,
                end: 23,
                name: "Multiple spaces used instead of one"
            }
        ],
    );
}

#[test]
fn markdown_table_invalid2() {
    assert_eq!(
        textcheck::check(
            "random text here
| 1            | 2      | 3     x
| :----------- | :----: | ----: |
| one          | two    | three |"
        ),
        vec![
            textcheck::Mistake {
                line: 1,
                start: 3,
                end: 14,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 1,
                start: 18,
                end: 23,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 1,
                start: 27,
                end: 31,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 2,
                start: 1,
                end: 1,
                name: "There shouldn't be a space before punctuation"
            },
            textcheck::Mistake {
                line: 2,
                start: 16,
                end: 16,
                name: "There shouldn't be a space before punctuation"
            },
            textcheck::Mistake {
                line: 3,
                start: 5,
                end: 14,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 3,
                start: 20,
                end: 23,
                name: "Multiple spaces used instead of one"
            }
        ],
    );
}

#[test]
fn markdown_table_invalid3() {
    assert_eq!(
        textcheck::check(
            "random text here
| 1            | 2      | 3     |
| :----------- | :----: | ----: |
v one          | two    | three |"
        ),
        vec![
            textcheck::Mistake {
                line: 3,
                start: 5,
                end: 14,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 3,
                start: 20,
                end: 23,
                name: "Multiple spaces used instead of one"
            }
        ],
    );
}

#[test]
fn markdown_table_invalid4() {
    assert_eq!(
        textcheck::check(
            "random text here
| 1            | 2      | 3     |
| :----------- | :----: | ----: |
| one          | two    | three |
| one          | two    | three |
v one          | two    | three |"
        ),
        vec![
            textcheck::Mistake {
                line: 5,
                start: 5,
                end: 14,
                name: "Multiple spaces used instead of one"
            },
            textcheck::Mistake {
                line: 5,
                start: 20,
                end: 23,
                name: "Multiple spaces used instead of one"
            }
        ],
    );
}
