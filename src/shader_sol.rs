// ============================================
// SHADER DE ESTRELLA / SOL ⭐☀️
// Implementa múltiples tipos de ruido y efectos solares
// ============================================

use raylib::prelude::*;
use crate::fragment::{Fragment, FragmentOutput};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::matrix::multiply_matrix_vector4;

// ============================================
// FUNCIONES DE RUIDO
// ============================================

/// Función hash para generar valores pseudo-aleatorios
/// Usada como base para los ruidos
fn hash(p: Vector2) -> f32 {
    let p3 = Vector3::new(p.x, p.y, p.x).fract_element();
    let p3 = Vector3::new(
        p3.x + (p3.y * p3.z) * 17.0,
        p3.y + (p3.z * p3.x) * 17.0,
        p3.z + (p3.x * p3.y) * 17.0,
    );
    ((p3.x + p3.y) * p3.z).fract()
}

/// Extensión de Vector3 para operación fract por elemento
trait Vector3Ext {
    fn fract_element(&self) -> Vector3;
}

impl Vector3Ext for Vector3 {
    fn fract_element(&self) -> Vector3 {
        Vector3::new(
            self.x - self.x.floor(),
            self.y - self.y.floor(),
            self.z - self.z.floor(),
        )
    }
}

/// PERLIN NOISE (suave y orgánico)
/// Perfecto para: turbulencias generales, movimiento de plasma
fn perlin_noise(p: Vector2) -> f32 {
    let i = Vector2::new(p.x.floor(), p.y.floor());
    let f = Vector2::new(p.x - i.x, p.y - i.y);
    
    // Interpolación suave (smoothstep)
    let u = Vector2::new(
        f.x * f.x * (3.0 - 2.0 * f.x),
        f.y * f.y * (3.0 - 2.0 * f.y),
    );
    
    // Mezclar valores de las esquinas
    let a = hash(i);
    let b = hash(Vector2::new(i.x + 1.0, i.y));
    let c = hash(Vector2::new(i.x, i.y + 1.0));
    let d = hash(Vector2::new(i.x + 1.0, i.y + 1.0));
    
    // Interpolación bilinear
    let mix1 = a * (1.0 - u.x) + b * u.x;
    let mix2 = c * (1.0 - u.x) + d * u.x;
    mix1 * (1.0 - u.y) + mix2 * u.y
}

/// SIMPLEX-LIKE NOISE (más detallado y eficiente)
/// Perfecto para: detalles finos, granularidad de superficie
fn simplex_noise(p: Vector2) -> f32 {
    // Simplificación de Simplex usando ruido de valor
    let skew = 0.366025404; // (sqrt(3)-1)/2
    let unskew = 0.211324865; // (3-sqrt(3))/6
    
    let s = (p.x + p.y) * skew;
    let i = Vector2::new((p.x + s).floor(), (p.y + s).floor());
    
    let t = (i.x + i.y) * unskew;
    let x0 = p.x - (i.x - t);
    let y0 = p.y - (i.y - t);
    
    let n0 = hash(i);
    let n1 = hash(Vector2::new(i.x + 1.0, i.y + 1.0));
    
    let dist = x0 * x0 + y0 * y0;
    let contribution = (1.0 - dist).max(0.0).powi(4);
    
    n0 * contribution + n1 * (1.0 - contribution)
}

/// CELLULAR NOISE / WORLEY NOISE (patrón celular)
/// Perfecto para: manchas solares, regiones de actividad magnética
fn cellular_noise(p: Vector2, scale: f32) -> f32 {
    let scaled_p = Vector2::new(p.x * scale, p.y * scale);
    let cell = Vector2::new(scaled_p.x.floor(), scaled_p.y.floor());
    
    let mut min_dist = f32::MAX;
    
    // Buscar en celdas vecinas
    for i in -1..=1 {
        for j in -1..=1 {
            let neighbor = Vector2::new(cell.x + i as f32, cell.y + j as f32);
            
            // Punto aleatorio dentro de la celda
            let point = Vector2::new(
                neighbor.x + hash(neighbor),
                neighbor.y + hash(Vector2::new(neighbor.y, neighbor.x)),
            );
            
            // Distancia al punto
            let diff = Vector2::new(point.x - scaled_p.x, point.y - scaled_p.y);
            let dist = (diff.x * diff.x + diff.y * diff.y).sqrt();
            
            min_dist = min_dist.min(dist);
        }
    }
    
    min_dist
}

