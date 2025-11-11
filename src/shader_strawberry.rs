//shader_strawberry.rs
use raylib::prelude::*;
use crate::fragment::{Fragment, FragmentOutput};
use crate::Uniforms;


// CAPA 1: Base roja grumosa 
pub fn strawberry_base_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    // Colores de la fresa
    let red_light = Vector3::new(0.93, 0.37, 0.35);  // Rojo claro
    let red_dark = Vector3::new(1.0, 0.6, 0.6);   // Rojo oscuro
    
    // Textura grumosa usando múltiples frecuencias de ruido
    let noise1 = ((fragment.world_position.x * 15.0).sin() 
                * (fragment.world_position.y * 30.0).cos() 
                * (fragment.world_position.z * 15.0).sin()).abs();
    
    let noise2 = ((fragment.world_position.x * 25.0 + 1.5).cos() 
                * (fragment.world_position.y * 20.0 + 2.0).sin()).abs();
    
    let noise3 = ((fragment.world_position.x * 35.0).sin() 
                * (fragment.world_position.z * 30.0).cos()).abs();
    
    // Combinar ruidos para textura grumosa
    let grumpy_factor = (noise1 * 0.5 + noise2 * 0.3 + noise3 * 0.2).powf(0.8);
    
    // Mezclar colores según la textura
    let base_color = red_light * (1.0 - grumpy_factor * 0.4) + red_dark * (grumpy_factor * 0.4);
    
    // Iluminación
    let light_dir = Vector3::new(0.5, 0.8, 1.0).normalized();
    let intensity = fragment.normal.dot(light_dir).max(0.2); // Luz ambiental mínima
    
    FragmentOutput {
        color: base_color * intensity,
        alpha: 1.0, // Opaco (capa base)
    }
}


// CAPA 2: Semillas (puntos amarillos/dorados)
pub fn strawberry_seeds_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let seed_color = Vector3::new(0.95, 0.85, 0.3); // Amarillo dorado
    
    // Crear patrón de semillas usando función de distancia a puntos
    // Usamos coordenadas esféricas para distribuir las semillas uniformemente
    
    // Convertir a coordenadas esféricas aproximadas
    let radius = (fragment.world_position.x * fragment.world_position.x 
                + fragment.world_position.y * fragment.world_position.y 
                + fragment.world_position.z * fragment.world_position.z).sqrt();
    
    // Grid de semillas
    let seed_grid_x = (fragment.world_position.x * 12.0).floor();
    let seed_grid_y = (fragment.world_position.y * 12.0).floor();
    let seed_grid_z = (fragment.world_position.z * 12.0).floor();
    
    // Posición relativa en cada celda del grid
    let cell_x = (fragment.world_position.x * 12.0) - seed_grid_x;
    let cell_y = (fragment.world_position.y * 12.0) - seed_grid_y;
    let cell_z = (fragment.world_position.z * 12.0) - seed_grid_z;
    
    // Centro de la semilla en cada celda (con un poco de variación)
    let seed_center_x = 0.5 + ((seed_grid_x + seed_grid_y).sin() * 0.2);
    let seed_center_y = 0.5 + ((seed_grid_y + seed_grid_z).cos() * 0.2);
    let seed_center_z = 0.5 + ((seed_grid_z + seed_grid_x).sin() * 0.2);
    
    // Distancia al centro de la semilla
    let dist_to_seed = ((cell_x - seed_center_x) * (cell_x - seed_center_x) 
                      + (cell_y - seed_center_y) * (cell_y - seed_center_y)
                      + (cell_z - seed_center_z) * (cell_z - seed_center_z)).sqrt();
    
    // Tamaño de las semillas
    let seed_size = 0.4;
    
    // Determinar si estamos en una semilla
    let is_seed = if dist_to_seed < seed_size {
        // Suavizar los bordes de la semilla
        let edge_factor = (1.0 - (dist_to_seed / seed_size)).powf(2.0);
        edge_factor
    } else {
        0.0
    };
    
    // Iluminación de las semillas
    let light_dir = Vector3::new(0.5, 0.8, 1.0).normalized();
    let intensity = fragment.normal.dot(light_dir).max(0.3);
    
    FragmentOutput {
        color: seed_color * intensity,
        alpha: is_seed, // Transparente donde no hay semillas
    }
}


// CAPA 3: Hoja verde (solo en el tercio superior)
pub fn strawberry_leaf_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let leaf_green = Vector3::new(0.33, 0.83, 0.37);  // Verde hoja
    let leaf_dark = Vector3::new(0.07, 0.51, 0.11);  // Verde oscuro
    
    // Determinar si estamos en el tercio superior (donde va la hoja)
    // Usamos la coordenada Y del mundo
    let y_normalized = fragment.world_position.y; // Asumiendo esfera centrada en origen
    
    // La hoja solo aparece cuando Y > 0.4 (tercio superior ajustable)
    let leaf_threshold = 0.25;
    let leaf_transition = 0.1; // Zona de transición suave
    
    // Calcular alpha basado en la altura
    let height_factor = if y_normalized > leaf_threshold {
        // Dentro de la zona de la hoja
        ((y_normalized - leaf_threshold) / leaf_transition).min(1.0)
    } else {
        0.0 // Fuera de la zona de la hoja = transparente
    };
    
    // Textura de venas de la hoja
    let vein_pattern = ((fragment.world_position.x * 20.0).sin() 
                      * (fragment.world_position.z * 20.0).cos()).abs();
    
    // Textura de bordes irregulares
    let edge_noise = (fragment.world_position.x * 30.0 + fragment.world_position.z * 25.0).sin() * 0.5 + 0.5;
    
    // Color de la hoja con venas
    let leaf_color = leaf_green * (1.0 - vein_pattern * 0.3) + leaf_dark * (vein_pattern * 0.3);
    
    // Hacer los bordes más irregulares
    let edge_alpha = if y_normalized > leaf_threshold + 0.05 {
        (edge_noise * 0.5 + 0.5) * height_factor
    } else {
        height_factor * 0.5 // Bordes más suaves en la transición
    };
    
    // Iluminación
    let light_dir = Vector3::new(0.5, 0.8, 1.0).normalized();
    let intensity = fragment.normal.dot(light_dir).max(0.25);
    
    FragmentOutput {
        color: leaf_color * intensity,
        alpha: edge_alpha.min(1.0), // Transparente fuera del tercio superior
    }
}


//  Mezcla las 3 capas con alpha blending
pub fn combine_strawberry_layers(
    fragment: &Fragment,
    uniforms: &Uniforms,
) -> FragmentOutput {
    // Obtener cada capa
    let base = strawberry_base_layer(fragment, uniforms);
    let seeds = strawberry_seeds_layer(fragment, uniforms);
    let leaf = strawberry_leaf_layer(fragment, uniforms);
    
    // Empezar con la capa base (opaca)
    let mut final_color = base.color;
    let mut final_alpha = base.alpha;
    
    // Mezclar semillas sobre la base
    // Alpha blending: resultado = src * alpha + dst * (1 - alpha)
    final_color = seeds.color * seeds.alpha + final_color * (1.0 - seeds.alpha);
    
    // Mezclar hoja sobre el resultado anterior
    final_color = leaf.color * leaf.alpha + final_color * (1.0 - leaf.alpha);
    
    FragmentOutput {
        color: final_color,
        alpha: 1.0, // El resultado final es opaco
    }
}


// SHADER PRINCIPAL: Fresita completa 🍓
pub fn fragment_shader_strawberry(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    combine_strawberry_layers(fragment, uniforms)
}

