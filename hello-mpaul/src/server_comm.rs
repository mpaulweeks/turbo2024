use turbo::borsh::{BorshDeserialize, BorshSerialize};
use turbo::os;
use crate::action::Action;
use crate::{GameState, PROGRAM_ID};
use crate::util::Rands;

const ACTIONS_FILE: &str = "actions";

type ActionList = Vec<Action>;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct ActionRequest {
    match_id: u32,
    action: Action,
}

impl ActionRequest {
    pub fn new(match_id: u32, action: Action) -> ActionRequest {
        ActionRequest { match_id, action }
    }
}

pub fn server_play_move(state: &mut GameState, action: Action)
{
    if state.testing {
        state.history.actions.push(action.clone());
        return;
    }

    let request = ActionRequest::new(state.match_info.match_id, action.clone());
    let bytes =borsh::to_vec(&request).unwrap();
    os::client::exec(PROGRAM_ID, "server_play_move", &bytes);
}

pub fn server_refresh_action_history(state: &mut GameState)
{
    if state.testing {
        return;
    }

    let updated_actions = os::client::watch_file(PROGRAM_ID, &actions_file_name(state.match_info.match_id))
    .data
    .and_then(|file| ActionList::try_from_slice(&file.contents).ok());

    if let Some(a) = updated_actions {
        state.history.actions = a;
    }
}

fn actions_file_name(match_id: u32) -> String {
    format!("{}/actions", match_id)
}
#[export_name = "turbo/server_play_move"]
unsafe extern "C" fn on_play_move() -> usize {

    let req=os::server::command!(ActionRequest);
    let file_name = actions_file_name(req.match_id);

    let mut actions = os::server::read_or!(ActionList, &file_name, Vec::new());

    actions.push(req.action);

    let Ok(_) = os::server::write!(&file_name, actions) else {
        return os::server::CANCEL;
    };

    os::server::COMMIT
}