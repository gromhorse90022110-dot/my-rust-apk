use macroquad::prelude::*;

#[macroquad::main("RustGame")]
async fn main() {
    let mut color = BLUE;

    loop {
        // Очищаем фон
        clear_background(BLACK);

        // Если коснулись экрана — меняем цвет на случайный
        if is_mouse_button_pressed(MouseButton::Left) {
            color = Color::new(
                rand::gen_range(0.0, 1.0),
                rand::gen_range(0.0, 1.0),
                rand::gen_range(0.0, 1.0),
                1.0,
            );
        }

        // Рисуем круг в центре
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, color);

        // Рисуем текст-инструкцию
        draw_text("TAP TO CHANGE COLOR", 20.0, 50.0, 40.0, WHITE);
        draw_text(&format!("RUST VERSION 2.0"), 20.0, 100.0, 30.0, GREEN);

        next_frame().await
    }
}
