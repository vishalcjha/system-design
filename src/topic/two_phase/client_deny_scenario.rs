use std::cell::Cell;

use super::{ComputeStatusChanger, TwoPhasePositions};
use crate::{
    graphics::draw_lines_concurrently,
    model::arrow::{Arrow, Directional, Edge, Offset},
    topic::scenario::Scenario,
};

#[derive(Debug, Clone)]
pub(super) struct ClientDenyScenario {
    steps: Vec<Vec<Arrow>>,
    current_step: Cell<usize>,
}

impl ClientDenyScenario {
    pub(super) const NAME: &'static str = "ClientDeny";
}

impl ClientDenyScenario {
    pub(super) fn new() -> ClientDenyScenario {
        let positions = TwoPhasePositions::default();

        let server_pos = positions.server_pos();
        let client_one_pos = positions.client_one_pos();
        let client_two_pos = positions.client_two_pos();

        let server_to_client_one_arrow = Arrow::new(
            (Edge::Right, server_pos.clone(), None),
            (Edge::Left, client_two_pos.clone(), None),
            Directional::UniDirectional,
        );

        let server_to_client_two_arrow = Arrow::new(
            (Edge::Left, server_pos.clone(), None),
            (Edge::Right, client_one_pos.clone(), None),
            Directional::UniDirectional,
        );

        let client_one_to_server = server_to_client_one_arrow.reverse();
        let client_two_to_server = server_to_client_two_arrow.reverse();

        let mut steps = Vec::new();
        steps.push(vec![
            server_to_client_one_arrow.with_offset(Offset(2)),
            server_to_client_two_arrow.with_offset(Offset(2)),
        ]);
        steps.push(vec![
            client_one_to_server.with_offset(Offset(4)),
            client_two_to_server.with_offset(Offset(4)),
        ]);

        steps.push(vec![
            server_to_client_one_arrow.with_offset(Offset(6)),
            server_to_client_two_arrow.with_offset(Offset(6)),
        ]);

        ClientDenyScenario {
            steps,
            current_step: Cell::new(0),
        }
    }
}

impl Scenario for ClientDenyScenario {
    fn execute(&self) -> Option<()> {
        let Some(steps) = self.steps.get(self.current_step.get()) else {
            return None;
        };
        self.current_step.set(self.current_step.get() + 1);

        let to_be_drawn: Vec<&Arrow> = steps.iter().collect();
        draw_lines_concurrently(to_be_drawn, self.current_step.get() as u32);
        if self.current_step.get() >= self.steps.len() {
            None
        } else {
            Some(())
        }
    }

    fn is_playing(&self) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str {
        ClientDenyScenario::NAME
    }
}

impl ComputeStatusChanger for ClientDenyScenario {}
