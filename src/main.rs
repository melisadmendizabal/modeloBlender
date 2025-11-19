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
use crate::shaders::fragment_shader_papel;
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
        .title("Sistema Solar 🚀 - Debug Mode")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.0, 0.0, 0.05));
    framebuffer.init_texture(&mut window, &raylib_thread);

    println!("✅ Cargando modelos...");
    let obj = Obj::load("./Models/sphere.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();
    println!("✅ Esfera cargada: {} vértices", vertex_array.len());

    let ship_obj = Obj::load("./Models/barcoPapel.obj").expect("Failed to load barco");
    let ship_vertices = ship_obj.get_vertex_array();
    println!("✅ Nave cargada: {} vértices", ship_vertices.len());
    
    let mut solar_system = create_default_solar_system();
    solar_system.time_scale = 1.0;
    println!("✅ Sistema solar creado");
    
    let mut camera = Camera::new(
        Vector3::new(0.0, 5.0, 15.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    
    let fov_y = PI / 3.0;
    let aspect = window_width as f32 / window_height as f32;
    let near = 0.1;
    let far = 200.0;
    
    let light = Light::new(Vector3::new(0.0, 5.0, 0.0));
    
    let mut show_orbits = true;
    let mut paused = false;
    
    let mut uniforms = Uniforms {
        model_matrix: Matrix::identity(),
        view_matrix: Matrix::identity(),
        projection_matrix: Matrix::identity(),
        viewport_matrix: Matrix::identity(),
        time: 0.0,
        shader_mode: 0,
    };
    
    let mut last_frame_time = start_time;
    let mut frame_count = 0u64;

    println!("🚀 Iniciando loop principal...\n");
    println!("💡 Tip: Presiona ESC para salir limpiamente\n");

    while !window.window_should_close() {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;
        
        frame_count += 1;
        let elapsed = get_current_time_seconds(&start_time);
        
        // 🔍 DEBUG: Log cada segundo
        if frame_count % 60 == 0 {
            println!("⏱️  Frame {}: t={:.2}s | Ship pos=({:.1}, {:.1}, {:.1}) | yaw={:.2} pitch={:.2}",
                frame_count,
                elapsed,
                solar_system.spaceship.position.x,
                solar_system.spaceship.position.y,
                solar_system.spaceship.position.z,
                solar_system.spaceship.yaw,
                solar_system.spaceship.pitch
            );
        }
        
        // 🔍 DEBUG: Log específico cerca del segundo 9.3
        if elapsed > 9.0 && elapsed < 10.0 && frame_count % 10 == 0 {
            println!("🔴 CERCA DEL SEGUNDO 9.3!");
            println!("   Camera eye: ({:.2}, {:.2}, {:.2})", 
                camera.eye.x, camera.eye.y, camera.eye.z);
            println!("   Camera target: ({:.2}, {:.2}, {:.2})", 
                camera.target.x, camera.target.y, camera.target.z);
        }
        
        // INPUT
        solar_system.spaceship.process_input(&window);

        let had_collision = solar_system.resolve_collision();
        if had_collision && frame_count % 30 == 0 {
            println!("🛡️ Sistema de colisiones activo");
        }

        // ACTUALIZAR CÁMARA (después de resolver colisiones)
        camera.eye = solar_system.spaceship.get_camera_position();
        camera.target = solar_system.spaceship.get_camera_target();
        
        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
            println!("⏸️  Pausado: {}", paused);
        }
        
        if window.is_key_pressed(KeyboardKey::KEY_O) {
            show_orbits = !show_orbits;
        }
        
        if window.is_key_pressed(KeyboardKey::KEY_EQUAL) {
            solar_system.time_scale *= 1.5;
            println!("⏩ Velocidad: {:.1}x", solar_system.time_scale);
        }
        if window.is_key_pressed(KeyboardKey::KEY_MINUS) {
            solar_system.time_scale /= 1.5;
            println!("⏪ Velocidad: {:.1}x", solar_system.time_scale);
        }

        // 🆕 H: Resetear posición de la nave (HOME)
        if window.is_key_pressed(KeyboardKey::KEY_H) {
            solar_system.spaceship.position = Vector3::new(80.0, 10.0, 45.0);
            solar_system.spaceship.yaw = 0.0;
            solar_system.spaceship.pitch = 0.0;
            solar_system.spaceship.roll = 0.0;
            println!("🏠 Nave reseteada a posición inicial");
        }

        if window.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            if window.is_key_pressed(KeyboardKey::KEY_KP_8) {
                solar_system.spaceship.camera_offset.y += 0.5;
                println!("📷 Camera Y: {:.2}", solar_system.spaceship.camera_offset.y);
            }
            if window.is_key_pressed(KeyboardKey::KEY_KP_2) {
                solar_system.spaceship.camera_offset.y -= 0.5;
                println!("📷 Camera Y: {:.2}", solar_system.spaceship.camera_offset.y);
            }
            if window.is_key_pressed(KeyboardKey::KEY_KP_7) {
                solar_system.spaceship.camera_offset.z += 0.5;
                println!("📷 Camera Z: {:.2}", solar_system.spaceship.camera_offset.z);
            }
            if window.is_key_pressed(KeyboardKey::KEY_KP_9) {
                solar_system.spaceship.camera_offset.z -= 0.5;
                println!("📷 Camera Z: {:.2}", solar_system.spaceship.camera_offset.z);
            }
        }
        
        // ACTUALIZAR CÁMARA
        camera.eye = solar_system.spaceship.get_camera_position();
        camera.target = solar_system.spaceship.get_camera_target();
        
        // Verificar si la cámara es válida
        if camera.eye.x.is_nan() || camera.eye.y.is_nan() || camera.eye.z.is_nan() {
            eprintln!("❌ ERROR: camera.eye tiene NaN!");
            eprintln!("   Ship pos: ({:.2}, {:.2}, {:.2})", 
                solar_system.spaceship.position.x,
                solar_system.spaceship.position.y,
                solar_system.spaceship.position.z
            );
            break;
        }
        
        if camera.target.x.is_nan() || camera.target.y.is_nan() || camera.target.z.is_nan() {
            eprintln!("❌ ERROR: camera.target tiene NaN!");
            break;
        }
        
        if !paused {
            solar_system.update(delta_time);
        }
        
       
    


        // RENDERIZAR
framebuffer.clear();

let view_matrix = camera.get_view_matrix();
let projection_matrix = create_projection_matrix(fov_y, aspect, near, far);
let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

uniforms.view_matrix = view_matrix;
uniforms.projection_matrix = projection_matrix;
uniforms.viewport_matrix = viewport_matrix;
uniforms.time = elapsed;

// ============================================
// ✅ RENDERIZAR TODOS LOS OBJETOS EN UN LOOP
// Ahora el Sol es el primer planeta (index 0)
// ============================================

for (idx, planet) in solar_system.planets.iter().enumerate() {
    let position = planet.get_position();
    let rotation = planet.get_rotation();
    
    uniforms.model_matrix = create_model_matrix(position, planet.scale, rotation);
    
    // Shader mode: Sol = 6, planetas = idx
    uniforms.shader_mode = if planet.name == "Sol" { 
        6 
    } else { 
        idx as i32 
    };
    
    render(
        &mut framebuffer,
        &uniforms,
        &vertex_array,
        &light,
        planet.vertex_shader,
        planet.fragment_shader,
    );
    
    // Anillo de Júpiter
    if planet.name == "Júpiter" {
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
// NAVE (siempre al final)
// ============================================
let ship_position = solar_system.spaceship.get_world_position();
let ship_rotation = solar_system.spaceship.get_world_rotation();

uniforms.model_matrix = create_model_matrix(
    ship_position,
    solar_system.spaceship.scale,
    ship_rotation
);
uniforms.shader_mode = 99;

render(
    &mut framebuffer,
    &uniforms,
    &ship_vertices,
    &light,
    vertex_shader,
    fragment_shader_papel,
);

framebuffer.swap_buffers(&mut window, &raylib_thread);




        
        let mut d = window.begin_drawing(&raylib_thread);
        
        let ui_x = 20;
        let ui_y = 20;
        let line_h = 22;

                // En main.rs, después del loop de renderizado
        if frame_count % 120 == 0 {
            println!("🌍 Posiciones:");
            for planet in &solar_system.planets {
                let pos = planet.get_position();
                println!("   {} → ({:.1}, {:.1}, {:.1})", 
                    planet.name, pos.x, pos.y, pos.z);
            }
        }
            
                
        d.draw_text("=== DEBUG MODE ===", ui_x, ui_y, 20, Color::RED);
        
        d.draw_text(
            &format!("Frame: {} | Time: {:.2}s", frame_count, elapsed),
            ui_x,
            ui_y + line_h,
            16,
            Color::YELLOW,
        );
        
        d.draw_text(
            &format!("Pos: ({:.1}, {:.1}, {:.1})", 
                solar_system.spaceship.position.x,
                solar_system.spaceship.position.y,
                solar_system.spaceship.position.z
            ),
            ui_x,
            ui_y + line_h * 2,
            16,
            Color::WHITE,
        );
        
        d.draw_text(
            &format!("Yaw: {:.2} | Pitch: {:.2}", 
                solar_system.spaceship.yaw,
                solar_system.spaceship.pitch
            ),
            ui_x,
            ui_y + line_h * 3,
            16,
            Color::LIGHTGRAY,
        );
        
        // 🆕 MOSTRAR DISTANCIA AL PLANETA MÁS CERCANO
        if let Some((idx, distance, name)) = solar_system.get_distance_to_closest_planet(solar_system.spaceship.position) {
            let color = if distance < 3.0 {
                Color::RED  // ¡Peligro! Muy cerca
            } else if distance < 5.0 {
                Color::ORANGE  // Advertencia
            } else {
                Color::GREEN  // Seguro
            };
            
            d.draw_text(
                &format!("Cerca de: {} ({:.1} unidades)", name, distance),
                ui_x,
                ui_y + line_h * 4,
                16,
                color,
            );
        }
        
        // 🆕 DETECTAR COLISIÓN
        if let Some(planet_name) = solar_system.check_collision_with_planet(solar_system.spaceship.position, 2.0) {
            d.draw_text(
                &format!("⚠️ COLISIÓN CON {}!", planet_name),
                ui_x,
                ui_y + line_h * 5,
                18,
                Color::RED,
            );
        }
        
        // Crosshair
        let center_x = window_width / 2;
        let center_y = window_height / 2;
        d.draw_circle(center_x, center_y, 3.0, Color::new(255, 0, 0, 150));
        
        thread::sleep(Duration::from_millis(16));
    }
    
    println!("\n✅ Programa terminado correctamente");
}