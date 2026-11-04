mod card;
mod db;

use card::card::{Card, Keywords};
use db::db::{CardFilter, fetch_cards_all, fetch_cards_filtered};

use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(long)]
    query: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let filter = CardFilter {
        full_search: None,
        name: None,
    };

    let cards = fetch_cards_filtered(&filter)?;

    println!("Card Count: {}", cards.iter().count());

    Ok(())
}
