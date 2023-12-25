use std::cell::Cell;

use super::{get_computes, ComputeStatusChanger, PositionHolder};
use crate::{
    graphics::draw_lines_concurrently,
    model::arrow::{Arrow, Directional, Edge, Offset},
    topic::scenario::Scenario,
};

#[derive(Debug, Clone)]
pub(super) struct OneClientDown {
    steps: Vec<Vec<Arrow>>,
    current_step: Cell<usize>,
}

impl OneClientDown {
    pub(super) fn new() -> OneClientDown {
        let positions = PositionHolder::default();

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
        steps.push(vec![server_to_client_two_arrow.with_offset(Offset(6))]);
        let mut server_to_client_one_bidirectional =
            server_to_client_one_arrow.with_offset(Offset(8));
        server_to_client_one_bidirectional.directional = Directional::BiDirectional;
        steps.push(vec![server_to_client_one_bidirectional.reverse()]);

        OneClientDown {
            steps,
            current_step: Cell::new(0),
        }
    }
}

impl Scenario for OneClientDown {
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
}

impl ComputeStatusChanger for OneClientDown {
    fn status_updater(
        &self,
        computes: &Vec<(String, crate::model::compute::Compute)>,
    ) -> Option<Vec<(String, crate::model::compute::Compute)>> {
        if self.current_step.get() == 2 {
            Some(
                computes
                    .iter()
                    .map(|compute| match compute.1.compute_type {
                        crate::model::compute::ComputeType::Client(which) if which == 2 => {
                            let mut down_client = compute.1.clone();
                            down_client.to_down_compute();
                            (compute.0.clone(), down_client)
                        }
                        _ => compute.clone(),
                    })
                    .collect(),
            )
        } else if self.current_step.get() == 3 {
            Some(get_computes())
        } else {
            None
        }
    }
}
