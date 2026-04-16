use macroquad::prelude::*;

#[macroquad::main("Smart Road")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        draw_text("Smart Road - Intersection", 20.0, 40.0, 30.0, WHITE);

        next_frame().await;
    }
}
