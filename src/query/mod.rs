use std::{
    collections::HashMap,
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
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] [{}] [{}]", self.field, self.operator, self.value)
    }
}

#[derive(Debug)]
pub enum QueryExpr {
    Condition(Token),    // Leaf node: a single atomic check (e.g., Color = "W")
    And(Vec<QueryExpr>), // All must match
    Or(Vec<QueryExpr>),  // Any must match
    Not(Box<QueryExpr>), // Must not match
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
                '"' => {
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

            tokens.push(Token {
                field,
                operator,
                value,
            });
        }

        Ok(tokens)
    }
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<QueryExpr, String> {
        if tokens.is_empty() {
            return Ok(QueryExpr::And(vec![]));
        }

        // Group tokens by field so repeated keys become `OR` branches
        let mut grouped: HashMap<TokenField, Vec<QueryExpr>> = HashMap::new();
        let mut field_order: Vec<TokenField> = Vec::new();

        for token in tokens {
            if !grouped.contains_key(&token.field) {
                field_order.push(token.field);
            }
            let expr = Self::expand_token(token)?;
            grouped.entry(expr_field(&expr)).or_default().push(expr);
        }

        let mut root_and = Vec::with_capacity(field_order.len());
        for field in field_order {
            let mut exprs = grouped.remove(&field).unwrap();
            if exprs.len() == 1 {
                root_and.push(exprs.pop().unwrap());
            } else {
                root_and.push(QueryExpr::Or(exprs));
            }
        }

        Ok(if root_and.len() == 1 {
            root_and.pop().unwrap()
        } else {
            QueryExpr::And(root_and)
        })
    }

    fn expand_token(token: Token) -> Result<QueryExpr, String> {
        let parts: Vec<String> = match token.field {
            TokenField::Color => token.value.chars().map(|c| c.to_string()).collect(),
            TokenField::Type | TokenField::Keyword => {
                token.value.split_whitespace().map(str::to_string).collect()
            }
            TokenField::Name | TokenField::Cmc => vec![token.value],
        };

        if parts.is_empty() {
            return Err(format!("Empty value provided for '{}'", token.field));
        }

        let mut conditions: Vec<QueryExpr> = parts
            .into_iter()
            .map(|val| {
                QueryExpr::Condition(Token {
                    field: token.field,
                    operator: token.operator,
                    value: val,
                })
            })
            .collect();

        Ok(if conditions.len() == 1 {
            conditions.pop().unwrap()
        } else {
            QueryExpr::And(conditions)
        })
    }
}

fn expr_field(expr: &QueryExpr) -> TokenField {
    match expr {
        QueryExpr::Condition(t) => t.field,
        QueryExpr::And(list) | QueryExpr::Or(list) => expr_field(&list[0]),
        QueryExpr::Not(inner) => expr_field(inner),
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
            println!("{token}");
        }

        Ok(String::new())
    }
}
