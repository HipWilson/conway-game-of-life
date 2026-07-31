use raylib::prelude::*;

/// Framebuffer: nuestro "lienzo" propio. Solo exponemos dos operaciones
/// de dibujo: `point` (pintar un pixel) y `get_color` (leer un pixel).
/// Todo el juego de la vida se implementa usando SOLO estas dos funciones.
pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    /// Cambia el color con el que se va a pintar en las próximas llamadas a `point`.
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Limpia todo el framebuffer al color de fondo.
    /// NOTA: para el Game of Life NO usamos esta función en cada frame,
    /// porque la lógica del juego ya repinta cada celda (viva o muerta).
    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    /// La única función para "dibujar": pinta un pixel del color actual.
    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    /// Devuelve el color de una celda. Fuera de rango devuelve el color de fondo
    /// (útil si en algún momento quieres tratar el borde como "muerto").
    /// Nota: recibe &mut self porque Image::get_color de raylib-rs lo requiere,
    /// aunque conceptualmente sea una operación de lectura.
    pub fn get_color(&mut self, x: i32, y: i32) -> Color {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return self.background_color;
        }
        self.color_buffer.get_color(x, y)
    }

    /// Dibuja el framebuffer en la ventana, escalado para llenarla
    /// (framebuffer chico -> ventana grande, como pide el enunciado).
    pub fn swap_buffer(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.clear_background(Color::BLACK);

            let screen_w = renderer.get_screen_width() as f32;
            let screen_h = renderer.get_screen_height() as f32;
            let scale = (screen_w / self.width as f32).min(screen_h / self.height as f32);

            renderer.draw_texture_ex(&texture, Vector2::new(0.0, 0.0), 0.0, scale, Color::WHITE);
        }
    }
}