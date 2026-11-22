use crate::card::card;

use std::{
    fmt::{self, Display},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use strum::{self, Display, EnumString};

pub struct Tokenizer {
    pub content: String,
}

impl Tokenizer {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn split_tokens(&self) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut in_quotes = false;
        let mut current_token = String::new();

        for ch in self.content.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current_token.push(ch);
                }

                c if c.is_whitespace() && !in_quotes => {
                    if !current_token.is_empty() {
                        tokens.push(std::mem::take(&mut current_token));
                    }
                }

                _ => {
                    current_token.push(ch);
                }
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token.clone());
        }

        tokens
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum TokenField {
    Name(String),
    Color(String),
    Cmc(String),
}

pub struct Token {
    field: String,
    operator: String,
    value: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{field}] [{operator}] [{value}]",
            field = self.field,
            operator = self.operator,
            value = self.value
        )
    }
}

pub struct Lexer {
    pub content: Vec<String>,
}

impl Lexer {
    pub fn new(content: Vec<String>) -> Self {
        Self { content }
    }

    pub fn tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut field: String;
        let mut operator: String;
        let mut value: String;

        for token in self.content.iter() {
            let mut n = 0;

            while n < token.len()
                && let ch = token.chars().nth(n).expect("Invalid char")
                && ch.is_alphanumeric()
            {
                n += 1;
            }

            field = token[0..n].to_string();

            TokenField::from_str(&field).expect(&format!("ERROR: Unknown field [{}]", field));

            operator = String::new();

            while n < token.len()
                && let ch = token.chars().nth(n).expect("Invalid char")
                && [':', '=', '<', '>'].contains(&ch)
            {
                if !operator.is_empty() && ch != '=' {
                    panic!("ERROR: Syntax error in int comparitor")
                }

                operator.push(ch);

                n += 1;
            }

            value = token[n..].to_string();

            while n < token.len()
                && let _ch = token.chars().nth(n).expect("Invalid char")
            {
                n += 1;
            }

            tokens.push(Token {
                field,
                operator,
                value,
            });
        }

        tokens
    }
}
