use raylib::prelude::*;
use crate::fragment::{Fragment, FragmentOutput};
use crate::Uniforms;


// CAPA 1: Rayas de colores que cambian cada segundo
pub fn gas_planet_stripes_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    // Paleta de colores que rotarán
    let color_palettes = [
        // Paleta 0: Amarillo y Azul cielo
        (Vector3::new(1.0, 0.95, 0.3), Vector3::new(0.58, 0.77, 0.89)),
        // Paleta 1: Naranja y Morado
        (Vector3::new(1.0, 0.6, 0.2), Vector3::new(0.7, 0.3, 0.9)),
        // Paleta 2: Rosa y Cyan
        (Vector3::new(1.0, 0.4, 0.7), Vector3::new(0.3, 0.9, 0.9)),
        // Paleta 3: Verde lima y Magenta
        (Vector3::new(0.7, 1.0, 0.3), Vector3::new(0.9, 0.2, 0.7)),
        // Paleta 4: Azul y Amarillo dorado
        (Vector3::new(0.2, 0.5, 1.0), Vector3::new(1.0, 0.85, 0.2)),
    ];
    
    // Cambiar de paleta cada segundo
    let palette_index = (uniforms.time as usize) % color_palettes.len();
    let (color1, color2) = color_palettes[palette_index];
    
    // Interpolar suavemente entre paletas durante el cambio
    let transition = (uniforms.time % 1.0).min(0.3) / 0.3; // Transición de 0.3s
    let next_palette_index = (palette_index + 1) % color_palettes.len();
    let (next_color1, next_color2) = color_palettes[next_palette_index];
    
    // Colores interpolados
    let yellow = color1 * (1.0 - transition) + next_color1 * transition;
    let purple = color2 * (1.0 - transition) + next_color2 * transition;
    
    // Patrón de rayas
    let roughness = ((fragment.world_position.x * 8.0 + uniforms.time * 100.0).sin()
        * (fragment.world_position.y * 5.0).cos())
        .abs();
    
    // Mezclar colores
    let color = yellow * (1.0 - roughness) + purple * roughness;
    
    // Iluminación
    let light_dir = Vector3::new(0.0, 0.0, 1.0);
    let intensity = fragment.normal.dot(light_dir).max(0.3);
    
    let ambient = 0.4;
    let diffuse = 0.6;
    let final_intensity = ambient + diffuse * intensity;
    
    let final_color = color * final_intensity;
    
    FragmentOutput {
        color: final_color,
        alpha: 1.0, // Opaco (capa base)
    }
}


// CAPA 2: Nubes 
pub fn gas_planet_clouds_layer(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    // Color de las nubes (blanco con tinte)
    let cloud_light = Vector3::new(1.0, 1.0, 1.0);
    let cloud_shadow = Vector3::new(0.8, 0.85, 0.9);
    
    // Múltiples capas de ruido para nubes realistas
    // Capa 1: Nubes grandes que se mueven lentamente
    let noise1 = ((fragment.world_position.x * 3.0 + uniforms.time * 0.5).sin()
                * (fragment.world_position.y * 3.0 + uniforms.time * 0.3).cos()
                * (fragment.world_position.z * 3.0).sin()).abs();
    
    // Capa 2: Nubes medianas que se mueven más rápido
    let noise2 = ((fragment.world_position.x * 6.0 - uniforms.time * 0.8).cos()
                * (fragment.world_position.y * 5.0 + uniforms.time * 0.5).sin()
                * (fragment.world_position.z * 6.0).cos()).abs();
    
    // Capa 3: Detalles finos
    let noise3 = ((fragment.world_position.x * 12.0 + uniforms.time * 1.2).sin()
                * (fragment.world_position.z * 10.0).cos()).abs();
    
    // Combinar ruidos con diferentes pesos
    let cloud_density = (noise1 * 0.5 + noise2 * 0.9 + noise3 * 0.9).powf(1.5);
    
    // Umbral para crear nubes (ajusta esto para más/menos nubes)
    let cloud_threshold = 0.4;
    let cloud_softness = 0.8;
    
    // Calcular alpha de las nubes con transición suave
    let cloud_alpha = if cloud_density > cloud_threshold {
        let edge_factor = ((cloud_density - cloud_threshold) / cloud_softness).min(1.0);
        edge_factor.powf(0.7) * 0.8 // Máximo 70% de opacidad para que se vean las rayas
    } else {
        0.0
    };
    
    // Color de la nube con variación de sombra
    let cloud_color = cloud_light * (1.0 - noise2 * 0.3) + cloud_shadow * (noise2 * 0.3);
    
    // Iluminación de las nubes
    let light_dir = Vector3::new(0.5, 0.8, 1.0).normalized();
    let intensity = fragment.normal.dot(light_dir).max(0.4);
    
    let final_cloud_color = cloud_color * intensity;
    
    FragmentOutput {
        color: final_cloud_color,
        alpha: cloud_alpha, // Transparente donde no hay nubes
    }
}


// Mezcla rayas + nubes
pub fn combine_gas_planet_layers(
    fragment: &Fragment,
    uniforms: &Uniforms,
) -> FragmentOutput {
    // Obtener cada capa
    let stripes = gas_planet_stripes_layer(fragment, uniforms);
    let clouds = gas_planet_clouds_layer(fragment, uniforms);
    
    // Empezar con la capa de rayas (base)
    let mut final_color = stripes.color;
    
    // Mezclar nubes sobre las rayas usando alpha blending
    // resultado = nubes * alpha + rayas * (1 - alpha)
    final_color = clouds.color * clouds.alpha + final_color * (1.0 - clouds.alpha);
    
    FragmentOutput {
        color: final_color,
        alpha: 1.0, // El resultado final es opaco
    }
}


// SHADER PRINCIPAL
pub fn fragment_shader_gaseoso(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    combine_gas_planet_layers(fragment, uniforms)
}