use crate::token::Token;
use crate::token;

use std::fmt;
use std::error::Error;
use std::mem;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer {
    input: String,
    iter : Peekable<Chars<'static>>,
}

impl Lexer {
    pub fn new (input: String) -> Lexer {
        // HATE THIS
        let iter = unsafe { mem::transmute(input.chars().peekable()) };
        Lexer {
            input,
            iter,
        }
    }
    // Take until prelude or none
    // This always returns a string
    fn take_while_or_none(
        &mut self,
        mut predicate : impl FnMut(char) -> bool,
    ) -> String {
        let mut string = String::new();
        while let Some(&c) = self.iter.peek() {
            if predicate(c) {
                self.iter.next();
                string.push(c);
            } else {
                break;
            }
        }
        string
    }

    // Take until prelude errors if hits none before the predicate fails.
    fn take_while(
        &mut self,
        mut p : impl FnMut(char) -> bool,
    ) -> Result<String, LexerError> {
        let mut success : bool = false;
        let mut string = String::new();

        while let Some(&c) = self.iter.peek() {
            if p(c) {
                self.iter.next();
                string.push(c);
            } else {
                success = true;
                break;
            }
        }
        if success {
            Ok(string)
        } else {
            Err(LexerError::UnexpectedEndOfFile)
        }

    }

    // What we deem as whitespace and to skip completely
    fn skip_whitespace(&mut self) {

        self.take_while_or_none(|c| c.is_whitespace());
    }
    

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        if let Some(current_char) = self.iter.peek().cloned() {
            // We need to advance the iter
            match current_char {
                // Unambiguous single char tokens
                // Advance once then return
                ',' => { self.iter.next(); Ok(Token::Comma) },
                '-' => { self.iter.next(); Ok(Token::Minus) },
                '+' => { self.iter.next(); Ok(Token::Plus) },
                '*' => { self.iter.next(); Ok(Token::Times) },
                '(' => { self.iter.next(); Ok(Token::LParen) },
                ')' => { self.iter.next(); Ok(Token::RParen) },
                '{' => { self.iter.next(); Ok(Token::LBrace) },
                '}' => { self.iter.next(); Ok(Token::RBrace) },
                '[' => { self.iter.next(); Ok(Token::LBrack) },
                ']' => { self.iter.next(); Ok(Token::RBrack) },
                '%' => { self.iter.next(); Ok(Token::Mod) },
                ';' => { self.iter.next(); Ok(Token::SemiColon) },
                ':' => { self.iter.next(); Ok(Token::Colon) },
                '.' => { self.iter.next(); Ok(Token::Period) },
                '^' => { self.iter.next(); Ok(Token::Pow) },

                // Ambiguous Symbols that require more than one char to parse.
                // These all can have a '=' after
                '='|'<'|'>'|'!' =>  {
                    self.iter.next();
                    let equal = self.iter.peek() == Some(&'=');
                    if  equal { self.iter.next(); } // Consume the equals
                    match current_char {
                        '=' => if equal {Ok(Token::Equal)} else {Ok(Token::Assign)},
                        '<' => if equal {Ok(Token::LtEq )} else {Ok(Token::Lt)},
                        '>' => if equal {Ok(Token::GtEq )} else {Ok(Token::Gt)},
                        '!' => if equal {Ok(Token::NtEq )} else {Ok(Token::Not)},
                        _ => Err(LexerError::InvalidChar(current_char)),
                    }
                },
                '&' => {
                    self.iter.next();
                    match self.iter.peek() {
                        Some(&'&') => { self.iter.next(); Ok(Token::And) },
                        Some(c)    => Err(LexerError::ExpectedChar('&',*c)),
                        None       => Err(LexerError::UnexpectedEndOfFile),
                    }
                },
                '|' => {
                    self.iter.next();
                    match self.iter.peek() {
                        Some(&'|') => { self.iter.next(); Ok(Token::Or) },
                        Some(c)    => Err(LexerError::ExpectedChar('|',*c)),
                        None       => Err(LexerError::UnexpectedEndOfFile),
                    }
                },
                '"' => {
                    self.iter.next(); // Ignore the first '"'!
                    let string = self.take_while(|c| c != '"')?;
                    self.iter.next(); // Consume last '"'
                    Ok(Token::LiteralString(string))
                },

                '/' => {
                    // next char determines what kind of comment this is.
                    // / = single line
                    // * = multi line
                    // if neither its just a normal divide token
                    match self.iter.peek() {
                        Some('/') => {
                            // Skip to next line and then get the token from there
                            self.take_while_or_none(|c| c != '\n');
                            self.next_token()
                        },
                        Some('*') => {
                            // Loop until we find a '*' followed by a '/'
                            loop {
                                self.take_while(|c| c != '*')?;
                                self.iter.next();
                                if self.iter.peek() == Some(&'/') { break; }
                            }
                            // Then return the next token we find instead of
                            // returning anything here
                            self.next_token()
                        },
                        _ => Ok(Token::Div)
                    }
                },
                
                c if c.is_alphabetic() => {

                    // First lets read this word, this shouldn't fail as if we reach
                    // the end of the file we just want to return up to that.
                    let word = self.take_while_or_none(|c| c.is_alphanumeric());
                    if let Some(token) = token::keyword_to_token(&word) {
                        Ok(token)
                    } else {
                        Ok(Token::Identifier(word))
                    }
                },
                // TODO Handle Floats correctly
                // Allow to use of conversions
                // Binary (b), Hex (x), Float (f) 
                c if c.is_numeric() => {
                    let digits = self.take_while(
                        |c|c.is_numeric() ||
                        c == '.' ||
                        c == '_')?;
                        println!("Attempting to parse {} as int!", digits);
                    let int : i64 = digits.parse::<i64>()?;
                    Ok(Token::LiteralInt(int))
                },
                c => {
                    Err(LexerError::InvalidChar(c.clone()))
                },
            }
        } else {
            Ok(Token::EOF)
        }
    }
}

