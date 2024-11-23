mod action;
use action::*;
mod card;
use card::*;
mod game;
use game::*;
mod history;
use history::*;
mod impulse;
use impulse::*;
mod player;
use player::*;
mod sim;
use sim::*;
mod unit;
use unit::*;
mod util;
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
  } = {
    Self {
      history: create_game(),
    }
  }
}

turbo::go!({
    update();
    render();
});

#[export_name = "turbo/hello"]
unsafe extern "C" fn on_hello() -> usize {
    os::server::log!("Hello, world!");
    return os::server::COMMIT;
}
