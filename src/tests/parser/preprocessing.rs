#[cfg(test)]
use crate::parser::parser_preprocessing::remove_extra_spaces_and_lines;

#[test]
fn remove_extra_spaces_and_lines_removes_instances_of_double_spaces() {
    let result = remove_extra_spaces_and_lines("Double  space".to_string());
    assert_eq!(result, "Double space");
}

#[test]
fn remove_extra_spaces_and_lines_removes_multiple_instances_of_double_spaces() {
    let result = remove_extra_spaces_and_lines("THIS  HAS  MULTIPLE  DOUBLE SPACES".to_string());
    assert_eq!(result, "THIS HAS MULTIPLE DOUBLE SPACES");
}

#[test]
fn remove_extra_spaces_and_lines_culls_all_unnecessary_spaces_down_to_one() {
    let result =
        remove_extra_spaces_and_lines("Double  Triple   Triple   Quadruple   .".to_string());
    assert_eq!(result, "Double Triple Triple Quadruple .");
}

#[test]
fn remove_extra_spaces_and_lines_does_not_delete_single_spaces() {
    let result = remove_extra_spaces_and_lines("Single Single Double  .".to_string());
    assert_eq!(result, "Single Single Double .");
}

#[test]
fn remove_extra_spaces_and_lines_removes_extra_lines() {
    let result = remove_extra_spaces_and_lines("Space\nTwoSpaces\n\nSpace\nSpace".to_string());
    assert_eq!(result, "Space\nTwoSpaces\nSpace\nSpace");
}
