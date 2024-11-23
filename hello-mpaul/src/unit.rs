use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UnitCard {
    pub impulse_turn: usize,
    pub impulse_type: ImpulseType,
    pub card: Card,
}

pub type Deck = Vec<UnitCard>;

pub fn create_deck() -> Deck {
    let mut deck = Vec::new();
    for _ in 0..10 {
        deck.push(UnitCard {
            impulse_turn: 1,
            impulse_type: ImpulseType::Blue,
            card: Card {
                card_id: deck.len() as u32 + 1,
                sprite: "VICardForward_Front".to_string(),
            },
        });
    }
    // todo shuffle
    return deck;
}
