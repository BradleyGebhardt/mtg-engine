mod card;
mod db;
mod query;

use db::db::{CardFilter, fetch_cards_filtered};
use query::{Tokenizer, Lexer};

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

    println!("{ast:?}");

    Ok(())
}
