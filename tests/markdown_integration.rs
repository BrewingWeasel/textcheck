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
fn markdown_inline_codeblock() {
    assert_eq!(
        textcheck::check("Ooooh: `now    anything  should     be   ok `"),
        Vec::new()
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
fn markdown_table_and_then_some() {
    assert_eq!(
        textcheck::check(
            "\"no error here,\"
| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |
| one          | two    | three |
and no error here"
        ),
        Vec::new()
    );
}

#[test]
fn good_markdown_table_and_then_again() {
    assert_eq!(
        textcheck::check(
            "\"no error here,\"
| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
and no error here
| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
and none here!"
        ),
        Vec::new()
    );
}

#[test]
fn good_markdown_table_and_then_again_with_pipe() {
    assert_eq!(
        textcheck::check(
            "\"no error here,\"
| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
and no error | here
| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
and none here!"
        ),
        Vec::new()
    );
}

#[test]
fn good_markdown_table_and_then_twice_more_empty_lines() {
    assert_eq!(
        textcheck::check(
            "\"no error here,\"
| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
| vienas       | du     | trys  |

| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
| vienas       | du     | trys  |
| uno          | dos    | tres  |

| 1            | 2      | 3     |
| :----------: | :----: | :---: |
| one          | two    | three |
| vienas       | du     | trys  |"
        ),
        Vec::new()
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
