turbo::cfg! {r#"
    name = "hello-mpaul"
    version = "1.0.0"
    author = "Turbo"
    description = "Your first turbo os program"
    [settings]
    resolution = [132, 224]
    [turbo-os]
    api-url = "https://os.turbo.computer"
"#}

turbo::init! {
  struct GameState {
    paddle1: struct Paddle {
      x: f32,
      y: f32,
      height: f32,
      color: u32,
    },
    paddle2: Paddle,
  } = {
    let res = resolution();
    let w = res[0] as f32;
    let h = res[1] as f32;
    let paddle_width = 5.0;
    let paddle_height = 10.0;
    Self {
      paddle1: Paddle { x: 10.0, y: h / 2.0 - paddle_height / 2.0, height: paddle_height, color: 0xFF0000FF },
      paddle2: Paddle { x: w - paddle_width - 10.0, y: h / 2.0 - paddle_height / 2.0, height: paddle_height, color: 0x0000FFFF },
    }
  }
}

fn render_paddle(p: Paddle) {
    rect!(x = p.x, y = p.y, w = 8, h = p.height, color = p.color);
}

turbo::go!({
    let mut state = GameState::load();

    let paddle_speed = 4.0;

    let res = resolution();
    let screen_h = res[1] as f32;

    let gp1 = gamepad(0);

    // Move paddle 1
    if gp1.up.pressed() && state.paddle1.y > 0.0 {
        state.paddle1.y -= paddle_speed;
    }
    if gp1.down.pressed() && state.paddle1.y + state.paddle1.height < screen_h {
        state.paddle1.y += paddle_speed;
    }

    state.save();

    let paddles = vec![state.paddle1, state.paddle2];
    for pad in paddles.iter() {
        render_paddle(pad.clone());
    }
});

#[export_name = "turbo/hello"]
unsafe extern "C" fn on_hello() -> usize {
    os::server::log!("Hello, world!");
    return os::server::COMMIT;
}
