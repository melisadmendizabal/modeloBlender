// ============================================
// NUEVO ARCHIVO: solar_system.rs
// ============================================

use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::fragment::{Fragment, FragmentOutput};
use crate::Uniforms;
use std::f32::consts::PI;

// ============================================
// Estructura de un Planeta en el Sistema Solar
// ============================================
pub struct Planet {
    // Identificación
    pub name: String,
    // Propiedades orbitales
    pub orbit_radius: f32,        // Distancia al sol
    pub orbit_speed: f32,         // Velocidad de traslación (rad/s)
    pub orbit_angle: f32,         // Ángulo actual en la órbita
    
    // Propiedades de rotación
    pub rotation_speed: f32,      // Velocidad de rotación sobre su eje
    pub rotation_angle: f32,      // Ángulo actual de rotación
    pub axis_tilt: f32,           // Inclinación del eje (opcional)
    
    // Propiedades físicas
    pub scale: f32,               // Tamaño del planeta
    pub color: Vector3,           // Color base (para debug)
    
    // Shaders
    pub vertex_shader: fn(&Vertex, &Uniforms) -> Vertex,
    pub fragment_shader: fn(&Fragment, &Uniforms) -> FragmentOutput,
}

impl Planet {
    /// Crear un nuevo planeta con parámetros básicos
    pub fn new(
        name: &str,
        orbit_radius: f32,
        orbit_speed: f32,
        rotation_speed: f32,
        scale: f32,
        vertex_shader: fn(&Vertex, &Uniforms) -> Vertex,
        fragment_shader: fn(&Fragment, &Uniforms) -> FragmentOutput,
    ) -> Self {
        Planet {
            name: name.to_string(),
            orbit_radius,
            orbit_speed,
            orbit_angle: 0.0,
            rotation_speed,
            rotation_angle: 0.0,
            axis_tilt: 0.0,
            scale,
            color: Vector3::new(1.0, 1.0, 1.0),
            vertex_shader,
            fragment_shader,
        }
    }
    
    /// Actualizar posición orbital y rotación del planeta
    pub fn update(&mut self, delta_time: f32) {
        // Actualizar órbita (traslación)
        self.orbit_angle += self.orbit_speed * delta_time;
        
        // Mantener el ángulo en [0, 2π]
        if self.orbit_angle > std::f32::consts::PI * 2.0 {
            self.orbit_angle -= std::f32::consts::PI * 2.0;
        }
        
        // Actualizar rotación sobre su eje
        self.rotation_angle += self.rotation_speed * delta_time;
        
        if self.rotation_angle > std::f32::consts::PI * 2.0 {
            self.rotation_angle -= std::f32::consts::PI * 2.0;
        }
    }
    
    /// Obtener posición actual del planeta en el espacio
    pub fn get_position(&self) -> Vector3 {
        Vector3::new(
            self.orbit_radius * self.orbit_angle.cos(),
            0.0, // Plano eclíptico (Y = 0)
            self.orbit_radius * self.orbit_angle.sin(),
        )
    }
    
    /// Obtener vector de rotación actual
    pub fn get_rotation(&self) -> Vector3 {
        Vector3::new(
            self.axis_tilt,        // Inclinación del eje
            self.rotation_angle,   // Rotación principal
            0.0,
        )
    }
}


// ============================================
// NUEVA ESTRUCTURA: Nave con física propia
// ============================================
pub struct Spaceship {
    // Posición absoluta en el mundo
    pub position: Vector3,
    
    // Orientación (yaw, pitch, roll)
    pub yaw: f32,      // Rotación izquierda/derecha
    pub pitch: f32,    // Rotación arriba/abajo
    pub roll: f32,     // Inclinación lateral
    
    // Velocidad
    pub velocity: Vector3,
    pub speed: f32,           // Velocidad de movimiento
    pub rotation_speed: f32,  // Velocidad de rotación
    
    // Visual
    pub scale: f32,
    
    // Offset de cámara (cuánto atrás y arriba está la cámara)
    pub camera_offset: Vector3,
}

