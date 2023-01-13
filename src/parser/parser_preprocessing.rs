//replaces any instance of multiple space characters in a row with a single space character
pub fn remove_extra_spaces_and_lines(string: String) -> String {
    let mut new_string = String::new();

    for c in string.chars() {
        // If this character is a space, only add it to the
        // resulting string if this is the first space.
        // If it's not a space, just pass it along to the new string.
        if (c != ' ' || !new_string.ends_with(c)) && (c != '\n' || !new_string.ends_with(c)) {
            new_string.push(c);
        }
    }

    new_string
}
