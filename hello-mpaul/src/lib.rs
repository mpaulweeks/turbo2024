mod card;
use card::*;
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
    p1: Player,
    p2: Player,
  } = {
    Self {
      p1: create_player(0),
      p2: create_player(1),
    }
  }
}

turbo::go!({
    let state = GameState::load();
    // let mut state = GameState::load();

    // let gp1 = gamepad(0);

    // if gp1.up.pressed() && state.card1.y > 0.0 {
    //     state.card1.y -= paddle_speed;
    // }
    // if gp1.down.pressed() && state.card1.y + state.card1.height < screen_h {
    //     state.card1.y += paddle_speed;
    // }

    state.save();

    let players = vec![state.p1, state.p2];
    for p in players.iter() {
        for (i, c) in p.hand.iter().enumerate() {
            render_card(c.clone(), p.index, i, p.index == 0);
        }
    }
});

#[export_name = "turbo/hello"]
unsafe extern "C" fn on_hello() -> usize {
    os::server::log!("Hello, world!");
    return os::server::COMMIT;
}
