use raylib::prelude::*;
use crate::fragment::{Fragment, FragmentOutput};
use crate::Uniforms;

// ============================================
// CAPA 1: Terreno base de Marte con variación geológica
// ============================================
pub fn red_planet_terrain_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    // Paleta de rojos marcianos
    let rust_red = Vector3::new(0.8, 0.3, 0.2);      // Rojo óxido
    let deep_red = Vector3::new(0.6, 0.2, 0.15);     // Rojo oscuro
    let orange_red = Vector3::new(0.85, 0.4, 0.25);  // Naranja rojizo
    let brown_red = Vector3::new(0.5, 0.25, 0.2);    // Marrón rojizo
    
    // Ruido para variación geológica - múltiples capas
    let geology1 = ((fragment.world_position.x * 3.0).sin()
                  * (fragment.world_position.y * 3.0).cos()
                  * (fragment.world_position.z * 3.0).sin()).abs();
    
    let geology2 = ((fragment.world_position.x * 8.0 + 2.0).cos()
                  * (fragment.world_position.z * 7.0).sin()).abs();
    
    let geology3 = ((fragment.world_position.y * 12.0).sin()
                  * (fragment.world_position.x * 10.0).cos()).abs();
    
    // Combinar ruidos para diferentes zonas geológicas
    let region = (geology1 * 0.4 + geology2 * 0.4 + geology3 * 0.2).powf(0.9);
    
    // Mezclar colores según la región
    let mut base_color = if region < 0.3 {
        // Zonas oscuras (antiguos lechos de ríos)
        deep_red * (1.0 - region) + brown_red * region
    } else if region < 0.6 {
        // Zonas medias (llanuras)
        rust_red * (1.0 - (region - 0.3) * 3.3) + orange_red * ((region - 0.3) * 3.3)
    } else {
        // Zonas altas (montañas oxidadas)
        orange_red * (1.0 - (region - 0.6) * 2.5) + rust_red * ((region - 0.6) * 2.5)
    };
    
    // Textura fina de roca
    let rock_detail = ((fragment.world_position.x * 50.0).sin()
                     * (fragment.world_position.y * 50.0).cos()).abs() * 0.08;
    
    base_color = base_color * (1.0 - rock_detail) + rust_red * rock_detail;
    
    // Iluminación con luz cálida (sol lejano)
    let light_dir = Vector3::new(0.6, 0.7, 1.0).normalized();
    let intensity = fragment.normal.dot(light_dir).max(0.15);
    
    // Luz ambiental rojiza
    let ambient = 0.3;
    let diffuse = 0.7;
    let final_intensity = ambient + diffuse * intensity;
    
    FragmentOutput {
        color: base_color * final_intensity,
        alpha: 1.0,
    }
}

// ============================================
// CAPA 2: Tormentas de arena dinámicas
// ============================================
pub fn red_planet_dust_storms_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let dust_color = Vector3::new(0.9, 0.5, 0.35);  // Arena rojiza
    let storm_color = Vector3::new(0.7, 0.35, 0.25); // Tormenta oscura
    
    // Tormentas que se mueven con el tiempo
    let storm_pattern1 = ((fragment.world_position.x * 2.0 - uniforms.time * 0.3).sin()
                        * (fragment.world_position.y * 2.5 + uniforms.time * 0.2).cos()
                        * (fragment.world_position.z * 2.0).sin()).abs();
    
    let storm_pattern2 = ((fragment.world_position.x * 4.0 + uniforms.time * 0.5).cos()
                        * (fragment.world_position.z * 3.5 - uniforms.time * 0.4).sin()).abs();
    
    // Combinar patrones con diferentes intensidades
    let storm_density = (storm_pattern1 * 0.6 + storm_pattern2 * 0.4).powf(2.0);
    
    // Umbrales para crear tormentas dispersas
    let storm_threshold = 0.5;
    let storm_edge = 0.15;
    
    let dust_alpha = if storm_density > storm_threshold {
        let edge_factor = ((storm_density - storm_threshold) / storm_edge).min(1.0);
        edge_factor.powf(0.8) * 0.5 // Máximo 50% opacidad
    } else {
        0.0
    };
    
    // Color de la tormenta varía con la densidad
    let storm_final_color = dust_color * (1.0 - storm_pattern2 * 0.4) 
                          + storm_color * (storm_pattern2 * 0.4);
    
    // Iluminación de las tormentas
    let light_dir = Vector3::new(0.6, 0.7, 1.0).normalized();
    let intensity = fragment.normal.dot(light_dir).max(0.4);
    
    FragmentOutput {
        color: storm_final_color * intensity,
        alpha: dust_alpha,
    }
}

