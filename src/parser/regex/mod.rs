mod automata;
mod parser;

pub enum Regex {
    Char(u8),
    Concat(Box<Regex>, Box<Regex>),
    Alternation(Box<Regex>, Box<Regex>),
    Repeat(Box<Regex>),
}

// Uses the API from the standard regex crate
pub struct Match<'a> {
    start_index: usize,
    end_index: usize,
    string: &'a str,
}

impl Regex {
    pub fn new(regex: &str) -> Result<Regex, &str> {
        Err("Failed to parse string")
    }

    pub fn is_match(&self, haystack: &str) -> bool {
        return false;
    }

    pub fn find<'a>(&self, haystack: &'a str) -> Option<Match<'a>> {
        return None;
    }
}

struct RegexParser<'a> {
    parse_string: &'a [u8],
    cur_index: usize,
}

impl RegexParser<'_> {
    fn new(pattern_string: &str) -> RegexParser {
        RegexParser {
            parse_string: pattern_string.as_bytes(),
            cur_index: 0,
        }
    }

    fn next_char(&mut self) -> Option<u8> {
        let cur_char = *self.parse_string.get(self.cur_index)?;
        self.cur_index += 1;

        if cur_char != b'\\' {
            return Some(cur_char);
        }

        let esc_char = self.parse_string.get(self.cur_index)?;
        self.cur_index += 1;

        let escaped_char = match esc_char {
            // Ascii
            b't' => b'\t',
            b'n' => b'\n',
            b'r' => b'\r',
            b'\\' => b'\\',

            // Regex special chars
            b'*' => b'*',
            b'+' => b'+',

            b'|' => b'|',

            b'(' => b'(',
            b')' => b')',

            b'[' => b'[',
            b']' => b']',

            b'.' => b'.',
            b'?' => b'?',
            _ => *esc_char,
        };

        return Some(escaped_char);
    }

    fn main(&mut self) -> Result<Regex, String> {
        return self.regexpr();
    }

    fn regexpr(&mut self) -> Result<Regex, String> {
        let cur = self.next_char().ok_or("Unexpectedly reached end of file")?;

        if is_regex_control(cur) {
            return Ok(Regex::Concat(
                Box::new(Regex::Char(cur)),
                Box::new(self.regexpr_prime()?),
            ));
        } else if cur == b'(' {
            let reg = self.regexpr()?;

            let err_str = format!("Expected ')' at char index {0}", self.cur_index);
            let error = Err(err_str.clone());

            let is_closed = self.next_char().ok_or(err_str)?;
            if is_closed != b')' {
                error
            } else {
                Ok(reg)
            }
        } else {
            Err(String::from("Invalid expression passed in "))
        }
    }

    // For the production rule correponding to R'
    // constructed after removing left recursive rules
    fn regexpr_prime(&mut self) -> Result<Regex, String> {
        todo!();
    }
}

fn is_regex_control(byte: u8) -> bool {
    byte == b'*'
        || byte == b'+'
        || byte == b'|'
        || byte == b'('
        || byte == b')'
        || byte == b'['
        || byte == b']'
        || byte == b'.'
        || byte == b'?'
}
