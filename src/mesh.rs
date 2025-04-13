use crate::{triangle::Face, vector::Vec3};

pub const N_MESH_VERTICES: usize = 8;
pub const N_MESH_FACES: usize = 12;

pub const MESH_VERTICES: [Vec3; N_MESH_VERTICES] = [
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    }, //1
    Vec3 {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    }, //2
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    }, //3,
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    }, //4
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }, //5
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    }, //6
    Vec3 {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    }, //7
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    }, //8
];

pub const MESH_FACES: [Face; N_MESH_FACES] = [
    // front
    Face { a: 1, b: 2, c: 3 },
    Face { a: 1, b: 3, c: 4 },
    // right
    Face { a: 4, b: 3, c: 5 },
    Face { a: 4, b: 5, c: 6 },
    // back
    Face { a: 6, b: 5, c: 7 },
    Face { a: 6, b: 7, c: 8 },
    // left
    Face { a: 8, b: 7, c: 2 },
    Face { a: 8, b: 2, c: 1 },
    // top
    Face { a: 2, b: 7, c: 5 },
    Face { a: 2, b: 5, c: 3 },
    // bottom
    Face { a: 6, b: 8, c: 1 },
    Face { a: 6, b: 1, c: 4 },
];
