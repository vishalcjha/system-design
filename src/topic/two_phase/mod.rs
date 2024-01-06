use crate::model::{
    compute::{Compute, Pos},
    dom_position::DomPosition,
};

use super::{scenario::Scenario, util::position_holder::PositionHolder};

mod all_good_scenario;
mod client_deny_scenario;
pub mod one_client_down;
pub mod server_down;
pub mod two_phase_commit;

struct TwoPhasePositions {
    position_holder: PositionHolder,
}

impl TwoPhasePositions {
    const SERVER: &'static str = "Orchestrator";
    const CLIENT_ONE: &'static str = "Service1";
    const CLIENT_TWO: &'static str = "Service2";
}

impl Default for TwoPhasePositions {
    fn default() -> Self {
        Self {
            position_holder: PositionHolder::new(vec![
                String::from(TwoPhasePositions::SERVER),
                String::from(TwoPhasePositions::CLIENT_ONE),
                String::from(TwoPhasePositions::CLIENT_TWO),
            ]),
        }
    }
}

impl TwoPhasePositions {
    pub(super) fn server_pos(&self) -> &DomPosition {
        self.position_holder
            .positions
            .get(TwoPhasePositions::SERVER)
            .unwrap()
    }

    pub(super) fn client_one_pos(&self) -> &DomPosition {
        self.position_holder
            .positions
            .get(TwoPhasePositions::CLIENT_ONE)
            .unwrap()
    }

    pub(super) fn client_two_pos(&self) -> &DomPosition {
        self.position_holder
            .positions
            .get(TwoPhasePositions::CLIENT_TWO)
            .unwrap()
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
            String::from(TwoPhasePositions::CLIENT_ONE),
            Compute::new_client(Pos(4, 1), 1, Some(String::from("Service"))),
        ),
        (
            String::from(TwoPhasePositions::SERVER),
            Compute::new_server(Pos(4, 4), Some(String::from("Orchestrator"))),
        ),
        (
            String::from(TwoPhasePositions::CLIENT_TWO),
            Compute::new_client(Pos(4, 8), 2, Some(String::from("Service"))),
        ),
    ];
    stable_computes
}
