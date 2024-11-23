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
    let colors = [ImpulseType::Red, ImpulseType::Green, ImpulseType::Blue];
    let mut deck: Vec<ImpulseCard> = Vec::new();
    for resource in colors.iter() {
        for _ in 0..4 {
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
    }
    return shuffle(deck);
}

pub fn create_impulse_state(deck: Vec<ImpulseCard>) -> ImpulseState {
    return ImpulseState {
        board: Vec::new(),
        deck,
    };
}

struct PositionedImpulse {
    impulse: ImpulseCard,
    pos: CardPosition,
}

fn position_impulse(state: ImpulseState) -> Vec<PositionedImpulse> {
    return state
        .board
        .iter()
        .enumerate()
        .map(|tuple| {
            let (index, impulse) = tuple;
            return PositionedImpulse {
                impulse: impulse.clone(),
                pos: position_card(2.0, index as f32, None),
            };
        })
        .collect();
}
pub fn render_impulse(state: ImpulseState) {
    for pimp in position_impulse(state).iter() {
        render_card(
            pimp.pos.clone(),
            pimp.impulse.card.sprite.clone(),
            true,
            None,
        );
    }
}

pub fn impulse_check(unit: UnitCard, state: ImpulseState) -> bool {
    let impulse_types: Vec<ImpulseType> =
        state.board.iter().map(|ic| ic.resource.clone()).collect();
    if unit.impulse_turn > impulse_types.len() {
        return false;
    }
    let mut impulse_counts: HashMap<ImpulseType, usize> = HashMap::new();
    for it in impulse_types.iter() {
        *impulse_counts.entry(it.clone()).or_default() += 1;
    }
    let mut can_afford = true;
    for cost in unit.impulse_cost.iter() {
        let (it, num) = cost;
        let count = *impulse_counts.entry(it.clone()).or_default();
        can_afford = can_afford && count >= *num;
    }
    return can_afford;
}
