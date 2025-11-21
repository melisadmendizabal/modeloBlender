// sistema_solar.rs 

use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::fragment::{Fragment, FragmentOutput};
use crate::Uniforms;
use std::f32::consts::PI;

pub struct Planet {
    pub name: String,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub orbit_angle: f32,
    pub rotation_speed: f32,
    pub rotation_angle: f32,
    pub axis_tilt: f32,
    pub scale: f32,
    pub color: Vector3,
    pub vertex_shader: fn(&Vertex, &Uniforms) -> Vertex,
    pub fragment_shader: fn(&Fragment, &Uniforms) -> FragmentOutput,
}

impl Planet {
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
    
    pub fn update(&mut self, delta_time: f32) {
        // Solo actualizar órbita si tiene radio > 0 (no es el Sol)
        if self.orbit_radius > 0.0 {
            self.orbit_angle += self.orbit_speed * delta_time;
            
            if self.orbit_angle > PI * 2.0 {
                self.orbit_angle -= PI * 2.0;
            }
        }
        
        self.rotation_angle += self.rotation_speed * delta_time;
        
        if self.rotation_angle > PI * 2.0 {
            self.rotation_angle -= PI * 2.0;
        }
    }
    
    pub fn get_position(&self) -> Vector3 {
        // Si orbit_radius es 0 → Sol en el centro
        if self.orbit_radius == 0.0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        
        Vector3::new(
            self.orbit_radius * self.orbit_angle.cos(),
            0.0,
            self.orbit_radius * self.orbit_angle.sin(),
        )
    }
    
    pub fn get_rotation(&self) -> Vector3 {
        Vector3::new(
            self.axis_tilt,
            self.rotation_angle,
            0.0,
        )
    }
}

pub struct Spaceship {
    pub position: Vector3,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub velocity: Vector3,
    pub speed: f32,
    pub rotation_speed: f32,
    pub scale: f32,
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
        if window.is_key_down(KeyboardKey::KEY_Q) {
            self.pitch += self.rotation_speed;
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            self.pitch -= self.rotation_speed;
        }
        
        self.pitch = self.pitch.clamp(-PI / 2.5, PI / 2.5);
        
        while self.yaw > PI * 2.0 { self.yaw -= PI * 2.0; }
        while self.yaw < 0.0 { self.yaw += PI * 2.0; }
        while self.roll > PI * 2.0 { self.roll -= PI * 2.0; }
        while self.roll < -PI * 2.0 { self.roll += PI * 2.0; }
        
