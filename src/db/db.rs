use rusqlite::{Connection, Result};

use crate::card::card::{Card};

#[derive(Debug)]
pub struct CardFilter {
    pub full_search: Option<String>,

    pub name: Option<String>,
}

impl CardFilter {
    fn serialize_to_string(&self) -> String {
        let mut query = String::from("");

        if let Some(name) = &self.name {
            query.push_str(&format!("name LIKE '%{name}%'"));
        }

        query
    }

    fn deserialize(&self, query: String) -> CardFilter {
        let groups = query.split(" ");

        for query in groups {
            if query.contains("name:") {
                return CardFilter {
                    full_search: None,
                    name: Some(String::from("")),
                };
            }
        }

        return CardFilter {
            full_search: None,
            name: Some(String::from("")),
        };
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

    let string_filter = filter.serialize_to_string();

    let mut statement = if string_filter == String::from("") {
        conn.prepare(&format!("SELECT * FROM cards"))?
    } else {
        conn.prepare(&format!("SELECT * FROM cards WHERE {string_filter}"))?
    };

    let mut cards: Vec<Card> = Vec::new();

    let card_rows = serde_rusqlite::from_rows::<Card>(statement.query([])?);

    for row in card_rows {
        let row = row.expect("ERROR: Error reading filtered rows");

        cards.push(row);
    }

    Ok(cards)
}
