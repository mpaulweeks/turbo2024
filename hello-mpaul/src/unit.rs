use crate::*;

type UnitCost = Vec<(ImpulseType, usize)>;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UnitCard {
    pub impulse_cost: UnitCost,
    pub impulse_turn: usize,
    pub power: i32,
    pub attacking: bool,
    pub revealed: bool,
    pub card: Card,
}

#[derive(Clone)]
pub struct PositionedUnit {
    pub unit: UnitCard,
    pub pos: CardPosition,
}

pub fn create_ready_unit() -> UnitCard {
    return UnitCard {
        impulse_cost: vec![],
        impulse_turn: 0,
        power: 0,
        attacking: false,
        revealed: true,
        card: Card {
            card_id: READY_CARD_ID,
            sprite: "READY".to_string(),
        },
    };
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
                attacking: false,
                revealed: false,
                card: Card {
                    card_id: deck.len() as u32 + 1,
                    sprite: ud.sprite.clone(),
                },
            });
        }
    }
    return shuffle(deck);
}

const POWER_WIDTH: f32 = 16.0;
const POWER_HEIGHT: f32 = 32.0;
const POWER_MARGIN_X: f32 = 5.0;
const POWER_MARGIN_Y: f32 = 2.0;

const CLOCK_WIDTH: f32 = 16.0;
const CLOCK_HEIGHT: f32 = 16.0;
const CLOCK_MARGIN_X: f32 = 7.0;
const CLOCK_MARGIN_Y: f32 = -5.0;

const MANA_WIDTH: f32 = 8.0;
const MANA_MARGIN: f32 = 0.0;

impl PositionedUnit {
    pub fn render_target(&self) {
        // Draw the planned attack
        // Get the mouse state for player 1
        let m = mouse(0);

        // Get the mouse's x and y positions
        let [mx, my] = m.position;

        // Draw a circular cursor
        circ!(d = 16, x = mx - 8, y = my - 8, color = 0xe64539ff);

        // Draw line from attacker to cursor
        path!(
            start = (self.pos.x, self.pos.y),
            end = (mx, my),
            width = 2,
            color = 0xff00ffff,
        );
    }
}

pub fn render_unit(player: PlayerState, punit: PositionedUnit, visible: bool) {
    let is_ready = punit.unit.card.card_id == READY_CARD_ID;
    let visible = visible || punit.unit.revealed;
    let is_targetting = if let Some(targetter) = player.targeting {
        targetter == punit.unit.card.card_id
    } else {
        false
    };
    let rimlight = if punit.unit.revealed {
        Some(0x00FFFF80)
    } else {
        None
    };
    let highlight = if is_targetting {
        Some(0x00FF0080)
    } else if punit.unit.attacking {
        Some(0xFF000080)
    } else if is_ready && player.ready {
        Some(0x00FF0080)
    } else {
        None
    };
    render_card(
        punit.pos.clone(),
        punit.unit.card.sprite,
        visible,
        rimlight,
        highlight,
    );
    if visible && !is_ready {
        // draw unit details
        sprite!(
            &get_atk_sprite(punit.unit.power),
            x = punit.pos.x + punit.pos.w / 2.0 - (POWER_WIDTH + POWER_MARGIN_X),
            y = punit.pos.y + punit.pos.h / 2.0 - (POWER_HEIGHT + POWER_MARGIN_Y),
        );
        sprite!(
            &get_clock_sprite(punit.unit.impulse_turn),
            x = punit.pos.x + punit.pos.w / 2.0 - (CLOCK_WIDTH + CLOCK_MARGIN_X),
            y = punit.pos.y - punit.pos.h / 2.0 - (CLOCK_MARGIN_Y),
        );

        let mut impulse_types: Vec<ImpulseType> = Vec::new();
        for tuple in punit.unit.impulse_cost.iter() {
            let (it, num) = tuple;
            for _ in 0..(*num as i32) {
                impulse_types.push(it.clone());
            }
        }
        for (index, it) in impulse_types.iter().enumerate() {
            sprite!(
                &get_mana_sprite(it.clone()),
                x = punit.pos.x + punit.pos.w / 2.0
                    - (CLOCK_WIDTH
                        + CLOCK_MARGIN_X
                        + 1.0
                        + ((MANA_MARGIN + MANA_WIDTH) * (index as f32 + 1.0))),
                y = 3.0 + punit.pos.y - punit.pos.h / 2.0 + (MANA_MARGIN),
            );
        }
    }
}

struct UnitDraft {
    impulse_cost: UnitCost,
    impulse_turn: usize,
    power: i32,
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
            impulse_cost: vec![],
            impulse_turn: 1,
            power: 1,
            sprite: "Jack_of_Club".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![],
            impulse_turn: 1,
            power: 1,
            sprite: "Jack_of_Diamond".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Red, 1)],
            impulse_turn: 1,
            power: 2,
            sprite: "Jack_of_Heart".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Blue, 1)],
            impulse_turn: 2,
            power: 3,
            sprite: "Jack_of_Spade".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Red, 3), (ImpulseType::Blue, 2)],
            impulse_turn: 10,
            power: 14,
            sprite: "King_Chance".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![],
            impulse_turn: 8,
            power: 7,
            sprite: "Queen_Card".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Blue, 1)],
            impulse_turn: 3,
            power: 4,
            sprite: "TangoNiner".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![],
            impulse_turn: 1,
            power: 0,
            sprite: "joker".to_string(),
        },
        UnitDraft {
            impulse_cost: vec![(ImpulseType::Red, 1), (ImpulseType::Blue, 1)],
            impulse_turn: 4,
            power: 6,
            sprite: "ace_one".to_string(),
        },
    ];
}
