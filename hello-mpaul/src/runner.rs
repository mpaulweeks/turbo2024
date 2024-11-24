use crate::server_comm::*;
use crate::*;

pub fn update(state: &mut GameState) {
    if state.history.actions.len() as i32 > state.history.action_index
        && state.history.action_ticks >= MAX_ACTION_TICKS
    {
        state.history.action_index += 1;
        state.history.action_ticks = MIN_ACTION_TICKS;
    }
    state.history.action_ticks = (state.history.action_ticks + 1.0).clamp(-9999.0, 9999.0);

    let logic_snapshot = simulate_game(state.history.clone()).current;
    let local = state.history.local.clone();

    // todo prevent actions while waiting for animation?
    if gamepad(0).a.just_pressed() {
        state.history.actions.pop();
        state.history.action_index -= 1;
    } else if mouse(0).left.just_pressed() {
        let clicker = match local {
            None => PlayerId::P1,
            Some(PlayerId::P1) => PlayerId::P1,
            Some(PlayerId::P2) => PlayerId::P2,
        };
        if let Some(action) = logic_snapshot.check_click(clicker) {
            server_play_move(state, action);
        }
    } else if mouse(0).right.just_pressed() && local.is_none() {
        if let Some(action) = logic_snapshot.check_click(PlayerId::P2) {
            server_play_move(state, action);
        }
    }

    server_refresh_action_history(state);

    // log!("DEBUG: {:?}", state.history.actions);
    // os::server::log!()
}

pub fn render(state: &GameState) {
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
        color = 0x2f5753ff,
    );

    render_impulse(delta.current.clone().impulse);
    let pos1 =
        delta
            .current
            .p1
            .render_player(delta.previous.p1, action_progress, delta.current.clone());
    let pos2 =
        delta
            .current
            .p2
            .render_player(delta.previous.p2, action_progress, delta.current.clone());
    if delta.current.p1.visible {
        delta.current.p1.render_attacks(pos1.clone(), pos2.clone());
        delta.current.p1.render_target(pos1.clone());
    }
    if delta.current.p2.visible {
        delta.current.p2.render_attacks(pos2.clone(), pos1.clone());
        delta.current.p2.render_target(pos2.clone());
    }
    render_round(delta.current.clone());
}
