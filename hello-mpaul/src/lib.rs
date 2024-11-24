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
mod util;
mod main_menu;
use main_menu::*;

use util::*;

turbo::cfg! {r#"
    name = "hello-mpaul"
    version = "1.0.0"
    author = "Turbo"
    description = "Your first turbo os program"
    [settings]
    resolution = [800, 600]
    [turbo-os]
    api-url = "https://os.turbo.computer"
"#}

turbo::init! {
  struct GameState {
        history: GameHistory,
        main_menue_state: MainMenuState,
        match_info: MatchInfo,
        game_mode: enum GameMode{
            MainMenu,
            PlayingMatch
        },
        testing: bool,
  } = {
    Self {
            history: create_game(),
            main_menue_state: MainMenuState { searching_for_match:false,},
            match_info: MatchInfo::new(),
            game_mode: GameMode::MainMenu,
            testing: false,
        }
  }
}

turbo::go!({
    let mut state = GameState::load();

    match state.game_mode {
        GameMode::MainMenu =>{
            main_menu_go(&mut state);
        },
        GameMode::PlayingMatch => {
            update();
            render();
        }
    }

    state.save();
});

#[export_name = "turbo/hello"]
unsafe extern "C" fn on_hello() -> usize {
    os::server::log!("Hello, world!");
    return os::server::COMMIT;
}
