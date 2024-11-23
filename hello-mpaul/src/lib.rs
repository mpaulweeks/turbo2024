mod action;
use action::*;
mod card;
use card::*;
mod deck;
use deck::*;
mod game;
use game::*;
mod player;
use player::*;

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
    let mut state = GameState::load();
    let logic_snapshot = simulate_game(state.history.clone());

    let clicked = mouse(0).left.just_pressed();
    if clicked {
        if let Some(action) = click_action(logic_snapshot.p1) {
            state.history.actions.push(action);
        }
    }

    state.save();

    // render
    let render_snapshot = simulate_game(state.history.clone());
    let players = vec![render_snapshot.p1, render_snapshot.p2];
    for p in players.iter() {
        render_player(p.clone());
    }
});

#[export_name = "turbo/hello"]
unsafe extern "C" fn on_hello() -> usize {
    os::server::log!("Hello, world!");
    return os::server::COMMIT;
}