// ERROR HANDLING

#[derive(Debug)]
pub enum LexerError {
    UnexpectedChar(char),
    InvalidChar(char),
    ExpectedChar(char,char),
    UnexpectedEndOfFile,
    ParseInt,
    ParseFloat,
}


// TODO upgrade this to be more helpful!
// * Show line and col of where the error what
// * Show some of the source code of where the error was
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar(c)  => write!(f,"Unexpected Char: {:?}", c),
            LexerError::InvalidChar(c)     => write!(f,"Invalid Char: {:?}", c),
            LexerError::ExpectedChar(a,b)  => write!(f,"Expected a {:?} here found {:?} instead.",a,b),
            LexerError::UnexpectedEndOfFile=> write!(f,"Unexpected end of file."),
            LexerError::ParseInt   => write!(f,"Unable to parse an expected integer number"),
            LexerError::ParseFloat => write!(f,"Unable to parse a given float number"),
        }
    }
}
impl Error for LexerError {}
impl From<std::num::ParseIntError> for LexerError {
    fn from(_: std::num::ParseIntError) -> LexerError {
        LexerError::ParseInt
    }
}
// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_keywords() -> Result<(), LexerError> {
        let mut l = Lexer::new("if let for else loop true while false return".to_string());
        assert_eq!(l.next_token()?,Token::If);
        assert_eq!(l.next_token()?,Token::Let);
        assert_eq!(l.next_token()?,Token::For);
        assert_eq!(l.next_token()?,Token::Else);
        assert_eq!(l.next_token()?,Token::Loop);
        assert_eq!(l.next_token()?,Token::True);
        assert_eq!(l.next_token()?,Token::While);
        assert_eq!(l.next_token()?,Token::False);
        assert_eq!(l.next_token()?,Token::Return);
        Ok(())
    }

    #[test]
    fn correct_strings() -> Result<(), LexerError> {
        let mut l1 = Lexer::new("\"Hello\"".to_string());// "Hello"
        let mut l2 = Lexer::new("\"\"".to_string());     // ""
        assert_eq!(l1.next_token()?,Token::LiteralString("Hello".to_string()));
        assert_eq!(l2.next_token()?,Token::LiteralString("".to_string()));
        Ok(())
    }

    #[test]
    fn single_chars() -> Result<(), LexerError> {
        let mut l1 = Lexer::new(",-+*(){}[]%;:.^".to_string());
        assert_eq!(l1.next_token()?, Token::Comma);
        assert_eq!(l1.next_token()?, Token::Minus);
        assert_eq!(l1.next_token()?, Token::Plus);
        assert_eq!(l1.next_token()?, Token::Times);
        assert_eq!(l1.next_token()?, Token::LParen);
        assert_eq!(l1.next_token()?, Token::RParen);
        assert_eq!(l1.next_token()?, Token::LBrace);
        assert_eq!(l1.next_token()?, Token::RBrace);
        assert_eq!(l1.next_token()?, Token::LBrack);
        assert_eq!(l1.next_token()?, Token::RBrack);
        assert_eq!(l1.next_token()?, Token::Mod);
        assert_eq!(l1.next_token()?, Token::SemiColon);
        assert_eq!(l1.next_token()?, Token::Colon);
        assert_eq!(l1.next_token()?, Token::Period);
        assert_eq!(l1.next_token()?, Token::Pow);
        Ok(())
    }

    #[test]
    fn multiple_chars() -> Result<(), LexerError> {
        let mut l1 = Lexer::new("&& || <= != >= == < > ! = ".to_string());
        let mut l2 = Lexer::new("<=!=>=".to_string());
        assert_eq!(l1.next_token()?,Token::And);
        assert_eq!(l1.next_token()?,Token::Or);
        assert_eq!(l1.next_token()?,Token::LtEq);
        assert_eq!(l1.next_token()?,Token::NtEq);
        assert_eq!(l1.next_token()?,Token::GtEq);
        assert_eq!(l1.next_token()?,Token::Equal);
        assert_eq!(l1.next_token()?,Token::Lt);
        assert_eq!(l1.next_token()?,Token::Gt);
        assert_eq!(l1.next_token()?,Token::Not);
        assert_eq!(l1.next_token()?,Token::Assign);

        assert_eq!(l2.next_token()?,Token::LtEq);
        assert_eq!(l2.next_token()?,Token::NtEq);
        assert_eq!(l2.next_token()?,Token::GtEq);
        Ok(())
    }
/*
    #[test]
    fn multiple_chars_error() {
        let mut l1 = Lexer::new("&".to_string());
        let mut l2 = Lexer::new("& &".to_string());
        let mut l3 = Lexer::new("&|".to_string());
        let mut l4 = Lexer::new("|".to_string());
        let mut l5 = Lexer::new("| |".to_string());
        let mut l6 = Lexer::new("| &".to_string());

        Ok(())

    }
*/
}