impl Spaceship {
    pub fn new() -> Self {
        Spaceship {
            position: Vector3::new(30.0, -4.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            velocity: Vector3::zero(),
            speed: 0.3,
            rotation_speed: 0.02,
            scale: 0.5,
            camera_offset: Vector3::new(0.0, 2.0, 6.0),
        }
    }
    
    pub fn process_input(&mut self, window: &RaylibHandle) {
        // // ============================================
        // // ROTACIÓN CON A/D (YAW - izquierda/derecha)
        // // ============================================
        // if window.is_key_down(KeyboardKey::KEY_A) {
        //     self.yaw += self.rotation_speed; // Rotar izquierda
        // }
        // if window.is_key_down(KeyboardKey::KEY_D) {
        //     self.yaw -= self.rotation_speed; // Rotar derecha
        // }
        
        // ============================================
        // ROTACIÓN CON Q/E (PITCH - arriba/abajo)
        // ============================================
        if window.is_key_down(KeyboardKey::KEY_Q) {
            self.pitch += self.rotation_speed; // Mirar arriba
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            self.pitch -= self.rotation_speed; // Mirar abajo
        }
        
        // 🛡️ PROTECCIÓN: Limitar pitch
        self.pitch = self.pitch.clamp(-PI / 2.5, PI / 2.5);
        
        // 🛡️ PROTECCIÓN: Normalizar ángulos para evitar overflow
        while self.yaw > PI * 2.0 { self.yaw -= PI * 2.0; }
        while self.yaw < 0.0 { self.yaw += PI * 2.0; }
        while self.roll > PI * 2.0 { self.roll -= PI * 2.0; }
        while self.roll < -PI * 2.0 { self.roll += PI * 2.0; }
        
        // Calcular vectores de dirección
        let forward = self.get_forward_safe();
        let right = self.get_right_safe();
        
        // ============================================
        // MOVIMIENTO CON W/S (Adelante/Atrás en dirección de la nave)
        // ============================================
        if window.is_key_down(KeyboardKey::KEY_S) {
            // Adelante (en la dirección que mira)
            self.position.x += forward.x * self.speed;
            self.position.y += forward.y * self.speed;
            self.position.z += forward.z * self.speed;
        }
        if window.is_key_down(KeyboardKey::KEY_W) {
            // Atrás (opuesto a la dirección que mira)
            self.position.x -= forward.x * self.speed;
            self.position.y -= forward.y * self.speed;
            self.position.z -= forward.z * self.speed;
        }
        
        // ============================================
        // MOVIMIENTO CON FLECHAS
        // ============================================
        
        // ↑ : Subir (eje Y mundial)
        if window.is_key_down(KeyboardKey::KEY_UP) {
            self.position.y += self.speed;
        }
        
        // ↓ : Bajar (eje Y mundial)
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            self.position.y -= self.speed;
        }
        
        // ← : Moverse izquierda (strafe)
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.position.x -= right.x * self.speed;
            self.position.z -= right.z * self.speed;
        }
        
        // → : Moverse derecha (strafe)
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.position.x += right.x * self.speed;
            self.position.z += right.z * self.speed;
        }
    }
    
    // 🛡️ Funciones seguras con protección contra NaN/Inf
    fn get_forward_safe(&self) -> Vector3 {
        let forward = Vector3::new(
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        );
        
        // Verificar si es válido
        if forward.x.is_nan() || forward.y.is_nan() || forward.z.is_nan() {
            eprintln!("⚠️ WARNING: forward es NaN!");
            return Vector3::new(0.0, 0.0, -1.0); // Default forward
        }
        
        self.safe_normalize(forward)
    }
    
    fn get_right_safe(&self) -> Vector3 {
        let right = Vector3::new(
            -self.yaw.sin(),
            0.0,
            self.yaw.cos(),
        );
        
        if right.x.is_nan() || right.z.is_nan() {
            eprintln!("⚠️ WARNING: right es NaN!");
            return Vector3::new(1.0, 0.0, 0.0);
        }
        
        self.safe_normalize(right)
    }
    
    fn get_up_safe(&self) -> Vector3 {
        let forward = self.get_forward_safe();
        let right = self.get_right_safe();
        
        // Cross product: right × forward
        let up = Vector3::new(
            right.y * forward.z - right.z * forward.y,
            right.z * forward.x - right.x * forward.z,
            right.x * forward.y - right.y * forward.x,
        );
        
        if up.x.is_nan() || up.y.is_nan() || up.z.is_nan() {
            eprintln!("⚠️ WARNING: up es NaN!");
            return Vector3::new(0.0, 1.0, 0.0);
        }
        
        self.safe_normalize(up)
    }
    
    // 🛡️ Normalización segura
    fn safe_normalize(&self, v: Vector3) -> Vector3 {
        let length_sq = v.x * v.x + v.y * v.y + v.z * v.z;
        
        // Evitar división por cero
        if length_sq < 0.000001 {
            eprintln!("⚠️ WARNING: Vector casi cero, no se puede normalizar!");
            return Vector3::new(0.0, 0.0, 1.0);
        }
        
        let length = length_sq.sqrt();
        Vector3::new(v.x / length, v.y / length, v.z / length)
    }
    
    pub fn get_world_position(&self) -> Vector3 {
        self.position
    }
    
    pub fn get_world_rotation(&self) -> Vector3 {
        Vector3::new(self.pitch, self.yaw, self.roll)
    }
    
    pub fn get_camera_position(&self) -> Vector3 {
        let forward = self.get_forward_safe();
        let up = self.get_up_safe();
        
        // Cámara detrás y arriba de la nave
        Vector3::new(
            self.position.x - forward.x * self.camera_offset.z + up.x * self.camera_offset.y,
            self.position.y - forward.y * self.camera_offset.z + up.y * self.camera_offset.y,
            self.position.z - forward.z * self.camera_offset.z + up.z * self.camera_offset.y,
        )
    }
    
    pub fn get_camera_target(&self) -> Vector3 {
        let forward = self.get_forward_safe();
        
        // Target adelante de la nave
        Vector3::new(
            self.position.x + forward.x * 2.0,
            self.position.y + forward.y * 2.0,
            self.position.z + forward.z * 2.0,
        )
    }
}



