mod card;
mod db;

use db::db::{CardFilter, fetch_cards_filtered};

use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(long)]
    query: Option<String>,
    #[arg(long)]
    json: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let filter = match args.query {
        Some(ref q) => CardFilter::from_query_str(q),
        None => CardFilter::default(),
    };

    let cards = fetch_cards_filtered(&filter)?;

    // println!("{}", card::card::Colors::White.as_ref());

    // std::process::exit(0);

    if args.json {
        println!("{}", serde_json::to_string(&cards)?);
    }

    Ok(())
}
