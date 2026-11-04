use rusqlite::{Connection, Result};

use crate::card::card::{Card, Keywords};

#[derive(Debug, Default)]
pub struct CardFilter {
    pub name: Option<String>,
    pub colors: Option<String>,
    pub keywords: Vec<Keywords>,
}

impl CardFilter {
    pub fn serialize_to_string(&self) -> String {
        let mut query = String::from("");

        if let Some(name) = &self.name {
            query.push_str(&format!("name LIKE '%{name}%'"));
        }

        query
    }

    pub fn from_query_str(query: &str) -> Self {
        let mut filter = CardFilter::default();
        let tokens = CardFilter::tokenize_query(query);

        for token in tokens {
            if let Some(val) = token.strip_prefix("name:") {
                filter.name = Some(val.to_string());
            } else if let Some(val) = token.strip_prefix("colors:") {
                filter.colors = Some(val.to_string());
            } else if let Some(val) = token.strip_prefix("keyword:") {
                if let Ok(kw) = std::str::FromStr::from_str(val) {
                    filter.keywords.push(kw);
                }
            } else {
                // If filter.name already has text, append a space and the new token
                // Otherwise, initialize it with the current token
                filter.name = match filter.name {
                    Some(existing) => Some(format!("{} {}", existing, token)),
                    None => Some(token),
                };
            }
        }

        filter
    }

    fn tokenize_query(query: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        for c in query.chars() {
            match c {
                '"' => {
                    in_quotes = !in_quotes;
                }
                ' ' if !in_quotes => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                _ => {
                    current.push(c);
                }
            }
        }
        if !current.is_empty() {
            tokens.push(current);
        }
        tokens
    }
}

pub fn fetch_cards_all() -> Result<Vec<Card>> {
    let conn = Connection::open("AllPrintings.sqlite")?;
    let mut statement = conn.prepare("SELECT * FROM cards")?;

    let mut cards: Vec<Card> = Vec::new();

    let card_rows = serde_rusqlite::from_rows::<Card>(statement.query([])?);

    for row in card_rows {
        let row = row.expect("ERROR: Error reading rows");

        cards.push(row);
    }

    Ok(cards)
}

pub fn fetch_cards_filtered(filter: &CardFilter) -> Result<Vec<Card>> {
    let conn = Connection::open("AllPrintings.sqlite")?;

    let mut clauses = Vec::new();
    // We will hold our dynamic params as pairs of (&str, String)
    let mut params: Vec<(String, String)> = Vec::new();

    // 1. Bind Name
    if let Some(name) = &filter.name {
        clauses.push("name LIKE :name".to_string());
        params.push((":name".to_string(), format!("%{name}%")));
    }

    // 2. Bind Colors
    if let Some(colors) = &filter.colors {
        clauses.push("colors LIKE :colors".to_string());
        if colors.to_ascii_lowercase() == "c" {
            params.push((":colors".to_string(), String::from("")));
        } else {
            params.push((":colors".to_string(), format!("%{colors}%")));
        }
    }

    // 3. Bind Keywords
    // Since keywords can have multiples, we append a unique index to each parameter key
    for (idx, keyword) in filter.keywords.iter().enumerate() {
        let param_key = format!(":kw_{idx}");
        clauses.push(format!("keywords LIKE {param_key}"));
        params.push((param_key, format!("%{}%", keyword.as_ref())));
    }

    // Assemble the final query string safely
    let sql = if clauses.is_empty() {
        "SELECT * FROM cards".to_string()
    } else {
        format!("SELECT * FROM cards WHERE {}", clauses.join(" AND "))
    };

    let mut statement = conn.prepare(&sql)?;

    // Map our dynamic params vector into standard rusqlite named parameter pairs
    let named_params: Vec<(&str, &dyn rusqlite::ToSql)> = params
        .iter()
        .map(|(k, v)| (k.as_str(), v as &dyn rusqlite::ToSql))
        .collect();

    let mut cards: Vec<Card> = Vec::new();
    let card_rows = serde_rusqlite::from_rows::<Card>(statement.query(named_params.as_slice())?);

    for row in card_rows {
        let row = row.expect("ERROR: Error reading filtered rows");
        cards.push(row);
    }

    Ok(cards)
}
