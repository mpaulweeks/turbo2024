use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Card {
    pub instance_id: u32,
    sprite: String,
}

#[derive(Clone)]
pub struct PositionedCard {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    pub hover: bool,
    pub card: Card,
}

pub fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    for _ in 0..10 {
        deck.push(Card {
            instance_id: deck.len() as u32 + 1,
            sprite: "VICardForward_Front".to_string(),
        });
    }
    return deck;
}

pub fn position_card(card: Card, row: u8, column: usize) -> PositionedCard {
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    let slot_width = screen_width / 5.0;
    let slot_height = screen_height / 4.0;
    let card_width = 80.0;
    let card_height = 112.0;
    let x = slot_width * (column as f32 + 0.5);
    let y = slot_height * (row as f32 + 0.5);
    let left = (x - card_width / 2.0) as i32;
    let right = (x + card_width / 2.0) as i32;
    let top = (y - card_height / 2.0) as i32;
    let bottom = (y + card_height / 2.0) as i32;
    let [mx, my] = mouse(0).position;
    let hover = mx > left && mx < right && my > top && my < bottom;
    return PositionedCard {
        x,
        y,
        w: card_width,
        h: card_height,
        hover,
        card,
    };
}

pub fn render_card(pcard: PositionedCard, visible: bool) {
    if pcard.hover {
        let margin = pcard.w * 0.2;
        rect!(
            x = pcard.x - (pcard.w + margin) / 2.0,
            y = pcard.y - (pcard.h + margin) / 2.0,
            w = pcard.w + margin,
            h = pcard.h + margin,
            color = 0x8080ffff,
            border_radius = 10,
        );
    }
    let sprite_name = if visible {
        pcard.card.sprite
    } else {
        "VICard_Back".to_string()
    };
    sprite!(
        &sprite_name,
        x = pcard.x - (pcard.w / 2.0),
        y = pcard.y - (pcard.h / 2.0),
        w = pcard.w,
        h = pcard.h,
    );
}
