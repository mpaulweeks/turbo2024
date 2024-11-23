use crate::*;

pub struct GameHistory {
    deck: Deck,
    actions: Vec<Action>,
}

pub fn create_game() -> GameHistory {
    return GameHistory {
        deck: create_deck(),
        actions: Vec::new(),
    };
}
