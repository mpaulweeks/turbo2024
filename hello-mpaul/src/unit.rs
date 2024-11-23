use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UnitCard {
    pub impulse_turn: usize,
    pub impulse_type: ImpulseType,
    pub card: Card,
}

pub type Deck = Vec<UnitCard>;
pub fn create_deck() -> Deck {
    let drafts = unique_cards();
    let mut deck = Vec::new();
    for ud in drafts.iter() {
        for _ in 0..3 {
            deck.push(UnitCard {
                impulse_turn: ud.impulse_turn,
                impulse_type: ud.impulse_type.clone(),
                card: Card {
                    card_id: deck.len() as u32 + 1,
                    sprite: ud.sprite.clone(),
                },
            });
        }
    }
    return shuffle(deck);
}

struct UnitDraft {
    impulse_turn: usize,
    impulse_type: ImpulseType,
    sprite: String,
}

fn unique_cards() -> Vec<UnitDraft> {
    return vec![
        UnitDraft {
            impulse_turn: 1,
            impulse_type: ImpulseType::Blue,
            sprite: "Chancellor_Tenn".to_string(),
        },
        UnitDraft {
            impulse_turn: 1,
            impulse_type: ImpulseType::Blue,
            sprite: "Jack_of_Club".to_string(),
        },
        UnitDraft {
            impulse_turn: 1,
            impulse_type: ImpulseType::Blue,
            sprite: "Jack_of_Diamond".to_string(),
        },
        UnitDraft {
            impulse_turn: 1,
            impulse_type: ImpulseType::Blue,
            sprite: "Jack_of_Heart".to_string(),
        },
        UnitDraft {
            impulse_turn: 3,
            impulse_type: ImpulseType::Blue,
            sprite: "Jack_of_Spade".to_string(),
        },
        UnitDraft {
            impulse_turn: 10,
            impulse_type: ImpulseType::Blue,
            sprite: "King_Chance".to_string(),
        },
        UnitDraft {
            impulse_turn: 8,
            impulse_type: ImpulseType::Blue,
            sprite: "Queen_Card".to_string(),
        },
        UnitDraft {
            impulse_turn: 1,
            impulse_type: ImpulseType::Blue,
            sprite: "TangoNiner".to_string(),
        },
    ];
}
