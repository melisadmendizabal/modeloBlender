use raylib::prelude::*;
use crate::fragment::Fragment; // Asegúrate de que el módulo `fragments` esté correctamente importado

pub fn line(
    start: Vector2,
    end: Vector2,
) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        // Creamos un fragmento en lugar de escribir en el framebuffer
        let fragment = Fragment::new(
            x0 as f32,
            y0 as f32,
            Vector3::new(1.0, 1.0, 1.0), // color blanco
            0.0, // profundidad por defecto
        );
        fragments.push(fragment);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    fragments
}
