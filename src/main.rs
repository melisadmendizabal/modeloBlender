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
        .title("Planetitas :D")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.27,0.1,0.3)); //azul oscuro

    framebuffer.init_texture(&mut window, &raylib_thread);



   
    framebuffer.texture.as_mut().map(|tex| {
        let colors = framebuffer.image.get_image_data();
        let data: &[u8] = unsafe {
            std::slice::from_raw_parts(colors.as_ptr() as *const u8, colors.len() * 4)
        };
        tex.update_texture(data).unwrap();
    });



    let translation = Vector3::new(0.0, 0.0, 0.0);
    let rotation_y = 0.0f32;
    
    // ============================================
    // NUEVO: Sistema de rotación configurable
    // ============================================
    let mut auto_rotate = true;              // Toggle rotación automática
    let rotation_speed_y = 0.3f32;       // Velocidad rotación Y (izq/der)
    let rotation_speed_x = 0.0f32;       // Velocidad rotación X (arriba/abajo)
    let rotation_speed_z = 0.0f32;       // Velocidad rotación Z (inclinación)
    
    let mut rotation_x = 0.0f32;
    let mut rotation_z = 0.0f32;
    
    // Configuraciones preestablecidas por planeta
    let current_preset = 0; // 0 = custom, 1-5 = presets
    
    let scale = 1.0f32;





    let camera_position = Vector3::new(0.0, 1.0, 5.0);
    let camera_target = Vector3::new(0.0, 0.0, 0.0);
    let camera_up = Vector3::new(0.0, 1.0, 0.0);
    // Inicializar cámara
    let mut camera = Camera::new(
        camera_position,
        camera_target,
        camera_up
    );

    let fov_y = PI / 3.0;
    let aspect = window_width as f32 / window_height as f32;
    let near = 0.1;
    let far = 100.0;

    // Parámetros de transformación del modelo (fijos)
    let translation = Vector3::new(0.0, 0.0, 0.0);
    let mut rotation_y = 0.0f32;
    let rotation_speed = 0.0f32;
    let scale = 1.0f32;
      // Light

    let light = Light::new(Vector3::new(5.0, 5.0, 5.0));

    //let obj = Obj::load("./Models/barco.obj").expect("Failed to load obj");
    //let obj = Obj::load("./Models/cuboo.obj").expect("Failed to load obj");
    let obj = Obj::load("./Models/sphere.obj").expect("Failed to load obj");

  

    // vertex_array ya es Vec<Vertex> gracias a los cambios en obj.rs
    let vertex_array = obj.get_vertex_array();

    //menu de los shaders
    let mut current_shader = 6;
    let mut uniforms = Uniforms {
        model_matrix: Matrix::identity(),
        view_matrix: Matrix::identity(),
        projection_matrix: Matrix::identity(),
        viewport_matrix: Matrix::identity(),
        time: 0.0,
        shader_mode: 0,
    };


    while !window.window_should_close() {
        camera.process_input(&window);

        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            auto_rotate = !auto_rotate;
            println!("Auto-rotación: {}", if auto_rotate { "ON" } else { "OFF" });
        }

        if auto_rotate {
            rotation_y += rotation_speed_y * 0.1;  // Multiplicador para suavizar
            rotation_x += rotation_speed_x * 0.01;
            rotation_z += rotation_speed_z * 0.01;
        }

        rotation_y += rotation_speed;
        
        framebuffer.clear();

        let rotation = Vector3::new(rotation_x, rotation_y, rotation_z);
        // Crear matrices de transformación
        let model_matrix = create_model_matrix (translation, scale, rotation);
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(fov_y, aspect, near, far);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        // Crear uniforms
        uniforms.model_matrix = model_matrix;
        uniforms.view_matrix = view_matrix;
        uniforms.projection_matrix = projection_matrix;
        uniforms.viewport_matrix = viewport_matrix;


        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            current_shader = 1;
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            current_shader = 2;
        }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_shader = 3;
        }
        if window.is_key_pressed(KeyboardKey::KEY_FOUR) {
            current_shader = 4; // Nuevo shader: toroide
        }
        if window.is_key_pressed(KeyboardKey::KEY_FIVE) {
            current_shader = 5; // Nuevo shader: toroide
        }
        if window.is_key_pressed(KeyboardKey::KEY_SIX) {
            current_shader = 6; // Nuevo shader: toroide
        }

        // Actualizar uniforms
        uniforms.time = get_current_time_seconds(&start_time);
        uniforms.shader_mode = current_shader;

        if current_shader == 4 {
            // Renderizar planeta con su shader
            render(
                &mut framebuffer, 
                &uniforms, 
                &vertex_array, 
                &light, 
                vertex_shader,
                fragment_shader_personalizado);

            // Generar y renderizar toroide con su shader
            let torus_vertices = generate_torus_vertices(&uniforms);
            render(&mut framebuffer, &uniforms, &torus_vertices, &light, vertex_shader, fragment_shader_torus);

        } else if current_shader == 6 {
            // ✨ SOL - USA VERTEX SHADER PERSONALIZADO
            render(
                &mut framebuffer, 
                &uniforms, 
                &vertex_array, 
                &light, 
                vertex_shader_star,        // ✅ Vertex shader del sol!
                fragment_shader_star_flares // ✅ Fragment shader del sol!
            );

        } else {
            // Renderizar normalmente según el shader seleccionado
            let shader_fn = match current_shader {
                1 => fragment_shader_crater_hybrid,
                2 => fragment_shader_gaseoso,
                3 => fragment_shader_strawberry,
                4 => fragment_shader_torus,
                5 => fragment_shader_red_planet,
                _ => fragment_shader_crater_hybrid,
            };
            render(&mut framebuffer, &uniforms, &vertex_array, &light,vertex_shader, shader_fn);
        }



        framebuffer.swap_buffers(&mut window, &raylib_thread);

        let mut d = window.begin_drawing(&raylib_thread);

        // Dibujar un menú visual simple
        let options = ["1. Planeta Rocoso", "2. Planeta Gaseoso", "3. Planeta Fresita", "4. Planeta Anillos", "5. Planeta Rojo", "6. Sol"];
        let start_y = 60;
        for (i, &option) in options.iter().enumerate() {
            let y = start_y + i as i32 * 25;
            let color = if i as i32 + 1 == current_shader {
                Color::YELLOW // resalta el shader activo
            } else {
                Color::RAYWHITE
            };
            d.draw_text(option, 20, y, 20, color);
        }


        let center_x = window_width / 2;
        let center_y = window_height / 2;
        let crosshair_size = 10;

        d.draw_line(center_x - crosshair_size, center_y, center_x + crosshair_size, center_y, Color::WHITE);
        d.draw_line(center_x, center_y - crosshair_size, center_x, center_y + crosshair_size, Color::WHITE);

        //implementacion del menu
        // Mostrar el shader activo en pantalla
        let shader_name = match current_shader {
            1 => "*.°- Rocoso -°.*",
            2 => "*.°- Gaseoso -°.*",
            3 => "*.°- Fresita -°.*",
            4 => "*.°- Anillos -°.*",
            5 => "*.°- Rojito -°.*",
            6 => "*.°- Sol _°.*",
            _ => "*.°- Rocoso -°.*",
        };

        d.draw_text(
            &format!("Time: {:.1}s", uniforms.time),
            window_width - 150,
            15,
            20,
            Color::LIGHTGRAY,
        );


        
        thread::sleep(Duration::from_millis(16));
    }
}


