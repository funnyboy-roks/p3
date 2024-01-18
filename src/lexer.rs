use std::{
    collections::HashSet,
    io::{self, Write},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftSquare,
    RightSquare,
    LeftAngle,
    RightAngle,
    Colon,
    ColonEquals,
    SemiColon,
    Comma,
    Equal,
    DoubleEqual,
    Asterisk,
    AsteriskEquals,
    DoubleAsterisk,
    DoubleAsteriskEquals,
    Ampersand,
    AmpersandEquals,
    Pipe,
    PipeEquals,
    ThinArrow,
    Minus,
    MinusEquals,
    Plus,
    PlusEquals,
    Percent,
    PercentEquals,
    Slash,
    SlashEquals,
    DoubleSlash,
    DoubleSlashEquals,
    NotEqual,
    Dot,
    NewLine,
    LeftShift,
    LeftShiftEquals,
    RightShift,
    RightShiftEquals,
    Indent(usize),
    StrLit {
        tags: HashSet<char>,
        val: String,
    },
    Comment(String),
    /// Contains a string since python integers are infinitely sized
    IntLit(String),
    /// Contains a string since python floats are infinitely sized
    FloatLit(String),

    BooleanLit(bool),

    // == Keywords ==
    And,
    As,
    Assert,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    None,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,

    // == Custom Tokens ==
    DoubleAmpersand,
    DoublePipe,
}

impl Token {
    pub fn write_to<W>(&self, w: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        macro_rules! write_str {
            ($e: expr) => {{
                write!(w, "{}", $e)?;
            }};
            ($e: expr ,s) => {{
                write!(w, "{} ", $e)?;
            }};
        }
        match self {
            Token::Ident(d) => write_str!(d, s),
            Token::LeftParen => write_str!('('),
            Token::RightParen => write_str!(')'),
            Token::LeftCurly => write_str!('{'),
            Token::RightCurly => write_str!('}'),
            Token::LeftSquare => write_str!('['),
            Token::RightSquare => write_str!(']'),
            Token::LeftAngle => write_str!('<'),
            Token::RightAngle => write_str!('>'),
            Token::Colon => write_str!(':'),
            Token::ColonEquals => write_str!(":="),
            Token::SemiColon => write_str!(';'),
            Token::Comma => write_str!(','),
            Token::Equal => write_str!('='),
            Token::DoubleEqual => write_str!("=="),
            Token::Asterisk => write_str!('*'),
            Token::AsteriskEquals => write_str!("*="),
            Token::DoubleAsterisk => write_str!("**"),
            Token::DoubleAsteriskEquals => write_str!("**="),
            Token::Ampersand => write_str!('&'),
            Token::AmpersandEquals => write_str!("&="),
            Token::Pipe => write_str!('|'),
            Token::PipeEquals => write_str!("|="),
            Token::ThinArrow => write_str!("->"),
            Token::Minus => write_str!('-'),
            Token::MinusEquals => write_str!("-="),
            Token::Plus => write_str!('+'),
            Token::PlusEquals => write_str!("+="),
            Token::Percent => write_str!('%'),
            Token::PercentEquals => write_str!("%="),
            Token::Slash => write_str!('/'),
            Token::SlashEquals => write_str!("/="),
            Token::DoubleSlash => write_str!("//"),
            Token::DoubleSlashEquals => write_str!("//="),
            Token::NotEqual => write_str!("!="),
            Token::Dot => write_str!('.'),
            Token::NewLine => write_str!('\n'),
            Token::LeftShift => write_str!("<<"),
            Token::LeftShiftEquals => write_str!("<<="),
            Token::RightShift => write_str!(">>"),
            Token::RightShiftEquals => write_str!(">>="),
            Token::Indent(n) => write_str!("    ".repeat(*n)),
            Token::StrLit { tags, val } => {
                let tags: String = tags.iter().collect();
                let val = val.replace('\'', "\\\'");
                write!(w, "{}'{}'", tags, val)?;
            }
            Token::Comment(text) => write!(w, "#{}", text)?,
            Token::IntLit(n) => write_str!(n, s),
            Token::FloatLit(n) => write_str!(n, s),
            Token::BooleanLit(b) => write_str!(if *b { "True" } else { "False" }, s),
            Token::And => write_str!("and", s),
            Token::As => write_str!("as", s),
            Token::Assert => write_str!("assert", s),
            Token::Break => write_str!("break", s),
            Token::Class => write_str!("class", s),
            Token::Continue => write_str!("continue", s),
            Token::Def => write_str!("def", s),
            Token::Del => write_str!("del", s),
            Token::Elif => write_str!("elif", s),
            Token::Else => write_str!("else"),
            Token::Except => write_str!("except", s),
            Token::Finally => write_str!("finally", s),
            Token::For => write_str!("for", s),
            Token::From => write_str!("from", s),
            Token::Global => write_str!("global", s),
            Token::If => write_str!("if", s),
            Token::Import => write_str!("import", s),
            Token::In => write_str!("in", s),
            Token::Is => write_str!("is", s),
            Token::Lambda => write_str!("lambda", s),
            Token::None => write_str!("None", s),
            Token::Nonlocal => write_str!("nonlocal", s),
            Token::Not => write_str!("not", s),
            Token::Or => write_str!("or", s),
            Token::Pass => write_str!("pass", s),
            Token::Raise => write_str!("raise", s),
            Token::Return => write_str!("return", s),
            Token::Try => write_str!("try", s),
            Token::While => write_str!("while", s),
            Token::With => write_str!("with", s),
            Token::Yield => write_str!("yield", s),

            Token::DoubleAmpersand => write_str!("and", s),
            Token::DoublePipe => write_str!("or", s),
        };

        Ok(())
    }
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    start_of_line: bool,
}

