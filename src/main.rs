// main.rs
mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod fragment;
mod shaders;
mod obj;
mod matrix;
mod camera;
mod light;
mod shader_anillo;
mod shader_strawberry;
mod shader_gaseoso;
mod shader_rocoso;
mod shader_rojo;
mod shader_sol;
mod sistema_solar;

use matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix};
use camera::Camera;
use light::Light;
use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader};
use obj::Obj;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;
use shader_anillo::generate_torus_vertices;
use shader_anillo::fragment_shader_personalizado;
use shader_anillo::fragment_shader_torus;
use shader_strawberry::fragment_shader_strawberry;
use shader_gaseoso::fragment_shader_gaseoso;
use shader_rocoso::fragment_shader_crater_hybrid;
use shader_rojo::fragment_shader_red_planet;
use sistema_solar::{SolarSystem, create_default_solar_system, get_orbit_points};
//use shader_sol::fragment_shader_star;
use shader_sol::fragment_shader_star_flares;
use shader_sol::vertex_shader_star;
//use crate::shaders::fragment_shader_rocoso;
use crate::fragment::Fragment;
use crate::fragment::FragmentOutput;
use std::time::Instant;



pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32,
    pub shader_mode: i32,
}

fn render(
    framebuffer: &mut Framebuffer, 
    uniforms: &Uniforms, 
    vertex_array: &[Vertex],
    light: &Light,
    vertex_shader: fn(&Vertex, &Uniforms) -> Vertex,  // ✨ NUEVO!
    fragment_shader: fn(&Fragment, &Uniforms) -> FragmentOutput,
) {
    // Vertex Shader Stage - Ahora usa el shader correcto
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms); // ✅ Shader personalizado
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let out = fragment_shader(&fragment, uniforms);
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            out.color, 
            fragment.depth,
            out.alpha,
        );
    }
}

// fn get_current_time_seconds() -> f32 {
//     use std::time::SystemTime;
//     let start = SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap();
//     start.as_secs_f32()
// }

fn get_current_time_seconds(start_time: &Instant) -> f32 {
    start_time.elapsed().as_secs_f32()
}


