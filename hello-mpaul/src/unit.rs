use crate::*;

type UnitCost = Vec<(ImpulseType, usize)>;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UnitCard {
    pub impulse_cost: UnitCost,
    pub impulse_turn: usize,
    pub power: u32,
    pub card: Card,
}

#[derive(Clone)]
pub struct PositionedUnit {
    pub unit: UnitCard,
    pub pcard: PositionedCard,
}

pub type Deck = Vec<UnitCard>;
pub fn create_deck() -> Deck {
    let drafts = unique_cards();
    let mut deck = Vec::new();
    for ud in drafts.iter() {
        for _ in 0..3 {
            deck.push(UnitCard {
                power: ud.power,
                impulse_turn: ud.impulse_turn,
                impulse_cost: ud.impulse_cost.clone(),
                card: Card {
                    card_id: deck.len() as u32 + 1,
                    sprite: ud.sprite.clone(),
                },
            });
        }
    }
    return shuffle(deck);
}

const POWER_WIDTH: f32 = 15.0;
const POWER_HEIGHT: f32 = 20.0;

pub fn render_unit(punit: PositionedUnit, visible: bool) {
    render_card(punit.pcard.clone(), visible);
    if visible {
        text!(
            &punit.unit.power.to_string(),
            x = -POWER_WIDTH + punit.pcard.x + punit.pcard.w / 2.0,
            y = -POWER_HEIGHT + punit.pcard.y + punit.pcard.h / 2.0,
            color = 0xffffffff,
            font = Font::L,
        );
    }
}

struct UnitDraft {
    impulse_cost: UnitCost,
    impulse_turn: usize,
    power: u32,
    sprite: String,
}

fn unique_cards() -> Vec<UnitDraft> {
    return vec![
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Red, 3)],
            impulse_turn: 5,
            power: 5,
            sprite: "Chancellor_Tenn".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Blue, 1)],
            impulse_turn: 5,
            power: 3,
            sprite: "Jack_of_Club".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Red, 1)],
            impulse_turn: 5,
            power: 3,
            sprite: "Jack_of_Diamond".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Red, 1)],
            impulse_turn: 5,
            power: 3,
            sprite: "Jack_of_Heart".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Blue, 3)],
            impulse_turn: 3,
            power: 4,
            sprite: "Jack_of_Spade".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Red, 3), (ImpulseType::Blue, 1)],
            impulse_turn: 10,
            power: 9,
            sprite: "King_Chance".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![],
            impulse_turn: 8,
            power: 6,
            sprite: "Queen_Card".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![],
            impulse_turn: 3,
            power: 2,
            sprite: "TangoNiner".to_string(),
        },
    ];
}
