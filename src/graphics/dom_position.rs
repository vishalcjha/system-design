#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DomPosition {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

impl DomPosition {
    pub fn new(top: f64, left: f64, bottom: f64, right: f64) -> DomPosition {
        DomPosition {
            top,
            left,
            bottom,
            right,
        }
    }
}
