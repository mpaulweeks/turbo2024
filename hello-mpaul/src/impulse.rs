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

pub fn create_impulse_deck(rands: &mut Rands) -> Vec<ImpulseCard> {
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
    return shuffle(deck, rands);
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
                pos: position_card(2.0, index as f32, None, state.board.len()),
            };
        })
        .collect();
}
pub fn render_impulse(state: ImpulseState) {
    for pimp in position_impulse(state).iter() {
        pimp.pos
            .render_card(pimp.impulse.card.sprite.clone(), true, None, None);
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

pub fn render_ui(state: ImpulseState) {
    let mut blue = 0.0;
    let mut red = 0.0;
    let mut green = 0.0;
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    let grid_width = screen_width * 0.9;
    let slot_width = grid_width / 8.0;
    let slot_height = screen_height / 5.0;

    let impulse_types: Vec<ImpulseType> =
        state.board.iter().map(|ic| ic.resource.clone()).collect();
    // if unit.impulse_turn > impulse_types.len() {
    //   return false;
    // }

    let mut impulse_counts: HashMap<ImpulseType, usize> = HashMap::new();
    for it in impulse_types.iter() {
        *impulse_counts.entry(it.clone()).or_default() += 1;
    }

    let blueCount = impulse_counts.get(&ImpulseType::Blue).unwrap_or(&0);
    let redCount = impulse_counts.get(&ImpulseType::Red).unwrap_or(&0);
    let greenCount = impulse_counts.get(&ImpulseType::Green).unwrap_or(&0);
    let timeCount = redCount + blueCount + greenCount;

    sprite!(
        "Mana_Sprites1",
        x = screen_width / 5.25,
        y = screen_height / 2.0,
    );
    sprite!(
        "Mana_Sprites2",
        x = screen_width / 5.25,
        y = (screen_height / 2.0) + 20.0,
    );
    sprite!(
        "Mana_Sprites3",
        x = screen_width / 5.25,
        y = (screen_height / 2.0) - 20.0,
    );

    text!(
        &format!("{}", blueCount),
        x = screen_width / 5.25 - 7.0,
        y = screen_height / 2.0,
    );

    text!(
        &format!("{}", redCount),
        x = screen_width / 5.25 - 7.0,
        y = (screen_height / 2.0) + 20.0,
    );

    text!(
        &format!("{}", greenCount),
        x = screen_width / 5.25 - 7.0,
        y = (screen_height / 2.0) - 20.0,
    );

    text!(
        &format!("{}", greenCount),
        x = screen_width / 5.25 - 7.0,
        y = (screen_height / 2.0) - 20.0,
    );

    text!(
        &format!("[TURN {}]", timeCount),
        x = screen_width * 0.075,
        y = screen_height / 2.0 + 20.0,
        font = Font::L
    );
}
