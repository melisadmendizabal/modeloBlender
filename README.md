# 🌌 Laboratorio: Static Shaders — Rama `Sol`

## 🎯 Descripción
En este laboratorio se practica el diseño procedural de una estrella (Sol) utilizando shaders en Rust.
La superficie y animación se generan únicamente mediante funciones de ruido y variables de tiempo, sin texturas ni materiales precargados. Este laboratorio se hizo teniendo de base la rama de planetas, por lo que el sol aparece como un planeta extra.

El shader combina múltiples tipos de ruido (Perlin, Simplex, Cellular y Fractal Brownian Motion) para simular:
- Turbulencias solares
- Llamaradas energéticas
- Manchas solares
- Pulsaciones luminosas
- Distorsiones superficiales (flare)
El resultado es una animación continua y cíclica, controlada mediante una variable uniforme de tiempo (uniforms.time).
---

## 🪐 Sol
(se mira bien trabado porque le tuve que bajar calidad al vídeo para poder subirlo)



![Sol (1)](https://github.com/user-attachments/assets/5cca31c3-cecb-4b1f-a973-1fc974d4e446)



---
## ⚙️ Estructura general

El shader está dividido en dos etapas principales:
1. Vertex Shader (vertex_shader_star): Deforma la geometría de la esfera base para simular turbulencias y llamaradas solares.
2. Fragment Shader (fragment_shader_star y fragment_shader_star_flares)
Calcula el color, emisión y brillo de cada fragmento según su “temperatura” y la actividad solar simulada.

---
## 🧠 Variables y Uniforms
-  `uniforms.time`: Tiempo en segundos desde el inicio del programa. Controla toda la animación (ruidos, pulsaciones, flares, etc.). 
- `uniforms.model_matrix`, `view_matrix`, `projection_matrix`, `viewport_matrix`: Matrices estándar para transformar el modelo de espacio local a pantalla. 
- `vertex.position`, `vertex.normal`: Posición y normal del vértice original. Se modifican en el vertex shader según ruido.

---
##🌋 Funciones de ruido implementadas

Cada tipo de ruido cumple un propósito visual distinto en la simulación del sol:

1. Perlin Noise
   Uso: Turbulencias suaves y base de movimiento.
   Efecto visual: Ondas orgánicas que fluyen sobre la superficie, simulando el plasma solar.
   Implementación: Se interpola suavemente entre valores pseudoaleatorios para obtener transiciones continuas.

2. Simplex Noise
   Uso: Granulación solar fina (textura de superficie).
   Efecto visual: Añade pequeños detalles que dan sensación de movimiento granular, similar a la fotosfera.

4. Cellular / Worley Noise
   Uso: Manchas solares.
   Efecto visual: Crea regiones de menor intensidad o “puntos fríos” que aparecen y desaparecen dinámicamente.

4. Fractal Brownian Motion (FBM)
   Uso: Combina múltiples octavas de Perlin para formar patrones más ricos.
   Efecto visual: Superpone diferentes escalas de ruido, logrando profundidad visual y movimiento natural.

---
## 💫 Vertex Shader: vertex_shader_star

Propósito:
Es un vertex shader adicional al que se usa con lo planetas, ya que deforma la superficie de la esfera base según valores de ruido dependientes del tiempo, generando picos o depresiones que simulan llamaradas y vibración solar. 

Funcionamiento:
- Calcula coordenadas esféricas del vértice.
- Evalúa funciones de ruido (Perlin + FBM) para obtener una intensidad de turbulencia.
- Aplica una escala variable al radio del vértice (scale = 1.0 + distortion).
- Deforma también la normal para lograr un efecto de luz dinámico.

Efecto visible:
La esfera “late” y se expande en zonas aleatorias, como una estrella activa.

---

## ⌨️ Controles y Atajos de Teclado

Durante la ejecución del renderer:

| Tecla | Acción |
|:------:|:-------|
| **1** | Cambia al **Planeta Rocoso** |
| **2** | Cambia al **Gigante Gaseoso** |
| **3** | Cambia al **Planeta Fresa ** |
| **4** | Cambia al **Planeta Anillos** |
| **5** | Cambia al **Planeta Rojo** |
| **R** | Reinicia rotación del planeta |
| **↑ / ↓** | Aumentar o disminuir velocidad de rotación |
| **← / →** | Cambiar tipo de shader activo (variantes) |
| **A** | Activar/desactivar **anillos procedurales** |
| **ESPACIO** | Detiene/Activa la rotación |

> Todos los planetas giran sobre su eje, y las animaciones dependen del tiempo (`uniforms.time`).

---

## ⚙️ Parámetros (Uniforms) y Documentación Técnica general de planetas y estrella

| Uniform | Tipo | Descripción |
|----------|------|-------------|
| `time` | `f32` | Tiempo acumulado (usado para animaciones, rotación, transiciones de color, ruido dinámico). |
| `light_dir` | `Vector3` | Dirección de la luz principal usada para iluminación difusa. |
| `view_dir` | `Vector3` | Dirección del observador (para efectos de Fresnel y resplandor atmosférico). |
| `model_matrix` | `Matrix` | Transformaciones del planeta (rotación, traslación). |
| `normal_matrix` | `Matrix` | Corrección de normales tras transformaciones. |
| `camera_position` | `Vector3` | Posición de la cámara usada en algunos efectos de luz especular. |

Cada `fragment shader` recibe un `Fragment` estructurado con:
- `world_position`: posición 3D del fragmento en el espacio global.
- `normal`: vector normal en el punto.
- `uv`: coordenadas opcionales para distorsión o proyección esférica.

La salida (`FragmentOutput`) siempre contiene:
- `color`: valor RGB calculado en base a capas y luz.
- `alpha`: opacidad (para blending entre capas).

---

## 🛠️ Requisitos
- Rust
- raylib-rs

## 🔧 Instalación y ejecución

1. Clona este repositorio
2. Compila y ejecuta:

    ```bash
    cargo run
    ```


