//replaces any instance of multiple space characters in a row with a single space character
pub fn remove_extra_spaces(string: String) -> String {
    let mut string_as_char_vec: Vec<char> = string.chars().collect();

    //if a character is a space and the character before it was a space as well, it removes the current character
    for i in 1..string_as_char_vec.len() {
        if i >= string_as_char_vec.len() {
            break;
        }

        while string_as_char_vec[i] == ' ' && string_as_char_vec[i - 1] == ' ' {
            string_as_char_vec.remove(i);
        }
    }

    return string_as_char_vec.into_iter().collect();
}
