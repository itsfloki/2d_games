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

        // draw circle
        draw_circle_lines(
            screen_width() - 100.0,
            screen_height() - 100.0,
            30.0,
            10.0,
            YELLOW,
        );

        // draw cross
        draw_cross(
            screen_width() - 300.0,
            screen_width() - 100.0,
            70.0,
            10.0,
            YELLOW
        );

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

fn draw_cross(x: f32, y: f32, line_length: f32, thickness: f32, color: Color) {
    // top-left endpoint
    let x1 = x - (line_length / 2.0);
    let y1 = y - (line_length / 2.0);

    // bottom-right endpoint
    let x2 = x + (line_length / 2.0);
    let y2 = y + (line_length / 2.0);

    // top_right endpoint
    let x3 = x + (line_length / 2.0);
    let y3 = y - (line_length / 2.0);

    // bottom-left endpoint
    let x4 = x - (line_length / 2.0);
    let y4 = y + (line_length / 2.0);

    draw_line(x1, y1, x2, y2, thickness, color);
    draw_line(x3, y3, x4, y4, thickness, color);
}
