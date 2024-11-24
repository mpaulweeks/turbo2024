use crate::*;

pub type CardId = u32;
pub const EMPTY_CARD_ID: CardId = 9998;
pub const READY_CARD_ID: CardId = 9999;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Card {
    pub card_id: CardId,
    pub sprite: String,
}

#[derive(Clone)]
pub struct CardPosition {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub hover: bool,
    pub action: Option<Action>,
}

pub fn position_card(row: f32, col: f32, action: Option<Action>) -> CardPosition {
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    let grid_width = screen_width * 0.8;
    let slot_width = grid_width / 8.0;
    let slot_height = screen_height / 5.0;
    let card_width = 80.0;
    let card_height = 112.0;
    let x = (screen_width - grid_width) + slot_width * (col + 0.5);
    let y = slot_height * (row + 0.5);
    let left = (x - card_width / 2.0) as i32;
    let right = (x + card_width / 2.0) as i32;
    let top = (y - card_height / 2.0) as i32;
    let bottom = (y + card_height / 2.0) as i32;
    let [mx, my] = mouse(0).position;
    let hover = mx > left && mx < right && my > top && my < bottom;
    return CardPosition {
        x,
        y,
        w: card_width,
        h: card_height,
        hover,
        action,
    };
}

pub fn render_card(
    pcard: CardPosition,
    sprite: String,
    visible: bool,
    rimlight: Option<u32>,
    highlight: Option<u32>,
) {
    let maybe_rim: Option<u32> = if pcard.hover {
        if visible && pcard.action.is_some() {
            Some(0x00FF00FF)
        } else {
            Some(0x8080FFFF)
        }
    } else {
        rimlight
    };
    if let Some(color) = maybe_rim {
        let margin = pcard.w * 0.2;
        rect!(
            x = pcard.x - (pcard.w + margin) / 2.0,
            y = pcard.y - (pcard.h + margin) / 2.0,
            w = pcard.w + margin,
            h = pcard.h + margin,
            color = color,
            border_radius = 10,
        );
    }
    let sprite_name = if visible {
        sprite
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
    if let Some(color) = highlight {
        rect!(
            x = pcard.x - (pcard.w / 2.0),
            y = pcard.y - (pcard.h / 2.0),
            w = pcard.w,
            h = pcard.h,
            color = color,
            border_radius = 10,
        );
    }
}

fn tween(start: f32, end: f32, percent: f32) -> f32 {
    return start + (end - start) * percent;
}

pub fn tween_card(current: CardPosition, previous: CardPosition, percent: f32) -> CardPosition {
    // https://stackoverflow.com/a/25730573
    let ease = percent * percent * (3.0 - 2.0 * percent);
    return CardPosition {
        x: tween(previous.x, current.x, ease),
        y: tween(previous.y, current.y, ease),
        w: tween(previous.w, current.w, ease),
        h: tween(previous.h, current.h, ease),
        hover: current.hover,
        action: current.action,
    };
}
