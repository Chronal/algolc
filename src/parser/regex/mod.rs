mod automata;
mod parser;

enum Regex {
    Char(u8),
    Concat(Box<Regex>, Box<Regex>),
    Alternation(Box<Regex>, Box<Regex>),
    Repeat(Box<Regex>),
}

impl Regex {}
