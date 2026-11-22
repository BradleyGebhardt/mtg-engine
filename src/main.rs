mod card;
mod db;
mod query;

use card::card::Card;
use db::db::{CardFilter, fetch_cards_filtered};
use query::{Lexer, SqlBuilder, Tokenizer};
use rusqlite::{Connection, params_from_iter};
use serde_rusqlite::from_rows;

use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    query: Option<String>,
    #[arg(long)]
    json: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let tokenizer = match args.query {
        Some(ref q) => Tokenizer::new(q),
        None => Tokenizer::new(""),
    };

    let lexer = Lexer::new(tokenizer.split_tokens()?);
    let ast = query::Parser::parse(lexer.tokens()?)?;

    let mut builder = SqlBuilder::default();
    let where_clause = builder.build(&ast)?;
    let params = &builder.params;

    let conn = Connection::open("AllPrintings.sqlite")?;

    // Select the exact camelCase columns matching `Card`'s fields
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

    println!("{sql}");

    let rows = stmt
        .query(params_from_iter(params.iter()))
        .map_err(|e| format!("SQL query error: {}", e))?;

    let cards = from_rows::<Card>(rows)
        .collect::<Result<Vec<Card>, _>>()
        .map_err(|e| format!("Deserialization error: {}", e))?;

    for card in cards {
        println!("{card}\n");
    }

    Ok(())
}
