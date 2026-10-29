pub mod meta;

use meta::Card;

use std::fmt::{Display, Formatter, Result};

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Card:{}|{}|{}", self.name, self.colors, self.edhrec_rank.unwrap_or(-1))
    }
}