impl Lexer {
    pub fn new(chars: Vec<char>) -> Self {
        Self {
            chars,
            pos: 0,
            start_of_line: true,
        }
    }

    fn peek_char(&self) -> char {
        self.chars[self.pos]
    }

    fn take_char(&mut self) -> char {
        self.pos += 1; // increase position
        self.chars[self.pos - 1] // return prev char
    }

    fn chars_left(&self) -> usize {
        self.chars.len() - self.pos
    }

    fn take_ident(&mut self) -> &[char] {
        let i = self.pos;
        while matches!(self.peek_char(), 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
            self.take_char();
        }
        assert_ne!(self.pos, i, "Ident had 0 characters");
        &self.chars[i..self.pos]
    }

    fn take_str_lit(&mut self) -> Token {
        let mut tags = HashSet::new();
        let mut tag = self.take_char();
        while tag != '\'' && tag != '"' {
            tags.insert(tag);
            tag = self.take_char();
        }
        let quote = tag;
        assert!(quote == '\'' || quote == '"');

        // TODO: triple quotes
        // let triple = self.peek_char() == quote;
        // if triple {
        //     self.take_char();
        //     assert_eq!(self.take_char(), quote);
        // }
        let mut out = String::new();
        let mut escape_next = false;
        loop {
            match self.take_char() {
                '\\' => {
                    escape_next = true;
                }
                c if escape_next => {
                    out.push(match c {
                        '\'' | '"' => c,
                        '\\' => '\\',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        'x' => {
                            // hex character
                            let mut s = String::with_capacity(2);
                            s.push(self.take_char());
                            s.push(self.take_char());
                            char::from_u32(u32::from_str_radix(&s, 16).unwrap()).unwrap()
                        }
                        '0' => {
                            // octal character
                            let mut s = String::with_capacity(2);
                            s.push(self.take_char());
                            s.push(self.take_char());
                            char::from_u32(u32::from_str_radix(&s, 8).unwrap()).unwrap()
                        }
                        _ => {
                            panic!("Unexpected escaped char: '{}'", c)
                        }
                    });
                    escape_next = false;
                }
                c if c == quote => break,
                c => {
                    out.push(c);
                }
            }
        }
        // TODO: triple quotes
        // if triple {
        //     assert_eq!(self.take_char(), quote);
        //     assert_eq!(self.take_char(), quote);
        // }

        Token::StrLit { tags, val: out }
    }

    fn take_comment(&mut self) -> String {
        let mut out = String::new();
        loop {
            if self.chars_left() == 0 || self.peek_char() == '\n' {
                return out;
            }
            out.push(self.take_char());
        }
    }

    fn take_number(&mut self) -> Token {
        // TODO: other number literals
        let mut float = false;
        let mut out = String::new();
        loop {
            if self.chars_left() == 0 {
                return if float {
                    Token::FloatLit(out)
                } else {
                    Token::IntLit(out)
                };
            }

            let c = self.peek_char();
            match c {
                '0'..='9' | 'x' | 'b' => {
                    out.push(self.take_char());
                }
                '.' => {
                    out.push(self.take_char());
                    float = true;
                }
                _ => {
                    return if float {
                        Token::FloatLit(out)
                    } else {
                        Token::IntLit(out)
                    };
                }
            }
        }
    }

