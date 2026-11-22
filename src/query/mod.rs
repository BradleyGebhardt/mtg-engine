use std::{
    fmt::{self, Display},
    str::FromStr,
};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum TokenField {
    Name,
    Color,
    Cmc,
    Keyword,
    Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
pub enum Operator {
    #[strum(serialize = ":")]
    Colon,
    #[strum(serialize = "=")]
    Eq,
    #[strum(serialize = "<")]
    Lt,
    #[strum(serialize = "<=")]
    Lte,
    #[strum(serialize = ">")]
    Gt,
    #[strum(serialize = ">=")]
    Gte,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub field: TokenField,
    pub operator: Operator,
    pub value: String,
    pub and: Option<Box<Self>>,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] [{}] [{}]", self.field, self.operator, self.value)
    }
}

impl Token {
    pub fn chain(field: TokenField, operator: Operator, values: Vec<String>) -> Option<Self> {
        let mut head: Option<Token> = None;

        for value in values.into_iter().rev() {
            head = Some(Token {
                field,
                operator,
                value,
                and: head.map(Box::new),
            });
        }

        head
    }
}

pub struct Tokenizer<'a> {
    pub content: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }

    pub fn split_tokens(&self) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();
        let mut in_quotes = false;
        let mut current_token = String::new();

        for ch in self.content.chars() {
            match ch {
                '"' | '\'' => {
                    in_quotes = !in_quotes;
                }
                c if c.is_whitespace() && !in_quotes => {
                    if !current_token.is_empty() {
                        tokens.push(std::mem::take(&mut current_token));
                    }
                }
                _ => current_token.push(ch),
            }
        }

        if in_quotes {
            return Err("Syntax error: Unclosed quotation mark in query".into());
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        Ok(tokens)
    }
}

pub struct Lexer {
    pub content: Vec<String>,
}

impl Lexer {
    pub fn new(content: Vec<String>) -> Self {
        Self { content }
    }

    pub fn tokens(&self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::with_capacity(self.content.len());

        for raw in &self.content {
            // Find the byte index where the field name ends
            let op_start = raw
                .char_indices()
                .find(|&(_, ch)| !ch.is_alphanumeric())
                .map(|(idx, _)| idx)
                .ok_or_else(|| format!("Missing operator and value in '{}'", raw))?;

            let field_str = &raw[..op_start];
            let field = TokenField::from_str(&field_str.to_ascii_lowercase())
                .map_err(|_| format!("Unknown field '{}'", field_str))?;

            // Find the byte index where the operator ends
            let rest = &raw[op_start..];
            let val_start = rest
                .char_indices()
                .find(|&(_, ch)| !matches!(ch, ':' | '=' | '<' | '>'))
                .map(|(idx, _)| idx)
                .unwrap_or(rest.len());

            let op_str = &rest[..val_start];
            // `strum` automatically rejects invalid combos like `:=` or `>>`!
            let operator = Operator::from_str(op_str)
                .map_err(|_| format!("Invalid operator '{}' in '{}'", op_str, raw))?;

            let value = rest[val_start..].to_string();
            if value.is_empty() {
                return Err(format!("Missing value after '{}{}'", field_str, op_str));
            }

            let parts: Vec<String> = match field {
                // Each character in `color:WR` becomes a separate node (`"W"`, `"R"`)
                TokenField::Color => value.chars().map(|c| c.to_string()).collect(),

                // Space-separated words in `type:"Creature Angel"` or `keyword:"Flash DoubleStrike"`
                TokenField::Type | TokenField::Keyword => {
                    value.split_whitespace().map(str::to_string).collect()
                }

                // `name` and `cmc` stay as a single value
                TokenField::Name | TokenField::Cmc => vec![value],
            };

            let token = Token::chain(field, operator, parts)
                .ok_or_else(|| format!("Missing value after '{}{}'", field_str, op_str))?;

            tokens.push(token);
        }

        Ok(tokens)
    }
}

#[derive(Debug, Clone)]
pub enum SqlParam {
    Text(String),
    Int(i64),
}

#[derive(Debug, Default)]
pub struct SqlBuilder {
    pub params: Vec<SqlParam>,
}

impl SqlBuilder {
    fn bind(&mut self, param: SqlParam) -> &'static str {
        self.params.push(param);
        "?"
    }

    pub fn build(&mut self, tokens: Vec<Token>) -> Result<String, String> {
        if tokens.is_empty() {
            return Ok("1=1".to_string());
        }

        for token in tokens {
            match token.field {
                TokenField::Color => {
                    println!("{token}");
                    let mut nested = &token.and;
                    let mut indent = 1;

                    while let Some(and) = nested {
                        println!("{}AND {and}", "\t".repeat(indent));
                        nested = &and.and;
                        indent += 1;
                    }
                },
                _ => println!("{token}")
            }
        }

        Ok(String::new())
    }
}
