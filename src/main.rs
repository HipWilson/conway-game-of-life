mod framebuffer;
mod organisms;

use framebuffer::Framebuffer;
use raylib::prelude::*;

const DEAD: Color = Color::BLACK;

// true  = mundo tipo "loop" (orillas conectadas con el lado opuesto)
// false = todo lo que está fuera del framebuffer se considera muerto
const WRAP_EDGES: bool = true;

fn is_alive_color(color: Color) -> bool {
    color != DEAD
}

/// Coloca un organismo (lista de coords relativas) en el framebuffer con un color dado.
fn place_pattern(fb: &mut Framebuffer, pattern: &[(i32, i32)], offset_x: i32, offset_y: i32, color: Color) {
    fb.set_current_color(color);
    for (dx, dy) in pattern {
        fb.point(offset_x + dx, offset_y + dy);
    }
}

/// Dada una celda (x,y) y un desplazamiento (dx,dy), devuelve la coordenada
/// del vecino aplicando la regla de bordes elegida (o None si cae fuera y no hay wrap).
fn neighbor_coords(fb: &Framebuffer, x: i32, y: i32, dx: i32, dy: i32) -> Option<(i32, i32)> {
    let (nx, ny) = (x + dx, y + dy);

    if WRAP_EDGES {
        Some((
            ((nx % fb.width) + fb.width) % fb.width,
            ((ny % fb.height) + fb.height) % fb.height,
        ))
    } else if nx < 0 || nx >= fb.width || ny < 0 || ny >= fb.height {
        None
    } else {
        Some((nx, ny))
    }
}

/// Revisa los 8 vecinos de (x,y) y devuelve (cuántos están vivos, sus colores).
fn neighbor_info(fb: &mut Framebuffer, x: i32, y: i32) -> (u8, Vec<Color>) {
    let mut count = 0;
    let mut colors = Vec::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some((nx, ny)) = neighbor_coords(fb, x, y, dx, dy) {
                let c = fb.get_color(nx, ny);
                if is_alive_color(c) {
                    count += 1;
                    colors.push(c);
                }
            }
        }
    }

    (count, colors)
}

/// Promedia una lista de colores (para el color de una célula recién nacida).
fn average_color(colors: &[Color]) -> Color {
    if colors.is_empty() {
        return Color::WHITE;
    }
    let n = colors.len() as u32;
    let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
    for c in colors {
        r += c.r as u32;
        g += c.g as u32;
        b += c.b as u32;
    }
    Color::new((r / n) as u8, (g / n) as u8, (b / n) as u8, 255)
}

/// Aplica las 4 reglas de Conway: lee de `current` (con get_color),
/// escribe la siguiente generación en `next` (con point).
fn step(current: &mut Framebuffer, next: &mut Framebuffer) {
    for y in 0..current.height {
        for x in 0..current.width {
            let self_color = current.get_color(x, y);
            let self_alive = is_alive_color(self_color);
            let (neighbors, neighbor_colors) = neighbor_info(current, x, y);

            let next_alive = matches!(
                (self_alive, neighbors),
                (true, 2) | (true, 3) | (false, 3)
            );

            let next_color = if !next_alive {
                DEAD
            } else if self_alive {
                self_color
            } else {
                average_color(&neighbor_colors)
            };

            next.set_current_color(next_color);
            next.point(x, y);
        }
    }
}

fn build_initial_pattern(fb: &mut Framebuffer) {
    use organisms::*;

    // --- Still lifes (blanco) ---
    place_pattern(fb, &block(), 5, 5, Color::RED);
    place_pattern(fb, &beehive(), 15, 5, Color::RED);
    place_pattern(fb, &loaf(), 25, 5, Color::RED);
    place_pattern(fb, &boat(), 35, 5, Color::WHITE);
    place_pattern(fb, &tub(), 45, 5, Color::RED);

    // --- Osciladores (amarillo) ---
    place_pattern(fb, &blinker(), 5, 20, Color::YELLOW);
    place_pattern(fb, &toad(), 15, 20, Color::YELLOW);
    place_pattern(fb, &beacon(), 25, 20, Color::YELLOW);
    place_pattern(fb, &pulsar(), 40, 15, Color::YELLOW);
    place_pattern(fb, &pentadecathlon(), 5, 35, Color::YELLOW);

    // --- Spaceships (cada tipo con su color) ---
    place_pattern(fb, &glider(), 60, 5, Color::SKYBLUE);
    place_pattern(fb, &lwss(), 10, 55, Color::LIME);
    place_pattern(fb, &mwss(), 30, 55, Color::ORANGE);
    place_pattern(fb, &hwss(), 50, 55, Color::PURPLE);

    // Gliders extra
    place_pattern(fb, &glider(), 70, 70, Color::SKYBLUE);
    place_pattern(fb, &glider(), 5, 70, Color::SKYBLUE);
}

fn main() {
    let window_width = 500;
    let window_height = 500;

    let grid_width = 100;
    let grid_height = 100;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(30);

    let mut current = Framebuffer::new(grid_width, grid_height, DEAD);
    let mut next = Framebuffer::new(grid_width, grid_height, DEAD);

    build_initial_pattern(&mut current);

    while !rl.window_should_close() {
        step(&mut current, &mut next);
        std::mem::swap(&mut current, &mut next);
        current.swap_buffer(&mut rl, &thread);
    }
}