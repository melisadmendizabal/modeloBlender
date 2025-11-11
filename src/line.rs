// line.rs
use raylib::prelude::*;
use crate::fragment::Fragment;

pub fn line(
    start: Vector3, // x,y,z  -> usar Vector3 para incluir depth
    end: Vector3,
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

    // total distance (for depth interpolation)
    let dist = (((x1 - x0) as f32).hypot((y1 - y0) as f32)).max(1.0);

    // We'll track a step counter to compute t in [0,1] to lerp z
    let mut steps = 0f32;

    loop {
        let t = steps / dist;
        let depth = start.z * (1.0 - t) + end.z * t;

        let fragment = Fragment::new(
            x0 as f32,
            y0 as f32,
            Vector3::new(1.0, 1.0, 1.0),
            depth,
            Vector3::new(0.0, 0.0, 1.0), // default normal for lines (or pass better)
            Vector3::zero(),
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

        steps += 1.0;
    }

    fragments
}
