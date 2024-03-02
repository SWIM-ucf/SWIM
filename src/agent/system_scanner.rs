use std::collections::VecDeque;

pub struct Scanner {
    input: VecDeque<String>,
    offset: usize,
}

impl Scanner {
    /// Constructs a new, empty scanner.
    pub fn new() -> Scanner {
        Scanner {
            input: VecDeque::new(),
            offset: 0,
        }
    }

    /// Feeds a line into the Scanner
    pub fn feed(&mut self, line: String) {
        self.input.push(line);
    }

    /// Attempts to read an int from the Scanner. The read effectively finds the first match to the
    /// following regular expression: `[0-9]+`. Any characters before the match are discarded and
    /// the Scanner is advanced to the character immediately after the last character of the match.
    ///
    /// If no digits are found in the entire remainder of the unscanned input, the function will
    /// return None.
    pub fn next_int(&mut self) -> Option<u64> {
        todo!()
    }

    /// Attempts to read a double from the Scanner. The read effectively finds the first match to
    /// the following regular expression: `[0-9]+((\.[0-9]+)|)`. Any characters before the match are
    /// discarded and the Scanner is advanced to the character immediately after the last character
    /// of the match.
    ///
    /// If no matches are found in the entire remainder of the unscanned input, the function will
    /// return None.
    pub fn next_double(&mut self) -> Option<f64> {
        todo!()
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