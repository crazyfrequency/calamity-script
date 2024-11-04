use std::{fs::File, sync::Arc};

use crate::utils::structs::tokens::{DelimitersGroup, DigitType, KeywordsGroup, TokenGroupLexer};

mod error;
use error::{LexerError, LexerResult};

#[derive(Debug, Clone)]
pub struct Lexer {
    path: String,
    position: usize,
    buffer: Vec<char>,

    line: usize,
    inline_position: usize,

    character: char,
    next_character: char,
    input: Arc<File>
}

impl Lexer {
    pub fn new(file: File, path: impl Into<String>) -> Self {
        let file = Arc::new(file);
        Self {
            path: path.into(),
            position: 0,
            buffer: Vec::new(),

            line: 1,
            inline_position: 0,

            character: ' ',
            next_character: read_char::read_next_char(&mut file.clone()).expect("Не удалось прочитать первый символ, возможно файл пустой"),
            input: file
        }
    }

    pub fn next_token(&mut self) -> LexerResult<(TokenGroupLexer, usize, usize)> {
        let res = self.skip_whitespace();
        if let Err(e) = res {
            return Err(e);
        }

        let line = self.line;
        let inline_position = self.inline_position;

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
                return Ok((
                    match identified.as_str() {
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
                        name if name.contains('_') => return Err(LexerError {
                            path: self.path.clone(),
                            position: inline_position,
                            line: self.line,
                            token: TokenGroupLexer::Identifier(name.into()),
                            message: "встречен символ '_'".into()
                        }),
                        name => TokenGroupLexer::Identifier(name.into())
                    },
                    line,
                    inline_position
                ));
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

        Ok((token, line, inline_position))
    }

    fn read_char(&mut self) {
        self.character = self.next_character;
        match read_char::read_next_char(&mut self.input.clone()) {
            Ok(v) => self.next_character = v,
            Err(_) => self.next_character = '\0'
        }

        self.position += 1;
        self.inline_position += 1;
    }

    fn next_char(&self) -> char {
        self.next_character
    }

    fn prev_char(&self) -> char {
        match self.buffer.get(self.buffer.len()-1) {
            Some(v) => *v,
            None => ' '
        }
        
    }

    fn read_identifier(&mut self) -> String {

        while self.character.is_ascii_alphanumeric() || self.character == '_' {
            self.buffer.push(self.character);
            self.read_char();
        }

        let res = String::from_iter(&self.buffer);

        self.buffer.clear();

        return res;
    }

    fn read_digit(&mut self) -> LexerResult<(TokenGroupLexer, usize, usize)> {
        let line = self.line;
        let inline_position = self.inline_position;

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
                            return self.variable_error("шестнадцатеричное число");
                        },
                    _ => if digit_type.clone() >> DigitType::Binary {
                        self.buffer.push(self.character);
                        self.read_char();
                        break;
                    } else {
                        return self.variable_error("двоичное число");
                    }
                },
                'D'|'d' => match self.next_char() {
                    '0'..'9'|'a'..='f'|'A'..='F'|'.'|'h'|'H' => 
                        if digit_type.clone() >> DigitType::Hex {
                            digit_type = DigitType::Hex;
                        } else {
                            return self.variable_error("шестнадцатеричное число");
                        },
                    _ => if digit_type.clone() >> DigitType::Digital {
                        self.buffer.push(self.character);
                        self.read_char();
                        break;
                    } else {
                        return self.variable_error("десятичное число");
                    }
                },
                'E'|'e' =>
                    if digit_type.clone() >> DigitType::HexPoint || digit_type.clone() >> DigitType::Point {
                        if digit_type.clone() >> DigitType::HexPoint {
                            digit_type = DigitType::HexPoint;
                        }
                        if exp {
                            return self.variable_error("число с плавающей точкой, обнаружен второй символ 'E'");
                        }
                        exp = true;
                    } else {
                        return self.variable_error("число с плавающей точкой");
                    },
                'H'|'h' => match self.next_char() {
                    '0'..'9'|'a'..='f'|'A'..='F'|'.'|'h'|'H' =>
                        return self.variable_error("шестнадцатеричное число, так как обнаружен не поддерживаемый символ 'H' или 'h'"),
                    _ => if digit_type.clone() >> DigitType::Hex {
                        self.buffer.push(self.character);
                        self.read_char();
                        break;
                    } else {
                        return self.variable_error("шестнадцатеричное число");
                    }
                },
                '.' => if digit_type.clone() >> DigitType::Point {
                    digit_type = DigitType::Point;
                    if exp {
                        return self.variable_error("число с плавающей точкой, неожиданная точка в экспоненте");
                    }
                    match self.next_char() {
                        '0'..='9' => (),
                        _ => return self.variable_error("число с плавающей точкой, неожиданная точка без цифр после неё")
                    }
                } else {
                    return self.variable_error("число с плавающей точкой");
                },
                'O'|'o' => match self.next_char() {
                    '0'..'9'|'a'..='f'|'A'..='F'|'.'|'h'|'H' => {
                        return self.variable_error("восьмеричное число, так как дальше обнаружен не поддерживаемый символ");
                    },
                    _ => if digit_type.clone() >> DigitType::Octal {
                        self.next_char();
                        break;
                    } else {
                        return self.variable_error("восьмеричное число");
                    }
                },
                '+'|'-' => if exp {
                    match self.prev_char() {
                        'E'|'e' => if digit_type.clone() >> DigitType::Point {
                            digit_type = DigitType::Point;
                        } else {
                            return self.variable_error("число с плавающей точкой");
                        },
                        _ => {
                            let res = String::from_iter(&self.buffer);
                            self.buffer.clear();
                            return Ok(
                                (TokenGroupLexer::Variables(res),
                                line,
                                inline_position
                            ))
                        }
                    }
                } else {
                    break;
                },
                'A'..='Z'|'a'..='z' => if digit_type.clone() >> DigitType::Hex {
                    digit_type = DigitType::Hex;
                } else {
                    return self.variable_error("шестнадцатеричное число");
                },
                _ => if digit_type.clone() >> DigitType::Digital {
                    break;
                } else if digit_type.clone() >> DigitType::Point {
                    match self.prev_char() {
                        '0'..='9' => break,
                        _ => return self.variable_error("число с плавающей точкой"),
                    }
                } else {
                    return self.variable_error("десятичное число или число с плавающей точкой");
                }
            };
            self.buffer.push(self.character);
            self.read_char();
        };

        let res = String::from_iter(&self.buffer);
        self.buffer.clear();

        Ok((
            TokenGroupLexer::Variables(res),
            line,
            inline_position
        ))
    }

    fn variable_error<T>(&mut self, message: impl Into<String>) -> LexerResult<T> {
        let res = String::from_iter(&self.buffer);
        self.buffer.clear();
        Err(LexerError {
            path: self.path.clone(),
            position: self.inline_position,
            line: self.line,
            token: TokenGroupLexer::Variables(String::new()),
            message: format!(
                "Не возможно интерпретировать '{}' как {}",
                res,
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