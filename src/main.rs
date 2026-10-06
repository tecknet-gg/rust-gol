use macroquad::prelude::*;

#[macroquad::main("GoL")]
async fn main() {
    loop {
        clear_background(RED);

        next_frame().await;

    }
}