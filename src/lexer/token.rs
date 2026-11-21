/// All Possible Token Types
pub enum TokenType {

    /// Special Token Type Helpers
    Illegal,
    EndOfFile,
    
    /// Identifiers
    Identifier,
    EightBitInteger,
    SixteenBitInteger,
    ThirtyTwoBitInteger,
    SixtyFourBitInteger,
    
    /// Operators
    Assignment,
    Plus,

    /// Delimiters
    Comma, 
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Arrow,

    /// Keywords
    Let,
    Function,
    
    /// Mutability
    Var,
    Const,
}

impl TokenType {

    const fn to_literal(&self) -> &'static str {
        match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::EndOfFile => "EOF",
            TokenType::Identifier => "IDENT",
            TokenType::EightBitInteger => "u8",
            TokenType::SixteenBitInteger => "u16",
            TokenType::ThirtyTwoBitInteger => "u32",
            TokenType::SixtyFourBitInteger => "u64",
            TokenType::Assignment => "=",
            TokenType::Plus => "+",
            TokenType::Comma => ",",
            TokenType::Colon => ":",
            TokenType::Semicolon => ";",
            TokenType::LeftParen => "(",
            TokenType::RightParen => "(",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::Arrow => "->",
            TokenType::Let => "let",
            TokenType::Function => "fn",
            TokenType::Var => "var",
            TokenType::Const => "const",
        }
    }
    
}

pub struct Token {
    token_type: TokenType,
    literal: &'static str,
}
