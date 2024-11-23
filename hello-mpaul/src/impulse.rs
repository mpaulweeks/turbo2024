use std::collections::HashMap;

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum ImpulseType {
    Red,
    Green,
    Blue,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct ImpulseCard {
    card: Card,
    resource: ImpulseType,
}

#[derive(Clone)]
pub struct ImpulseState {
    pub board: Vec<ImpulseCard>,
    pub deck: Vec<ImpulseCard>,
}

pub fn create_impulse_deck() -> Vec<ImpulseCard> {
    let impulse_sprites: HashMap<ImpulseType, String> = HashMap::from([
        (ImpulseType::Red, "VICard_ResourceRed".to_string()),
        (ImpulseType::Green, "VICard_ResourceGreen".to_string()),
        (ImpulseType::Blue, "VICard_ResourceNavy".to_string()),
    ]);
    let colors = [
        ImpulseType::Red,
        ImpulseType::Red,
        ImpulseType::Green,
        ImpulseType::Green,
        ImpulseType::Blue,
        ImpulseType::Blue,
    ];
    let mut deck: Vec<ImpulseCard> = Vec::new();
    for resource in colors.iter() {
        if let Some(sprite) = impulse_sprites.get(resource) {
            deck.push(ImpulseCard {
                resource: resource.clone(),
                card: Card {
                    card_id: deck.len() as u32 + 1,
                    sprite: sprite.to_string(),
                },
            });
        }
    }
    // todo shuffle
    return deck;
}

pub fn create_impulse_state(deck: Vec<ImpulseCard>) -> ImpulseState {
    return ImpulseState {
        board: Vec::new(),
        deck,
    };
}

pub fn render_impulse(state: ImpulseState) {
    for (index, impulse) in state.board.iter().enumerate() {
        let pcard = position_card(impulse.card.clone(), 2, index, None);
        render_card(pcard, true);
    }
}

pub fn impulse_check(card: Card, state: ImpulseState) -> bool {
    return true;
}
