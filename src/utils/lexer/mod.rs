use crate::utils::structs::tokens::{DelimitersGroup, DigitType, KeywordsGroup, TokenGroupLexer};

mod error;
use error::{LexerError, LexerResult};

#[derive(Debug, Clone)]
pub struct Lexer {
    path: String,
    position: usize,
    length: usize,

    line: usize,
    inline_position: usize,

    character: char,
    input: Vec<char>
}

impl Lexer {
    pub fn new(buf: Vec<char>, path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            position: 0,
            length: buf.len(),

            line: 1,
            inline_position: 0,

            character: ' ',
            input: buf.into()
        }
    }

    pub fn next_token(&mut self) -> LexerResult<TokenGroupLexer> {
        let res = self.skip_whitespace();
        if let Err(e) = res {
            return Err(e);
        }

        let token = match self.character {
            '{' => TokenGroupLexer::Delimiters(DelimitersGroup::LeftCurlyBracket),
            '}' => TokenGroupLexer::Delimiters(DelimitersGroup::RightCurlyBracket),
            '(' => TokenGroupLexer::Delimiters(DelimitersGroup::LeftParenthesis),
            ')' => TokenGroupLexer::Delimiters(DelimitersGroup::RightParenthesis),
            ',' => TokenGroupLexer::Delimiters(DelimitersGroup::Comma),
            ':' => TokenGroupLexer::Delimiters(DelimitersGroup::Colon),
            ';' => TokenGroupLexer::Delimiters(DelimitersGroup::Semicolon),
            '+' => TokenGroupLexer::Delimiters(DelimitersGroup::Plus),
            '-' => TokenGroupLexer::Delimiters(DelimitersGroup::Minus),
            '*' => TokenGroupLexer::Delimiters(DelimitersGroup::Asterisk),
            '/' => TokenGroupLexer::Delimiters(DelimitersGroup::Slash),
            '=' => match self.next_char() {
                '=' => {
                    self.read_char();
                    TokenGroupLexer::Delimiters(DelimitersGroup::Identical)
                },
                _ => TokenGroupLexer::Delimiters(DelimitersGroup::Equal)
            },
            '!' => match self.next_char() {
                '=' => {
                    self.read_char();
                    TokenGroupLexer::Delimiters(DelimitersGroup::NotEqual)
                },
                _ => TokenGroupLexer::Delimiters(DelimitersGroup::Not)
            },
            '<' => match self.next_char() {
                '=' => {
                    self.read_char();
                    TokenGroupLexer::Delimiters(DelimitersGroup::LessEqual)
                },
                _ => TokenGroupLexer::Delimiters(DelimitersGroup::Less)
            },
            '>' => match self.next_char() {
                '=' => {
                    self.read_char();
                    TokenGroupLexer::Delimiters(DelimitersGroup::GreaterEqual)
                },
                _ => TokenGroupLexer::Delimiters(DelimitersGroup::Greater)
            },
            '&' => match self.next_char() {
                '&' => {
                    self.read_char();
                    TokenGroupLexer::Delimiters(DelimitersGroup::And)
                },
                _ => TokenGroupLexer::Illegal((self.character, self.line, self.inline_position))
            },
            '|' => match self.next_char() {
                '|' => {
                    self.read_char();
                    TokenGroupLexer::Delimiters(DelimitersGroup::Or)
                },
                _ => TokenGroupLexer::Illegal((self.character, self.line, self.inline_position))
            },
            'a'..='z'|'A'..='Z' => {
                let identified = self.read_identifier();
                return Ok(match identified.as_str() {
                    "var" => TokenGroupLexer::Keywords(KeywordsGroup::Var),
                    "let" => TokenGroupLexer::Keywords(KeywordsGroup::Let),
                    "if" => TokenGroupLexer::Keywords(KeywordsGroup::If),
                    "then" => TokenGroupLexer::Keywords(KeywordsGroup::Then),
                    "else" => TokenGroupLexer::Keywords(KeywordsGroup::Else),
                    "end_else" => TokenGroupLexer::Keywords(KeywordsGroup::EndElse),
                    "for" => TokenGroupLexer::Keywords(KeywordsGroup::For),
                    "do" => TokenGroupLexer::Keywords(KeywordsGroup::Do),
                    "while" => TokenGroupLexer::Keywords(KeywordsGroup::While),
                    "loop" => TokenGroupLexer::Keywords(KeywordsGroup::Loop),
                    "input" => TokenGroupLexer::Keywords(KeywordsGroup::Input),
                    "output" => TokenGroupLexer::Keywords(KeywordsGroup::Output),
                    "integer" => TokenGroupLexer::Keywords(KeywordsGroup::Integer),
                    "real" => TokenGroupLexer::Keywords(KeywordsGroup::Real),
                    "boolean" => TokenGroupLexer::Keywords(KeywordsGroup::Boolean),
                    "true" => TokenGroupLexer::Keywords(KeywordsGroup::True),
                    "false" => TokenGroupLexer::Keywords(KeywordsGroup::False),
                    name => TokenGroupLexer::Identifier(name.into())
                });
            },
            '0'..='9'|'.' => return self.read_digit(),
            '\0' => TokenGroupLexer::Eof,
            _ => TokenGroupLexer::Illegal((
                self.character,
                self.line,
                self.inline_position
            ))
        };

        self.read_char();

        Ok(token)
    }

    fn read_char(&mut self) {
        if self.position >= self.length {
            self.character = '\0';
        } else {
            self.character = self.input[self.position];
        }

        self.position += 1;
        self.inline_position += 1;
    }

    fn next_char(&self) -> char {
        match self.input.get(self.position) {
            Some(v) => *v,
            None => '\0'
        }
    }

    fn prev_char(&self) -> char {
        if self.position <= 0 {
            ' '
        } else {
            self.input[self.position-2]
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position-1;

        while match self.character {
            'a'..='z'|'A'..='Z'|'0'..='9' => true,
            _ => false,
        } {
            self.read_char();
        }

        return String::from_iter(self.input[pos..self.position-1].to_vec());
    }

    fn read_digit(&mut self) -> LexerResult<TokenGroupLexer> {
        let pos = self.position-1;

        let mut exp = false;
        let mut digit_type = DigitType::Binary;

        loop {
            match self.character {
                '0'|'1' => (),
                '2'..='7' => 
                    if digit_type.clone() >> DigitType::Octal {
                        digit_type = DigitType::Octal;
                    },
                '8'..='9' =>
                    if digit_type.clone() >> DigitType::Digital {
                        digit_type = DigitType::Digital;
                    },
                'B'|'b' => match self.next_char() {
                    '0'..'9'|'a'..='f'|'A'..='F'|'.'|'h'|'H' =>
                        if digit_type.clone() >> DigitType::Hex {
                            digit_type = DigitType::Hex;
                        } else {
                            return self.variable_error(pos, "шестнадцатеричное число");
                        },
                    _ => if digit_type.clone() >> DigitType::Binary {
                        self.read_char();
                        break;
                    } else {
                        return self.variable_error(pos, "двоичное число");
                    }
                },
                'D'|'d' => match self.next_char() {
                    '0'..'9'|'a'..='f'|'A'..='F'|'.'|'h'|'H' => 
                        if digit_type.clone() >> DigitType::Hex {
                            digit_type = DigitType::Hex;
                        } else {
                            return self.variable_error(pos, "шестнадцатеричное число");
                        },
                    _ => if digit_type.clone() >> DigitType::Digital {
                        self.read_char();
                        break;
                    } else {
                        return self.variable_error(pos, "десятичное число");
                    }
                },
                'E'|'e' =>
                    if digit_type.clone() >> DigitType::HexPoint || digit_type.clone() >> DigitType::Point {
                        if digit_type.clone() >> DigitType::HexPoint {
                            digit_type = DigitType::HexPoint;
                        }
                        if exp {
                            return self.variable_error(pos, "число с плавающей точкой, обнаружен второй символ 'E'");
                        }
                        exp = true;
                    } else {
                        return self.variable_error(pos, "число с плавающей точкой");
                    },
                'H'|'h' => match self.next_char() {
                    '0'..'9'|'a'..='f'|'A'..='F'|'.'|'h'|'H' =>
                        return self.variable_error(pos, "шестнадцатеричное число, так как обнаружен не поддерживаемый символ 'H' или 'h'"),
                    _ => if digit_type.clone() >> DigitType::Hex {
                        self.read_char();
                        break;
                    } else {
                        return self.variable_error(pos, "шестнадцатеричное число");
                    }
                },
                '.' => if digit_type.clone() >> DigitType::Point {
                    digit_type = DigitType::Point;
                    if exp {
                        return self.variable_error(pos, "число с плавающей точкой, неожиданная точка в экспоненте");
                    }
                    match self.next_char() {
                        '0'..='9' => (),
                        _ => return self.variable_error(pos, "число с плавающей точкой, неожиданная точка без цифр после неё")
                    }
                } else {
                    return self.variable_error(pos, "число с плавающей точкой");
                },
                'O'|'o' => match self.next_char() {
                    '0'..'9'|'a'..='f'|'A'..='F'|'.'|'h'|'H' => {
                        return self.variable_error(pos, "восьмеричное число, так как дальше обнаружен не поддерживаемый символ");
                    },
                    _ => if digit_type.clone() >> DigitType::Octal {
                        self.next_char();
                        break;
                    } else {
                        return self.variable_error(pos, "восьмеричное число");
                    }
                },
                '+'|'-' => if exp {
                    match self.prev_char() {
                        'E'|'e' => if digit_type.clone() >> DigitType::Point {
                            digit_type = DigitType::Point;
                        } else {
                            return self.variable_error(pos, "число с плавающей точкой");
                        },
                        _ => return Ok(TokenGroupLexer::Variables(
                            String::from_iter(
                                self.input[pos..self.position-1].to_vec())
                            )
                        )
                    }
                },
                _ => if digit_type.clone() >> DigitType::Digital {
                    break;
                } else if digit_type.clone() >> DigitType::Point {
                    match self.prev_char() {
                        '0'..='9' => break,
                        _ => return self.variable_error(pos, "число с плавающей точкой"),
                    }
                } else {
                    return self.variable_error(pos, "десятичное число или число с плавающей точкой");
                }
            };
            self.read_char();
        };

        Ok(TokenGroupLexer::Variables(
            String::from_iter(
                self.input[pos..self.position-1].to_vec())
            )
        )
    }

    fn variable_error(&self, pos: usize, message: impl Into<String>) -> LexerResult<TokenGroupLexer> {
        Err(LexerError {
            path: self.path.clone(),
            position: self.inline_position,
            line: self.line,
            token: TokenGroupLexer::Variables(String::new()),
            message: format!(
                "Не возможно интерпретировать '{}' как {}",
                String::from_iter(self.input[pos..self.position-1].to_vec()),
                message.into()
            )
        })
    }

    fn skip_whitespace(&mut self) -> Result<(), LexerError> {
        while match self.character {
            '\n' => {
                self.line += 1;
                self.inline_position = 0;
                true
            },
            '%' => {
                self.read_char();
                while self.character != '%' && self.character != '\0' {
                    if self.character == '\n' {
                        self.line += 1;
                        self.inline_position = 0;
                    }
                    self.read_char();
                };
                if self.character == '\0' {
                    return Err(LexerError {
                        path: self.path.clone(),
                        position: self.inline_position,
                        line: self.line,
                        token: TokenGroupLexer::Illegal((self.character, self.line, self.inline_position)),
                        message: String::from("Неожиданный конец файла")
                    });
                };
                true
            },
            character => character.is_ascii_whitespace()
        } {
            self.read_char();
        }
        Ok(())
    }

}