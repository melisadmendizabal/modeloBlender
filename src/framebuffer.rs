// framebuffer.rs
use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub image: Image,
    pub background_color: Vector3,
    pub texture:Option<Texture2D>,
    pub depth_buffer: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let image = Image::gen_image_color(width as i32, height as i32, Color::WHITE); // Un color por defecto
        let buffe_size = (width * height) as usize;
        let depth_buffer = vec![f32::INFINITY; buffe_size];
        Framebuffer {
            width,
            height,
            image,
            background_color: Vector3::zero(),
            texture: None,
            depth_buffer,
        }
    }

    pub fn init_texture(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.texture = Some(rl.load_texture_from_image(thread, &self.image). unwrap());  
    }

    pub fn clear(&mut self) {
        let bg_color = Color::new(
            (self.background_color.x * 205.0) as u8,
            (self.background_color.y * 205.0) as u8,
            (self.background_color.z * 205.0) as u8,
            255,
        );

        self.image.clear_background(bg_color);

        self.depth_buffer.fill(f32::INFINITY);
    }

    
    pub fn point(&mut self, x: i32, y: i32, color: Vector3, depth: f32, alpha: f32) -> bool {
        if x >= 0 && y >= 0 && x < self.width  as i32 && y < self.height  as i32{
            let index = (y * self.width as i32 + x) as usize;

            //if depth < self.depth_buffer[index] {
                 // Obtener el color previo del pixel
                let colors = self.image.get_image_data();

                let old_color_ray = colors[index];
                let old_color = Vector3::new(
                    old_color_ray.r as f32 / 255.0,
                    old_color_ray.g as f32 / 255.0,
                    old_color_ray.b as f32 / 255.0,
                );

                // Mezclar el color previo con el nuevo
                let blended = old_color * (1.0 - alpha) + color * alpha;

                // Guardar el color mezclado en la imagen
                let pixel_color = Color::new(
                    (blended.x.clamp(0.0, 1.0) * 255.0) as u8,
                    (blended.y.clamp(0.0, 1.0) * 255.0) as u8,
                    (blended.z.clamp(0.0, 1.0) * 255.0) as u8,
                    255,
                );

                self.image.draw_pixel(x, y, pixel_color);
                self.depth_buffer[index] = depth;

                return true;
           // }
        }
        false
    }

     pub fn set_background_color(&mut self, color: Vector3) {
        self.background_color = color;
    }

    pub fn swap_buffers(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if let Some(texture) = &mut self.texture {
            let colors = self.image.get_image_data();

            let data: &[u8] = unsafe{
                std::slice::from_raw_parts(
                    colors.as_ptr() as * const u8,
                    colors.len() * 4,

                )
            };

            texture.update_texture(data).unwrap();

            let mut d = rl.begin_drawing(thread);
            d.clear_background(Color::WHITE);
            d.draw_texture(texture,0, 0, Color::WHITE);

        } else {
            panic!("No hay textura ")
        }
    }  
    
}