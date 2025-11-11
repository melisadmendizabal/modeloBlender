//shadersAnillos.rs
use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::matrix::multiply_matrix_vector4;
use crate::fragment::FragmentOutput;

pub fn fragment_shader_anillo(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let base_color = Vector3::new(0.8, 0.7, 0.5); // color principal del anillo
    let band_color = Vector3::new(1.0, 1.0, 1.0); // bandas más claras

    // Coordenadas polares del fragmento (relativo al centro del planeta)
    let radius = (fragment.position.x * fragment.position.x + fragment.position.y * fragment.position.y).sqrt();
    let angle = fragment.position.y.atan2(fragment.position.x);

    // Simulación de relieve: altura aparente
    let bump = (radius * 20.0 + uniforms.time * 2.0).sin() * 0.1; 
    // opcional: más ruido con angle:
    let bump = bump + (angle * 10.0).sin() * 0.05;

    // Modificar la normal para simular la luz sobre relieve
    let mut normal = fragment.normal;
    normal.y += bump; // elevar la normal según el bump
    normal.normalize();

    // Patrón de bandas
    let band = ((radius * 15.0).sin() * 0.5 + 0.5).powf(2.0);
    let color = base_color * (1.0 - band) + band_color * band;

    // Luz direccional
    let light_dir = Vector3::new(0.0, 0.0, 1.0);
    let intensity = normal.dot(light_dir).max(0.0);

    color * intensity
}

pub fn vertex_shader_torus(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let R = 0.8; //interior
    let r = 0.2; //grosor
    let flatten_factor = 0.2;

    let theta = vertex.position.x * std::f32::consts::PI * 2.0;
    let phi = vertex.position.y * std::f32::consts::PI * 2.0;

    let x = (R + r * phi.cos()) * theta.cos();
    let y = r * phi.sin() * flatten_factor;
    let z = (R + r * phi.cos()) * theta.sin();

    // Crear vector homogéneo
    let position_vec4 = Vector4::new(x, y, z, 1.0);

    // Transformaciones
    let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);
    let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);
    let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);
    let clip_w = clip_position.w;

    // Perspectiva
    let ndc = if clip_w != 0.0 {
        Vector3::new(
            clip_position.x / clip_w,
            clip_position.y / clip_w,
            clip_position.z / clip_w,
        )
    } else {
        Vector3::new(clip_position.x, clip_position.y, clip_position.z)
    };

    let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

    let transformed_position = Vector3::new(
        screen_position.x,
        screen_position.y,
        screen_position.z,
    );

    let mut normal = Vector3::new(
        phi.cos() * theta.cos(),
        phi.sin(),
        phi.cos() * theta.sin(),
    );
    normal.normalize();

    Vertex {
        position: Vector3::new(x, y, z),
        normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: normal,
        w: clip_w,
    }
}

pub fn generate_torus_vertices(uniforms: &Uniforms) -> Vec<Vertex> {
    let mut vertex_array = Vec::new();
    let steps_theta = 60;
    let steps_phi = 10;
    let R = 1.0;
    let r = 0.1;
    

    for i in 0..steps_theta {
        for j in 0..steps_phi {
            let u = i as f32 / steps_theta as f32;
            let v = j as f32 / steps_phi as f32;

            // Posición inicial (vertex_shader_torus lo normaliza luego)
            let vertex = Vertex {
                position: Vector3::new(u, v, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                tex_coords: Vector2::new(u, v),
                color: Vector3::new(1.0, 1.0, 1.0),
                transformed_position: Vector3::new(0.0, 0.0, 0.0),
                transformed_normal: Vector3::new(0.0, 0.0, 0.0),
                w: 1.0,
            };

            vertex_array.push(vertex_shader_torus(&vertex, uniforms));
        }
    }

    vertex_array
}


pub fn fragment_shader_torus(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let mut light_dir = Vector3::new(0.0, 1.0, 1.0);
    light_dir.normalize();
    let intensity = (fragment.normal.x * light_dir.x
    + fragment.normal.y * light_dir.y
    + fragment.normal.z * light_dir.z)
    .max(0.0);



    let base_color = Vector3::new(0.98, 0.72, 0.1);
    let highlight = Vector3::new(1.0, 1.0, 0.8);

    let roughness = ((fragment.position.x*5.0 + uniforms.time).sin() * (fragment.position.y*3.0).cos()).abs();
    let color = base_color * (1.0 - roughness) + highlight * roughness;

    //color * intensity + highlight * intensity.powf(4.0)

    let r = (fragment.position.x * 10.0).sin().abs();
    let g = (fragment.position.y * 10.0).sin().abs();
    let b = ((fragment.position.x + fragment.position.y)*5.0).sin().abs();
    let color = Vector3::new(r, g, b);
    //color
    let final_color = base_color * intensity + highlight * intensity.powf(4.0);

    FragmentOutput {
        color: final_color,
        alpha: 1.0, // 👈 este shader será semitransparente
    }
}



pub fn fragment_shader_personalizado(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput  {
    let base = Vector3::new(1.0, 0.28, 0.9);
    let bands = ((fragment.world_position.y * 12.0).sin() * 0.5 + 0.5).powf(1.5);
    let ring_color = Vector3::new(0.6, 0.7, 0.89);

    let mut color = base * (1.0 - bands) + ring_color * bands * 0.7;

    // Luz direccional
    let light_dir = Vector3::new(0.0, 0.0, 1.0);
    let intensity = fragment.normal.dot(light_dir).max(0.0);

    // color *= intensity;
    // color
    let final_color = color * intensity;
    FragmentOutput {
        color: final_color,
        alpha: 0.7, // 👈 este shader será semitransparente
    }
}



