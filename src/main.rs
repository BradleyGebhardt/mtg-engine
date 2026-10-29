mod card;

use sqlite::{Connection, Result, Value};

fn generate_card_rs() -> Result<()> {
    let conn = Connection::open("AllPrintings.sqlite")?;

    let statement = conn.prepare("PRAGMA table_info('cards');")?;

    println!("#[derive(Debug)]");
    println!("pub struct Card {{");

    for row in statement.into_iter().map(|r| r.unwrap()) {
        let name = row.read::<&str, _>("name");
        let d_type = row.read::<&str, _>("type");

        let rust_type = match d_type {
            "TEXT" => "String",
            "INTEGER" => "Option<i64>",
            "REAL" => "Option<f64>",
            "BOOLEAN" => "bool",
            &_ => panic!("Unexpected value encountered: {:?}", d_type),
        };

        println!("    pub {}: {},", name, rust_type);
    }

    println!("}}");

    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("AllPrintings.sqlite")?;

    let statement = conn.prepare("SELECT * FROM cards")?;

    for row in statement.into_iter().map(|r| r.unwrap()) {
        let card = card::meta::Card {
            name: row.read::<&str, _>("name").to_string(),
            colors: row.read::<&str, _>("colors").to_string(),
            edhrec_rank: row.read("edhrecRank")
        };
        println!("{}", card);
    }

    Ok(())
}
