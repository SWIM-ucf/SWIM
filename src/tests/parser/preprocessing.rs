#[cfg(test)]
use crate::parser::parser_preprocessing::string_cleaning;

#[test]
fn string_cleaning_removes_instances_of_double_spaces() {
    let result = string_cleaning("Double  space".to_string());
    assert_eq!(result, "Double space");
}

#[test]
fn string_cleaning_removes_multiple_instances_of_double_spaces() {
    let result = string_cleaning("THIS  HAS  MULTIPLE  DOUBLE SPACES".to_string());
    assert_eq!(result, "THIS HAS MULTIPLE DOUBLE SPACES");
}

#[test]
fn string_cleaning_culls_all_unnecessary_spaces_down_to_one() {
    let result = string_cleaning("Double  Triple   Triple   Quadruple   .".to_string());
    assert_eq!(result, "Double Triple Triple Quadruple .");
}

#[test]
fn string_cleaning_does_not_delete_single_spaces() {
    let result = string_cleaning("Single Single Double  .".to_string());
    assert_eq!(result, "Single Single Double .");
}

#[test]
fn string_cleaning_removes_extra_lines() {
    let result = string_cleaning("Space\nTwoSpaces\n\nSpace\nSpace".to_string());
    assert_eq!(result, "Space\nTwoSpaces\nSpace\nSpace");
}

#[test]
fn string_cleaning_removes_spaces_at_start_of_lines() {
    let result = string_cleaning("LINE\n SPACE-LINE\n SPACE-LINE\nLINE".to_string());
    assert_eq!(result, "LINE\nSPACE-LINE\nSPACE-LINE\nLINE");
}

#[test]
fn string_cleaning_removes_spaces_at_the_end_of_lines() {
    let result = string_cleaning("LINE\nSPACE-LINE \nSPACE-LINE \nLINE".to_string());
    assert_eq!(result, "LINE\nSPACE-LINE\nSPACE-LINE\nLINE");
}
#[test]
fn string_cleaning_removes_comments_at_the_end_of_a_line() {
    let result = string_cleaning("line\nline#comment\nline".to_string());
    assert_eq!(result, "line\nline\nline");
}
#[test]
fn string_cleaning_removes_comments_on_their_own_line() {
    let result = string_cleaning("line\n #this is a comment \nbut this isn't".to_string());
    assert_eq!(result, "line\nbut this isn't");
}
#[test]
fn string_cleaning_removes_spaces_and_new_lines_at_start_of_string() {
    let result_space = string_cleaning(" space at start\nsecond line".to_string());
    let result_new_line = string_cleaning("\nnew line at start\nsecond line".to_string());
    assert_eq!(result_space, "space at start\nsecond line");
    assert_eq!(result_new_line, "new line at start\nsecond line");
}
#[test]
fn string_cleaning_removes_spaces_and_new_lines_at_end_of_string() {
    let result_space = string_cleaning("line\nspace at end ".to_string());
    let result_new_line = string_cleaning("line\nnew line at end ".to_string());
    assert_eq!(result_space, "line\nspace at end");
    assert_eq!(result_new_line, "line\nnew line at end");
}
