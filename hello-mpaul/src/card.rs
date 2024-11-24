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

pub fn render_background() {
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    let grid_width = screen_width * 0.9;
    let slot_width = grid_width / 8.0;
    let slot_height = screen_height / 5.0;

    let mut n = 7;

    sprite! {
        "TIME_TILE",
        x = -7.0 + (screen_width - grid_width) + slot_width,
        y = slot_height * 2.0,

    }

    while n > 0 {
        sprite!(
            "Card_Place_Tile",
            x = -4.0 + (screen_width - grid_width) + slot_width * (n) as f32,
            y = slot_height * 1.0 - 25.0,
            flip_y = true,
        );
        sprite!(
            "Card_Place_Tile",
            x = -4.0 + (screen_width - grid_width) + slot_width * n as f32,
            y = slot_height * 3.0 + 25.0,
        );
        n -= 1;
    }
}

pub fn position_card(row: f32, col: f32, action: Option<Action>, arr_len: usize) -> CardPosition {
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    let grid_width = screen_width * 0.9;
    let base_slot_width = grid_width / 8.0;
    let slot_width = grid_width / (arr_len as f32 + 2.0).clamp(8.0, 16.0);
    let slot_height = screen_height / 5.0;
    let card_width = 80.0;
    let card_height = 112.0;
    let mut h = 0;
    let x = (screen_width - grid_width) + base_slot_width + slot_width * (col + 0.5);
    let mut y = slot_height * (row + 0.5);

    if row == 4.0 {
        y += 60.0;
        h = -40;
        // x -= 30.0;
    }
    let left = (x - card_width / 2.0) as i32;
    let right = (x + card_width / 2.0) as i32;
    let top = h + (y - card_height / 2.0) as i32;
    let bottom = (y + card_height / 2.0) as i32;
    let [mx, my] = mouse(0).position;
    let hover = mx > left && mx < right && my > top && my < bottom;

    if row == 3.0 {
        y += 25.0;
    }

    if row == 1.0 {
        y -= 25.0;
    }

    if row == 0.0 {
        y -= 35.0;
    }

    if hover && row >= 1.0 {
        y -= 10.0;
        if row == 4.0 {
            y -= 50.0;
        }
    }
    return CardPosition {
        x,
        y,
        w: card_width,
        h: card_height,
        hover,
        action,
    };
}

impl CardPosition {
    pub fn render_card(
        &self,
        sprite: String,
        visible: bool,
        rimlight: Option<u32>,
        highlight: Option<u32>,
    ) {
        let maybe_rim: Option<u32> = if self.hover {
            if visible && self.action.is_some() {
                Some(0x00FF00FF)
            } else {
                Some(0x8080FFFF)
            }
        } else {
            rimlight
        };
        if let Some(color) = maybe_rim {
            let margin = self.w * 0.2;
            rect!(
                x = self.x - (self.w + margin) / 2.0,
                y = self.y - (self.h + margin) / 2.0,
                w = self.w + margin,
                h = self.h + margin,
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
            x = self.x - (self.w / 2.0),
            y = self.y - (self.h / 2.0),
            w = self.w,
            h = self.h,
        );
        if let Some(color) = highlight {
            rect!(
                x = self.x - (self.w / 2.0),
                y = self.y - (self.h / 2.0),
                w = self.w,
                h = self.h,
                color = color,
                border_radius = 10,
            );
        }
    }

    pub fn render_target(&self, other: Option<CardPosition>) {
        if let Some(o) = other {
            // Draw a circular cursor
            circ!(d = 16, x = o.x - 8.0, y = o.y - 8.0, color = 0xe64539ff);
            path!(
                start = (self.x, self.y),
                end = (o.x, o.y),
                width = 2,
                color = 0xe64539ff,
            );
        } else {
            // Draw the planned attack
            // Get the mouse state for player 1
            let m = mouse(0);

            // Get the mouse's x and y positions
            let [mx, my] = m.position;

            // Draw a circular cursor
            circ!(d = 16, x = mx - 8, y = my - 8, color = 0xe64539ff);

            // Draw line from attacker to cursor
            path!(
                start = (self.x, self.y),
                end = (mx, my),
                width = 2,
                color = 0xe64539ff,
            );
        }
    }
}

fn tween(start: f32, end: f32, percent: f32) -> f32 {
    return start + (end - start) * percent;
}

pub fn tween_card(current: CardPosition, previous: CardPosition, percent: f32) -> CardPosition {
    // https://stackoverflow.com/a/25730573
    let ease = percent * percent * (5.0 - 4.0 * percent);
    return CardPosition {
        x: tween(previous.x, current.x, ease),
        y: tween(previous.y, current.y, ease),
        w: tween(previous.w, current.w, ease),
        h: tween(previous.h, current.h, ease),
        hover: current.hover,
        action: current.action,
    };
}
