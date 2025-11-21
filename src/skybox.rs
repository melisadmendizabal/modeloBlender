// skybox.rs
use raylib::math::{Vector2, Vector3, Vector4};
use crate::vertex::Vertex;
use crate::fragment::{Fragment, FragmentOutput};
use crate::Uniforms;
use crate::texture::Skybox as SkyboxTexture;
use crate::matrix::multiply_matrix_vector4;

/// Genera los vértices de un cubo invertido gigante para el skybox
pub fn generate_skybox_vertices() -> Vec<Vertex> {
    let size = 10.0; // Cubo muy grande que rodea toda la escena
    
    let positions = vec![
        // Front face (+Z)
        Vector3::new(-size, -size,  size), Vector3::new( size, -size,  size), Vector3::new( size,  size,  size),
        Vector3::new(-size, -size,  size), Vector3::new( size,  size,  size), Vector3::new(-size,  size,  size),
        
        // Back face (-Z)
        Vector3::new( size, -size, -size), Vector3::new(-size, -size, -size), Vector3::new(-size,  size, -size),
        Vector3::new( size, -size, -size), Vector3::new(-size,  size, -size), Vector3::new( size,  size, -size),
        
        // Left face (-X)
        Vector3::new(-size, -size, -size), Vector3::new(-size, -size,  size), Vector3::new(-size,  size,  size),
        Vector3::new(-size, -size, -size), Vector3::new(-size,  size,  size), Vector3::new(-size,  size, -size),
        
        // Right face (+X)
        Vector3::new( size, -size,  size), Vector3::new( size, -size, -size), Vector3::new( size,  size, -size),
        Vector3::new( size, -size,  size), Vector3::new( size,  size, -size), Vector3::new( size,  size,  size),
        
        // Top face (+Y)
        Vector3::new(-size,  size,  size), Vector3::new( size,  size,  size), Vector3::new( size,  size, -size),
        Vector3::new(-size,  size,  size), Vector3::new( size,  size, -size), Vector3::new(-size,  size, -size),
        
        // Bottom face (-Y)
        Vector3::new(-size, -size, -size), Vector3::new( size, -size, -size), Vector3::new( size, -size,  size),
        Vector3::new(-size, -size, -size), Vector3::new( size, -size,  size), Vector3::new(-size, -size,  size),
    ];
    
    positions.iter().map(|&pos| {
        Vertex::new(
            pos,
            pos.normalized(), // La normal apunta hacia el centro
            Vector2::zero()
        )
    }).collect()
}

/// Vertex shader especial para skybox
/// El skybox NO debe moverse con la cámara, solo rotar
pub fn vertex_shader_skybox(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Usar solo la rotación de la cámara, no la posición
    // Esto hace que el skybox esté siempre centrado en la cámara
    let view_no_translation = uniforms.view_matrix;
    // Nota: En una implementación más avanzada, removerías el componente
    // de traslación de la matriz view, pero aquí simplificaremos
    
    let position_vec4 = Vector4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    
    // Solo aplicar rotación de la cámara
    let view_position = multiply_matrix_vector4(&view_no_translation, &position_vec4);
    
    // Forzar el depth al máximo (lejos) para que siempre se dibuje detrás
    let mut clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);
    
    // Truco: forzar z = w para que depth sea 1.0 (máximo)
    let clip_w = clip_position.w;
    clip_position.z = clip_w;
    
    let ndc = if clip_w != 0.0 {
        Vector3::new(
            clip_position.x / clip_w,
            clip_position.y / clip_w,
            1.0, // Depth = 1.0 (lo más lejos posible)
        )
    } else {
        Vector3::new(clip_position.x, clip_position.y, 1.0)
    };
    
    let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);
    
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vector3::new(
            screen_position.x,
            screen_position.y,
            1.0, 
        ),
        transformed_normal: vertex.normal,
        w: clip_w,
        view_depth: 1000.0, // Muy lejos
    }
}

/// Fragment shader para skybox con textura
pub fn fragment_shader_skybox(
    fragment: &Fragment, 
    uniforms: &Uniforms,
    skybox: &SkyboxTexture
) -> FragmentOutput {
    // Usar la posición del mundo como dirección para samplear el cubemap
    let direction = fragment.world_position.normalized();
    
    // Samplear la textura del skybox
    let color = skybox.sample(direction);
    
    FragmentOutput {
        color,
        alpha: 1.0,
    }
}

/// Versión fallback: skybox procedural (sin texturas)
pub fn fragment_shader_skybox_procedural(
    fragment: &Fragment,
    _uniforms: &Uniforms
) -> FragmentOutput {
    let dir = fragment.world_position.normalized();
    
    // Gradiente espacio exterior
    let horizon = dir.y.abs().powf(0.5);
    
    let space_dark = Vector3::new(0.01, 0.01, 0.05);
    let space_blue = Vector3::new(0.05, 0.1, 0.2);
    
    let color = space_dark * (1.0 - horizon) + space_blue * horizon;
    
    // Estrellas procedurales simples
    let star_noise = ((dir.x * 1000.0).sin() * (dir.y * 1000.0).cos() * (dir.z * 1000.0).sin()).abs();
    let star = if star_noise > 0.998 { 0.5 } else { 0.0 };
    
    let final_color = color + Vector3::new(star, star, star);
    
    FragmentOutput {
        color: final_color,
        alpha: 1.0,
    }
}