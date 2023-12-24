use std::collections::HashMap;

use crate::{graphics::find_element_pos, model::dom_position::DomPosition};

mod all_good_scenario;
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