/// FRACTAL BROWNIAN MOTION (FBM) - Combinación de múltiples octavas de ruido
/// Crea patrones naturales complejos
fn fbm_perlin(p: Vector2, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        let sample_p = Vector2::new(p.x * frequency, p.y * frequency);
        value += perlin_noise(sample_p) * amplitude;
        
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    
    value
}

// ============================================
// VERTEX SHADER: Distorsión de superficie
// ============================================

/// Vertex shader que distorsiona la superficie de la estrella
/// Simula turbulencias y llamaradas solares
pub fn vertex_shader_star(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let mut position = vertex.position;
    
    // Coordenadas esféricas para ruido uniforme
    let theta = position.z.atan2(position.x);
    let phi = (position.y / (position.x * position.x + position.y * position.y + position.z * position.z).sqrt()).asin();
    
    // Ruido base para turbulencias
    let noise_coord = Vector2::new(
        theta * 2.0 + uniforms.time * 0.1,
        phi * 2.0 + uniforms.time * 0.15,
    );
    
    let turbulence = fbm_perlin(noise_coord, 4) * 0.1;
    
    // Llamaradas solares (picos ocasionales)
    let flare_coord = Vector2::new(theta * 3.0, phi * 3.0 + uniforms.time * 0.5);
    let flare = perlin_noise(flare_coord).powf(4.0) * 0.15;
    
    // Distorsionar superficie
    let distortion = turbulence + flare;
    let scale = 1.0 + distortion;
    
    position.x *= scale;
    position.y *= scale;
    position.z *= scale;
    
    // Aplicar transformaciones estándar
    let position_vec4 = Vector4::new(position.x, position.y, position.z, 1.0);
    let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);
    let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);
    let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);
    let clip_w = clip_position.w;
    
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
    
    // Normal distorsionada
    let mut normal = vertex.normal;
    normal.x += turbulence * 0.5;
    normal.y += flare * 0.5;
    normal = normal.normalized();
    
    Vertex {
        position,
        normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: normal,
        w: clip_w,
    }
}

// ============================================
// GRADIENTE DE TEMPERATURA
// ============================================

/// Convierte temperatura (en factor 0-1) a color
/// Simula el espectro de radiación de cuerpo negro
fn temperature_to_color(temp: f32) -> Vector3 {
    // temp: 0.0 = frío (rojo oscuro), 1.0 = muy caliente (blanco-azul)
    
    if temp < 0.2 {
        // Zonas frías: rojo oscuro
        let t = temp / 0.2;
        Vector3::new(0.1 + t * 0.5, 0.0, 0.0)
    } else if temp < 0.4 {
        // Rojo-naranja
        let t = (temp - 0.2) / 0.2;
        Vector3::new(0.8 + t * 0.2, t * 0.3, 0.0)
    } else if temp < 0.6 {
        // Naranja-amarillo
        let t = (temp - 0.4) / 0.2;
        Vector3::new(1.0, 0.3 + t * 0.5, t * 0.2)
    } else if temp < 0.8 {
        // Amarillo-blanco
        let t = (temp - 0.6) / 0.2;
        Vector3::new(1.0, 0.8 + t * 0.2, 0.2 + t * 0.6)
    } else {
        // Blanco-azul (zonas más calientes)
        let t = (temp - 0.8) / 0.2;
        Vector3::new(1.0, 1.0, 0.8 + t * 0.2)
    }
}

// ============================================
// FRAGMENT SHADER PRINCIPAL: Superficie de la estrella
// ============================================

