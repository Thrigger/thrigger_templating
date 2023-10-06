use crate::token::{Token, TokenType};

pub fn scan(cont: &str) -> Vec<Token> {
    let mut i = 0;
    let mut tokens = vec![];

    while i < cont.len() {
        if let Some(tok) = get_one_char_token(cont, i) {
            tokens.push(tok);
            i += 1;
        } else if let Some((tok, new_i))  = get_keyword_token(cont, i) {
            tokens.push(tok);
            i = new_i;
        } else if let Some((tok, new_i))  = get_string_token(cont, i) {
            tokens.push(tok);
            i = new_i;
        } else {
            i+=1;
        }
    }
    tokens
}

fn get_string_token(s: &str, i: usize) -> Option<(Token, usize)> {
    if &s[i..i+1] == "\"" {
        let string_start = i + 1;
        if let Some(i_end) = &s[string_start..].find("\"") {
            return Some(
                (Token::new(TokenType::Str(String::from(&s[string_start..string_start+i_end])), None),
                string_start+1+i_end));
        }
    }
    None
}

fn get_int_token(s: &str, i: usize) -> Option<(Token, usize)> {
    let mut multiplier = 1;
    let mut offset = i;

    while offset < s.len() && &s[offset..offset+1] == "-" {
        multiplier *= -1;
        offset += 1;
    }

    let mut sum = 0;
    let mut is_int = false;
    while offset < s.len() {
        if let Ok(char_val) = &s[offset..offset+1].parse::<i128>() {
            is_int = true;
            sum *= 10;
            sum += char_val;
            offset += 1;
        } else {
            break;
        }
    }

    if is_int {
        Some((Token::new(TokenType::Numb(sum * multiplier), None), offset))
    } else {
        None
    }
}

fn get_one_char_token(s: &str, i: usize) -> Option<Token> {
    match &s[i..i+1] {
        "(" => Some(Token::new(TokenType::LeftPara, None)),
        ")" => Some(Token::new(TokenType::RightPara, None)),
        _ => None,
    }
}

fn get_keyword_token(s: &str, i: usize) -> Option<(Token, usize)> {
    if s.len() >= i + 7 && &s[i..i+7] == "println" {
        return Some((Token::new(TokenType::Println, None), 7));
    } else if s.len() >= i + 5 && &s[i..i+5] == "print" {
        return Some((Token::new(TokenType::Print, None), 5));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_get_one_char_token() {
        let code = "(x)";
        let expected = vec![
            Some(Token::new(TokenType::LeftPara, None)),
            None,
            Some(Token::new(TokenType::RightPara, None))];

        for i in 0..code.len() {
            assert_eq!(get_one_char_token(code, i), expected[i]);
        }
    }

    #[test]
    fn ok_get_string_token() {
        let code = String::from("\"abc\"");

        if let Some((token, i_new)) = get_string_token(&code, 0) {
            assert_eq!(4, i_new);
            assert_eq!("abc", token.token_type.get_string().unwrap());
        } else {
            panic!("Does not parse correct!");
        }
    }

    #[test]
    fn ok_get_int_token() {
        let code = String::from("-123");

        if let Some((token, i_new)) = get_int_token(&code, 0) {
            assert_eq!(4, i_new);
            assert_eq!(-123, token.token_type.get_int().unwrap());
        } else {
            panic!("Does not parse correct!");
        }

        if let Some((token, i_new)) = get_int_token(&code[1..], 0) {
            assert_eq!(3, i_new);
            assert_eq!(123, token.token_type.get_int().unwrap());
        } else {
            panic!("Does not parse correct!");
        }
    }
}

