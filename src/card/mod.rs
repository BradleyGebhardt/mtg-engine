pub mod meta;

use meta::Card;

use std::fmt::{Display};

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card:{}|{}|{}", self.name, self.colors, self.edhrec_rank.unwrap_or(-1))
    }
}