// ============================================
// Sistema Solar Completo
// ============================================
pub struct SolarSystem {
    pub planets: Vec<Planet>,
    pub sun_scale: f32,
    pub time_scale: f32,  // Multiplicador de velocidad del tiempo+
    pub spaceship: Spaceship,
}

impl SolarSystem {
    pub fn new() -> Self {
        SolarSystem {
            planets: Vec::new(),
            sun_scale: 2.0,
            time_scale: 1.0,
            spaceship: Spaceship::new(),
        }
    }
    
    /// Agregar un planeta al sistema
    pub fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }
    
    /// Actualizar todos los planetas
    pub fn update(&mut self, delta_time: f32) {
        let scaled_time = delta_time * self.time_scale;
        for planet in &mut self.planets {
            planet.update(scaled_time);
        }
    }
    
    /// Obtener el planeta más cercano a una posición
    pub fn get_closest_planet(&self, position: Vector3) -> Option<usize> {
        if self.planets.is_empty() {
            return None;
        }
        
        let mut closest_idx = 0;
        let mut min_distance = f32::MAX;
        
        for (i, planet) in self.planets.iter().enumerate() {
            let planet_pos = planet.get_position();
            let dx = planet_pos.x - position.x;
            let dz = planet_pos.z - position.z;
            let distance = (dx * dx + dz * dz).sqrt();
            
            if distance < min_distance {
                min_distance = distance;
                closest_idx = i;
            }
        }
        
        Some(closest_idx)
    }



/// 🆕 Obtener distancia al planeta más cercano
    pub fn get_distance_to_closest_planet(&self, position: Vector3) -> Option<(usize, f32, String)> {
        if self.planets.is_empty() {
            return None;
        }
        
        let mut closest_idx = 0;
        let mut min_distance = f32::MAX;
        
        for (i, planet) in self.planets.iter().enumerate() {
            let planet_pos = planet.get_position();
            let dx = planet_pos.x - position.x;
            let dy = planet_pos.y - position.y;
            let dz = planet_pos.z - position.z;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            
            if distance < min_distance {
                min_distance = distance;
                closest_idx = i;
            }
        }
        
        let planet_name = self.planets[closest_idx].name.clone();
        Some((closest_idx, min_distance, planet_name))
    }
    
    /// 🆕 Verificar si hay colisión con algún planeta
    pub fn check_collision_with_planet(&self, position: Vector3, collision_radius: f32) -> Option<String> {
        for planet in &self.planets {
            let planet_pos = planet.get_position();
            let dx = planet_pos.x - position.x;
            let dy = planet_pos.y - position.y;
            let dz = planet_pos.z - position.z;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            
            // Radio de colisión = escala del planeta + radio de seguridad
            let planet_collision_radius = planet.scale + collision_radius;
            
            if distance < planet_collision_radius {
                return Some(planet.name.clone());
            }
        }
        
        None
    }
}

