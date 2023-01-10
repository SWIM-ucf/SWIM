#[cfg(test)]
use crate::parser::parser_preprocessing::remove_extra_spaces;

#[test]
fn remove_extra_spaces_removes_instances_of_double_spaces() {
    let result = remove_extra_spaces("Double  space".to_string());
    assert_eq!(result, "Double space");
}

#[test]
fn remove_extra_spaces_removes_multiple_instances_of_double_spaces() {
    let result = remove_extra_spaces("THIS  HAS  MULTIPLE  DOUBLE SPACES".to_string());
    assert_eq!(result, "THIS HAS MULTIPLE DOUBLE SPACES");
}

#[test]
fn remove_extra_spaces_culls_all_unnecessary_spaces_down_to_one() {
    let result = remove_extra_spaces("Double  Triple   Triple   Quadruple   .".to_string());
    assert_eq!(result, "Double Triple Triple Quadruple .");
}

#[test]
fn remove_extra_spaces_does_not_delete_single_spaces() {
    let result = remove_extra_spaces("Single Single Double  .".to_string());
    assert_eq!(result, "Single Single Double .");
}
