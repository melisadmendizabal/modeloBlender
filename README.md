# 🌌 Proyecto — Rama `SpaceTravel`
Un motor de renderizado 3D completamente desde cero que simula el sistema solar con física orbital, shaders procedurales personalizados y navegación espacial interactiva.
---
link del video:
[🎥 Ver demostración en video](https://uvggt-my.sharepoint.com/:v:/g/personal/men23778_uvg_edu_gt/IQDcpLMJ4W0pTIVpLqWlkuJlARNVhSSPX_lPgFupq5PdBpE?nav=eyJyZWZlcnJhbEluZm8iOnsicmVmZXJyYWxBcHAiOiJTdHJlYW1XZWJBcHAiLCJyZWZlcnJhbFZpZXciOiJTaGFyZURpYWxvZy1MaW5rIiwicmVmZXJyYWxBcHBQbGF0Zm9ybSI6IldlYiIsInJlZmVycmFsTW9kZSI6InZpZXcifX0%3D&e=2rZU8m)
<img width="1296" height="897" alt="image" src="https://github.com/user-attachments/assets/28218eff-c117-4515-84c5-388094c16937" />

---


## 🎯 Descripción del Proyecto

Este proyecto implementa un **software renderer completo** (sin usar APIs como OpenGL/Vulkan) que renderiza un sistema solar funcional. Cada componente del pipeline gráfico fue programado manualmente: transformaciones matriciales, rasterización de triángulos, z-buffering, y shaders procedurales para cada cuerpo celeste.

**Lo destacado:** No se usaron texturas externas. Toda la apariencia visual de los planetas proviene de cálculos matemáticos en tiempo real usando funciones de ruido, gradientes y composición por capas.

---

## ✨ Características Principales

### 🪐 Sistema Solar 
**6 cuerpos celestes únicos:**
- ☀️ **Sol**: Superficie estelar animada con llamaradas solares, manchas y turbulencias usando Perlin Noise, Simplex y ruido celular
- 🌑 **Mercurio**: Planeta rocoso con cráteres generados por normal mapping procedural y oclusión ambiental
- 🌫️ **Venus**: Atmósfera densa con múltiples capas de nubes dinámicas que cambian de color cada segundo
- 🍓 **Tierra**: Diseño creativo "tipo fresa" con base texturizada, semillas doradas distribuidas proceduralmente y hoja verde polar
- 🔴 **Marte**: Superficie marciana con tormentas de arena animadas, casquetes polares de hielo, venas minerales y resplandor atmosférico
- 🪐 **Júpiter**: Gigante gaseoso con bandas de colores rotantes y un **anillo toroidal generado completamente en vertex shader** (sin modelo 3D)

Cada planeta rota sobre su eje y orbita alrededor del Sol siguiendo el plano eclíptico.

### 🚀 Nave Espacial 
- Modelo 3D personalizado de **barco de papel** que sigue a la cámara
- Sistema de física espacial con 6 grados de libertad (yaw, pitch, roll)
- Cámara third-person dinámica con offset configurable

### 🌠 Skybox Estelar 
- Campo de estrellas envolvente renderizado al fondo
- Soporte para texturas cubemap o generación procedural
- Optimizado para renderizarse solo en píxeles vacíos

### 🎮 Movimiento 3D Completo 
- Control total de la cámara en 3 ejes
- Rotación suave con protección contra gimbal lock
- Movimiento en 6 direcciones con física espacial realista

---

## 🎨 Sistema de Shaders Procedurales

Todos los efectos visuales se generan matemáticamente sin texturas externas:

### Técnicas Implementadas

**Funciones de Ruido:**
- **Perlin Noise**: Turbulencias orgánicas y movimiento de plasma
- **Simplex Noise**: Detalles finos y granulación de superficie
- **Cellular (Worley) Noise**: Manchas solares y patrones celulares
- **FBM (Fractal Brownian Motion)**: Combinación de múltiples octavas para complejidad natural

**Efectos Visuales:**
- **Normal Mapping**: Relieve visual sin geometría adicional (cráteres de Mercurio)
- **Alpha Blending por Capas**: Hasta 5 capas combinadas (terreno, nubes, hielo, minerales, atmósfera)
- **Vertex Deformation**: Generación de geometría toroidal para anillos en tiempo real
- **Gradientes de Temperatura**: Mapeo de temperatura a color simulando espectro de cuerpo negro
- **Efecto Fresnel**: Resplandor atmosférico en bordes planetarios

### Ejemplos de Composición

**Marte (5 capas):**
1. Terreno base → paleta de rojos marcianos con variación geológica
2. Tormentas de arena → patrones dinámicos que se mueven con el tiempo
3. Casquetes polares → hielo en polos (Y > ±0.6) con bordes irregulares
4. Venas minerales → oro/cian distribuido pseudo-aleatoriamente
5. Resplandor atmosférico → efecto Fresnel anaranjado en los bordes

**Venus (2 capas con animación):**
1. Rayas de colores → paleta que rota entre 5 combinaciones cada segundo
2. Nubes procedurales → 3 octavas de ruido con movimiento independiente y transparencia

---

## 🏗️ Arquitectura del Renderer

### Pipeline Gráfico Completo

El motor implementa cada etapa desde cero:

1. **Vertex Shader** → Transformaciones Model-View-Projection con cálculo de profundidad
2. **Primitive Assembly** → Agrupación de vértices en triángulos
3. **Rasterization** → Coordenadas baricéntricas con corrección de perspectiva
4. **Fragment Shader** → Shaders procedurales personalizados por planeta
5. **Depth Test & Blending** → Z-buffer en view space + alpha blending

### Componentes Clave

- **Framebuffer (`framebuffer.rs`)**: Color buffer RGB + Depth buffer float + alpha blending
- **Transformaciones (`matrix.rs`)**: Matrices 4x4 para Model, View, Projection y Viewport
- **Rasterización (`triangle.rs`)**: Interpolación baricéntrica con corrección de perspectiva
- **Sistema Solar (`sistema_solar.rs`)**: Física orbital, colisiones y lógica de la nave
- **Shaders**: 6 archivos especializados, uno por tipo de cuerpo celeste

---

## 🎮 Controles

### Movimiento de la Nave
- **W/S**: Retroceder/Avanzar
- **A/D**: Izquierda/Derecha
- **↑/↓**: Subir/Bajar

### Otros
- **H**: Resetear nave a posición inicial

---

## 📁 Estructura del Proyecto

```
src/
├── main.rs                    # Loop principal y configuración
├── camera.rs                  # Sistema de cámara 3D
├── framebuffer.rs             # Buffers de color y profundidad
├── matrix.rs                  # Matemáticas de transformaciones
├── triangle.rs                # Rasterización de triángulos
├── shaders.rs                 # Dispatcher de shaders
├── shader_sol.rs              # Shader estelar (Sol)
├── shader_rocoso.rs           # Shader de cráteres (Mercurio)
├── shader_gaseoso.rs          # Shader de atmósfera (Venus)
├── shader_strawberry.rs       # Shader creativo (Tierra)
├── shader_rojo.rs             # Shader marciano (Marte)
├── shader_anillo.rs           # Shader toroidal (Júpiter)
├── sistema_solar.rs           # Física orbital y colisiones
├── skybox.rs                  # Fondo espacial
├── texture.rs                 # Cargador de texturas (skybox)
└── obj.rs                     # Cargador de modelos .obj

Models/
├── sphere.obj                 # Esfera base para planetas
└── barcoPapel.obj            # Modelo de la nave

Textures/skybox/              # Texturas opcionales del skybox
```



---

## 🎓 Detalles Técnicos Destacados

### Optimizaciones Implementadas
- **Early Z-test**: Descarte rápido de fragmentos ocultos
- **Bounding box rasterization**: Reduce píxeles testeados por triángulo
- **Skybox inteligente**: Solo se dibuja en depth = INFINITY

### Innovaciones
- **Shader Layering System**: Hasta 5 capas con alpha blending automático
- **Hybrid Normal Mapping**: Combina altura procedural con perturbación de normales
- **Dynamic Shader Selection**: Cada planeta usa su propio par vertex/fragment

---

## 📊 Performance

**Configuración de prueba:**
- Resolución: 1300×900 pixels
- Target: 60 FPS
- ~50,000 vértices por frame
- 6 planetas + nave + skybox

---

## 🌟 Créditos

**Desarrollado por:** Andrea Elías  
**Curso:** Gráficos por Computadora  
**Tecnologías:** Rust, Raylib, matemáticas personalizadas  

### Librerías
- `raylib` - Windowing y contexto
- `tobj` - Carga de modelos .obj
- `image` - Procesamiento de texturas

---


## 🛠️ Requisitos
- Rust

## 🔧 Instalación y ejecución

1. Clona este repositorio
2. Compila y ejecuta:

    ```bash
    cargo run
    ```


