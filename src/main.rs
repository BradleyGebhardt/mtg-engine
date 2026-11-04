mod card;
mod db;

use card::card::{Card, Keywords};
use db::db::{CardFilter, fetch_cards_filtered};

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

    // Check if a query argument was passed, otherwise default to an empty filter
    let filter = match args.query {
        Some(ref q) => CardFilter::from_query_str(q),
        None => CardFilter::default(),
    };

    let cards = fetch_cards_filtered(&filter)?;

    for card in cards {
        println!("{card}");
    }

    // println!("{}", cards.len());

    Ok(())
}
