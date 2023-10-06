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
    Numb(i128),
    /* Keywords */
    Print,
    Println,
}

impl TokenType {
    pub fn get_string(&self) -> Option<&str> {
        match self {
            TokenType::Str(s) => Some(&s),
            _ => None,
        }
    }

    pub fn get_int(&self) -> Option<i128> {
        match self {
            TokenType::Numb(i) => Some(*i),
            _ => None,
        }
    }
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


