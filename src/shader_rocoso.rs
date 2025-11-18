use raylib::prelude::*;
use crate::fragment::{Fragment, FragmentOutput};
use crate::Uniforms;

// ============================================
// MÉTODO 1: Tu idea original (3 capas simples)
// ============================================

// CAPA 1: Textura base rugosa (puntillismo)
pub fn crater_base_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    // Colores de roca lunar
    let rock_light = Vector3::new(0.7, 0.7, 0.65);  // Gris claro
    let rock_dark = Vector3::new(0.4, 0.4, 0.35);   // Gris oscuro
    
    // Puntillismo: múltiples capas de ruido fino
    let grain1 = ((fragment.world_position.x * 80.0).sin() 
                * (fragment.world_position.y * 80.0).cos()
                * (fragment.world_position.z * 80.0).sin()).abs();
    
    let grain2 = ((fragment.world_position.x * 120.0 + 5.0).cos()
                * (fragment.world_position.z * 100.0).sin()).abs();
    
    let grain3 = ((fragment.world_position.y * 150.0).sin()
                * (fragment.world_position.z * 130.0).cos()).abs();
    
    // Combinar granos
    let roughness = (grain1 * 0.4 + grain2 * 0.3 + grain3 * 0.3).powf(0.8);
    
    let base_color = rock_light * (1.0 - roughness * 0.4) + rock_dark * (roughness * 0.4);
    
    // Iluminación básica
    let light_dir = Vector3::new(0.5, 0.8, 1.0).normalized();
    let intensity = fragment.normal.dot(light_dir).max(0.2);
    
    FragmentOutput {
        color: base_color * intensity,
        alpha: 1.0,
    }
}



// CAPA 3: Bordes brillantes de cráteres
pub fn crater_rims_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let rim_color = Vector3::new(0.85, 0.85, 0.8); // Gris brillante
    
    // Generar posiciones de cráteres (igual que antes)
    let crater_x = (fragment.world_position.x * 5.0).floor();
    let crater_y = (fragment.world_position.y * 5.0).floor();
    let crater_z = (fragment.world_position.z * 5.0).floor();
    
    let cell_x = (fragment.world_position.x * 5.0) - crater_x;
    let cell_y = (fragment.world_position.y * 5.0) - crater_y;
    let cell_z = (fragment.world_position.z * 5.0) - crater_z;
    
    let center_x = 0.5 + ((crater_x + crater_y).sin() * 0.3);
    let center_y = 0.5 + ((crater_y + crater_z).cos() * 0.3);
    let center_z = 0.5 + ((crater_z + crater_x).sin() * 0.3);
    
    let dist = ((cell_x - center_x).powi(2) 
              + (cell_y - center_y).powi(2)
              + (cell_z - center_z).powi(2)).sqrt();
    
    let crater_size = 0.3 + ((crater_x * crater_y * crater_z).sin().abs() * 0.2);
    
    // Borde brillante: solo en el anillo exterior del cráter
    let rim_inner = crater_size * 0.8;
    let rim_outer = crater_size * 1.0;
    
    let rim_alpha = if dist > rim_inner && dist < rim_outer {
        let rim_factor = 1.0 - ((dist - rim_inner) / (rim_outer - rim_inner));
        rim_factor.powf(3.0) * 0.4 // Máximo 40% opacidad
    } else {
        0.0
    };
    
    FragmentOutput {
        color: rim_color,
        alpha: rim_alpha,
    }
}

// ============================================
// MÉTODO 2: Normal Mapping (MÁS REALISTA)
// ============================================

// Helper: Calcular si hay un cráter y su profundidad
fn crater_depth_at_position(world_pos: Vector3) -> f32 {
    let crater_x = (world_pos.x * 5.0).floor();
    let crater_y = (world_pos.y * 5.0).floor();
    let crater_z = (world_pos.z * 5.0).floor();
    
    let cell_x = (world_pos.x * 5.0) - crater_x;
    let cell_y = (world_pos.y * 5.0) - crater_y;
    let cell_z = (world_pos.z * 5.0) - crater_z;
    
    let center_x = 0.5 + ((crater_x + crater_y).sin() * 0.3);
    let center_y = 0.5 + ((crater_y + crater_z).cos() * 0.3);
    let center_z = 0.5 + ((crater_z + crater_x).sin() * 0.3);
    
    let dist = ((cell_x - center_x).powi(2) 
              + (cell_y - center_y).powi(2)
              + (cell_z - center_z).powi(2)).sqrt();
    
    let crater_size = 0.3 + ((crater_x * crater_y * crater_z).sin().abs() * 0.2);
    
    if dist < crater_size {
        // Perfil parabólico del cráter
        let normalized_dist = dist / crater_size;
        let depth = (1.0 - normalized_dist.powi(2)) * 0.3;
        depth
    } else {
        0.0
    }
}

pub fn crater_planet_normal_mapped(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    // Color base
    let rock_color = Vector3::new(0.6, 0.6, 0.55);
    
    // Calcular normal perturbada (normal mapping)
    let epsilon = 0.01;
    
    // Muestrear profundidad en posiciones vecinas
    let depth_center = crater_depth_at_position(fragment.world_position);
    let depth_x = crater_depth_at_position(Vector3::new(
        fragment.world_position.x + epsilon,
        fragment.world_position.y,
        fragment.world_position.z,
    ));
    let depth_y = crater_depth_at_position(Vector3::new(
        fragment.world_position.x,
        fragment.world_position.y + epsilon,
        fragment.world_position.z,
    ));
    
    // Calcular gradiente (cómo cambia la altura)
    let gradient_x = (depth_x - depth_center) / epsilon;
    let gradient_y = (depth_y - depth_center) / epsilon;
    
    // Perturbar la normal según el gradiente
    let mut perturbed_normal = fragment.normal;
    perturbed_normal.x -= gradient_x * 5.0;
    perturbed_normal.y -= gradient_y * 5.0;
    perturbed_normal = perturbed_normal.normalized();
    
    // Iluminación con la normal perturbada
    let light_dir = Vector3::new(0.5, 0.8, 1.0).normalized();
    let intensity = perturbed_normal.dot(light_dir).max(0.15);
    
    // Agregar oclusión ambiental en los cráteres
    let ao = 1.0 - (depth_center * 0.7);
    
    // Textura fina
    let grain = ((fragment.world_position.x * 100.0).sin() 
               * (fragment.world_position.y * 100.0).cos()).abs() * 0.1;
    
    let final_color = rock_color * intensity * ao * (1.0 - grain);
    
    FragmentOutput {
        color: final_color,
        alpha: 1.0,
    }
}

// ============================================
// COMBINADORES
// ============================================




// Versión 3: Híbrida (lo mejor de ambos mundos)
pub fn fragment_shader_crater_hybrid(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    // Base con normal mapping
    let base = crater_planet_normal_mapped(fragment, uniforms);
    
    // Agregar bordes brillantes adicionales
    let rims = crater_rims_layer(fragment, uniforms);
    
    let final_color = rims.color * rims.alpha * 0.5 + base.color * (1.0 - rims.alpha * 0.5);
    
    FragmentOutput {
        color: final_color,
        alpha: 1.0,
    }
}