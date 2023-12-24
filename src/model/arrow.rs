#![allow(dead_code)]

use super::dom_position::DomPosition;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Directional {
    UnDirectional,
    UniDirectional,
    BiDirectional,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Offset(pub i32);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Arrow {
    from: (Edge, DomPosition, Option<Offset>),
    to: (Edge, DomPosition, Option<Offset>),
    pub directional: Directional,
}

impl Arrow {
    pub fn new(
        from: (Edge, DomPosition, Option<Offset>),
        to: (Edge, DomPosition, Option<Offset>),
        directional: Directional,
    ) -> Self {
        Arrow {
            from,
            to,
            directional,
        }
    }

    pub fn reverse(&self) -> Arrow {
        Arrow {
            from: self.to.clone(),
            to: self.from.clone(),
            directional: self.directional.clone(),
        }
    }

    pub fn with_offset(&self, offset: Offset) -> Arrow {
        let mut from = self.from.clone();
        from.2 = Some(offset.clone());
        let mut to = self.to.clone();
        to.2 = Some(offset);
        Arrow {
            from,
            to,
            directional: self.directional.clone(),
        }
    }

    pub fn to_different_end(&self, to: (Edge, DomPosition, Option<Offset>)) -> Arrow {
        Arrow {
            from: self.from.clone(),
            to,
            directional: self.directional.clone(),
        }
    }
}

impl Arrow {
    fn get_pos(dom_position: &DomPosition, edge: &Edge, offset: &Offset) -> (f64, f64) {
        match edge {
            Edge::Top => (dom_position.left + (offset.0 * 10) as f64, dom_position.top),
            Edge::Bottom => (
                dom_position.left + (offset.0 * 10) as f64,
                dom_position.bottom,
            ),
            Edge::Left => (dom_position.left, dom_position.top + (offset.0 * 10) as f64),
            Edge::Right => (
                dom_position.right,
                dom_position.top + (offset.0 * 10) as f64,
            ),
        }
    }

    pub fn get_from_to(&self) -> ((f64, f64), (f64, f64)) {
        let (from, to) = (&self.from, &self.to);
        (
            (Arrow::get_pos(&from.1, &from.0, &from.2.clone().unwrap_or(Offset(0)))),
            (Arrow::get_pos(&to.1, &to.0, &to.2.clone().unwrap_or(Offset(0)))),
        )
    }
}