    fn parse_ident(s: String) -> Token {
        match s.as_str() {
            "and" => Token::And,
            "as" => Token::As,
            "assert" => Token::Assert,
            "break" => Token::Break,
            "class" => Token::Class,
            "continue" => Token::Continue,
            "def" => Token::Def,
            "del" => Token::Del,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "except" => Token::Except,
            "False" | "false" => Token::BooleanLit(true),
            "finally" => Token::Finally,
            "for" => Token::For,
            "from" => Token::From,
            "global" => Token::Global,
            "if" => Token::If,
            "import" => Token::Import,
            "in" => Token::In,
            "is" => Token::Is,
            "lambda" => Token::Lambda,
            "None" => Token::None,
            "nonlocal" => Token::Nonlocal,
            "not" => Token::Not,
            "or" => Token::Or,
            "pass" => Token::Pass,
            "raise" => Token::Raise,
            "return" => Token::Return,
            "True" | "true" => Token::BooleanLit(true),
            "try" => Token::Try,
            "while" => Token::While,
            "with" => Token::With,
            "yield" => Token::Yield,
            _ => Token::Ident(s),
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = loop {
            if self.chars_left() == 0 {
                return None;
            }
            match self.take_char() {
                ' ' if self.start_of_line => {
                    let mut count = 1;
                    while self.peek_char() == ' ' {
                        count += 1;
                        self.take_char();
                    }
                    if count == 1 {
                        continue;
                    }
                    assert_eq!(count % 4, 0);
                    break Token::Indent(count / 4);
                }
                '\n' => {
                    break Token::NewLine;
                }
                c if c.is_ascii_whitespace() => {
                    continue;
                }
                // TODO: This needs to work for multiple tags: rf"hello"
                'a'..='z' | 'A'..='Z' if matches!(self.peek_char(), '\'' | '"') => {
                    self.pos -= 1;
                    break self.take_str_lit();
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.pos -= 1;
                    break Self::parse_ident(String::from_iter(self.take_ident()));
                }

                '(' => break Token::LeftParen,
                ')' => break Token::RightParen,
                '{' => break Token::LeftCurly,
                '}' => break Token::RightCurly,
                '[' => break Token::LeftSquare,
                ']' => break Token::RightSquare,

                ':' => {
                    if self.peek_char() == '=' {
                        self.take_char();
                        break Token::ColonEquals;
                    }
                    break Token::Colon;
                }
                ';' => break Token::SemiColon,
                ',' => break Token::Comma,
                '&' => match self.peek_char() {
                    '=' => {
                        self.take_char();
                        break Token::AmpersandEquals;
                    }
                    '&' => {
                        self.take_char();
                        break Token::DoubleAmpersand;
                    }
                    _ => break Token::Ampersand,
                },
                '|' => match self.peek_char() {
                    '=' => {
                        self.take_char();
                        break Token::PipeEquals;
                    }
                    '|' => {
                        self.take_char();
                        break Token::DoublePipe;
                    }
                    _ => break Token::Pipe,
                },
                '-' => match self.peek_char() {
                    '=' => {
                        self.take_char();
                        break Token::MinusEquals;
                    }
                    '>' => {
                        self.take_char();
                        break Token::ThinArrow;
                    }
                    _ => break Token::Minus,
                },
                '+' => {
                    if self.peek_char() == '=' {
                        self.take_char();
                        break Token::PlusEquals;
                    }
                    break Token::Plus;
                }
                '%' => {
                    if self.peek_char() == '=' {
                        self.take_char();
                        break Token::PercentEquals;
                    }
                    break Token::Percent;
                }

                '=' => {
                    if self.peek_char() == '=' {
                        self.take_char();
                        break Token::DoubleEqual;
                    }
                    break Token::Equal;
                }
                '!' => {
                    assert_eq!(
                        self.peek_char(),
                        '=',
                        "expected '!=', got '!{}'",
                        self.peek_char()
                    );

                    self.take_char();
                    break Token::NotEqual;
                }
                '/' => {
                    if self.peek_char() == '/' {
                        self.take_char();
                        if self.peek_char() == '=' {
                            self.take_char();
                            break Token::DoubleSlashEquals;
                        }
                        break Token::DoubleSlash;
                    }
                    break Token::Slash;
                }
                '*' => match self.peek_char() {
                    '*' => {
                        self.take_char();
                        if self.peek_char() == '=' {
                            self.take_char();
                            break Token::DoubleAsteriskEquals;
                        }
                        break Token::DoubleAsterisk;
                    }
                    '=' => {
                        self.take_char();
                        break Token::AsteriskEquals;
                    }
                    _ => break Token::Asterisk,
                },
                '<' => {
                    if self.peek_char() == '<' {
                        self.take_char();
                        break Token::LeftShift;
                    }
                    break Token::LeftAngle;
                }
                '>' => {
                    if self.peek_char() == '>' {
                        self.take_char();
                        break Token::RightShift;
                    }
                    break Token::RightAngle;
                }
                '#' => {
                    break Token::Comment(self.take_comment());
                }
                '.' => {
                    if self.peek_char().is_digit(10) {
                        self.pos -= 1;
                        break self.take_number();
                    }
                    break Token::Dot;
                }
                '0'..='9' => {
                    self.pos -= 1;
                    break self.take_number();
                }
                '\'' | '"' => {
                    self.pos -= 1;
                    break self.take_str_lit();
                }
                c => unimplemented!(
                    "'{}' nyi, context: \"{}\"",
                    c,
                    String::from_iter(
                        &self.chars
                            [(self.pos - 10).max(0)..(self.pos + 10).min(self.chars.len() - 1)]
                    )
                ),
            }
        };
        self.start_of_line = matches!(ret, Token::NewLine);
        Some(ret)
    }
}
