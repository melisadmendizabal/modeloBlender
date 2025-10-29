// triangle.rs
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Vector3;
use crate::light::Light;
use crate::Vector2;
use crate::line::line;
//use raylib::prelude::*;

// fn barycentric_coordinates(p_x: f32, p_y: f32, a: &Vertex, b: &Vertex, c: &Vertex)  -> (f32, f32, f32) {
//     let a_x = a.transformed_position.x;   
//     let a_y = a.transformed_position.y;

//     let b_x = b.transformed_position.x;
//     let b_y = b.transformed_position.y;

//     let c_x = c.transformed_position.x;
//     let c_y = c.transformed_position.y;
    
//     let denom = (b_y - c_y) * (a_x - c_x) * (c_x - b_x) * (a_y - c_y);

//     if denom.abs() < 1e-10 {
//         return (-1.0, -1.0, -1.0);
//     }

//     let w1 = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / denom;
//     let w2 = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / denom;
//     let w3 = 1.0 - w1 -w2;

//     (w1, w2, w3)
//     //bran
//     // let area = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);

//     // if area.abs() < 1e-10  {
//     //     return (-1.0, -1.0, -1.0);
//     // }
    
//     // let w = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / area;
//     // let v = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / area;
//     // let u = 1.0 - w - v;

//     // (w, v, u)
// }



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
    //AAAAAAAAAAAAAAAAAAAAAA
    // let a_x = v1.transformed_position.x;
    // let b_x = v2.transformed_position.x;
    // let c_x = v3.transformed_position.x;
    // let a_y = v1.transformed_position.y;
    // let b_y = v2.transformed_position.y;
    // let c_y = v3.transformed_position.y;

    // fragments.extend(line(Vector3::new(a_x, a_y, v1.transformed_position.z), Vector3::new(b_x, b_y, v2.transformed_position.z)));
    // fragments.extend(line(Vector3::new(b_x, b_y, v2.transformed_position.z), Vector3::new(c_x, c_y, v3.transformed_position.z)));
    // fragments.extend(line(Vector3::new(c_x, c_y, v3.transformed_position.z), Vector3::new(a_x, a_y, v1.transformed_position.z)));

    // fragments.extend(line(Vector2::new(a_x, a_y), Vector2::new(b_x, b_y)));
    // fragments.extend(line(Vector2::new(b_x, b_y), Vector2::new(c_x, c_y)));
    // fragments.extend(line(Vector2::new(c_x, c_y), Vector2::new(a_x, a_y)));
   
   

/*     let color_a = Vector3::new(1.0, 0.0, 0.0);
    let color_b = Vector3::new(0.0, 1.0, 0.0);
    let color_c = Vector3::new(0.0, 0.0, 1.0); */

    // let min_x = a_x.min(b_x).min(c_x).floor() as i32;
    // let min_y = a_y.min(b_y).min(c_y).floor() as i32;

    // let max_x = a_x.max(b_x).max(c_x).ceil() as i32;
    // let max_y = a_y.max(b_y).max(c_y).ceil() as i32;

    for y in min_y..= max_y {
        for x in min_x..=max_x {
            let p_x = x as f32 + 0.5;
            let p_y = y as f32 + 0.5;

            let (w1, w2, w3) = barycentric_coordinates(p_x, p_y, v1, v2, v3);

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {

                // perspectiva corregida para normales:
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

                // let interpolated_normal = (v1.transformed_normal * w1)
                //     + (v2.transformed_normal * w2)
                //     + (v3.transformed_normal * w3);

                // Normalizar usando la función integrada
                let normalized_normal = interpolated_normal.normalized();

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
                
                //normalize light direcction
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

                // calcula intensidad con face normal
                let light_dir_world = (light.position - world_pos).normalized();
                let face_intensity = face_normal.dot(light_dir_world).max(0.0);

                fragments.push(Fragment::new(p_x, p_y, Vector3::new(face_intensity, face_intensity, face_intensity), depth, face_normal));

            }

        }
    }

    // let light = light.position.normalized();

    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         let (w, v, u) = barycentric_coordinates(x  as f32, y as f32, v1, v2, v3);

    //         let normal = v1.transformed_normal;
    //         let depth = v1.transformed_position.z * w + v2.transformed_position.z * v + v3.transformed_position.z * u;
    //         let color = Vector3::new(1.0, 0.5, 0.5);
    //         //let color = color_a * w + color_b * v + color_c * u;

    //         let intensity = normal.dot(light).max(0.0);

    //         let final_color = color * intensity;

    //         if w >= 0.0 && v >= 0.0 && u >= 0.0 {
    //             fragments.push(Fragment::new(
    //                 x as f32,
    //                 y as f32,
    //                 final_color,
    //                 depth,
    //             ));
    //         }
    //     }
    // }

    fragments
}