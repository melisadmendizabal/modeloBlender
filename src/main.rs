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


pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
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
        // Verificar que el fragment esté dentro de los límites del framebuffer
        // if fragment.position.x >= 0.0 && fragment.position.x < framebuffer.width as f32 &&
        //    fragment.position.y >= 0.0 && fragment.position.y < framebuffer.height as f32 {
            
        //let color = shaders::fragment_shader_paper(&fragment);
        let final_color = fragment_shader(&fragment, uniforms);
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            final_color, 
            fragment.depth,

        );
        //}
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Barco de papel :D")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.2,0.2,0.4)); //azul oscuro

    framebuffer.init_texture(&mut window, &raylib_thread);

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

    let obj = Obj::load("./Models/barcoPapel.obj").expect("Failed to load obj");
    
    // vertex_array ya es Vec<Vertex> gracias a los cambios en obj.rs
    let vertex_array = obj.get_vertex_array();

    while !window.window_should_close() {
        camera.process_input(&window);

        rotation_y += rotation_speed;
        
        framebuffer.clear();

        let rotation = Vector3::new(0.0, rotation_y, 0.0);
        // Crear matrices de transformación
        let model_matrix = create_model_matrix (translation, scale, rotation);
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(fov_y, aspect, near, far);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        // Crear uniforms
        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
        };

        render(&mut framebuffer, &uniforms, &vertex_array, &light);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        let mut d = window.begin_drawing(&raylib_thread);
        let center_x = window_width / 2;
        let center_y = window_height / 2;
        let crosshair_size = 10;

        d.draw_line(center_x - crosshair_size, center_y, center_x + crosshair_size, center_y, Color::WHITE);
        d.draw_line(center_x, center_y - crosshair_size, center_x, center_y + crosshair_size, Color::WHITE);

        
        thread::sleep(Duration::from_millis(16));
    }
}