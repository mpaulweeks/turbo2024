use crate::*;

const MIN_ACTION_TICKS: f32 = 0.0;
const MAX_ACTION_TICKS: f32 = 60.0;

pub fn update() {
    let mut state = GameState::load();
    let logic_snapshot = simulate_game(state.history.clone()).current;

    state.history.action_ticks =
        (state.history.action_ticks + 1.0).clamp(MIN_ACTION_TICKS, MAX_ACTION_TICKS);

    // todo prevent actions while waiting for animation?
    if gamepad(0).a.just_pressed() {
        state.history.actions.pop();
    } else if mouse(0).left.just_pressed() {
        if let Some(action) = click_action(logic_snapshot.p1.clone(), logic_snapshot.clone()) {
            state.history.action_ticks = MIN_ACTION_TICKS;
            state.history.actions.push(action);
        }
    }

    state.save();
}

pub fn render() {
    let state = GameState::load();
    let action_progress = (state.history.action_ticks - MIN_ACTION_TICKS) / MAX_ACTION_TICKS;
    let delta = simulate_game(state.history.clone());

    // render board background
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    let grid_width = screen_width * 0.8;
    let panel_width = screen_width - grid_width;
    rect!(
        x = 0,
        y = 0,
        w = panel_width,
        h = screen_height,
        color = 0x404040ff,
    );
    rect!(
        x = panel_width,
        y = 0,
        w = grid_width,
        h = screen_height,
        color = 0x004000ff,
    );

    render_player(
        delta.current.clone().p1,
        delta.previous.p1,
        action_progress,
        delta.current.clone(),
    );
    render_player(
        delta.current.clone().p2,
        delta.previous.p2,
        action_progress,
        delta.current.clone(),
    );
    render_impulse(delta.current.clone().impulse);
    render_round(delta.current.clone());
}
