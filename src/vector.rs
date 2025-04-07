#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn rotate_x(vec3: Vec3, angle: f64) -> Vec3 {
        Vec3 {
            x: vec3.x,
            y: vec3.y * angle.cos() - vec3.z * angle.sin(),
            z: vec3.y * angle.sin() + vec3.z * angle.cos(),
        }
    }

    pub fn rotate_y(vec3: Vec3, angle: f64) -> Vec3 {
        Vec3 {
            x: vec3.x * angle.cos() - vec3.z * angle.sin(),
            y: vec3.y,
            z: vec3.x * angle.sin() + vec3.z * angle.cos(),
        }
    }

    pub fn rotate_z(vec3: Vec3, angle: f64) -> Vec3 {
        Vec3 {
            x: vec3.x * angle.cos() - vec3.y * angle.sin(),
            y: vec3.x * angle.sin() + vec3.y * angle.cos(),
            z: vec3.z,
        }
    }
}
