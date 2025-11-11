// triangle.rs
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Vector3;
use crate::light::Light;
use crate::Vector2;
use crate::line::line;

pub fn barycentric_coordinates(
    p_x: f32,
    p_y: f32,
    a: &Vertex,
    b: &Vertex,
    c: &Vertex,
) -> (f32, f32, f32) {
    // Convertimos a Vector2 para usar operaciones de Raylib
    let p = Vector2::new(p_x, p_y);
    let a2 = Vector2::new(a.transformed_position.x, a.transformed_position.y);
    let b2 = Vector2::new(b.transformed_position.x, b.transformed_position.y);
    let c2 = Vector2::new(c.transformed_position.x, c.transformed_position.y);

    // Vector2 no tiene cross() directamente, pero podemos implementarlo fácilmente:
    fn cross(v1: Vector2, v2: Vector2) -> f32 {
        v1.x * v2.y - v1.y * v2.x
    }

    // Vectores de los lados del triángulo
    let v0 = b2 - a2;
    let v1 = c2 - a2;
    let v2 = p - a2;

    // Área del triángulo ABC (doble área realmente)
    let denom = cross(v0, v1);

    if denom.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    // Usamos relaciones de área para las barycentrics
    let w1 = cross(v2, v1) / denom;
    let w2 = cross(v0, v2) / denom;
    let w3 = 1.0 - w1 - w2;

    (w1, w2, w3)
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, light: &Light) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let base_color = Vector3::new(0.5, 0.5, 0.5);
    
    let min_x = v1.transformed_position.x.min(v2.transformed_position.x).min(v3.transformed_position.x).floor() as i32;
    let max_x = v1.transformed_position.x.max(v2.transformed_position.x).max(v3.transformed_position.x).ceil() as i32;

    let min_y = v1.transformed_position.y.min(v2.transformed_position.y).min(v3.transformed_position.y).floor() as i32;
    let max_y = v1.transformed_position.y.max(v2.transformed_position.y).max(v3.transformed_position.y).ceil() as i32;

    for y in min_y..= max_y {
        for x in min_x..=max_x {
            let p_x = x as f32 + 0.5;
            let p_y = y as f32 + 0.5;

            let (w1, w2, w3) = barycentric_coordinates(p_x, p_y, v1, v2, v3);

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {

                // Perspectiva corregida para normales:
                let w1_div = w1 / v1.w;
                let w2_div = w2 / v2.w;
                let w3_div = w3 / v3.w;

                let denom = w1_div + w2_div + w3_div;
                let w1_corr = w1_div / denom;
                let w2_corr = w2_div / denom;
                let w3_corr = w3_div / denom;

                let interpolated_normal = (v1.transformed_normal * w1_corr)
                    + (v2.transformed_normal * w2_corr)
                    + (v3.transformed_normal * w3_corr);

                // Normalizar usando la función integrada
                let normalized_normal = interpolated_normal.normalized();

                // 👇 YA TIENES ESTO - Solo falta pasarlo al Fragment
                let world_pos = Vector3::new(
                    w1 * v1.position.x + w2 * v2.position.x + w3 * v3.position.x,
                    w1 * v1.position.y + w2 * v2.position.y + w3 * v3.position.y,
                    w1 * v1.position.z + w2 * v2.position.z + w3 * v3.position.z,
                );

                // Direccion de la luz para este fragmento
                let light_dir = Vector3::new(
                    light.position.x - world_pos.x,
                    light.position.y - world_pos.y,
                    light.position.z - world_pos.z,
                );
                
                // Normalize light direction
                let light_dir_norm = light_dir.normalized();

                // Producto punto entre el normal y la luz
                let intensity = normalized_normal.dot(light_dir_norm).max(0.0);
                
                let shaded_color = Vector3::new(
                    base_color.x * intensity,
                    base_color.y * intensity,
                    base_color.z * intensity,
                );

                let depth = w1 * v1.transformed_position.z
                          + w2 * v2.transformed_position.z
                          + w3 * v3.transformed_position.z;

                let edge1 = v2.position - v1.position;
                let edge2 = v3.position - v1.position;
                let mut face_normal = edge1.cross(edge2);
                face_normal.normalize();

                // Calcula intensidad con face normal
                let light_dir_world = (light.position - world_pos).normalized();
                let face_intensity = face_normal.dot(light_dir_world).max(0.0);

                // 👇 CAMBIO: Agregar world_pos como último parámetro
                fragments.push(Fragment::new(
                    p_x, 
                    p_y, 
                    Vector3::new(face_intensity, face_intensity, face_intensity), 
                    depth, 
                    face_normal,
                    world_pos  // 👈 AQUÍ: pasar la posición del mundo
                ));
            }
        }
    }

    fragments
}