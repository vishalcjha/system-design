#![allow(dead_code)]

use super::compute::Pos;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Directional {
    UnDirectional,
    UniDirectional,
    BiDirectional,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Arrow {
    from: Pos,
    to: Pos,
    directional: Directional,
}

impl Arrow {
    pub fn new(from: Pos, to: Pos, directional: Directional) -> Self {
        Arrow {
            from,
            to,
            directional,
        }
    }
}
