use macroquad::prelude::*;
use std::f32::consts::PI;

const SCALAR: f32 = 8.0; 

fn window_conf() -> Conf {
    Conf {
        window_title: ":3".to_owned(),
        fullscreen: false,
        window_height: 480,
        window_width: 640,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main () {
    next_frame().await;
    let wall_texture = Image::from_file_with_format(
        include_bytes!("assets/wall.png"),
        Some(ImageFormat::Png),
    );
    let mut player_x: f32 = 1.0;
    let mut player_y: f32 = 1.0;
    let mut player_a: f32 = 0.0;
    let map_height: i32 = 16;
    let map_width: i32 = 16;
    let fov = PI/4.0;
    let screen_width = screen_width();
    let screen_height = screen_height();
    let mut map = vec![
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,
        1,0,0,0,0,0,0,0,0,1,1,1,0,0,0,1,
        1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
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
            player_x += 1.0 * player_a.sin() * player_speed *get_frame_time();
            player_y += 1.0 * player_a.cos() * player_speed *get_frame_time();

            let index = (player_y as i32 * map_width + player_x as i32) as usize;

            if (index as i32) < map_width*map_height {
                if map[index as usize] == 1 {
                    player_x -= player_a.sin() * player_speed * get_frame_time();
                    player_y -= player_a.cos() * player_speed * get_frame_time();
                } 
            }
        }
        if is_key_down(KeyCode::S) {
            player_x -= 1.0 * player_a.sin() * player_speed *get_frame_time();
            player_y -= 1.0 * player_a.cos() * player_speed *get_frame_time();

            let index = (player_y as i32 * map_width + player_x as i32) as usize;

            if (index as i32) < map_width*map_height {
                if map[index] == 1 {
                    player_x += player_a.sin() * player_speed * get_frame_time();
                    player_y += player_a.cos() * player_speed * get_frame_time();
                } 
            }
        }
        if is_key_down(KeyCode::Q) {
            player_x -= 1.0 * player_a.cos() * player_speed *get_frame_time();
            player_y += 1.0 * player_a.sin() * player_speed *get_frame_time();

            let index = (player_y as i32 * map_width + player_x as i32) as usize;

            if (index as i32) < map_width*map_height {
                if map[index] == 1 {
                    player_x += player_a.cos() * player_speed * get_frame_time();
                    player_y -= player_a.sin() * player_speed * get_frame_time();
                } 
            }
        }
        if is_key_down(KeyCode::E) {
            player_x += 1.0 * player_a.cos() * player_speed *get_frame_time();
            player_y -= 1.0 * player_a.sin() * player_speed *get_frame_time();

            let index = (player_y as i32 * map_width + player_x as i32) as usize;

            if (index as i32) < map_width*map_height {
                if map[index] == 1 {
                    player_x -= player_a.cos() * player_speed * get_frame_time();
                    player_y += player_a.sin() * player_speed * get_frame_time();
                } 
            }
        }

        for x in (0..screen_width as i32).step_by(SCALAR as usize) {
            let ray_angle = (player_a-fov/2.0)+(x as f32/screen_width * fov);
            let mut distance_to_wall = 0.0;
            let mut hit_wall = false;
            let mut boundary = false;
            let eye_x = ray_angle.sin();
            let eye_y = ray_angle.cos();

            while !hit_wall && distance_to_wall < depth {
                distance_to_wall += 0.1;

                let test_x = (player_x + eye_x * distance_to_wall) as i32; 
                let test_y = (player_y + eye_y * distance_to_wall) as i32;

                if test_x < 0 || test_x >= map_width || test_y < 0 || test_y >= map_height {
                    hit_wall = true;
                    distance_to_wall = depth;
                }
                else {
                    if map[(test_y * map_width + test_x) as usize] == 1 {
                        hit_wall = true;

                        let mut p : Vec<Vec2> = vec![];

                        for tx in 0..2 {
                            for ty in 0..2 {
                                let vy = test_y as f32 + ty as f32 - player_y;
                                let vx = test_x as f32 + tx as f32 - player_x;
                                let d = (vx*vx + vy*vy).sqrt();
                                let dot = (eye_x*vx/d)+(eye_y*vy/d);

                                p.push(Vec2::new(d,dot));
                            }

                            p = sort_dist(&mut p);

                            let bound = 0.01;
                            if p[0].y.acos() < bound {boundary = true}
                            if p[1].y.acos() < bound {boundary = true}
                        }
                    }
                }
            }

            let ceiling = (screen_height/2.0) - screen_height / distance_to_wall;
            let floor = screen_height - ceiling;

            let mut col = Color::new(0.0,0.0,0.0,0.0);

            if distance_to_wall <= depth / 4.0 { col = Color::new(1.0,0.0,0.0,1.0)}
            else if distance_to_wall < depth / 3.0 { col = Color::new(0.7,0.0,0.0,1.0)}
            else if distance_to_wall < depth / 2.0 { col = Color::new(0.5,0.0,0.0,1.0)}
            else if distance_to_wall < depth { col = Color::new(0.3,0.0,0.0,1.0)}

            if boundary {col = Color::new(0.0,0.0,0.0,1.0)};


            for y in (0..screen_height as i32).step_by(SCALAR as usize) {
                if (y as f32) < ceiling {
                    draw_rectangle(x as f32,y as f32,SCALAR,SCALAR,BLACK);
                } else if y as f32 > ceiling && y as f32<= floor {
                    draw_rectangle(x as f32,y as f32,SCALAR,SCALAR,col);
                } else {
                    let b = 1.0 - ((y as f32/2.0)/(screen_height / 2.0));
                    if b < 0.25 {col = Color::new(0.0,1.0,0.0,1.0)}
                    else if b < 0.5 {col = Color::new(0.0,0.8,0.0,1.0)}
                    else if b < 0.75 {col = Color::new(0.0,0.6,0.0,1.0)}
                    else if b < 0.9 {col = Color::new(0.0,0.4,0.0,1.0)}
                    else {col = Color::new(0.0,0.0,0.0,0.0)}
                    draw_rectangle(x as f32,y as f32,SCALAR,SCALAR,GREEN);
                }
            } 
        }

        next_frame().await
    }
}


fn sort_dist(p:&mut Vec<Vec2>) -> Vec<Vec2> {
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