// ============================================
// Factory: Crear sistema solar predefinido
// ============================================

use crate::shaders::vertex_shader;
use crate::shader_rocoso::fragment_shader_crater_hybrid;
use crate::shader_gaseoso::fragment_shader_gaseoso;
use crate::shader_strawberry::fragment_shader_strawberry;
use crate::shader_anillo::{fragment_shader_personalizado, fragment_shader_torus, generate_torus_vertices};
use crate::shader_rojo::fragment_shader_red_planet;
use crate::shader_sol::{vertex_shader_star, fragment_shader_star_flares};


pub fn create_default_solar_system() -> SolarSystem {
    let mut system = SolarSystem::new();
    
    // ============================================
    // SOL (se renderiza aparte, en el centro)
    // ============================================
    // No se agrega como planeta, está fijo en (0, 0, 0)
    
    // ============================================
    // PLANETA 1: Mercurio (Rocoso pequeño)
    // ============================================
    system.add_planet(Planet::new(
        "Mercurio",
        3.0,          // Cerca del sol
        0.1,          // Rápido
        0.3,          // Rotación lenta
        0.4,          // Pequeño
        vertex_shader,
        fragment_shader_crater_hybrid,
    ));
    
    // ============================================
    // PLANETA 2: Venus (Gaseoso)
    // ============================================
    system.add_planet(Planet::new(
        "Venus",
        5.0,          // Más lejos
        0.1,          // Velocidad media
        0.3,          // Rotación media
        0.8,          // Mediano
        vertex_shader,
        fragment_shader_gaseoso,
    ));
    
    // ============================================
    // PLANETA 3: Tierra (Fresita - creativo!)
    // ============================================
    system.add_planet(Planet::new(
        "Tierra",
        7.5,          // Tercera órbita
        0.1,          // Velocidad moderada
        0.3,          // Rotación rápida
        1.0,          // Tamaño normal
        vertex_shader,
        fragment_shader_strawberry,
    ));
    
    // ============================================
    // PLANETA 4: Marte (Rojo)
    // ============================================
    system.add_planet(Planet::new(
        "Marte",
        10.0,         // Cuarta órbita
        0.1,          // Más lento
        0.3,          // Rotación similar a Tierra
        0.6,          // Pequeño
        vertex_shader,
        fragment_shader_red_planet,
    ));
    
    // ============================================
    // PLANETA 5: Júpiter (Grande con anillos)
    // ============================================
    system.add_planet(Planet::new(
        "Júpiter",
        15.0,         // Quinta órbita
        0.1,          // Lento
        0.3,          // Rotación muy rápida
        1.8,          // Grande
        vertex_shader,
        fragment_shader_personalizado, // Base para anillos
    ));
    
    
    system
}

// ============================================
// Sistema Solar Personalizable
// ============================================

pub fn create_custom_solar_system(
    planet_configs: Vec<(
        &str,           // Nombre
        f32,            // Radio orbital
        f32,            // Velocidad orbital
        f32,            // Velocidad rotación
        f32,            // Escala
        fn(&Vertex, &Uniforms) -> Vertex,
        fn(&Fragment, &Uniforms) -> FragmentOutput,
    )>
) -> SolarSystem {
    let mut system = SolarSystem::new();
    
    for config in planet_configs {
        system.add_planet(Planet::new(
            config.0, config.1, config.2, config.3, config.4, config.5, config.6
        ));
    }
    
    system
}

// ============================================
// Helpers para órbitas
// ============================================

/// Dibujar órbita de un planeta (para debug/visualización)
pub fn get_orbit_points(radius: f32, segments: i32) -> Vec<Vector3> {
    let mut points = Vec::new();
    let angle_step = (std::f32::consts::PI * 2.0) / segments as f32;
    
    for i in 0..=segments {
        let angle = i as f32 * angle_step;
        points.push(Vector3::new(
            radius * angle.cos(),
            0.0,
            radius * angle.sin(),
        ));
    }
    
    points
}

/// Calcular posición en órbita elíptica (opcional, para mayor realismo)
pub fn get_elliptical_position(
    semi_major: f32,
    eccentricity: f32,
    angle: f32,
) -> Vector3 {
    let r = semi_major * (1.0 - eccentricity * eccentricity) 
          / (1.0 + eccentricity * angle.cos());
    
    Vector3::new(
        r * angle.cos(),
        0.0,
        r * angle.sin(),
    )
}