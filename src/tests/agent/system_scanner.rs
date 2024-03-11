use crate::agent::system_scanner::Scanner;

#[test]
fn next_int_basic() {
    let mut scanner = Scanner::new();

    scanner.feed("2478".to_string());

    assert_eq!(scanner.next_int().unwrap(), 2478)
}

#[test]
fn next_int_ignore_characters() {
    let mut scanner = Scanner::new();
    scanner.feed("Hello world, number here: 245".to_string());
    assert_eq!(scanner.next_int().unwrap(), 245)
}

#[test]
fn next_int_newline_delimiter() {
    let mut scanner = Scanner::new();
    scanner.feed("123".to_string());
    scanner.feed("456".to_string());
    assert_eq!(scanner.next_int().unwrap(), 123);
    assert_eq!(scanner.next_int().unwrap(), 456);
}

#[test]
fn next_int_return_max_on_overflow() {
    let mut scanner = Scanner::new();
    scanner.feed("173298127398127398127398127398217398217398217398127398".to_string());
    assert_eq!(scanner.next_int().unwrap(), u64::MAX);
}

#[test]
fn next_int_return_none_on_empty() {
    let mut scanner = Scanner::new();
    assert_eq!(scanner.next_int(), None);
}

#[test]
fn next_int_return_none_on_no_matches() {
    let mut scanner = Scanner::new();
    scanner.feed("This is a non-numeric string.".to_string());
    assert_eq!(scanner.next_int(), None);
}

#[test]
fn next_double_basic() {
    let mut scanner = Scanner::new();

    scanner.feed("2478".to_string());

    assert_eq!(scanner.next_double().unwrap(), 2478f64)
}

#[test]
fn next_double_decimal_point() {
    let mut scanner = Scanner::new();

    scanner.feed("123122.123111".to_string());

    assert_eq!(scanner.next_double().unwrap(), 123122.123111)
}

#[test]
fn next_double_allow_trailing_decimal() {
    let mut scanner = Scanner::new();

    scanner.feed("123123.".to_string());

    assert_eq!(scanner.next_double().unwrap(), 123123f64);
}

#[test]
fn next_double_ignore_characters() {
    let mut scanner = Scanner::new();
    scanner.feed("Hello world, number here: 245".to_string());
    assert_eq!(scanner.next_double().unwrap(), 245f64)
}

#[test]
fn next_double_newline_delimiter() {
    let mut scanner = Scanner::new();
    scanner.feed("123".to_string());
    scanner.feed("456".to_string());
    assert_eq!(scanner.next_double().unwrap(), 123f64);
    assert_eq!(scanner.next_double().unwrap(), 456f64);
}

#[test]
fn next_double_return_max_on_overflow() {
    let mut scanner = Scanner::new();
    // The max double value is very large
    scanner.feed("1797693134862315708145274237317043567980705675258449965989174768031572607800285387605895586327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687381771809192998812504040261841248583680".to_string());
    assert_eq!(scanner.next_double().unwrap(), f64::INFINITY);
}

#[test]
fn next_double_return_none_on_empty() {
    let mut scanner = Scanner::new();
    assert_eq!(scanner.next_double(), None);
}

#[test]
fn next_double_return_none_on_no_matches() {
    let mut scanner = Scanner::new();
    scanner.feed("This is a non-numeric string.".to_string());
    assert_eq!(scanner.next_double(), None);
}

#[test]
fn next_line_basic() {
    let mut scanner = Scanner::new();
    scanner.feed("This is a line123".to_string());
    scanner.feed("This is another line".to_string());
    assert_eq!(scanner.next_line().unwrap(), "This is a line123");
    scanner.feed("This is a final line456".to_string());
    assert_eq!(scanner.next_line().unwrap(), "This is another line");
    assert_eq!(scanner.next_line().unwrap(), "This is a final line456");
}

#[test]
fn mixed_reads() {
    let mut scanner = Scanner::new();
    scanner.feed("This should be ignored | 123.45".to_string());
    scanner.feed("Hello world!".to_string());
    scanner.feed("44 <-- int float --> 10.2".to_string());
    scanner.feed("Read as an int, then a float: 56.2".to_string());
    assert_eq!(scanner.next_float().unwrap(), 123.45f32);
    // Since the only thing left in the buffer is the newline from the last string.
    // Java does this, I promise people are used to it.
    assert_eq!(scanner.next_line().unwrap(), "");
    assert_eq!(scanner.next_line().unwrap(), "Hello world!");
    assert_eq!(scanner.next_int().unwrap(), 44);
    assert_eq!(scanner.next_double().unwrap(), 10.2f64);
    assert_eq!(scanner.next_int().unwrap(), 56);
    assert_eq!(scanner.next_double().unwrap(), 2f64);
}
