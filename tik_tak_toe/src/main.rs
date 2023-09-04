use crand::seq::SliceRandom;
use macroquad::{audio, prelude::*};

#[macroquad::main("Tik Tak Toe")]
async fn main() {
    set_pc_assets_folder("assets");

    let sound = audio::load_sound("sound.wav").await.unwrap();

    let grid_thickness = 2.0;
    let grid_width = 500;
    let grid_height = 500;
    let rows = 3;
    let cols = 3;

    let cell_width = (grid_width / cols) as f32;
    let cell_height = (grid_height / rows) as f32;

    let mut grid: [[Grid; 3]; 3] = [[Grid::default(); 3]; 3];

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
                player: Player::Nil,
            };
        }
    }

    loop {
        clear_background(DARKGRAY);

        // draw grid & shape
        for row in grid {
            for col in row {
                draw_rectangle_lines(
                    col.x,
                    col.y,
                    cell_width,
                    cell_height,
                    grid_thickness,
                    WHITE,
                );

                match col.player {
                    Player::Nil => {}
                    Player::Cross => {
                        draw_cross(
                            col.center_x,
                            col.center_y,
                            70.0,
                            10.0,
                            BLUE,
                        );
                    }
                    Player::Circle => {
                        draw_circle_lines(
                            col.center_x,
                            col.center_y,
                            30.0,
                            10.0,
                            GREEN,
                        );
                    }
                }
            }
        }

        // mouse event
        if is_mouse_button_pressed(MouseButton::Right) {
            for row in grid.iter_mut() {
                for col in row.iter_mut() {
                    col.player = Player::Nil;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            for row in grid.iter_mut() {
                for col in row.iter_mut() {
                    if (x >= col.x)
                        && (x <= (col.x + cell_width))
                        && (y >= col.y)
                        && (y <= (col.y + cell_height))
                    {
                        match col.player {
                            Player::Nil => {
                                col.player = Player::Circle;
                                audio::play_sound_once(&sound);
                            }
                            _ => {}
                        }
                        break;
                    }
                }
            }

            // get computer move
            if let Some((i, j)) = get_random_choice(&grid) {
                grid[i][j].player = Player::Cross;
            }
        }

        // check_winner
        let cross_player = check_winner(&grid, Player::Cross);
        let circle_player = check_winner(&grid, Player::Circle);

        if cross_player.0 {
            draw_text(
                "Winner is X",
                screen_width() - 300.0,
                200.0,
                30.0,
                YELLOW,
            );
            draw_text(
                "Right click to restart",
                screen_width() - 300.0,
                230.0,
                30.0,
                YELLOW,
            );
            draw_line(
                cross_player.1[0].0,
                cross_player.1[0].1,
                cross_player.1[2].0,
                cross_player.1[2].1,
                10.0,
                BLUE,
            );
        }

        if circle_player.0 {
            draw_text(
                "Winner is O. ",
                screen_width() - 300.0,
                200.0,
                30.0,
                YELLOW,
            );
            draw_text(
                "Right click to restart",
                screen_width() - 300.0,
                230.0,
                30.0,
                YELLOW,
            );
            draw_line(
                circle_player.1[0].0,
                circle_player.1[0].1,
                circle_player.1[2].0,
                circle_player.1[2].1,
                10.0,
                GREEN,
            );
        }

        next_frame().await
    }
}

#[derive(Copy, Clone, Debug)]
struct Grid {
    x: f32,
    y: f32,
    center_x: f32,
    center_y: f32,
    player: Player,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    Cross,
    Circle,
    Nil,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            x: 0.0,
            y: 0.0,
            center_x: 0.0,
            center_y: 0.0,
            player: Player::Nil,
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

fn get_random_choice(grid: &[[Grid; 3]; 3]) -> Option<(usize, usize)> {
    let mut spots: Vec<(usize, usize)> = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if let Player::Nil = col.player {
                spots.push((i, j));
            }
        }
    }

    spots.choose(&mut crand::thread_rng()).copied()
}

fn check_winner(
    grid: &[[Grid; 3]; 3],
    player: Player,
) -> (bool, Vec<(f32, f32)>) {
    let mut flag = false;
    let mut test: Vec<(f32, f32)> = Vec::new();

    // check rows
    for (_i, row) in grid.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            if col.player == player {
                flag = true;
                test.push((col.center_x, col.center_y));
            } else {
                flag = false;
                test.pop();
                break;
            }
        }

        if flag {
            return (flag, test);
        }
    }

    // check columns
    for (col, _) in grid[0].iter().enumerate() {
        flag = false;
        for (row, _) in grid.iter().enumerate() {
            if grid[row][col].player == player {
                flag = true;
                test.push((grid[row][col].center_x, grid[row][col].center_y));
            } else {
                flag = false;
                test.pop();
                break;
            }
        }

        if flag == true {
            return (flag, test);
        }
    }

    // check diagonals
    if (grid[0][0].player == player)
        && (grid[1][1].player == player)
        && (grid[2][2].player == player)
    {
        test.push((grid[0][0].center_x, grid[0][0].center_y));
        test.push((grid[1][1].center_x, grid[1][1].center_y));
        test.push((grid[2][2].center_x, grid[2][2].center_y));

        return (true, test);
    }

    if (grid[0][2].player == player)
        && (grid[1][1].player == player)
        && (grid[2][0].player == player)
    {
        test.push((grid[0][2].center_x, grid[0][2].center_y));
        test.push((grid[1][1].center_x, grid[1][1].center_y));
        test.push((grid[2][0].center_x, grid[2][0].center_y));
        return (true, test);
    }

    (flag, test)
}
