// framebuffer.rs - VERSIÓN CORREGIDA
use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub image: Image,
    pub background_color: Vector3,
    pub texture: Option<Texture2D>,
    pub depth_buffer: Vec<f32>,
    pub buffer: Vec<Vector3>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let image = Image::gen_image_color(width as i32, height as i32, Color::WHITE);
        let buffer_size = (width * height) as usize;

        // ✅ CORRECCIÓN: Usar f32::INFINITY (infinito positivo)
        // Esto significa "todos los pixels empiezan infinitamente lejos"
        let depth_buffer = vec![f32::INFINITY; buffer_size];
        let buffer = vec![Vector3::zero(); buffer_size];
        
        Framebuffer {
            width,
            height,
            image,
            background_color: Vector3::zero(),
            texture: None,
            depth_buffer,
            buffer,
        }
    }

    pub fn init_texture(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.texture = Some(rl.load_texture_from_image(thread, &self.image).unwrap());  
    }

    pub fn clear(&mut self) {
        let bg_color = Color::new(
            (self.background_color.x * 255.0) as u8,
            (self.background_color.y * 255.0) as u8,
            (self.background_color.z * 255.0) as u8,
            255,
        );

        self.image.clear_background(bg_color);
        
        // ✅ CORRECCIÓN: Resetear a INFINITY (lejos = infinito)
        self.depth_buffer.fill(f32::INFINITY);
        
        for pixel in &mut self.buffer {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: i32, y: i32, color: Vector3, depth: f32, alpha: f32) -> bool {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            let index = (y * self.width as i32 + x) as usize;

            // ✅ CORRECCIÓN: DEPTH TEST CORRECTO
            // Solo dibuja si el nuevo fragmento está MÁS CERCA (depth menor)
            if depth < self.depth_buffer[index] {
                // Solo actualizar depth buffer si el objeto es opaco
                if alpha >= 0.99 {
                    self.depth_buffer[index] = depth;
                }

                let old_color = self.buffer[index];
                let blended = if alpha >= 0.99 {
                    color
                } else {
                    old_color * (1.0 - alpha) + color * alpha
                };

                self.buffer[index] = blended;

                return true;
            }
        }
        false
    }

    pub fn set_background_color(&mut self, color: Vector3) {
        self.background_color = color;
    }

    pub fn swap_buffers(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                let color = self.buffer[index];
                
                let pixel_color = Color::new(
                    (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.z.clamp(0.0, 1.0) * 255.0) as u8,
                    255,
                );
                
                self.image.draw_pixel(x as i32, y as i32, pixel_color);
            }
        }

        if let Some(texture) = &mut self.texture {
            let colors = self.image.get_image_data();

            let data: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    colors.as_ptr() as *const u8,
                    colors.len() * 4,
                )
            };

            texture.update_texture(data).unwrap();

            let mut d = rl.begin_drawing(thread);
            d.clear_background(Color::WHITE);
            d.draw_texture(texture, 0, 0, Color::WHITE);
        } else {
            panic!("No hay textura")
        }
    }  
}