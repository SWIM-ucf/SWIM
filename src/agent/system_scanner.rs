use std::collections::VecDeque;

pub struct Scanner {
    input: VecDeque<char>,
}

enum ScannerState {
    /// State when the Scanner is waiting for the first matching character
    Waiting,
    /// State when the Scanner is reading the integer component of a decimal number or just an
    /// integer.
    ReadingInt,
    /// State when the Scanner is reading the decimal point or the remainder of a decimal number
    /// in a float.
    ReadingDecimalPoint,
    /// State when the Scanner has finished its read.
    Finished,
}

/// The Scanner is a data structure that pulls information (i.e. strings or ints) from a stream of
/// characters. To use, feed a string into the Scanner and use the next_x() functions to pull data
/// out. These functions are designed so they cannot fail, so any input that does not match the type
/// trying to be read will be ignored.
impl Scanner {
    /// Constructs a new, empty scanner.
    pub fn new() -> Scanner {
        Scanner {
            input: VecDeque::new(),
        }
    }

    /// Feeds a line into the Scanner.
    pub fn feed(&mut self, line: String) {
        for character in line.chars() {
            self.input.push_back(character);
        }
        self.input.push_back('\n');
    }

    /// Attempts to read an int from the Scanner. The read effectively finds the first match to the
    /// following regular expression: `[0-9]+`. Any characters before the match are discarded and
    /// the Scanner is advanced to the character immediately after the last character of the match.
    /// If the number exceeds the maximum possible size of a u64, u64::MAX is returned.
    ///
    /// If no digits are found in the entire remainder of the unscanned input, the function will
    /// return None.
    pub fn next_int(&mut self) -> Option<u64> {
        let mut state = ScannerState::Waiting;
        let mut result = String::new();

        // Process the Scanner's queue character by character
        while !self.input.is_empty() {
            let character = self.input.pop_front().unwrap();

            match state {
                ScannerState::Waiting => {
                    if character.is_ascii_digit() {
                        result.push(character);
                        state = ScannerState::ReadingInt;
                    }
                }
                ScannerState::ReadingInt => {
                    if character.is_ascii_digit() {
                        result.push(character);
                    } else {
                        state = ScannerState::Finished;
                        // Put the character back since we never actually utilized it.
                        self.input.push_front(character);
                    }
                }
                ScannerState::ReadingDecimalPoint => {
                    // Should be unreachable
                    unimplemented!()
                }
                ScannerState::Finished => {
                    // Put the character back in the queue to avoid consuming it and break out of
                    // the loop to return the int to the user.
                    self.input.push_front(character);
                    break;
                }
            }
        }

        let parsed = result.parse();
        if result.len() > 0 {
            match parsed {
                Ok(res) => Some(res),
                Err(_) => Some(u64::MAX),
            }
        } else {
            None
        }
    }

    /// Attempts to read a double from the Scanner. The read effectively finds the first match to
    /// the following regular expression: `[0-9]+((\.[0-9]+)|)`. Any characters before the match are
    /// discarded and the Scanner is advanced to the character immediately after the last character
    /// of the match.
    ///
    /// If no matches are found in the entire remainder of the unscanned input, the function will
    /// return None.
    pub fn next_double(&mut self) -> Option<f64> {
        let mut state = ScannerState::Waiting;
        let mut result = String::new();

        // Process the Scanner's queue character by character
        while !self.input.is_empty() {
            let character = self.input.pop_front().unwrap();

            match state {
                ScannerState::Waiting => {
                    if character.is_ascii_digit() {
                        result.push(character);
                        state = ScannerState::ReadingInt;
                    }
                }
                ScannerState::ReadingInt => {
                    if character.is_ascii_digit() {
                        result.push(character);
                    } else if character == '.' {
                        state = ScannerState::ReadingDecimalPoint;
                        result.push(character);
                    } else {
                        state = ScannerState::Finished;
                        // Put the character back since we never actually utilized it.
                        self.input.push_front(character);
                    }
                }
                ScannerState::ReadingDecimalPoint => {
                    if character.is_ascii_digit() {
                        result.push(character);
                    } else {
                        state = ScannerState::Finished;
                        // Put the character back since we never actually utilized it.
                        self.input.push_front(character);
                    }
                }
                ScannerState::Finished => {
                    // Put the character back in the queue to avoid consuming it and break out of
                    // the loop to return the int to the user.
                    self.input.push_front(character);
                    break;
                }
            }
        }

        let parsed = result.parse();
        if result.len() > 0 {
            match parsed {
                Ok(res) => Some(res),
                Err(_) => Some(f64::INFINITY),
            }
        } else {
            None
        }
    }

    /// Identical to next_double(), but it returns an f32 instead.
    pub fn next_float(&mut self) -> Option<f32> {
        self.next_double().map(|val| val as f32)
    }

    /// Returns the remainder of the current line in the Scanner. If the Scanner is empty, this
    /// function will return None.
    pub fn next_line(&mut self) -> Option<String> {
        todo!()
    }
}
