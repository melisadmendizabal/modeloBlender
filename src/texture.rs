// texture.rs
use image::{DynamicImage, GenericImageView, Rgba};
use raylib::math::Vector3;

pub struct Texture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Texture {
    /// Carga una imagen desde un archivo
    pub fn load(path: &str) -> Result<Self, String> {
        let img = image::open(path)
            .map_err(|e| format!("Error al cargar {}: {}", path, e))?;
        
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();
        
        println!("✅ Textura cargada: {} ({}x{})", path, width, height);
        
        Ok(Texture {
            data: rgba_img.into_raw(),
            width,
            height,
        })
    }

    /// Samplea un color en coordenadas UV [0,1]
    pub fn sample(&self, u: f32, v: f32) -> Vector3 {
        // Wrap coordinates (repetir textura)
        let u = u.fract();
        let v = v.fract();
        
        // Convertir UV a coordenadas de píxel
        let x = ((u * self.width as f32) as u32).min(self.width - 1);
        let y = ((v * self.height as f32) as u32).min(self.height - 1);
        
        let index = ((y * self.width + x) * 4) as usize;
        
        if index + 2 < self.data.len() {
            Vector3::new(
                self.data[index] as f32 / 255.0,
                self.data[index + 1] as f32 / 255.0,
                self.data[index + 2] as f32 / 255.0,
            )
        } else {
            Vector3::new(1.0, 0.0, 1.0) // Magenta = error
        }
    }

    /// Samplea con filtrado bilinear (más suave)
    pub fn sample_bilinear(&self, u: f32, v: f32) -> Vector3 {
        let u = u.fract();
        let v = v.fract();
        
        let x = u * (self.width - 1) as f32;
        let y = v * (self.height - 1) as f32;
        
        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);
        
        let fx = x.fract();
        let fy = y.fract();
        
        let c00 = self.get_pixel(x0, y0);
        let c10 = self.get_pixel(x1, y0);
        let c01 = self.get_pixel(x0, y1);
        let c11 = self.get_pixel(x1, y1);
        
        // Interpolación bilinear
        let c0 = c00 * (1.0 - fx) + c10 * fx;
        let c1 = c01 * (1.0 - fx) + c11 * fx;
        
        c0 * (1.0 - fy) + c1 * fy
    }
    
    fn get_pixel(&self, x: u32, y: u32) -> Vector3 {
        let index = ((y * self.width + x) * 4) as usize;
        Vector3::new(
            self.data[index] as f32 / 255.0,
            self.data[index + 1] as f32 / 255.0,
            self.data[index + 2] as f32 / 255.0,
        )
    }
}

/// Skybox con 6 caras
pub struct Skybox {
    pub right: Texture,   // +X
    pub left: Texture,    // -X
    pub top: Texture,     // +Y
    pub bottom: Texture,  // -Y
    pub front: Texture,   // +Z
    pub back: Texture,    // -Z
}

impl Skybox {
    /// Carga un skybox desde 6 archivos
    pub fn load(folder: &str) -> Result<Self, String> {
        println!("📦 Cargando skybox desde: {}", folder);
        
        Ok(Skybox {
            right: Texture::load(&format!("{}/estrellas.png", folder))?,
            left: Texture::load(&format!("{}/estrellas.png", folder))?,
            top: Texture::load(&format!("{}/estrellas.png", folder))?,
            bottom: Texture::load(&format!("{}/estrellas.png", folder))?,
            front: Texture::load(&format!("{}/estrellas.png", folder))?,
            back: Texture::load(&format!("{}/estrellas.png", folder))?,
        })
    }

    /// Samplea el skybox usando una dirección 3D
    pub fn sample(&self, dir: Vector3) -> Vector3 {
        let abs_x = dir.x.abs();
        let abs_y = dir.y.abs();
        let abs_z = dir.z.abs();
        
        // Determinar qué cara del cubo es la más relevante
        let (texture, u, v) = if abs_x >= abs_y && abs_x >= abs_z {
            // Cara X dominante
            if dir.x > 0.0 {
                // +X (right)
                let u = (-dir.z / abs_x + 1.0) * 0.5;
                let v = (-dir.y / abs_x + 1.0) * 0.5;
                (&self.right, u, v)
            } else {
                // -X (left)
                let u = (dir.z / abs_x + 1.0) * 0.5;
                let v = (-dir.y / abs_x + 1.0) * 0.5;
                (&self.left, u, v)
            }
        } else if abs_y >= abs_x && abs_y >= abs_z {
            // Cara Y dominante
            if dir.y > 0.0 {
                // +Y (top)
                let u = (dir.x / abs_y + 1.0) * 0.5;
                let v = (dir.z / abs_y + 1.0) * 0.5;
                (&self.top, u, v)
            } else {
                // -Y (bottom)
                let u = (dir.x / abs_y + 1.0) * 0.5;
                let v = (-dir.z / abs_y + 1.0) * 0.5;
                (&self.bottom, u, v)
            }
        } else {
            // Cara Z dominante
            if dir.z > 0.0 {
                // +Z (front)
                let u = (dir.x / abs_z + 1.0) * 0.5;
                let v = (-dir.y / abs_z + 1.0) * 0.5;
                (&self.front, u, v)
            } else {
                // -Z (back)
                let u = (-dir.x / abs_z + 1.0) * 0.5;
                let v = (-dir.y / abs_z + 1.0) * 0.5;
                (&self.back, u, v)
            }
        };
        
        texture.sample_bilinear(u, v)
    }
}