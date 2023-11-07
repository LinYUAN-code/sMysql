use regex::Regex;

pub struct Lexer {
    source: String,
    current: usize,
}

#[derive(Debug)]
pub enum Tocken {
    SELECT,
    FROM,
    WHERE,
    CREATE,
    TABLE,
    OpenParenthesis,
    CloseParenthesis,
    COMMA,
    SEMICON,
    AND,
    OR,
    EOF,
    EQUAL,
    ColumnName(String),
    StringLiteral(String),
    NumberLiteral(String),
}

pub struct ParseError {}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source,
            current: 0,
        }
    }
    pub fn next(&mut self, k: usize) -> Result<Tocken, ParseError> {
        let mut result = Tocken::EOF;
        let current = self.current;
        for _ in 0..k {
            result = self.next_pop()?;
        }
        self.current = current;
        Ok(result)
    }
    // todo! 用macro 优化
    pub fn next_pop(&mut self) -> Result<Tocken, ParseError> {
        match self.next_char() {
            Some(ch) => match ch {
                '(' => Ok(Tocken::OpenParenthesis),
                ')' => Ok(Tocken::CloseParenthesis),
                ';' => Ok(Tocken::SEMICON),
                'A' => {
                    self.expect('A')?;
                    self.expect('N')?;
                    self.expect('D')?;
                    Ok(Tocken::AND)
                }
                'O' => {
                    self.expect('O')?;
                    self.expect('R')?;
                    Ok(Tocken::OR)
                }
                'W' => {
                    self.expect('W')?;
                    self.expect('H')?;
                    self.expect('E')?;
                    self.expect('R')?;
                    self.expect('E')?;
                    Ok(Tocken::WHERE)
                }
                'S' => {
                    self.expect('S')?;
                    self.expect('E')?;
                    self.expect('L')?;
                    self.expect('E')?;
                    self.expect('C')?;
                    self.expect('T')?;
                    Ok(Tocken::SELECT)
                }
                'C' => {
                    self.expect('C')?;
                    self.expect('R')?;
                    self.expect('E')?;
                    self.expect('A')?;
                    self.expect('T')?;
                    self.expect('E')?;
                    Ok(Tocken::CREATE)
                }
                'T' => {
                    self.expect('T')?;
                    self.expect('A')?;
                    self.expect('B')?;
                    self.expect('L')?;
                    self.expect('E')?;
                    Ok(Tocken::TABLE)
                }
                'F' => {
                    self.expect('F')?;
                    self.expect('R')?;
                    self.expect('O')?;
                    self.expect('M')?;
                    Ok(Tocken::FROM)
                }
                '\"' => {
                    self.expect('\"')?;
                    let mut string_literal = String::new();
                    loop {
                        match self.next_char() {
                            Some(ch) => {
                                if ch == '\"' {
                                    break;
                                }
                                self.current += 1;
                                string_literal.push(ch);
                            }
                            None => return Err(ParseError {}),
                        }
                    }
                    self.expect('\"')?;
                    Ok(Tocken::StringLiteral(string_literal))
                }
                ',' => {
                    self.expect(',')?;
                    Ok(Tocken::COMMA)
                }
                '=' => {
                    self.expect('=')?;
                    Ok(Tocken::EQUAL)
                }
                ' ' => {
                    self.expect(' ')?;
                    self.next_pop()
                }
                _ => {
                    let num_pattern = Regex::new(r"^(([1-9][0-9]*)|([0-9]))").unwrap();
                    let column_pattern = Regex::new(r"^([_a-zA-Z][_a-zA-Z0-9]*)").unwrap();
                    if let Some(matches) =
                        num_pattern.captures(self.source.get(self.current..).unwrap())
                    {
                        self.current += matches[0].len();
                        return Ok(Tocken::NumberLiteral(matches[0].to_string()));
                    }
                    if let Some(matches) =
                        column_pattern.captures(self.source.get(self.current..).unwrap())
                    {
                        self.current += matches[0].len();
                        return Ok(Tocken::ColumnName(matches[0].to_string()));
                    }
                    Err(ParseError {})
                }
            },
            None => Ok(Tocken::EOF),
        }
    }
    fn expect(&mut self, ch: char) -> Result<(), ParseError> {
        if self.current == self.source.len() {
            return Err(ParseError {});
        }
        let result = self.source.chars().nth(self.current).unwrap() == ch;
        if result {
            self.current += 1;
            Ok(())
        } else {
            Err(ParseError {})
        }
    }
    fn next_char(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let num_pattern = Regex::new(r"^(([1-9][0-9]*)|([0-9]))").unwrap();
        let column_name_pattern = Regex::new(r"^([_a-zA-Z][_a-zA-Z0-9]*)").unwrap();
        let num = "123";
        let column_name = "_damn";
        assert_eq!(&num_pattern.captures(num).unwrap()[0], "123");
        assert_eq!(
            &column_name_pattern.captures(column_name).unwrap()[0],
            "_damn"
        );
    }

    #[test]
    fn test_next() {
        let mut lexer = Lexer::new(
            "SELECT age, hobby FROM student WHERE name = \"tom\" AND age = 10".to_string(),
        );
        while let Ok(tocken) = lexer.next_pop() {
            match tocken {
                Tocken::EOF => break,
                _ => (),
            }
            println!("{:?}", tocken)
        }
    }
}
