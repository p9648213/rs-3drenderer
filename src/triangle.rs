use crate::vector::Vec2;

pub struct Face {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

#[derive(Clone)]
pub struct Triangle {
    pub point: [Vec2; 3],
}
