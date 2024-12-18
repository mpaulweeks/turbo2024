mod action;
use action::*;
mod attack;
use attack::*;
mod card;
use card::*;
mod history;
use history::*;
mod impulse;
use impulse::*;
mod main_menu;
use main_menu::*;
mod player;
use player::*;
mod runner;
use runner::*;
mod sim;
use sim::*;
mod ui;
use ui::*;
mod unit;
use unit::*;
mod server_comm;
mod util;

use util::*;

turbo::cfg! {r#"
    name = "variable_instance"
    version = "1.0.0"
    author = "Turbo"
    description = "Your first turbo os program"
    [settings]
    resolution = [800, 600]
    [turbo-os]
    api-url = "https://os.turbo.computer"
"#}

const PROGRAM_ID: &str = "variable_instance";

turbo::init! {
  struct GameState {
        history: GameHistory,
        main_menue_state: MainMenuState,
        start_matchmaking_tick: usize,
        match_info: MatchInfo,
        game_mode: enum GameMode{
            MainMenu,
            PlayingMatch
        },
        testing: bool,
  } = {
    Self {
            history: create_game(None, &mut Vec::new()),
            main_menue_state: MainMenuState::TitleScreen,
            start_matchmaking_tick: 0,
            match_info: MatchInfo::new(),
            game_mode: GameMode::MainMenu,
            testing: false,
        }
  }
}

turbo::go!({
    let mut state = GameState::load();

    match state.game_mode {
        GameMode::MainMenu => {
            main_menu_go(&mut state);
            if state.testing {
                state.game_mode = GameMode::PlayingMatch;
            }
        }
        GameMode::PlayingMatch => {
            update(&mut state);
            render(&mut state);
        }
    }

    state.save();
});
