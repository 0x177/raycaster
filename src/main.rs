use libm::atan2;
use macroquad::prelude::*;
use std::f32::consts::PI;

const SCALAR: f32 = 6.0;

fn window_conf() -> Conf {
    Conf {
        window_title: ":3".to_owned(),
        // window_height: 480,
        // window_width: 640,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    next_frame().await;
    let wall_texture =
        Image::from_file_with_format(include_bytes!("../assets/wall.png"), Some(ImageFormat::Png));
    let mut player_x: f32 = 1.0;
    let mut player_y: f32 = 1.0;
    let mut player_a: f32 = 0.0;
    let map_height: i32 = 16;
    let map_width: i32 = 16;
    let fov = PI / 4.0;
    let screen_width = screen_width();
    let screen_height = screen_height();
    let mut map = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
        1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 
        1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 
        1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 
        1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 
        1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 
        1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    let depth = 16.0;
    let player_speed = 5.0;
    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::A) {
            player_a -= 1.0 * get_frame_time();
        }
        if is_key_down(KeyCode::D) {
            player_a += 1.0 * get_frame_time();
        }
        if is_key_down(KeyCode::W) {
            player_x += 1.0 * player_a.sin() * player_speed * get_frame_time();
            player_y += 1.0 * player_a.cos() * player_speed * get_frame_time();

            let index = (player_y as i32 * map_width + player_x as i32) as usize;

            if (index as i32) < map_width * map_height {
                if map[index as usize] == 1 {
                    player_x -= player_a.sin() * player_speed * get_frame_time();
                    player_y -= player_a.cos() * player_speed * get_frame_time();
                }
            }
        }
        if is_key_down(KeyCode::S) {
            player_x -= 1.0 * player_a.sin() * player_speed * get_frame_time();
            player_y -= 1.0 * player_a.cos() * player_speed * get_frame_time();

            let index = (player_y as i32 * map_width + player_x as i32) as usize;

            if (index as i32) < map_width * map_height {
                if map[index] == 1 {
                    player_x += player_a.sin() * player_speed * get_frame_time();
                    player_y += player_a.cos() * player_speed * get_frame_time();
                }
            }
        }
        if is_key_down(KeyCode::Q) {
            player_x -= 1.0 * player_a.cos() * player_speed * get_frame_time();
            player_y += 1.0 * player_a.sin() * player_speed * get_frame_time();

            let index = (player_y as i32 * map_width + player_x as i32) as usize;

            if (index as i32) < map_width * map_height {
                if map[index] == 1 {
                    player_x += player_a.cos() * player_speed * get_frame_time();
                    player_y -= player_a.sin() * player_speed * get_frame_time();
                }
            }
        }
        if is_key_down(KeyCode::E) {
            player_x += 1.0 * player_a.cos() * player_speed * get_frame_time();
            player_y -= 1.0 * player_a.sin() * player_speed * get_frame_time();

            let index = (player_y as i32 * map_width + player_x as i32) as usize;

            if (index as i32) < map_width * map_height {
                if map[index] == 1 {
                    player_x -= player_a.cos() * player_speed * get_frame_time();
                    player_y += player_a.sin() * player_speed * get_frame_time();
                }
            }
        }
        if is_key_down(KeyCode::Escape) {
            break;
        }

        for x in (0..screen_width as i32).step_by(SCALAR as usize) {
            let ray_angle = (player_a - fov / 2.0) + (x as f32 / screen_width * fov);
            let mut distance_to_wall = 0.0;
            let mut hit_wall = false;
            let eye_x = ray_angle.sin();
            let eye_y = ray_angle.cos();

            let mut sample_x = 0.0;

            while !hit_wall && distance_to_wall < depth {
                distance_to_wall += 0.1;

                let test_x = (player_x + eye_x * distance_to_wall) as i32;
                let test_y = (player_y + eye_y * distance_to_wall) as i32;

                if test_x < 0 || test_x >= map_width || test_y < 0 || test_y >= map_height {
                    hit_wall = true;
                    distance_to_wall = depth;
                } else {
                    if map[(test_y * map_width + test_x) as usize] == 1 {
                        hit_wall = true;

                        let block_mid_x = test_x as f32 + 0.5;
                        let block_mid_y = test_y as f32 + 0.5;

                        let test_point_x = player_x + eye_x * distance_to_wall;
                        let test_point_y = player_y + eye_y * distance_to_wall;

                        let test_angle = atan2(
                            (test_point_y - block_mid_y) as f64,
                            (test_point_x - block_mid_x) as f64,
                        );

                        if test_angle >= (-PI * 0.25) as f64 && test_angle < (PI * 0.25) as f64 {
                            sample_x = test_point_y - test_y as f32;
                        }
                        if test_angle >= (PI * 0.25) as f64 && test_angle < (PI * 0.75) as f64 {
                            sample_x = test_point_x - test_x as f32;
                        }
                        if test_angle < (-PI * 0.25) as f64 && test_angle >= (-PI * 0.75) as f64 {
                            sample_x = test_point_x - test_x as f32;
                        }
                        if test_angle >= (PI * 0.75) as f64 || test_angle < (-PI * 0.75) as f64 {
                            sample_x = test_point_y - test_y as f32;
                        }
                    }
                }
            }

            let ceiling = (screen_height / 2.0) - screen_height / distance_to_wall;
            let floor = screen_height - ceiling;

            for y in (0..(screen_height) as i32).step_by(SCALAR as usize) {
                if (y as f32) < ceiling {
                    draw_rectangle(x as f32, y as f32, SCALAR, SCALAR, BLACK);
                } else if y as f32 > ceiling && y as f32 <= floor {
                    let sample_y = (y as f32 - ceiling as f32) / (floor - ceiling);
                    if ((sample_y * 32.0 + sample_x) as u32) < 1022 {
                        let col = wall_texture
                            .as_ref()
                            .expect("cant get texture thingy idk why")
                            .get_pixel((sample_x * 31.0) as u32, (sample_y * 31.0) as u32);
                        draw_rectangle(x as f32, y as f32, SCALAR, SCALAR, col);
                    }
                } else {
                    draw_rectangle(x as f32, y as f32, SCALAR, SCALAR, GREEN);
                }
            }
        }

        next_frame().await
    }
}

fn sort_dist(p: &mut Vec<Vec2>) -> Vec<Vec2> {
    for i in 0..p.len() {
        for j in 0..p.len() {
            if p[i as usize].x < p[j as usize].x {
                let temp = p[j as usize];

                p[j as usize] = p[i as usize];
                p[i as usize] = temp;
            }
        }
    }

    return p.clone();
}
