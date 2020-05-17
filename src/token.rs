#[derive(Debug,PartialEq,Clone)]
pub enum Token {
    // Literals
    LiteralString(String),
    LiteralInt   (i64),
    LiteralFloat (f64),
    LiteralBool  (bool),

    Identifier   (String),

    // Operators
    Plus,     // +
    Minus,    // -
    Times,    // *
    Div,      // /
    Mod,      // %
    Pow,      // ^
    Equal,    // ==
    Assign,   // =
    Period,   // .

    Lt,       // <
    Gt,       // >
    LtEq,     // <=
    GtEq,     // >=
    NtEq,     // !=

    Not,      // !
    Or,       // ||
    And,      // &&

    // Structure
    Comma,    // ,
    SemiColon,// ;
    Colon,    // :
    LBrace,   // {
    RBrace,   // }
    LParen,   // (
    RParen,   // )
    LBrack,   // [
    RBrack,   // ]
    
    // Keywords
    If, For, While, Let, Else, Loop,
    True, False, Return,


    // Special
    EOF,
}

pub fn keyword_to_token(keyword: &str) -> Option<Token> {
    match &keyword.to_lowercase()[..] {
        "if"     => Some(Token::If),
        "let"    => Some(Token::Let),
        "for"    => Some(Token::For),
        "else"   => Some(Token::Else),
        "loop"   => Some(Token::Loop),
        "true"   => Some(Token::True),
        "while"  => Some(Token::While),
        "false"  => Some(Token::False),
        "return" => Some(Token::Return),
        _ => None
    }
}