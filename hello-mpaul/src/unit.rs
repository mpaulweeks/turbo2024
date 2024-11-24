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
pub fn create_deck(rands: &mut Rands) -> Deck {
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
    return shuffle(deck, rands);
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
    pub fn render_unit(&self, player: PlayerState, visible: bool) {
        let is_ready = self.unit.card.card_id == READY_CARD_ID;
        let visible = visible || self.unit.revealed;
        let is_targetting = if let Some(targetter) = player.targeting {
            targetter == self.unit.card.card_id
        } else {
            false
        };
        let rimlight = if self.unit.revealed {
            Some(0xFFFFFFFF)
        } else {
            None
        };
        let highlight = if is_targetting {
            Some(0x00FF0080)
        } else if self.unit.attacking {
            Some(0xFF000080)
        } else if is_ready && player.ready {
            Some(0x00FF0080)
        } else {
            None
        };
        self.pos
            .render_card(self.unit.card.sprite.clone(), visible, rimlight, highlight);
        if visible && !is_ready {
            // draw unit details
            sprite!(
                &get_atk_sprite(self.unit.power),
                x = self.pos.x + self.pos.w / 2.0 - (POWER_WIDTH + POWER_MARGIN_X),
                y = self.pos.y + self.pos.h / 2.0 - (POWER_HEIGHT + POWER_MARGIN_Y),
            );
            sprite!(
                &get_clock_sprite(self.unit.impulse_turn),
                x = self.pos.x + self.pos.w / 2.0 - (CLOCK_WIDTH + CLOCK_MARGIN_X),
                y = self.pos.y - self.pos.h / 2.0 - (CLOCK_MARGIN_Y),
            );

            let mut impulse_types: Vec<ImpulseType> = Vec::new();
            for tuple in self.unit.impulse_cost.iter() {
                let (it, num) = tuple;
                for _ in 0..(*num as i32) {
                    impulse_types.push(it.clone());
                }
            }
            for (index, it) in impulse_types.iter().enumerate() {
                sprite!(
                    &get_mana_sprite(it.clone()),
                    x = self.pos.x + self.pos.w / 2.0
                        - (CLOCK_WIDTH
                            + CLOCK_MARGIN_X
                            + 1.0
                            + ((MANA_MARGIN + MANA_WIDTH) * (index as f32 + 1.0))),
                    y = 3.0 + self.pos.y - self.pos.h / 2.0 + (MANA_MARGIN),
                );
            }
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
