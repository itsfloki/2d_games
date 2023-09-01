use macroquad::prelude::*;

#[macroquad::main("Tik Tak Toe")]
async fn main() {
    let grid_thickness = 2.0;
    let grid_width = 500;
    let grid_height = 500;
    let rows = 3;
    let cols = 3;

    let cell_width = (grid_width / cols) as f32;
    let cell_height = (grid_height / rows) as f32;

    loop {
        clear_background(DARKGRAY);

        // draw grid
        for row in 0..rows {
            for col in 0..cols {
                let cell_x = col as f32 * cell_width;
                let cell_y = row as f32 * cell_height;

                draw_rectangle_lines(
                    cell_x,
                    cell_y,
                    cell_width,
                    cell_height,
                    grid_thickness,
                    WHITE,
                )
            }
        }

        next_frame().await
    }
}
