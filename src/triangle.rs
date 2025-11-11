// triangle.rs
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Vector3;
use crate::light::Light;

// ============================================
// Coordenadas baricéntricas usando Regla de Cramer
// (Más preciso y robusto)
// ============================================
pub fn barycentric_coordinates(
    p_x: f32,
    p_y: f32,
    a: &Vertex,
    b: &Vertex,
    c: &Vertex,
) -> (f32, f32, f32) {
    // Extraer coordenadas de los vértices
    let (x1, y1) = (a.transformed_position.x, a.transformed_position.y);
    let (x2, y2) = (b.transformed_position.x, b.transformed_position.y);
    let (x3, y3) = (c.transformed_position.x, c.transformed_position.y);
    let (px, py) = (p_x, p_y);

    // Sistema de ecuaciones lineales:
    // α * x1 + β * x2 + γ * x3 = px
    // α * y1 + β * y2 + γ * y3 = py
    // α + β + γ = 1
    //
    // Donde α = w1, β = w2, γ = w3 (coordenadas baricéntricas)

    // Matriz A (coeficientes)
    let a11 = x1;
    let a12 = x2;
    let a13 = x3;
    let a21 = y1;
    let a22 = y2;
    let a23 = y3;
    let a31 = 1.0;
    let a32 = 1.0;
    let a33 = 1.0;

    // Vector B (resultados)
    let b1 = px;
    let b2 = py;
    let b3 = 1.0;

    // Determinante de A (usando regla de Sarrus)
    let det_a = a11 * (a22 * a33 - a23 * a32)
        - a12 * (a21 * a33 - a23 * a31)
        + a13 * (a21 * a32 - a22 * a31);

    // Si el determinante es casi cero, el triángulo es degenerado
    if det_a.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    // Regla de Cramer: calcular determinantes para cada variable

    // Determinante para α (reemplazar primera columna con B)
    let det_alpha = b1 * (a22 * a33 - a23 * a32)
        - a12 * (b2 * a33 - a23 * b3)
        + a13 * (b2 * a32 - a22 * b3);

    // Determinante para β (reemplazar segunda columna con B)
    let det_beta = a11 * (b2 * a33 - a23 * b3)
        - b1 * (a21 * a33 - a23 * a31)
        + a13 * (a21 * b3 - b2 * a31);

    // Determinante para γ (reemplazar tercera columna con B)
    let det_gamma = a11 * (a22 * b3 - b2 * a32)
        - a12 * (a21 * b3 - b2 * a31)
        + b1 * (a21 * a32 - a22 * a31);

    // Soluciones: dividir cada determinante por det_a
    let alpha = det_alpha / det_a;  // w1
    let beta = det_beta / det_a;    // w2
    let gamma = det_gamma / det_a;  // w3

    (alpha, beta, gamma)
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, light: &Light) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let base_color = Vector3::new(0.5, 0.5, 0.5);
    
    // Calcular bounding box del triángulo
    let min_x = v1.transformed_position.x
        .min(v2.transformed_position.x)
        .min(v3.transformed_position.x)
        .floor() as i32;
    
    let max_x = v1.transformed_position.x
        .max(v2.transformed_position.x)
        .max(v3.transformed_position.x)
        .ceil() as i32;

    let min_y = v1.transformed_position.y
        .min(v2.transformed_position.y)
        .min(v3.transformed_position.y)
        .floor() as i32;
    
    let max_y = v1.transformed_position.y
        .max(v2.transformed_position.y)
        .max(v3.transformed_position.y)
        .ceil() as i32;

    // Iterar sobre todos los píxeles en el bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // Centro del píxel
            let p_x = x as f32 + 0.5;
            let p_y = y as f32 + 0.5;

            // Calcular coordenadas baricéntricas usando Cramer
            let (w1, w2, w3) = barycentric_coordinates(p_x, p_y, v1, v2, v3);

            // Si el punto está dentro del triángulo (todas las coordenadas son no-negativas)
            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {

                // ============================================
                // Interpolación con corrección de perspectiva
                // ============================================
                let w1_div = w1 / v1.w;
                let w2_div = w2 / v2.w;
                let w3_div = w3 / v3.w;

                let denom = w1_div + w2_div + w3_div;
                let w1_corr = w1_div / denom;
                let w2_corr = w2_div / denom;
                let w3_corr = w3_div / denom;

                // Interpolar la normal con corrección de perspectiva
                let interpolated_normal = (v1.transformed_normal * w1_corr)
                    + (v2.transformed_normal * w2_corr)
                    + (v3.transformed_normal * w3_corr);

                let normalized_normal = interpolated_normal.normalized();

                // ============================================
                // Interpolar posición del mundo (3D)
                // ============================================
                let world_pos = Vector3::new(
                    w1 * v1.position.x + w2 * v2.position.x + w3 * v3.position.x,
                    w1 * v1.position.y + w2 * v2.position.y + w3 * v3.position.y,
                    w1 * v1.position.z + w2 * v2.position.z + w3 * v3.position.z,
                );

                // ============================================
                // Cálculo de iluminación
                // ============================================
                let light_dir = Vector3::new(
                    light.position.x - world_pos.x,
                    light.position.y - world_pos.y,
                    light.position.z - world_pos.z,
                );
                
                let light_dir_norm = light_dir.normalized();
                let intensity = normalized_normal.dot(light_dir_norm).max(0.0);

                let shaded_color = Vector3::new(
                    base_color.x * intensity,
                    base_color.y * intensity,
                    base_color.z * intensity,
                );

                // ============================================
                // Interpolar profundidad (Z-buffer)
                // ============================================
                let depth = w1 * v1.transformed_position.z
                          + w2 * v2.transformed_position.z
                          + w3 * v3.transformed_position.z;

                // ============================================
                // Calcular normal de la cara (para shading)
                // ============================================
                let edge1 = v2.position - v1.position;
                let edge2 = v3.position - v1.position;
                let mut face_normal = edge1.cross(edge2);
                face_normal.normalize();

                let light_dir_world = (light.position - world_pos).normalized();
                let face_intensity = face_normal.dot(light_dir_world).max(0.0);

                // ============================================
                // Crear el fragmento
                // ============================================
                fragments.push(Fragment::new(
                    p_x, 
                    p_y, 
                    Vector3::new(face_intensity, face_intensity, face_intensity), 
                    depth, 
                    face_normal,
                    world_pos  // Posición del mundo para shaders
                ));
            }
        }
    }

    fragments
}