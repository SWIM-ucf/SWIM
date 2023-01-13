///This function takes the initial version of the string provided by the user and removes any unnecessary spaces, extra lines,
///and comments from the string and returns this new, cleaned version.
pub fn string_cleaning(string: String) -> String {
    let mut new_string = String::new();
    let mut is_comment = false;

    for c in string.chars() {
        //If the character is part of a comment, it is not appended to the new_string so the rest of this iteration is skipped.
        if is_comment {
            if c == '\n' {
                //If the character is a new line character, the comment is considered ended.
                is_comment = false;
            } else {
                continue;
            }
        }
        if c == '#' {
            //If the character is a #, the rest of that line is not appended to the new_string
            is_comment = true;
            continue;
        }

        //This removes spaces at the end of lines.
        if c == '\n' && new_string.ends_with(' ') {
            new_string.pop();
        }

        //The character is only pushed to the new string if it is not a space following another space or new line character
        //and it is not a new line character following a new line character
        if (c != ' ' || (!new_string.ends_with(c) && !new_string.ends_with('\n')))
            && (c != '\n' || !new_string.ends_with(c))
        {
            new_string.push(c);
        }
    }

    //Removes any starting or ending spaces or new line characters.
    if new_string.starts_with(' ') || new_string.starts_with('\n') {
        new_string.remove(0);
    }
    if new_string.ends_with(' ') || new_string.ends_with('\n') {
        new_string.pop();
    }

    new_string
}
