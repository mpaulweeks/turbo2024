use crate::*;

pub type CardId = u32;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Card {
    pub card_id: CardId,
    pub sprite: String,
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

pub fn position_card(card: Card, row: u8, col: usize) -> PositionedCard {
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    let slot_width = screen_width / 8.0;
    let slot_height = screen_height / 5.0;
    let card_width = 80.0;
    let card_height = 112.0;
    let x = slot_width * (col as f32 + 0.5);
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

fn tween(start: f32, end: f32, percent: f32) -> f32 {
    return start + (end - start) * percent;
}

pub fn tween_card(
    current: PositionedCard,
    previous: PositionedCard,
    percent: f32,
) -> PositionedCard {
    // https://stackoverflow.com/a/25730573
    let ease = percent * percent * (3.0 - 2.0 * percent);
    return PositionedCard {
        x: tween(previous.x, current.x, ease),
        y: tween(previous.y, current.y, ease),
        w: tween(previous.w, current.w, ease),
        h: tween(previous.h, current.h, ease),
        hover: current.hover,
        card: current.card,
    };
}
