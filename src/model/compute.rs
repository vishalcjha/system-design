#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos(pub usize, pub usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Compute {
    pub pos: Pos,
    pub status: Status,
    pub compute_type: ComputeType,
}

impl Compute {
    pub fn new_server(pos: Pos, name: Option<String>) -> Compute {
        Compute {
            pos,
            status: Status::Up,
            compute_type: ComputeType::Server(name),
        }
    }

    pub fn new_client(pos: Pos, num: u32, name: Option<String>) -> Compute {
        Compute {
            pos,
            status: Status::Up,
            compute_type: ComputeType::Client(num, name),
        }
    }

    pub fn to_down_compute(&mut self) {
        self.status = Status::Down;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComputeType {
    Server(Option<String>),
    Client(u32, Option<String>),
}
