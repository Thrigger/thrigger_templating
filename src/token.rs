#[derive(Debug,PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: Option<String>,
}

#[derive(Debug,PartialEq)]
pub enum TokenType {
    /* One character lexeme */
    LeftPara,
    RightPara,
    /* Datatypes */
    Str(String),
    Numb(i64),
    /* Keywords */
    Print,
    Println,
}

impl Token {
    pub fn new(token_type: TokenType, lex: Option<&str>) -> Token {
        let lexeme = match lex {
            None => None,
            Some(s) => Some(s.to_string()),
        };
        Token { token_type, lexeme }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("Type: {:?}, Lexeme: {:?}", self.token_type, self.lexeme)
    }
}


