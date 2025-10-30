//shaders.rs
use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::matrix::multiply_matrix_vector4;

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



pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {


    let mut position = vertex.position;

    // 🌕 Si es Saturno (modo 2), deformar para anillos
  if uniforms.shader_mode == 2 {
    let radius = (position.x * position.x + position.z * position.z).sqrt();
    if radius > 0.4 && radius < 0.8 {
        // aplicar una función senoidal para hacer varias franjas
        let factor = ((radius - 0.4) * 20.0).sin() * 0.05; 
        position.y *= factor;
    }
}



  // Convert vertex position to homogeneous coordinates (Vec4) by adding a w-component of 1.0
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

  // Apply Projection transformation (perspective)
  let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);
  let clip_w = clip_position.w;
    let ndc = if clip_w != 0.0 {
        Vector3::new(clip_position.x / clip_w, clip_position.y / clip_w, clip_position.z / clip_w)
    } else {
        Vector3::new(clip_position.x, clip_position.y, clip_position.z)
    };

  // Perform perspective division to get NDC (Normalized Device Coordinates)
  let ndc = if clip_position.w != 0.0 {
      Vector3::new(
          clip_position.x / clip_position.w,
          clip_position.y / clip_position.w,
          clip_position.z / clip_position.w,
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
    transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix), // Note: Correct normal transformation is more complex
    w: clip_w,
  }
}

fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);

    // si no hay escala no uniforme, basta con esto:
    let transformed = multiply_matrix_vector4(model_matrix, &normal_vec4);
    let mut n = Vector3::new(transformed.x, transformed.y, transformed.z);
    n.normalize();
    n
}



 /// Patrón tipo papel aplicado a fragmentos
pub fn fragment_shader_papel(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let base_color = Vector3::new(1.0, 1.0, 0.8); // beige claro

    // Configuración de las líneas del papel
    let line_spacing = 20.0;     // separación entre líneas
    let thick_line_width = 2.0;  // grosor de la línea "sombreada"
    let thin_line_width = 1.0;   // grosor de la línea fina, opcional

    // Tomamos la coordenada Y del fragmento para el patrón
    let mod_y = fragment.position.y % line_spacing;

    // Determinar el color según la línea
    let color = if mod_y < thick_line_width {
        Vector3::new(0.9, 0.9, 0.9) // línea blanca gruesa
    } else if mod_y > line_spacing - thin_line_width {
        Vector3::new(0.8, 0.8, 1.0) // línea azul fina opcional
    } else {
        base_color // fondo de papel
    };

    let mut light_dir = Vector3::new(0.0, 0.0, 1.0); // dirección de la luz
    light_dir.normalize();

    let mut normal = fragment.normal;
    normal.normalize(); 

    let mut intensity = normal.dot(light_dir).max(0.0);
    intensity = intensity.max(0.0);
    let mut shaded_color = fragment.color * intensity;
    shaded_color.x = shaded_color.x.min(255.0).max(0.0);
    shaded_color.y = shaded_color.y.min(255.0).max(0.0);
    shaded_color.z = shaded_color.z.min(255.0).max(0.0);

    let mut a = color * intensity;

    a

    // Opcional: shading de luz simple sobre el papel, si carga la textura pero con cuadro negros
    // let intensity = (fragment.normal.dot(Vector3::new(0.0, 0.0, 1.0))).max(0.0);
    //color * intensity

    //esto no soluciona los cuadros negros o mal transparentados y no carga la textura se mira gris
    // let shaded_color = fragment.color * intensity;
    //shaded_color
    //color
    //return (fragment.normal * 0.5) + Vector3::new(0.5, 0.5, 0.5);
    //return Vector3::new(intensity, intensity, intensity);
    
    //light_dir

    //el problema es con la normal no importa si está normalizada
    
}


pub fn fragment_shader_rocoso(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    // color base tipo roca
    let base_blue = Vector3::new(0.2, 0.4, 0.8);
    let green = Vector3::new(0.1, 0.6, 0.2);
    let clouds = Vector3::new(1.0, 1.0, 1.0);

    // Patrón senoidal para “bandas” tipo nubes
    let band = ((fragment.position.y * 10.0 + uniforms.time * 2.0).sin() * 0.5 + 0.5).powf(2.0);

    // Mezcla colores
    let color = base_blue * (1.0 - band) + green * 0.3 + clouds * band * 0.6;

    // Luz
    let light_dir = Vector3::new(0.0, 0.0, 1.0);
    let intensity = fragment.normal.dot(light_dir).max(0.0);

    color * intensity
}

pub fn fragment_shader_gaseoso(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    // efecto ondulado o variable con el tiempo
    let base_red = Vector3::new(0.8, 0.3, 0.1);
    let darker = Vector3::new(0.5, 0.2, 0.1);

    // Rugosidad con seno y coseno
    let roughness = ((fragment.position.x * 8.0 + uniforms.time).sin()
        * (fragment.position.y * 5.0).cos())
        .abs();

    let color = base_red * (1.0 - roughness) + darker * roughness * 0.7;

    let light_dir = Vector3::new(0.0, 0.0, 1.0);
    let intensity = fragment.normal.dot(light_dir).max(0.0);

    color * intensity
}

pub fn fragment_shader_personalizado(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let base = Vector3::new(0.9, 0.8, 0.6);
    let bands = ((fragment.position.y * 12.0).sin() * 0.5 + 0.5).powf(1.5);
    let ring_color = Vector3::new(0.8, 0.7, 0.5);

    let mut color = base * (1.0 - bands) + ring_color * bands * 0.7;

    // Luz direccional
    let light_dir = Vector3::new(0.0, 0.0, 1.0);
    let intensity = fragment.normal.dot(light_dir).max(0.0);

    color *= intensity;
    color
}


pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    match uniforms.shader_mode {
        //0 => fragment_shader_papel(fragment, uniforms),
        1 => fragment_shader_rocoso(fragment, uniforms),
        2 => fragment_shader_anillo(fragment, uniforms),
        3 => fragment_shader_personalizado(fragment, uniforms),
        _ => Vector3::new(1.0, 0.0, 1.0),
    }
}


