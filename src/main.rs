mod card;
mod db;
mod query;

use db::db::{fetch_cards_filtered};

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

    let cards = fetch_cards_filtered(args.query)?;

    println!("Card Count: {}", cards.len());

    Ok(())
}
