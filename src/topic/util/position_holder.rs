use std::collections::HashMap;

use crate::{graphics::find_element_pos, model::dom_position::DomPosition};

#[derive(Debug, Clone)]
pub(crate) struct PositionHolder {
    pub positions: HashMap<String, DomPosition>,
}

impl PositionHolder {
    pub fn new(position_ids: Vec<String>) -> Self {
        let positions = position_ids
            .into_iter()
            .map(|id| {
                let pos = find_element_pos(&id).unwrap();
                (id, pos)
            })
            .collect();

        PositionHolder { positions }
    }
}
