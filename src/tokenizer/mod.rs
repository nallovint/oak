// Tokenizer
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
    // Manual tokenization without regex (finite state machine) approach
    let chars: Vec<char> = source.chars().collect();
    let mut pos = 0;

    while pos < chars.len() {
        let c = chars[pos];

        match c {
            // If the character si whitespace it continues until the
            // tokenizer encounters a character to match
            c if c.is_whitespace() => pos += 1,
            // Assignation of values to variables and function output
            ':' if pos + 1 < chars.len() && chars[pos + 1] == '=' => {
                tokens.push(Token::Assign);
                pos += 2;
            }
            '+' | '-' | '*' | '/' | '%' | '^' => {
                tokens.push(Token::Operator(c.to_string()));
                pos += 1;
            }
            '"' => {
                pos += 1;
                let start = pos;
                while pos < chars.len() && chars[pos] != '"' {
                    pos += 1;
                }
                let literal: String = chars[start..pos].iter().collect();
                tokens.push(Token::StringLiteral(literal));
                pos += 1; // consumes closing quote
            }
            // Analyses if the current token is an ascii_digit and parses it as a Number token
            // In future releases, the language will implement different types of numerical values
            // and different types of operations depending on the type of numerical value given to the interpreter/compiler
            c if c.is_ascii_digit() => {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_ascii_digit() || chars[pos] == '.') {
                    pos += 1;
                }
                let number_str: String = chars[start..pos].iter().collect();
                if let Ok(num) = number_str.parse::<f64>() {
                    tokens.push(Token::Number(num));
                } else {
                    tokens.push(Token::Unknown(number_str));
                }
            }
            // Gives names to variables (identifiers)
            c if c.is_ascii_alphabetic() => {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_ascii_alphanumeric() || chars[pos] == '_')
                {
                    pos += 1;
                }
                let ident: String = chars[start..pos].iter().collect();
                match ident.as_str() {
                    "var" => tokens.push(Token::Var),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            // ALL other type of tokens aren't parsed and identified as "unknown"
            _ => {
                tokens.push(Token::Unknown(c.to_string()));
                pos += 1;
            }
        }
    }

    tokens
}
