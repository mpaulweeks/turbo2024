use crate::*;
use std::string::ToString;
use turbo::borsh::*;

const BACKGROUND_COLOR: u32 = 0x324aa8ff;
const WHITE_COLOR: u32 = 0xFFFFFFff;
const GREEN_COLOR: u32 = 0x00FF7Fff;
const RED_COLOR: u32 = 0xFF4040ff;
const BUTTON_COLOR: u32 = 0x6ec25bff;
const BUTTON_TEXT_COLOR: u32 = 0xF0F8FFff;

const MATCHMAKING_FILE: &str = "matchmaker";

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum MainMenuState {
    TitleScreen,
    WaitingForMatch,
    WaitingForRands,
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
        Self {
            match_started: false,
            inviter_user: "".to_string(),
            joining_user: "".to_string(),
            match_id: 0,
            last_refresh_time: 0,
        }
    }
}

pub fn main_menu_go(state: &mut GameState) {
    match state.main_menue_state {
        MainMenuState::TitleScreen => title_screen_go(state),
        MainMenuState::WaitingForMatch => finding_opponent_go(state),
        MainMenuState::WaitingForRands => wait_for_rands(state),
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

    if tick() % 20 == 0 {
        let delta: i32 = 0;
        let bytes = delta.to_le_bytes();
        os::client::exec(PROGRAM_ID, "try_find_match", &bytes);
    }

    if (tick() - state.start_matchmaking_tick) < 60 {
        return;
    }

    let server_match_info = os::client::watch_file(PROGRAM_ID, &MATCHMAKING_FILE)
        .data
        .and_then(|file| MatchInfo::try_from_slice(&file.contents).ok());

    if let Some(m) = server_match_info {
        state.match_info = m;
    }

    if state.match_info.match_started {
        let user_id = os::client::user_id();
        if let Some(ref id) = user_id {
            if state.match_info.joining_user == id.to_string() {
                //tell server to create rands
                let bytes = state.match_info.match_id.to_le_bytes();
                os::client::exec(PROGRAM_ID, "initialize_rands", &bytes);
                state.main_menue_state = MainMenuState::WaitingForRands;
                log!("Initializing rands");
            } else if state.match_info.inviter_user == id.to_string() {
                let bytes = state.match_info.match_id.to_le_bytes();
                state.main_menue_state = MainMenuState::WaitingForRands;
                log!("Waiting for rands");
            }
        }
    }
}

#[export_name = "turbo/try_find_match"]
unsafe extern "C" fn on_try_find_match() -> usize {
    os::server::log!("Start");
    let mut new_match_info = MatchInfo {
        match_started: false,
        inviter_user: os::server::get_user_id(),
        joining_user: "".to_string(),
        match_id: os::server::random_number(),
        last_refresh_time: 0,
    };

    let user_id = os::server::get_user_id();
    let mut match_info = os::server::read_or!(MatchInfo, &MATCHMAKING_FILE, new_match_info.clone());

    let mut need_to_write_file = false;

    if os::server::secs_since_unix_epoch() - match_info.last_refresh_time > 2
    {
        // stale match, so make a new one
        need_to_write_file = true;
        new_match_info.last_refresh_time = os::server::secs_since_unix_epoch();
        match_info = new_match_info;
        os::server::log!("Creating Match: {}", match_info.match_id);
    }
    else if !match_info.match_started
    {
        if match_info.inviter_user == user_id
        {
            // our match invitation, so just refresh the time if needed
            if os::server::secs_since_unix_epoch() - match_info.last_refresh_time > 1
            {
                need_to_write_file = true;
                match_info.last_refresh_time = os::server::secs_since_unix_epoch();
                os::server::log!("refreshing time");
            }
        }
        else {
            // join match if it hasn't started
            need_to_write_file = true;
            match_info.joining_user = user_id;
            match_info.match_started = true;
            match_info.last_refresh_time = os::server::secs_since_unix_epoch();
            os::server::log!("Joining Match: {}", match_info.match_id);
        }
    }

    if need_to_write_file {
        os::server::log!("Writing file");
        let Ok(_) = os::server::write!(&MATCHMAKING_FILE, match_info) else {
            return os::server::CANCEL;
        };
    }

    os::server::COMMIT
}

fn draw_logo() {
    clear!(BACKGROUND_COLOR);
    sprite!("VARIABLE_INSTANCE_logo", x = 210, y = 100);
}

fn title_screen_go(state: &mut GameState) {
    draw_logo();

    // draw find match button
    let (w, h) = (220, 50);
    let (x, y) = (320, 380);
    let y2 = y + ((h as f32) * 1.5) as i32;
    let mut color = BUTTON_COLOR;
    let mut color2 = BUTTON_COLOR;

    let m = mouse(0);

    if button_contains_pos(m.position[0], m.position[1], w, h, x, y) {
        color = 0x000000FF;
        if m.left.just_pressed() {
            state.start_matchmaking_tick = tick();
            state.main_menue_state = MainMenuState::WaitingForMatch;
        }
    }
    draw_button(w, h, x, y, color);
    text!(
        "Find Match",
        x = x + 35,
        y = y + 18,
        font = Font::XL,
        color = BUTTON_TEXT_COLOR
    );

    if button_contains_pos(m.position[0], m.position[1], w, h, x, y2) {
        color2 = 0x000000FF;
        if m.left.just_pressed() {
            state.testing = true;
        }
    }
    draw_button(w, h, x, y2, color2);
    text!(
        "Play Local",
        x = x + 35,
        y = y2 + 18,
        font = Font::XL,
        color = BUTTON_TEXT_COLOR
    );
}

fn wait_for_rands(state: &mut GameState) {
    draw_logo();
    text!(
        "Starting match!",
        x = 235,
        y = 365,
        font = Font::XL,
        color = BUTTON_TEXT_COLOR
    );

    let rands = os::client::watch_file(PROGRAM_ID, &format!("{}/rands", state.match_info.match_id))
        .data
        .and_then(|file| Rands::try_from_slice(&file.contents).ok());

    if let Some(mut r) = rands {
        let mut player_id: Option<PlayerId> = None;
        let user_id = os::client::user_id();
        if let Some(ref uid) = user_id {
            if state.match_info.inviter_user == *uid {
                player_id = Option::from(PlayerId::P1);
            } else {
                player_id = Option::from(PlayerId::P2);
            }
        }
        state.history = create_game(player_id, &mut r);
        state.game_mode = GameMode::PlayingMatch;
    }
}

fn draw_button(w: i32, h: i32, x: i32, y: i32, color: u32) {
    rect!(w = w, h = h, y = y, x = x, color = color, border_radius = 6);
}

fn button_contains_pos(px: i32, py: i32, w: i32, h: i32, x: i32, y: i32) -> bool {
    px >= x && px <= x + w && py >= y && py <= y + h
}

#[export_name = "turbo/initialize_rands"]
unsafe extern "C" fn on_initialize_rands() -> usize {
    let mut server_rands = Rands::new();

    for _ in 1..100 {
        server_rands.push(os::server::random_number());
    }

    let match_id = os::server::command!(u32);

    let Ok(_) = os::server::write!(&format!("{}/rands", match_id), server_rands) else {
        return os::server::CANCEL;
    };

    os::server::COMMIT
}
