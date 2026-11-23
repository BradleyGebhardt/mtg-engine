use crate::{
    card::card::{Card},
    query::{self, Lexer, SqlBuilder, Tokenizer},
};

use rusqlite::{Connection, Result, params_from_iter};
use serde_rusqlite::from_rows;

use std::error::Error;

pub fn fetch_cards_filtered(query: Option<String>) -> Result<Vec<Card>, Box<dyn Error>> {
    let tokenizer = match query {
        Some(ref q) => Tokenizer::new(q),
        None => Tokenizer::new(""),
    };

    let lexer = Lexer::new(tokenizer.split_tokens()?);
    let ast = query::Parser::parse(lexer.tokens()?)?;

    let mut builder = SqlBuilder::default();
    let where_clause = builder.build(&ast)?;
    let params = &builder.params;

    let conn = Connection::open("AllPrintings.sqlite")?;

    let sql = format!(
        "SELECT name, colors, colorIdentity, defense, keywords, IFNULL(text, '') AS text
         FROM cards
         WHERE {}
         GROUP BY name
         ORDER BY name ASC",
        where_clause
    );

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("SQL prepare error: {}", e))?;

    let rows = stmt
        .query(params_from_iter(params.iter()))
        .map_err(|e| format!("SQL query error: {}", e))?;

    let cards = from_rows::<Card>(rows)
        .collect::<Result<Vec<Card>, _>>()
        .map_err(|e| format!("Deserialization error: {}", e))?;

    Ok(cards)
}
