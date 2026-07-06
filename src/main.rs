use macroquad::prelude::*;

#[macroquad::main("kurvfeber")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}