pub fn fragment_shader_star(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    
    let pos = fragment.world_position;
    
    // Coordenadas esféricas para muestreo de ruido
    let theta = pos.z.atan2(pos.x);
    let phi = (pos.y / (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt()).asin();
    
    // ============================================
    // CAPA 1: Temperatura base (Perlin Noise)
    // ============================================
    let base_coord = Vector2::new(
        theta * 2.0 + uniforms.time * 0.05,
        phi * 2.0 + uniforms.time * 0.08,
    );
    let base_temp = fbm_perlin(base_coord, 5) * 0.5 + 0.5; // [0, 1]
    
    // ============================================
    // CAPA 2: Granulación solar (Simplex Noise)
    // ============================================
    let grain_coord = Vector2::new(
        theta * 15.0 + uniforms.time * 0.3,
        phi * 15.0 - uniforms.time * 0.2,
    );
    let granulation = simplex_noise(grain_coord) * 0.15;
    
    // ============================================
    // CAPA 3: Manchas solares (Cellular Noise)
    // ============================================
    let spot_coord = Vector2::new(
        theta + uniforms.time * 0.02,
        phi - uniforms.time * 0.03,
    );
    let sunspots = cellular_noise(spot_coord, 3.0);
    let spot_factor = (1.0 - sunspots * 2.0).max(0.0).powf(3.0) * 0.3;
    
    // ============================================
    // CAPA 4: Llamaradas solares (Perlin animado)
    // ============================================
    let flare_coord = Vector2::new(
        theta * 4.0,
        phi * 4.0 + uniforms.time * 0.4,
    );
    let flare_intensity = perlin_noise(flare_coord).powf(6.0) * 2.0;
    
    // ============================================
    // CAPA 5: Turbulencias (FBM de alta frecuencia)
    // ============================================
    let turb_coord = Vector2::new(
        theta * 8.0 + uniforms.time * 0.6,
        phi * 8.0 - uniforms.time * 0.5,
    );
    let turbulence = fbm_perlin(turb_coord, 3) * 0.2;
    
    // ============================================
    // COMBINAR TODAS LAS CAPAS
    // ============================================
    let mut temperature = base_temp;
    temperature += granulation;
    temperature -= spot_factor; // Manchas son más frías
    temperature += flare_intensity;
    temperature += turbulence;
    temperature = temperature.clamp(0.0, 1.0);
    
    // ============================================
    // EMISIÓN VARIABLE (pulsaciones)
    // ============================================
    let pulse = (uniforms.time * 2.0).sin() * 0.1 + 0.9; // [0.8, 1.0]
    let emission_boost = 1.0 + flare_intensity * 3.0; // Llamaradas emiten más
    
    // ============================================
    // COLOR FINAL basado en temperatura
    // ============================================
    let base_color = temperature_to_color(temperature);
    
    // Aplicar emisión
    let emissive_color = base_color * pulse * emission_boost;
    
    // Agregar brillo especular en zonas calientes
    let hot_spots = (temperature - 0.7).max(0.0) * 3.0;
    let final_color = emissive_color + Vector3::new(hot_spots, hot_spots, hot_spots);
    
    // ============================================
    // EFECTO FRESNEL (brillo en los bordes)
    // ============================================
    let view_dir = pos.normalized();
    let normal = fragment.normal.normalized();
    let fresnel = (1.0 - view_dir.dot(normal).abs()).powf(3.0);
    let fresnel_color = Vector3::new(1.0, 0.9, 0.6) * fresnel * 0.5;
    
    let final_with_fresnel = final_color + fresnel_color;
    
    FragmentOutput {
        color: final_with_fresnel,
        alpha: 1.0,
    }
}





/// Estrella con llamaradas extremas
pub fn fragment_shader_star_flares(fragment: &Fragment, uniforms: &Uniforms) -> FragmentOutput {
    let mut result = fragment_shader_star(fragment, uniforms);
    
    result.color.x *= 0.8;
    result.color.y *= 0.9;
    result.color.z *= 1.3;
    // Intensificar emisión
    result.color = result.color * 1.5;
    
    result
}