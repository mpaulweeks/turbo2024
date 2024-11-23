use crate::*;

pub type Deck = Vec<Card>;

pub fn create_deck() -> Deck {
    let mut deck = Vec::new();
    for _ in 0..10 {
        deck.push(Card {
            card_id: deck.len() as u32 + 1,
            sprite: "VICardForward_Front".to_string(),
        });
    }
    // todo shuffle
    return deck;
}
