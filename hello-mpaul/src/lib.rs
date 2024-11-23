mod card;
use card::*;

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
    card1: Card,
    card2: Card,
  } = {
    let res = resolution();
    let w = res[0] as f32;
    let h = res[1] as f32;
    let paddle_width = 80.0;
    let paddle_height = 112.0;
    Self {
      card1: Card {
        x: 10.0,
        y: h / 2.0 - paddle_height / 2.0,
        width: paddle_width,
        height: paddle_height,
        sprite: "VICardForward_Front".to_string()
      },
      card2: Card {
        x: w - paddle_width - 10.0,
        y: h / 2.0 - paddle_height / 2.0,
        width: paddle_width,
        height: paddle_height,
        sprite: "VICard_Back".to_string()
      },
    }
  }
}

fn render_paddle(p: Card) {
    rect!(
        x = p.x,
        y = p.y,
        w = p.width,
        h = p.height,
        color = 0xffffff80
    );
    sprite!(&p.sprite, x = p.x, y = p.y, w = p.width, h = p.height);
}

turbo::go!({
    let mut state = GameState::load();

    let paddle_speed = 4.0;

    let res = resolution();
    let screen_h = res[1] as f32;

    let gp1 = gamepad(0);

    // Move paddle 1
    if gp1.up.pressed() && state.card1.y > 0.0 {
        state.card1.y -= paddle_speed;
    }
    if gp1.down.pressed() && state.card1.y + state.card1.height < screen_h {
        state.card1.y += paddle_speed;
    }

    state.save();

    let paddles = vec![state.card1, state.card2];
    for pad in paddles.iter() {
        render_paddle(pad.clone());
    }
});

#[export_name = "turbo/hello"]
unsafe extern "C" fn on_hello() -> usize {
    os::server::log!("Hello, world!");
    return os::server::COMMIT;
}
