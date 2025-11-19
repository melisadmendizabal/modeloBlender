use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::matrix::multiply_matrix_vector4;
use crate::fragment::FragmentOutput;
use crate::shader_anillo::fragment_shader_torus;
use crate::shader_strawberry::fragment_shader_strawberry;
use crate::shader_gaseoso::fragment_shader_gaseoso;
use crate::shader_rocoso::fragment_shader_crater_hybrid;
use crate::shader_rojo::fragment_shader_red_planet;
use crate::shader_sol::fragment_shader_star_flares;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let mut position = vertex.position;

    // 🌕 Si es Saturno (modo 2), deformar para anillos
    if uniforms.shader_mode == 2 {
        let radius = (position.x * position.x + position.z * position.z).sqrt();
        if radius > 0.4 && radius < 0.8 {
            let factor = ((radius - 0.4) * 20.0).sin() * 0.05; 
            position.y *= factor;
        }
    }

    // Convert vertex position to homogeneous coordinates
    let position_vec4 = Vector4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    // Apply Model transformation
    let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

    // Apply View transformation (camera)
    let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);

    // ✅ CRÍTICO: Guardar la profundidad en VIEW SPACE
    let view_depth = -view_position.z;

    // Apply Projection transformation (perspective)
    let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);
    let clip_w = clip_position.w;

    // Perform perspective division to get NDC
    let ndc = if clip_w != 0.0 {
        Vector3::new(
            clip_position.x / clip_w,
            clip_position.y / clip_w,
            clip_position.z / clip_w,
        )
    } else {
        Vector3::new(clip_position.x, clip_position.y, clip_position.z)
    };

    // Apply Viewport transformation to get screen coordinates
    let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

    let transformed_position = Vector3::new(
        screen_position.x,
        screen_position.y,
        screen_position.z,
    );

    // Create a new Vertex with the transformed position
    Vertex {
        position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
        w: clip_w,
        view_depth,  // ✅ Agregar view_depth
    }
}

fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);
    let transformed = multiply_matrix_vector4(model_matrix, &normal_vec4);
    let mut n = Vector3::new(transformed.x, transformed.y, transformed.z);
    n.normalize();
    n
}

pub fn fragment_shader_papel(fragment: &Fragment, _uniforms: &Uniforms) -> FragmentOutput {
    let base_color = Vector3::new(1.0, 1.0, 0.8);

    let line_spacing = 20.0;
    let thick_line_width = 2.0;
    let thin_line_width = 1.0;

    let mod_y = fragment.world_position.y % line_spacing;

    let color = if mod_y < thick_line_width {
        Vector3::new(0.9, 0.9, 0.9)
    } else if mod_y > line_spacing - thin_line_width {
        Vector3::new(0.8, 0.8, 1.0)
    } else {
        base_color
    };

    let mut light_dir = Vector3::new(0.0, 0.0, 1.0);
    light_dir.normalize();

    let mut normal = fragment.normal;
    normal.normalize(); 

    let intensity = normal.dot(light_dir).max(0.3);
    
    let final_color = color * intensity;

    FragmentOutput {
        color: final_color,
        alpha: 1.0,
    }
}

pub fn fragment_shader_rocoso(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput  {
    let base_blue = Vector3::new(0.2, 0.4, 0.8);
    let green = Vector3::new(0.1, 0.6, 0.2);
    let clouds = Vector3::new(1.0, 1.0, 1.0);

    let band = ((fragment.world_position.y * 10.0 + uniforms.time * 2.0).sin() * 0.5 + 0.5).powf(2.0);

    let color = base_blue * (1.0 - band) + green * 0.3 + clouds * band * 0.6;

    let light_dir = Vector3::new(0.0, 0.0, 1.0);
    let intensity = fragment.normal.dot(light_dir).max(0.0);

    let final_color = color * intensity;

    FragmentOutput {
        color: final_color,
        alpha: 0.1,
    }
}

pub fn combine_shaders(
    fragment: &Fragment,
    uniforms: &Uniforms,
    mix_factor: f32,
) -> FragmentOutput {
    let s1 = fragment_shader_rocoso(fragment, uniforms);
    let s2 = fragment_shader_gaseoso(fragment, uniforms);

    FragmentOutput {
        color: s1.color * (1.0 - mix_factor) + s2.color * mix_factor,
        alpha: s1.alpha * (1.0 - mix_factor) + s2.alpha * mix_factor,
    }
}

pub fn fragment_shader_compuesto(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let mix_factor = ((uniforms.time * 0.5).sin() * 0.5 + 0.5).powf(1.5);
    combine_shaders(fragment, uniforms, mix_factor)
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    match uniforms.shader_mode {
        1 => fragment_shader_crater_hybrid(fragment, uniforms),
        2 => fragment_shader_gaseoso(fragment, uniforms),
        3 => fragment_shader_strawberry(fragment, uniforms),
        4 => fragment_shader_torus(fragment, uniforms),
        5 => fragment_shader_red_planet(fragment, uniforms),
        6 => fragment_shader_star_flares(fragment, uniforms),
        _ => fragment_shader_crater_hybrid(fragment, uniforms),
    }
}