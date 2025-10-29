//shaders.rs
use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::matrix::multiply_matrix_vector4;

// This function manually multiplies a 4x4 matrix with a 4D vector (in homogeneous coordinates)
// fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
//     Vector4::new(
//         matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
//         matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
//         matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
//         matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
//     )
// }

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
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
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix), // Note: Correct normal transformation is more complex
  }
}

fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 1.0 );

    let transformed_normal_vec4 = multiply_matrix_vector4(model_matrix, &normal_vec4);

    let mut transformed_normal = Vector3::new(
        transformed_normal_vec4.x,
        transformed_normal_vec4.y,
        transformed_normal_vec4.z,
    );

    transformed_normal.normalize();
    transformed_normal
}


 /// Patrón tipo papel aplicado a fragmentos
pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
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
    let intensity = normal.dot(light_dir).max(0.0);
    let shaded_color = fragment.color * intensity;


    // Opcional: shading de luz simple sobre el papel, si carga la textura pero con cuadro negros
    // let intensity = (fragment.normal.dot(Vector3::new(0.0, 0.0, 1.0))).max(0.0);
    // color * intensity

    //esto no soluciona los cuadros negros o mal transparentados y no carga la textura se mira gris
    // let shaded_color = fragment.color * intensity;
    color * intensity
    
}
