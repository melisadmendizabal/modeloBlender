# Blender OBJ Renderer en Rust

Este proyecto es un renderizador 3D básico en Rust que carga modelos .obj exportados desde Blender y los dibuja en pantalla utilizando una transformación básica (traslación, rotación, escala). El modelo es rasterizado como triángulos en un framebuffer personalizado, y se visualiza usando la librería raylib-rs.


## Características

- Carga de modelos .obj simples (sin texturas, solo geometría)
- Renderizado en 2D (proyección ortográfica) de triángulos
- Transformaciones:
  - Traslación (mover)
  - Rotación en X, Y, Z
  - Escalado
- Controles en tiempo real con el teclado
- Uso de un framebuffer para dibujar manualmente los triángulos

📸 ## Captura de pantalla

<img width="1918" height="1012" alt="image" src="https://github.com/user-attachments/assets/b11efbee-ac87-41ea-b64e-144a099b42b3" />
