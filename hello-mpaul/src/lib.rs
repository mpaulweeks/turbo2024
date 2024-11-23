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

const MIN_ACTION_TIME: f32 = 0.0;
const MAX_ACTION_TIME: f32 = 180.0;

turbo::go!({
    let mut state = GameState::load();
    let logic_snapshot = simulate_game(state.history.clone()).current;
    // todo delta time?
    state.history.action_time =
        (state.history.action_time + 1.0).clamp(MIN_ACTION_TIME, MAX_ACTION_TIME);

    if gamepad(0).a.just_pressed() {
        state.history.actions.pop();
    } else if mouse(0).left.just_pressed() {
        if let Some(action) = click_action(logic_snapshot.p1) {
            state.history.action_time = MIN_ACTION_TIME;
            state.history.actions.push(action);
        }
    }

    state.save();

    // render
    let action_progress = (state.history.action_time - MIN_ACTION_TIME) / MAX_ACTION_TIME;
    let delta = simulate_game(state.history.clone());

    // todo render tween from prev to curr
    let players = vec![delta.current.p1, delta.current.p2];
    for p in players.iter() {
        render_player(p.clone());
    }
});

#[export_name = "turbo/hello"]
unsafe extern "C" fn on_hello() -> usize {
    os::server::log!("Hello, world!");
    return os::server::COMMIT;
}
