use textcheck;

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
