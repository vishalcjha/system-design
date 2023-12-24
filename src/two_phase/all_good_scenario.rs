use leptos::logging;

use super::PositionHolder;
use crate::{
    graphics::draw_line,
    model::arrow::{Arrow, Directional, Edge},
};

#[derive(Debug, Clone, Default)]
pub(super) struct AllGoodScenario {
    positions: PositionHolder,
}

impl AllGoodScenario {
    pub(super) fn execute(&self) {
        use Edge::*;
        let server_pos = self.positions.server_pos();
        let client_one_pos = self.positions.client_one_pos();
        let client_two_pos = self.positions.client_two_pos();

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

        draw_line(&server_to_client_one_arrow);
        draw_line(&server_to_client_two_arrow);

        logging::log!("{:?} {:?} {:?}", server_pos, client_one_pos, client_two_pos);
    }
}
