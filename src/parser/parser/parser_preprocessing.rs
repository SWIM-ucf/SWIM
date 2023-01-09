//replaces any instance of multiple space characters in a row with a single space character
pub fn remove_extra_spaces(string: String) -> String{

    let mut string_as_char_vec : Vec<char> = string.chars().collect();

    //if a character is a space and the character before it was a space as well, it removes the current character
    for i in 1..string_as_char_vec.len(){

        if i >= string_as_char_vec.len(){
            break;
        }

        while string_as_char_vec[i] == ' ' && string_as_char_vec[i - 1] == ' '{
            string_as_char_vec.remove(i);
        }
    }

    return string_as_char_vec.into_iter().collect();
}

#[cfg(test)]
mod pre_processing_tests{
    use crate::parser::parser::parser_preprocessing::remove_extra_spaces;

    #[test]
    fn remove_extra_spaces_removes_instances_of_double_spaces() {
        let result = remove_extra_spaces("Double  space".to_string());
        assert_eq!(result, "Double space");
    }

    #[test]
    fn remove_extra_spaces_removes_multiple_instances_of_double_spaces(){
        let result = remove_extra_spaces("THIS  HAS  MULTIPLE  DOUBLE SPACES".to_string());
        assert_eq!(result, "THIS HAS MULTIPLE DOUBLE SPACES");
    }

    #[test]
    fn remove_extra_spaces_culls_all_unnecessary_spaces_down_to_one(){
        let result = remove_extra_spaces("Double  Triple   Triple   Quadruple   .".to_string());
        assert_eq!(result, "Double Triple Triple Quadruple .");
    }

    #[test]
    fn remove_extra_spaces_does_not_delete_single_spaces(){
        let result = remove_extra_spaces("Single Single Double  .".to_string());
        assert_eq!(result, "Single Single Double .");
    }
}