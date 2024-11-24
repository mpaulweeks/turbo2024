use std::string::ToString;
use turbo::borsh::*;
use crate::*;

const BACKGROUND_COLOR: u32 = 0x324aa8ff;
const WHITE_COLOR: u32 = 0xFFFFFFff;
const GREEN_COLOR: u32 = 0x00FF7Fff;
const RED_COLOR: u32 = 0xFF4040ff;
const BUTTON_COLOR: u32 = 0x6ec25bff;
const BUTTON_TEXT_COLOR: u32 = 0xF0F8FFff;

const PROGRAM_ID: &str = "jw_test_turbo";

const MATCHMAKING_FILE: &str = "matchmaker";

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct MainMenuState {
    pub searching_for_match: bool,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct MatchInfo {
    match_started: bool,
    pub inviter_user: String,
    pub joining_user: String,
    pub match_id: u32,
    last_refresh_time: u32,
}

impl MatchInfo {
    pub fn new() -> MatchInfo {
        Self{
            match_started: false,
            inviter_user: "".to_string(),
            joining_user: "".to_string(),
            match_id: 0,
            last_refresh_time: 0,
        }
    }
}

pub fn main_menu_go(state: &mut GameState)
{
    if !state.main_menue_state.searching_for_match {
        title_screen_go(state);
    } else {
        finding_opponent_go(state);
    }
}



fn finding_opponent_go(state: &mut GameState) {
    draw_logo();
    let mut dot_txt = "";
    let dot_timer = ((tick() / 40) % 5) as i32;
    match dot_timer {
        0 => dot_txt = ".",
        1 => dot_txt = "..",
        2 => dot_txt = "...",
        3 => dot_txt = "....",
        _ => {}
    }

    let search_txt = format!("Searching for opponent{}", dot_txt);
    text!(
        search_txt.as_str(),
        x = 235,
        y = 365,
        font = Font::XL,
        color = BUTTON_TEXT_COLOR
    );
}

fn draw_logo(){
    clear!(BACKGROUND_COLOR);
    sprite!("VARIABLE_INSTANCE_logo",
        x = 210,
        y = 100);
}

fn draw_find_match_button(){
    let (w, h) = (30, 20);
    let (x, y) = (20, 180);
}

fn title_screen_go(state: &mut GameState){
    draw_logo();

    // draw find match button
    let (w, h) = (220, 50);
    let (x, y) = (320, 380);
    draw_button(w, h, x, y);

    text!(
        "Find Match",
        x = x+35,
        y = y+18,
        font = Font::XL,
        color = BUTTON_TEXT_COLOR
    );

    let m = mouse(0);

    if m.left.just_pressed() && button_contains_pos(m.position[0], m.position[1], w, h, x, y) {
        state.main_menue_state.searching_for_match = true;
    }

}

fn draw_button(w: i32, h: i32, x: i32, y: i32) {
    rect!(
        w = w,
        h = h,
        y = y,
        x = x,
        color = BUTTON_COLOR,
        border_radius = 6
    );
}

fn button_contains_pos(px: i32, py: i32, w: i32, h: i32, x: i32, y: i32) -> bool {
    px >= x && px <= x + w && py >= y && py <= y + h
}