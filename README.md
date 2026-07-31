# Conway's Game of Life — Rust + raylib

Implementación del algoritmo de Conway usando únicamente las funciones
`point` (pintar un pixel) y `get_color` (leer un pixel) de un framebuffer propio.

## Cómo funciona

- `src/framebuffer.rs`: el framebuffer, con `point`, `get_color`, `clear` y `swap_buffer`.
- `src/organisms.rs`: patrones clásicos (still lifes, osciladores, spaceships) como
  listas de coordenadas relativas.
- `src/main.rs`: arma el patrón inicial y corre el loop del juego (leer estado
  actual con `get_color`, calcular la siguiente generación, escribirla con `point`).

El framebuffer del juego es de 100x100 celdas, pero la ventana es de 1000x1000
píxeles, así que cada celda se ve como un bloque de 10x10 (fácil de ver la animación).

Las orillas están configuradas como "loop" (mundo toroidal) — puedes cambiar
`WRAP_EDGES` a `false` en `main.rs` si prefieres que los bordes se traten como muertos.

## Requisitos (Windows)

1. **Rust**: instala desde https://rustup.rs (te da `cargo` y el compilador).
2. **raylib** necesita un compilador de C. La forma más fácil en Windows es
   instalar el toolchain MSVC de Rust (rustup lo puede configurar):
   ```powershell
   rustup default stable-x86_64-pc-windows-msvc
   ```
   También necesitas las "Visual Studio Build Tools" (C++ build tools) instaladas.
   Si prefieres evitarte esto, usa el toolchain GNU:
   ```powershell
   rustup default stable-x86_64-pc-windows-gnu
   ```

## Cómo correrlo

```powershell
cd conway-game-of-life
cargo run --release
```

La primera compilación tarda un poco (compila raylib desde cero). Se abrirá una
ventana con la animación corriendo.

## Cómo grabar el GIF (Windows)

Opción simple con **ShareX** (gratis):
1. Descarga e instala ShareX.
2. Usa "Capture > Screen recording (GIF)", selecciona el área de la ventana del juego.
3. Graba unos 5-10 segundos, guarda el GIF.

Alternativa: la barra de juego de Windows (`Win + G`) graba video (mp4); puedes
convertirlo a GIF después con https://ezgif.com/video-to-gif.

## Entrega

1. `git init`, `git add .`, `git commit -m "Conway's Game of Life"`.
2. Sube el repo a GitHub (`HipWilson`).
3. Agrega el GIF a este README (o a la carpeta del repo) con:
   ```markdown
   ![demo](nombre-del-gif.gif)
   ```
4. Sube también el GIF al servidor de Discord de la clase.
