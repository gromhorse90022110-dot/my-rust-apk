use macroquad::prelude::*;

// Упрощенная "магия" для запуска на Android
#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate() {}

#[macroquad::main("my_app")]
async fn main() {
    let mut color = BLUE;

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            color = Color::new(
                rand::gen_range(0.0, 1.0),
                rand::gen_range(0.0, 1.0),
                rand::gen_range(0.0, 1.0),
                1.0,
            );
        }

        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, color);
        draw_text("TAP TO CHANGE COLOR", 20.0, 50.0, 40.0, WHITE);
        draw_text("VERSION 2.1", 20.0, 100.0, 30.0, GREEN);

        next_frame().await
    }
}
