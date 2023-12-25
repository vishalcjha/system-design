use std::collections::HashMap;

use crate::{
    graphics::find_element_pos,
    model::{
        compute::{Compute, Pos},
        dom_position::DomPosition,
    },
};

use super::scenario::Scenario;

mod all_good_scenario;
mod client_deny_scenario;
pub mod one_client_down;
pub mod server_down;
pub mod two_phase_commit;

#[derive(Debug, Clone)]
pub(crate) struct PositionHolder {
    pub positions: HashMap<String, DomPosition>,
}

impl PositionHolder {
    const SERVER: &'static str = "Server";
    const CLIENT_ONE: &'static str = "Client1";
    const CLIENT_TWO: &'static str = "Client2";
}

impl Default for PositionHolder {
    fn default() -> Self {
        let server_pos = find_element_pos(PositionHolder::SERVER);
        let client_one_pos = find_element_pos(PositionHolder::CLIENT_ONE);
        let client_two_pos = find_element_pos(PositionHolder::CLIENT_TWO);

        let positions = HashMap::from([
            (String::from(PositionHolder::SERVER), server_pos.unwrap()),
            (
                String::from(PositionHolder::CLIENT_ONE),
                client_one_pos.unwrap(),
            ),
            (
                String::from(PositionHolder::CLIENT_TWO),
                client_two_pos.unwrap(),
            ),
        ]);

        PositionHolder { positions }
    }
}
impl PositionHolder {
    pub(super) fn server_pos(&self) -> &DomPosition {
        self.positions.get(PositionHolder::SERVER).unwrap()
    }

    pub(super) fn client_one_pos(&self) -> &DomPosition {
        self.positions.get(PositionHolder::CLIENT_ONE).unwrap()
    }

    pub(super) fn client_two_pos(&self) -> &DomPosition {
        self.positions.get(PositionHolder::CLIENT_TWO).unwrap()
    }
}

trait ComputeStatusChanger: Scenario {
    fn status_updater(&self, _computes: &Vec<(String, Compute)>) -> Option<Vec<(String, Compute)>> {
        None
    }
}

fn get_computes() -> Vec<(String, Compute)> {
    let stable_computes = vec![
        (
            String::from(PositionHolder::CLIENT_ONE),
            Compute::new_client(Pos(4, 1), 1),
        ),
        (
            String::from(PositionHolder::SERVER),
            Compute::new_server(Pos(4, 4)),
        ),
        (
            String::from(PositionHolder::CLIENT_TWO),
            Compute::new_client(Pos(4, 8), 2),
        ),
    ];
    stable_computes
}