fn main() {
    let window_width = 1300;
    let window_height = 900;
    let start_time = Instant::now();

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Sistema Solar 🌍☀️")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.0, 0.0, 0.05)); // Espacio oscuro
    framebuffer.init_texture(&mut window, &raylib_thread);

    // ============================================
    // CREAR SISTEMA SOLAR
    // ============================================
    let mut solar_system = create_default_solar_system();
    solar_system.time_scale = 1.0; // Velocidad normal del tiempo
    
    // ============================================
    // CÁMARA - Ahora se mueve en el plano eclíptico
    // ============================================
    let mut camera = Camera::new(
        Vector3::new(0.0, 15.0, 25.0),  // Vista desde arriba y atrás
        Vector3::new(0.0, 0.0, 0.0),    // Mirando al sol
        Vector3::new(0.0, 1.0, 0.0),
    );
    
    // ============================================
    // CONFIGURACIÓN
    // ============================================
    let fov_y = PI / 3.0;
    let aspect = window_width as f32 / window_height as f32;
    let near = 0.1;
    let far = 200.0; // Mayor para ver todo el sistema
    
    let light = Light::new(Vector3::new(0.0, 5.0, 0.0)); // Luz desde el sol
    
    // Cargar modelo de esfera
    let obj = Obj::load("./Models/sphere.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();
    
    // ============================================
    // MODOS DE VISUALIZACIÓN
    // ============================================
    let mut show_orbits = true;
    let mut follow_planet: Option<usize> = None; // None = vista libre
    let mut paused = false;
    
    // ============================================
    // UNIFORMS
    // ============================================
    let mut uniforms = Uniforms {
        model_matrix: Matrix::identity(),
        view_matrix: Matrix::identity(),
        projection_matrix: Matrix::identity(),
        viewport_matrix: Matrix::identity(),
        time: 0.0,
        shader_mode: 0,
    };
    
    let mut last_frame_time = start_time;

    // ============================================
    // LOOP PRINCIPAL
    // ============================================
    while !window.window_should_close() {
        // Calcular delta time
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;
        
        // ============================================
        // INPUT - Controles adicionales
        // ============================================
        camera.process_input(&window);
        
        // ESPACIO: Pausar/Reanudar
        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }
        
        // O: Toggle órbitas
        if window.is_key_pressed(KeyboardKey::KEY_O) {
            show_orbits = !show_orbits;
        }
        
        // + / -: Velocidad del tiempo
        if window.is_key_pressed(KeyboardKey::KEY_EQUAL) {
            solar_system.time_scale *= 1.5;
            println!("Velocidad: {:.1}x", solar_system.time_scale);
        }
        if window.is_key_pressed(KeyboardKey::KEY_MINUS) {
            solar_system.time_scale /= 1.5;
            println!("Velocidad: {:.1}x", solar_system.time_scale);
        }
        
        // F1-F6: Seguir planetas
        if window.is_key_pressed(KeyboardKey::KEY_F1) {
            follow_planet = Some(0);
            println!("Siguiendo: {}", solar_system.planets[0].name);
        }
        if window.is_key_pressed(KeyboardKey::KEY_F2) {
            follow_planet = Some(1);
            println!("Siguiendo: {}", solar_system.planets[1].name);
        }
        if window.is_key_pressed(KeyboardKey::KEY_F3) {
            follow_planet = Some(2);
            println!("Siguiendo: {}", solar_system.planets[2].name);
        }
        if window.is_key_pressed(KeyboardKey::KEY_F4) {
            follow_planet = Some(3);
            println!("Siguiendo: {}", solar_system.planets[3].name);
        }
        if window.is_key_pressed(KeyboardKey::KEY_F5) {
            follow_planet = Some(4);
            println!("Siguiendo: {}", solar_system.planets[4].name);
        }
        if window.is_key_pressed(KeyboardKey::KEY_F6) {
            follow_planet = Some(5);
            println!("Siguiendo: {}", solar_system.planets[5].name);
        }
        if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            follow_planet = None;
            println!("Cámara libre");
        }
        
        // ============================================
        // ACTUALIZAR SISTEMA SOLAR
        // ============================================
        if !paused {
            solar_system.update(delta_time);
        }
        
        // ============================================
        // ACTUALIZAR CÁMARA (seguir planeta si está activo)
        // ============================================
        if let Some(planet_idx) = follow_planet {
            if planet_idx < solar_system.planets.len() {
                let planet = &solar_system.planets[planet_idx];
                let planet_pos = planet.get_position();
                
                // Cámara sigue al planeta desde atrás y arriba
                camera.target = planet_pos;
                camera.eye = Vector3::new(
                    planet_pos.x - 5.0,
                    planet_pos.y + 3.0,
                    planet_pos.z - 5.0,
                );
            }
        }
        
        // ============================================
        // RENDERIZAR
        // ============================================
        framebuffer.clear();
        
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(fov_y, aspect, near, far);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);
        
        uniforms.view_matrix = view_matrix;
        uniforms.projection_matrix = projection_matrix;
        uniforms.viewport_matrix = viewport_matrix;
        uniforms.time = get_current_time_seconds(&start_time);
        
        // ============================================
        // 1. RENDERIZAR SOL (centro del sistema)
        // ============================================
        let sun_translation = Vector3::new(0.0, 0.0, 0.0);
        let sun_rotation = Vector3::new(0.0, uniforms.time * 0.1, 0.0);
        let sun_scale = solar_system.sun_scale;
        
        uniforms.model_matrix = create_model_matrix(sun_translation, sun_scale, sun_rotation);
        uniforms.shader_mode = 6; // Modo sol
        
        render(
            &mut framebuffer,
            &uniforms,
            &vertex_array,
            &light,
            vertex_shader_star,
            fragment_shader_star_flares,
        );
        
        // ============================================
        // 2. RENDERIZAR PLANETAS
        // ============================================
        for (idx, planet) in solar_system.planets.iter().enumerate() {
            let position = planet.get_position();
            let rotation = planet.get_rotation();
            
            uniforms.model_matrix = create_model_matrix(position, planet.scale, rotation);
            uniforms.shader_mode = idx as i32 + 1;
            
            // Renderizar planeta con sus shaders específicos
            render(
                &mut framebuffer,
                &uniforms,
                &vertex_array,
                &light,
                planet.vertex_shader,
                planet.fragment_shader,
            );
            
            // CASO ESPECIAL: Planetas con anillos (Júpiter, Saturno)
            if planet.name == "Júpiter" || planet.name == "Saturno" {
                let torus_vertices = generate_torus_vertices(&uniforms);
                render(
                    &mut framebuffer,
                    &uniforms,
                    &torus_vertices,
                    &light,
                    vertex_shader,
                    fragment_shader_torus,
                );
            }
        }
        
        // ============================================
        // SWAP Y UI
        // ============================================
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        let mut d = window.begin_drawing(&raylib_thread);
        
        // ============================================
        // UI: Información del sistema
        // ============================================
        let ui_x = 20;
        let ui_y = 20;
        let line_h = 22;
        
        d.draw_text("=== SISTEMA SOLAR ===", ui_x, ui_y, 20, Color::YELLOW);
        
        d.draw_text(
            &format!("Tiempo: {:.1}s ({}x)", uniforms.time, solar_system.time_scale),
            ui_x,
            ui_y + line_h,
            16,
            if paused { Color::RED } else { Color::WHITE },
        );
        
        if let Some(idx) = follow_planet {
            d.draw_text(
                &format!("Siguiendo: {}", solar_system.planets[idx].name),
                ui_x,
                ui_y + line_h * 2,
                16,
                Color::GREEN,
            );
        } else {
            d.draw_text("Cámara libre", ui_x, ui_y + line_h * 2, 16, Color::LIGHTGRAY);
        }
        
        // Lista de planetas
        d.draw_text("=== PLANETAS ===", ui_x, ui_y + line_h * 4, 18, Color::YELLOW);
        for (i, planet) in solar_system.planets.iter().enumerate() {
            let color = if Some(i) == follow_planet {
                Color::GREEN
            } else {
                Color::RAYWHITE
            };
            d.draw_text(
                &format!("F{}: {}", i + 1, planet.name),
                ui_x,
                ui_y + line_h * (5 + i as i32),
                14,
                color,
            );
        }
        
        // Controles
        let controls_y = ui_y + line_h * 12;
        d.draw_text("=== CONTROLES ===", ui_x, controls_y, 18, Color::YELLOW);
        
        let controls = [
            "WASD: Rotar cámara",
            "↑↓: Zoom",
            "SPACE: Pausar",
            "O: Órbitas",
            "+/-: Velocidad tiempo",
            "F1-F6: Seguir planeta",
            "ESC: Cámara libre",
        ];
        
        for (i, &control) in controls.iter().enumerate() {
            d.draw_text(
                control,
                ui_x,
                controls_y + line_h * (i as i32 + 1),
                12,
                Color::LIGHTGRAY,
            );
        }
        
        // Crosshair (opcional)
        let center_x = window_width / 2;
        let center_y = window_height / 2;
        let crosshair_size = 5;
        d.draw_line(
            center_x - crosshair_size,
            center_y,
            center_x + crosshair_size,
            center_y,
            Color::new(255, 255, 255, 100),
        );
        d.draw_line(
            center_x,
            center_y - crosshair_size,
            center_x,
            center_y + crosshair_size,
            Color::new(255, 255, 255, 100),
        );
        
        thread::sleep(Duration::from_millis(16));
    }
}