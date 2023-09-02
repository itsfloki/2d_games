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

    let mut grid: [[Grid; 3]; 3] = [[Grid::default(); 3]; 3];
    let mut draw_shape_coordinates: Vec<(f32, f32, Player)> = Vec::new();
    let mut player_turn: bool = false;

    for row in 0..rows {
        for col in 0..cols {
            let cell_x = col as f32 * cell_width;
            let cell_y = row as f32 * cell_height;

            let center_x = cell_x + (cell_width / 2.0);
            let center_y = cell_y + (cell_height / 2.0);
            grid[row][col] = Grid {
                x: cell_x,
                y: cell_y,
                center_x,
                center_y,
            };
        }
    }

    loop {
        clear_background(DARKGRAY);

        // draw grid
        for row in grid {
            for col in row {
                draw_rectangle_lines(
                    col.x,
                    col.y,
                    cell_width,
                    cell_height,
                    grid_thickness,
                    WHITE,
                )
            }
        }

        for coordinates in &draw_shape_coordinates {
            match coordinates.2 {
                Player::Cross => {
                    draw_cross(
                        coordinates.0,
                        coordinates.1,
                        70.0,
                        10.0,
                        YELLOW,
                    );
                }
                Player::Cricle => {
                    draw_circle_lines(
                        coordinates.0,
                        coordinates.1,
                        30.0,
                        10.0,
                        YELLOW,
                    );
                }
            }
        }

        // mouse event
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            for row in grid {
                for col in row {
                    if (x >= col.x)
                        && (x <= (col.x + cell_width))
                        && (y >= col.y)
                        && (y <= (col.y + cell_height))
                    {
                        draw_shape_coordinates.push((
                            col.center_x,
                            col.center_y,
                            if player_turn {
                                Player::Cross
                            } else {
                                Player::Cricle
                            },
                        ));
                        player_turn = !player_turn;
                        break;
                    }
                }
            }
        }

        next_frame().await
    }
}

#[derive(Copy, Clone)]
struct Grid {
    x: f32,
    y: f32,
    center_x: f32,
    center_y: f32,
}

enum Player {
    Cross,
    Cricle,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            x: 0.0,
            y: 0.0,
            center_x: 0.0,
            center_y: 0.0,
        }
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
