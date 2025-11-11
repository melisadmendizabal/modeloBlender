# 🌌 Laboratorio: Static Shaders — Rama `planetas`

## 🎯 Objetivo
En este laboratorio se practica la creación de cuerpos celestes mediante shaders (sin texturas ni materiales externos).  
Cada planeta fue generado únicamente a partir de una esfera base, y toda su apariencia visual —colores, gradientes, efectos de iluminación y detalles superficiales— proviene de cálculos en los fragment y vertex shaders.
Los shaders implementados simulan materiales, atmósferas y superficies planetarias usando funciones matemáticas, ruido, gradientes y composición por capas.


## 🪐 Planetas Implementados

### 🪨 1. Planeta Rocoso 
- Basado en simulación de superficie lunar.
- Capas de shader:
  1. **Base rugosa** (ruido procedimental, tonos gris claro/oscuro).
  2. **Sombras de cráteres** (profundidad y oclusión por distancia al centro del cráter).
  3. **Bordes brillantes** (efecto de luz sobre el relieve).
  4. **Normal Mapping** para simular profundidad sin geometría adicional.
- Implementa iluminación direccional y ambient occlusion simulada.

<img width="887" height="656" alt="image" src="https://github.com/user-attachments/assets/d7693143-4c9e-4cc5-b7ad-3063508d69d6" />


---

### ☁️ 2. Gigante Gaseoso
- Capas de shader:
  1. **Rayas dinámicas:** alternan entre diferentes paletas de color cada segundo (animación continua).
  2. **Nubes procedurales:** ruido en múltiples frecuencias que genera profundidad y movimiento.

     
<img width="818" height="596" alt="image" src="https://github.com/user-attachments/assets/fea62362-e3bb-4b31-ae35-a2b950f0798a" />


---


### 🍓 3. Planeta Extra — *“Strawberry Planet”* 🍓
- Planeta adicional de carácter creativo.
- Capas de shader:
  1. **Base roja grumosa:** textura de superficie tipo fresa.
  2. **Semillas doradas:** distribuidas procedimentalmente en la superficie.
  3. **Hoja verde:** zona superior con textura de venas.
- Incluye blending por capas con transparencias suaves.
- Iluminación natural con difuso y especular leve.


<img width="821" height="661" alt="image" src="https://github.com/user-attachments/assets/5bdd1867-2109-4ce9-aa1b-cb93d479fcf1" />


---


### 💫 4. Sistema de Anillos Procedurales — *Vertex Shader + Fragment Shader*
Archivo: `shadersAnillos.rs`

Este shader genera anillos planetarios **completamente procedurales** usando **deformación de vértices** en un **torus virtual**.  
No se usa ningún modelo importado: toda la geometría y coloración se calcula en tiempo real.

#### 🧩 Vertex Shader (`vertex_shader_torus`)
- Genera un **toroide** a partir de coordenadas paramétricas `(u, v)`:
  - `R = 0.8` → radio interior del anillo.
  - `r = 0.2` → grosor del anillo.
  - Se aplica un **factor de aplanamiento (`flatten_factor`)** para hacerlo más delgado visualmente.
 

<img width="973" height="671" alt="image" src="https://github.com/user-attachments/assets/1cd114e9-85ad-4d79-9922-aab5588795f8" />

 
---

### 🔴 5. Planeta Rojo — *“Red Planet” (tipo Marte)*
- Simula geología marciana con efectos atmosféricos.
- Capas de shader:
  1. **Terreno base:** variación geológica por ruido (óxidos, llanuras, montañas).
  2. **Tormentas de arena:** patrones dinámicos que se desplazan con el tiempo.
  3. **Casquetes polares:** formación en los polos con bordes irregulares.

- Variantes:
  - `fragment_shader_red_planet_simple` → solo terreno.
  - `fragment_shader_red_planet_stormy` → tormentas intensas.
  - `fragment_shader_red_planet_night` → versión nocturna con brillo tenue.
- Simulación de iluminación cálida (sol rojizo).

<img width="975" height="683" alt="image" src="https://github.com/user-attachments/assets/0a88f3d2-e844-4db8-a19d-d9fa67999cc7" />



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

## ⚙️ Parámetros (Uniforms) y Documentación Técnica

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


