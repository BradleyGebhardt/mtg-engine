mod card;

use rusqlite::{Connection, Result};

fn _generate_card_rs() -> Result<()> {
    let conn = Connection::open("AllPrintings.sqlite")?;

    let mut statement = conn.prepare("PRAGMA table_info('cards');")?;

    println!("#[derive(Debug)]");
    println!("pub struct Card {{");

    if let Ok(row_iterator) = statement.query_map([], |row| {
        let name: String = row.get(1)?;
        let d_type: String = row.get(2)?;
        let rust_type = match d_type.as_str() {
            "TEXT" => String::from("String"),
            "INTEGER" => String::from("Option<i64>"),
            "REAL" => String::from("Option<f64>"),
            "BOOLEAN" => String::from("bool"),
            &_ => panic!("Unexpected value encountered: {:?}", d_type),
        };

        Ok([name, rust_type, d_type])
    }) {
        for row in row_iterator {
            let [name, rust_type, _d_type] = row?;
            println!("    pub {}: {},", name, rust_type);
        }
    }

    println!("}}");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("AllPrintings.sqlite")?;
    let mut statement = conn.prepare("SELECT * FROM cards")?;

    let card_rows = serde_rusqlite::from_rows::<card::meta::Card>(statement.query([])?);

    Ok(())
}