// ============================================
// CAPA 3: Casquetes polares de hielo (CO2 y agua)
// ============================================
pub fn red_planet_ice_caps_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let ice_white = Vector3::new(0.95, 0.95, 0.98);  // Blanco azulado
    let ice_blue = Vector3::new(0.85, 0.90, 0.95);   // Azul claro
    
    // Determinar si estamos en los polos (Y alto o bajo)
    let y_pos = fragment.world_position.y;
    
    // Polo norte (Y > 0.6) y polo sur (Y < -0.6)
    let north_pole_factor = if y_pos > 0.6 {
        ((y_pos - 0.6) / 0.4).min(1.0)
    } else {
        0.0
    };
    
    let south_pole_factor = if y_pos < -0.6 {
        ((y_pos + 0.6).abs() / 0.4).min(1.0)
    } else {
        0.0
    };
    
    let pole_factor = north_pole_factor.max(south_pole_factor);
    
    // Textura del hielo con grietas
    let ice_cracks = ((fragment.world_position.x * 25.0).sin()
                    * (fragment.world_position.z * 25.0).cos()).abs();
    
    let ice_detail = ((fragment.world_position.x * 40.0 + uniforms.time * 0.1).cos()
                    * (fragment.world_position.z * 35.0).sin()).abs();
    
    // Color del hielo con variación
    let ice_color = ice_white * (1.0 - ice_cracks * 0.2) + ice_blue * (ice_cracks * 0.2);
    
    // Bordes irregulares del casquete
    let edge_noise = (fragment.world_position.x * 15.0).sin()
                    * (fragment.world_position.z * 15.0).cos() * 0.5 + 0.5;
    
    let ice_alpha = if pole_factor > 0.1 {
        let smoothed = pole_factor.powf(1.5);
        smoothed * (0.8 + edge_noise * 0.2) // Bordes irregulares
    } else {
        0.0
    };
    
    // Iluminación brillante del hielo
    let light_dir = Vector3::new(0.6, 0.7, 1.0).normalized();
    let intensity = fragment.normal.dot(light_dir).max(0.5);
    
    // Hielo reflectante
    let specular = intensity.powf(8.0) * 0.3;
    
    FragmentOutput {
        color: ice_color * intensity + Vector3::new(1.0, 1.0, 1.0) * specular,
        alpha: ice_alpha.min(0.9), // Máximo 90% para ver algo de terreno debajo
    }
}

// ============================================
// CAPA 4: Venas de minerales brillantes (opcional)
// ============================================
pub fn red_planet_minerals_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let mineral_gold = Vector3::new(1.0, 0.8, 0.3);   // Dorado
    let mineral_cyan = Vector3::new(0.3, 0.8, 0.9);   // Cian (cobre oxidado)
    
    // Patrón de venas minerales
    let vein_x = (fragment.world_position.x * 20.0).floor();
    let vein_y = (fragment.world_position.y * 20.0).floor();
    let vein_z = (fragment.world_position.z * 20.0).floor();
    
    // Pseudo-aleatorio para venas
    let vein_random = ((vein_x * 12.9898 + vein_y * 78.233 + vein_z * 37.719).sin() * 43758.5453).fract();
    
    // Solo algunas celdas tienen minerales
    let has_mineral = vein_random > 0.95;
    
    if !has_mineral {
        return FragmentOutput {
            color: Vector3::zero(),
            alpha: 0.0,
        };
    }
    
    // Distancia a la vena
    let cell_x = (fragment.world_position.x * 20.0) - vein_x;
    let cell_y = (fragment.world_position.y * 20.0) - vein_y;
    let cell_z = (fragment.world_position.z * 20.0) - vein_z;
    
    let dist = (cell_x * cell_x + cell_y * cell_y + cell_z * cell_z).sqrt();
    
    let vein_width = 0.15;
    
    let mineral_alpha = if dist < vein_width {
        ((1.0 - dist / vein_width).powf(2.0)) * 0.6
    } else {
        0.0
    };
    
    // Alternar entre colores según posición
    let mineral_color = if vein_random > 0.975 {
        mineral_cyan
    } else {
        mineral_gold
    };
    
    // Brillo mineral
    let light_dir = Vector3::new(0.6, 0.7, 1.0).normalized();
    let intensity = fragment.normal.dot(light_dir).max(0.3);
    let specular = intensity.powf(16.0);
    
    FragmentOutput {
        color: mineral_color * intensity + Vector3::new(1.0, 1.0, 1.0) * specular,
        alpha: mineral_alpha,
    }
}

