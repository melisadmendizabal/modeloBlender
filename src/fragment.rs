//fragments
use raylib::math::{Vector2, Vector3};

//todos los valores son interpolados
//valores de cada pixel
pub struct Fragment {
    pub position: Vector2,
    pub color: Vector3,
    pub depth: f32,
    pub normal: Vector3,
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Vector3, depth: f32, normal: Vector3) -> Self {
        Fragment {
            position: Vector2::new(x, y),
            color,
            depth,
            normal,
        }
    }
}