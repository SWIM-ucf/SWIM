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
