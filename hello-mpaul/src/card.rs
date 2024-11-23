use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Card {
    pub sprite: String,
}

pub fn create_deck() -> Vec<Card> {
    return vec![
        Card {
            sprite: "VICardForward_Front".to_string(),
        },
        Card {
            sprite: "VICardForward_Front".to_string(),
        },
        Card {
            sprite: "VICardForward_Front".to_string(),
        },
        Card {
            sprite: "VICardForward_Front".to_string(),
        },
        Card {
            sprite: "VICardForward_Front".to_string(),
        },
        Card {
            sprite: "VICardForward_Front".to_string(),
        },
        Card {
            sprite: "VICardForward_Front".to_string(),
        },
    ];
}

pub fn render_card(p: Card, player: u8, slot: usize, visible: bool) {
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    let slot_width = screen_width / 5.0;
    let slot_height = screen_height / 2.0;
    let card_width = 80.0;
    let card_height = 112.0;
    let x = slot_width * (slot as f32 + 0.5) - (card_width / 2.0);
    let y = slot_height * (player as f32 + 0.5) - (card_height / 2.0);
    rect!(
        x = x,
        y = y,
        w = card_width,
        h = card_height,
        color = 0xffffff80
    );
    let sprite_name = if visible {
        p.sprite
    } else {
        "VICard_Back".to_string()
    };
    sprite!(&sprite_name, x = x, y = y, w = card_width, h = card_height);
}