        let forward = self.get_forward_safe();
        let right = self.get_right_safe();
        
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.position.x += forward.x * self.speed;
            self.position.y += forward.y * self.speed;
            self.position.z += forward.z * self.speed;
        }
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.position.x -= forward.x * self.speed;
            self.position.y -= forward.y * self.speed;
            self.position.z -= forward.z * self.speed;
        }
        
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            self.position.y += self.speed;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            self.position.y -= self.speed;
        }
        
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.position.x -= right.x * self.speed;
            self.position.z -= right.z * self.speed;
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.position.x += right.x * self.speed;
            self.position.z += right.z * self.speed;
        }
    }
    
    fn get_forward_safe(&self) -> Vector3 {
        let forward = Vector3::new(
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        );
        
        if forward.x.is_nan() || forward.y.is_nan() || forward.z.is_nan() {
            eprintln!("⚠️ WARNING: forward es NaN!");
            return Vector3::new(0.0, 0.0, -1.0);
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
    
    fn safe_normalize(&self, v: Vector3) -> Vector3 {
        let length_sq = v.x * v.x + v.y * v.y + v.z * v.z;
        
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
        
        Vector3::new(
            self.position.x - forward.x * self.camera_offset.z + up.x * self.camera_offset.y,
            self.position.y - forward.y * self.camera_offset.z + up.y * self.camera_offset.y,
            self.position.z - forward.z * self.camera_offset.z + up.z * self.camera_offset.y,
        )
    }
    
    pub fn get_camera_target(&self) -> Vector3 {
        let forward = self.get_forward_safe();
        
        Vector3::new(
            self.position.x + forward.x * 2.0,
            self.position.y + forward.y * 2.0,
            self.position.z + forward.z * 2.0,
        )
    }
}

pub struct SolarSystem {
    pub planets: Vec<Planet>,
    pub spaceship: Spaceship,
    pub time_scale: f32,
}

impl SolarSystem {
    pub fn new() -> Self {
        SolarSystem {
            planets: Vec::new(),
            spaceship: Spaceship::new(),
            time_scale: 1.0,
        }
    }
    
    pub fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }
    
    pub fn update(&mut self, delta_time: f32) {
        let scaled_time = delta_time * self.time_scale;
        for planet in &mut self.planets {
            planet.update(scaled_time);
        }
    }
    

    // SISTEMA DE COLISIONES
  
    
    /// Verifica y resuelve colisiones entre la nave y todos los planetas
    /// Retorna true si hubo colisión y la resolvió
    pub fn resolve_collision(&mut self) -> bool {
        let ship_radius = self.spaceship.scale * 1.5; // Radio de colisión de la nave (un poco más grande)
        let mut had_collision = false;
        
        // Verificar colisión con cada planeta
        for planet in &self.planets {
            let planet_pos = planet.get_position();
            let ship_pos = self.spaceship.position;
            
            // Calcular distancia entre centros
            let dx = ship_pos.x - planet_pos.x;
            let dy = ship_pos.y - planet_pos.y;
            let dz = ship_pos.z - planet_pos.z;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            
            // Radio de colisión = radio del planeta + radio de la nave + margen de seguridad
            let planet_radius = planet.scale * 1.2; // Usar escala del planeta como radio
            let collision_distance = planet_radius + ship_radius + 0.5; // +0.5 = margen de seguridad
            
            // ¿Hay colisión?
            if distance < collision_distance {
                had_collision = true;
                
                // 🔍 DEBUG
                println!("⚠️ COLISIÓN detectada con {}!", planet.name);
                println!("   Distancia: {:.2} | Min segura: {:.2}", distance, collision_distance);
                
                // Calcular vector de separación (desde planeta hacia nave)
                let separation = if distance > 0.001 {
                    Vector3::new(dx / distance, dy / distance, dz / distance)
                } else {
                    // Si están exactamente en el mismo punto, empujar en dirección aleatoria
                    Vector3::new(1.0, 0.0, 0.0)
                };
                
                // Calcular cuánto se está penetrando
                let penetration = collision_distance - distance;
                
                // 🛡️ RESOLVER COLISIÓN: Empujar la nave hacia afuera
                // Multiplicamos por 1.1 para asegurar que salga completamente
                let push_distance = penetration * 1.1;
                
                self.spaceship.position.x += separation.x * push_distance;
                self.spaceship.position.y += separation.y * push_distance;
                self.spaceship.position.z += separation.z * push_distance;
                
                println!("   Nave empujada {:.2} unidades hacia afuera", push_distance);
            }
        }
        
        had_collision
    }
    
    /// Verifica si la nave está cerca de algún planeta (sin resolver colisión)
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
    
    /// Verifica colisión simple (sin resolver)
    pub fn check_collision_with_planet(&self, position: Vector3, collision_radius: f32) -> Option<String> {
        for planet in &self.planets {
            let planet_pos = planet.get_position();
            let dx = planet_pos.x - position.x;
            let dy = planet_pos.y - position.y;
            let dz = planet_pos.z - position.z;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            
            let planet_collision_radius = planet.scale + collision_radius;
            
            if distance < planet_collision_radius {
                return Some(planet.name.clone());
            }
        }
        
        None
    }
}


// Factory
use crate::shaders::vertex_shader;
use crate::shader_rocoso::fragment_shader_crater_hybrid;
use crate::shader_gaseoso::fragment_shader_gaseoso;
use crate::shader_strawberry::fragment_shader_strawberry;
use crate::shader_anillo::fragment_shader_personalizado;
use crate::shader_rojo::fragment_shader_red_planet;
use crate::shader_sol::{vertex_shader_star, fragment_shader_star_flares};

pub fn create_default_solar_system() -> SolarSystem {
    let mut system = SolarSystem::new();
    
    // SOL (centro fijo)
    system.add_planet(Planet::new(
        "Sol",
        0.0,
        0.0,
        0.1,
        2.0,
        vertex_shader,
        fragment_shader_star_flares,
    ));
    
    // Planetas
    system.add_planet(Planet::new(
        "Mercurio",
        3.0,
        0.6,
        0.3,
        0.4,
        vertex_shader,
        fragment_shader_crater_hybrid,
    ));
    
    system.add_planet(Planet::new(
        "Venus",
        5.0,
        0.5,
        0.3,
        0.8,
        vertex_shader,
        fragment_shader_gaseoso,
    ));
    
    system.add_planet(Planet::new(
        "Tierra",
        7.5,
        0.4,
        0.3,
        1.0,
        vertex_shader,
        fragment_shader_strawberry,
    ));
    
    system.add_planet(Planet::new(
        "Marte",
        10.0,
        0.6,
        0.3,
        0.6,
        vertex_shader,
        fragment_shader_red_planet,
    ));
    
    system.add_planet(Planet::new(
        "Júpiter",
        15.0,
        0.2,
        0.3,
        1.8,
        vertex_shader,
        fragment_shader_personalizado,
    ));
    
    system
}

pub fn get_orbit_points(radius: f32, segments: i32) -> Vec<Vector3> {
    let mut points = Vec::new();
    let angle_step = (PI * 2.0) / segments as f32;
    
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