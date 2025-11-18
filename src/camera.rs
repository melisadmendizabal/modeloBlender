// ============================================
// REEMPLAZAR camera.rs COMPLETAMENTE
// ============================================

#![allow(dead_code)]
use raylib::prelude::*;
use crate::matrix::create_view_matrix;
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub yaw: f32,
    pub pitch: f32,
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

impl Camera {
    pub fn new(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        let direction = Vector3::new(
            target.x - eye.x,
            target.y - eye.y,
            target.z - eye.z,
        ).normalized();

        let pitch = direction.y.asin();
        let yaw = direction.z.atan2(direction.x);

        Camera {
            eye,
            target,
            up,
            yaw,
            pitch,
            movement_speed: 0.3,
            rotation_speed: 0.03,
        }
    }

    fn update_target(&mut self) {
        self.pitch = self.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let forward = Vector3::new(
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        );

        self.target = Vector3::new(
            self.eye.x + forward.x,
            self.eye.y + forward.y,
            self.eye.z + forward.z,
        );
    }

    /// 🛡️ VERSIÓN SEGURA: Get the view matrix with safety checks
    pub fn get_view_matrix(&self) -> Matrix {
        // Verificar si eye y target son válidos
        if self.eye.x.is_nan() || self.eye.y.is_nan() || self.eye.z.is_nan() {
            eprintln!("❌ ERROR: camera.eye tiene NaN");
            return Matrix::identity();
        }
        
        if self.target.x.is_nan() || self.target.y.is_nan() || self.target.z.is_nan() {
            eprintln!("❌ ERROR: camera.target tiene NaN");
            return Matrix::identity();
        }
        
        // Calcular la distancia entre eye y target
        let dx = self.target.x - self.eye.x;
        let dy = self.target.y - self.eye.y;
        let dz = self.target.z - self.eye.z;
        let distance_sq = dx * dx + dy * dy + dz * dz;
        
        // 🛡️ PROTECCIÓN: Si eye y target están demasiado cerca, usar un target por defecto
        if distance_sq < 0.01 {
            eprintln!("⚠️ WARNING: eye y target muy cercanos (dist²={:.6}), usando target por defecto", distance_sq);
            
            // Crear un target válido adelante de eye
            let default_target = Vector3::new(
                self.eye.x + 1.0,
                self.eye.y,
                self.eye.z
            );
            
            return create_view_matrix(self.eye, default_target, self.up);
        }
        
        create_view_matrix(self.eye, self.target, self.up)
    }

    pub fn get_forward(&self) -> Vector3 {
        let mut forward = Vector3::new(
            self.target.x - self.eye.x,
            self.target.y - self.eye.y,
            self.target.z - self.eye.z,
        );
        
        // 🛡️ Normalización segura
        let length_sq = forward.x * forward.x + forward.y * forward.y + forward.z * forward.z;
        if length_sq < 0.000001 {
            eprintln!("⚠️ WARNING: forward vector casi cero");
            return Vector3::new(0.0, 0.0, -1.0); // Default forward
        }
        
        let length = length_sq.sqrt();
        forward.x /= length;
        forward.y /= length;
        forward.z /= length;
        
        forward
    }

    pub fn get_right(&self) -> Vector3 {
        let forward = self.get_forward();
        let mut right = Vector3::new(
            forward.y * self.up.z - forward.z * self.up.y,
            forward.z * self.up.x - forward.x * self.up.z,
            forward.x * self.up.y - forward.y * self.up.x,
        );
        
        // 🛡️ Normalización segura
        let length_sq = right.x * right.x + right.y * right.y + right.z * right.z;
        if length_sq < 0.000001 {
            eprintln!("⚠️ WARNING: right vector casi cero");
            return Vector3::new(1.0, 0.0, 0.0); // Default right
        }
        
        let length = length_sq.sqrt();
        right.x /= length;
        right.y /= length;
        right.z /= length;
        
        right
    }

    pub fn get_up(&self) -> Vector3 {
        let forward = self.get_forward();
        let right = self.get_right();
        let mut up = Vector3::new(
            right.y * forward.z - right.z * forward.y,
            right.z * forward.x - right.x * forward.z,
            right.x * forward.y - right.y * forward.x,
        );
        
        // 🛡️ Normalización segura
        let length_sq = up.x * up.x + up.y * up.y + up.z * up.z;
        if length_sq < 0.000001 {
            eprintln!("⚠️ WARNING: up vector casi cero");
            return Vector3::new(0.0, 1.0, 0.0); // Default up
        }
        
        let length = length_sq.sqrt();
        up.x /= length;
        up.y /= length;
        up.z /= length;
        
        up
    }

    pub fn process_input(&mut self, window: &RaylibHandle) {
        // ROTACIÓN
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.yaw += self.rotation_speed;
            self.update_target();
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.yaw -= self.rotation_speed;
            self.update_target();
        }

        if window.is_key_down(KeyboardKey::KEY_W) {
            self.pitch += self.rotation_speed;
            self.update_target();
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.pitch -= self.rotation_speed;
            self.update_target();
        }

        // MOVIMIENTO
        let forward = self.get_forward();
        let right = self.get_right();

        if window.is_key_down(KeyboardKey::KEY_UP) {
            self.eye.x += forward.x * self.movement_speed;
            self.eye.y += forward.y * self.movement_speed;
            self.eye.z += forward.z * self.movement_speed;
            self.update_target();
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            self.eye.x -= forward.x * self.movement_speed;
            self.eye.y -= forward.y * self.movement_speed;
            self.eye.z -= forward.z * self.movement_speed;
            self.update_target();
        }
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            self.eye.x -= right.x * self.movement_speed;
            self.eye.y -= right.y * self.movement_speed;
            self.eye.z -= right.z * self.movement_speed;
            self.update_target();
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.eye.x += right.x * self.movement_speed;
            self.eye.y += right.y * self.movement_speed;
            self.eye.z += right.z * self.movement_speed;
            self.update_target();
        }

        if window.is_key_down(KeyboardKey::KEY_Q) {
            self.eye.y += self.movement_speed;
            self.update_target();
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            self.eye.y -= self.movement_speed;
            self.update_target();
        }

        if window.is_key_down(KeyboardKey::KEY_R) {
            self.eye.y += self.movement_speed;
            self.update_target();
        }
        if window.is_key_down(KeyboardKey::KEY_F) {
            self.eye.y -= self.movement_speed;
            self.update_target();
        }
    }
}