// ============================================
// CAPA 5: Brillo atmosférico (resplandor en los bordes)
// ============================================
pub fn red_planet_atmosphere_glow(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let glow_color = Vector3::new(1.0, 0.6, 0.4); // Resplandor anaranjado
    
    // Calcular ángulo de vista (fresnel effect)
    let view_dir = fragment.world_position.normalized();
    let normal = fragment.normal.normalized();
    
    // Producto punto entre normal y dirección de vista
    let facing = view_dir.dot(normal).abs();
    
    // Resplandor en los bordes (1 - facing para invertir)
    let edge_factor = (1.0 - facing).powf(3.0);
    
    // Solo en el borde del planeta
    let glow_alpha = if edge_factor > 0.3 {
        (edge_factor - 0.3) * 0.4 // Máximo 40% opacidad
    } else {
        0.0
    };
    
    FragmentOutput {
        color: glow_color,
        alpha: glow_alpha,
    }
}

// ============================================
// COMBINADOR: Mezcla todas las capas
// ============================================
pub fn combine_red_planet_layers(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    // Obtener todas las capas
    let terrain = red_planet_terrain_layer(fragment, uniforms);
    let storms = red_planet_dust_storms_layer(fragment, uniforms);
    let ice = red_planet_ice_caps_layer(fragment, uniforms);
    let minerals = red_planet_minerals_layer(fragment, uniforms);
    let glow = red_planet_atmosphere_glow(fragment, uniforms);
    
    // Empezar con el terreno base
    let mut final_color = terrain.color;
    
    // Agregar tormentas de arena
    final_color = storms.color * storms.alpha + final_color * (1.0 - storms.alpha);
    
    // Agregar venas minerales (antes del hielo)
    final_color = minerals.color * minerals.alpha + final_color * (1.0 - minerals.alpha);
    
    // Agregar casquetes polares
    final_color = ice.color * ice.alpha + final_color * (1.0 - ice.alpha);
    
    // Agregar resplandor atmosférico al final
    final_color = glow.color * glow.alpha + final_color * (1.0 - glow.alpha);
    
    FragmentOutput {
        color: final_color,
        alpha: 1.0,
    }
}

// ============================================
// SHADER PRINCIPAL: Planeta Rojo Completo 🔴
// ============================================
pub fn fragment_shader_red_planet(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    combine_red_planet_layers(fragment, uniforms)
}

// ============================================
// VARIANTES OPCIONALES
// ============================================

// Solo terreno (sin efectos)
pub fn fragment_shader_red_planet_simple(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    red_planet_terrain_layer(fragment, uniforms)
}

// Con tormentas intensas
pub fn fragment_shader_red_planet_stormy(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let terrain = red_planet_terrain_layer(fragment, uniforms);
    let mut storms = red_planet_dust_storms_layer(fragment, uniforms);
    
    // Tormentas más intensas
    storms.alpha = (storms.alpha * 1.5).min(0.8);
    
    let final_color = storms.color * storms.alpha + terrain.color * (1.0 - storms.alpha);
    
    FragmentOutput {
        color: final_color,
        alpha: 1.0,
    }
}

// Versión nocturna (lado oscuro)
pub fn fragment_shader_red_planet_night(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let mut result = combine_red_planet_layers(fragment, uniforms);
    
    // Oscurecer todo
    result.color = result.color * 0.3;
    
    // Agregar brillo tenue en los bordes
    let glow = red_planet_atmosphere_glow(fragment, uniforms);
    result.color = glow.color * glow.alpha * 0.5 + result.color * (1.0 - glow.alpha * 0.5);
    
    result
}