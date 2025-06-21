use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var,
    Identifier(String),
    Assign,
    Number(f64),
    StringLiteral(String),
    Operator(String),
    BeginSection(String),
    EndSection(String),
    Comment(String),
    Unknown(String),
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let string_re = Regex::new(r#""([^"]*)""#).unwrap();
    let number_re = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();

    for word in source.split_whitespace() {
        match word {
            "var" => tokens.push(Token::Var),
            ":=" => tokens.push(Token::Assign),
            "+" | "-" | "*" | "/" => tokens.push(Token::Operator(word.to_string())),
            _ => {
                if word.starts_with("\"") && word.ends_with("\"") {
                    if let Some(cap) = string_re.captures(word) {
                        tokens.push(Token::StringLiteral(cap[1].to_string()));
                    } else {
                        tokens.push(Token::Unknown(word.to_string()));
                    }
                } else if number_re.is_match(word) {
                    tokens.push(Token::Number(word.parse().unwrap()));
                } else {
                    tokens.push(Token::Identifier(word.to_string()));
                }
            }
        }
    }

    tokens
}
