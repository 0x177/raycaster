//TODO: remove unnecesery type casting

use libm::atan2;
use macroquad::prelude::*;
use std::f32::consts::PI;
// mod images;

const SCALAR: f32 = 16.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "raycaster".to_owned(),
        // window_height: 480,
        // window_width: 640,
        fullscreen: true,
        ..Default::default()
    }
}

struct SceneObject {
    pos: Vec2,
    index: usize,    
}
impl SceneObject {
    fn new(pos:Vec2,index:usize) -> Self {
        Self {
            pos,
            index,
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    next_frame().await;
    let wall_texture =
        Image::from_file_with_format(include_bytes!("../assets/wall.png"), Some(ImageFormat::Png));

    let object_sprite_vec = vec![
        Image::from_file_with_format(include_bytes!("../assets/face.png"), Some(ImageFormat::Png)),
    ];
    let object_list = vec![
        SceneObject::new(Vec2::new(8.5,8.5),0),
    ];
    let mut player_x: f32 = 1.0;
    let mut player_y: f32 = 1.0;
    let mut player_a: f32 = 0.0;
    let map_height: i32 = 16;
    let map_width: i32 = 16;
    let fov = PI / 4.0;
    let screen_width = screen_width()/SCALAR;
    let screen_height = screen_height()/SCALAR;
    let mut buffer = Image::gen_image_color((screen_width) as u16,(screen_height) as u16,BLACK);
    let map = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
        1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 1, 0, 0, 0, 0, 0, 2, 1, 1, 1, 1, 1, 0, 1, 
        1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1,
        1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 
        1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 
        1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    //maximum distance ray can travel
    let depth = 16.0;
    let player_speed = 5.0;

    loop {
        clear_background(BLACK);

        //rotation
        if is_key_down(KeyCode::A) {
            player_a -= 2.0 * get_frame_time();
        }
        if is_key_down(KeyCode::D) {
            player_a += 2.0 * get_frame_time();
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
        //strafing
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

        // for x in (0..screen_width as i32).step_by(SCALAR as usize) {
        for x in 0..(screen_width) as i32-1 {
            let ray_angle = (player_a - fov /2.0) + (x as f32 / screen_width * fov);
            let mut distance_to_wall = 0.0;
            let mut hit_wall = false;
            let eye_x = ray_angle.sin();
            let eye_y = ray_angle.cos();
            //stores offset for the texture. this is used to allow for easily using multiple wall textures
            //not the best variable name.
            let mut val = 1;

            //the x value for the sampler used for texturing
            let mut sample_x = 0.0;

            while !hit_wall && distance_to_wall < depth {
                distance_to_wall += 0.1;

                //get the player's map space location 
                let test_x = (player_x + eye_x * distance_to_wall) as i32;
                let test_y = (player_y + eye_y * distance_to_wall) as i32;

                if test_x < 0 || test_x >= map_width || test_y < 0 || test_y >= map_height {
                    hit_wall = true;
                    distance_to_wall = depth;
                } else {
                    if map[(test_y * map_width + test_x) as usize] >= 1 {
                        hit_wall = true;
                        val = map[(test_y *map_width + test_x) as usize] -1;

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

            for y in 0..(screen_height) as i32 {
                if (y as f32) < ceiling {
                    buffer.set_pixel(x as u32, y as u32, BLACK);
                } else if y as f32 > ceiling && y as f32 <= floor {
                    let sample_y = (y as f32 - ceiling as f32) / (floor - ceiling);
                    if ((sample_y * 32.0 + sample_x) as u32) < 1022 {
                    let col = wall_texture
                        .as_ref()
                        .expect("cant get texture thingy idk why")
                        .get_pixel((sample_x * 31.0) as u32, ((sample_y+val as f32) * 31.0) as u32);
                    buffer.set_pixel(x as u32, y as u32, col);

                    }
                } else {
                    buffer.set_pixel(x as u32, y as u32, GREEN);
                }
            }
            for object in &object_list {
                let vec = Vec2::new(object.pos.x-player_x,object.pos.y-player_y);
                let distance_from_player = ((vec.x*vec.x)+(vec.y*vec.y)).sqrt();

                let eye = Vec2::new(player_a.sin(),player_a.cos());
                let mut object_angle = libm::atan2(eye.y as f64,eye.x as f64) - libm::atan2(vec.y as f64,vec.x as f64);

                if object_angle < -std::f64::consts::PI {
                    object_angle += 2.0 * std::f64::consts::PI;
                }
                if object_angle > std::f64::consts::PI {
                    object_angle -= 2.0 * std::f64::consts::PI;
                }

                let in_player_fov = object_angle.abs() < (fov /2.0) as f64;

                if in_player_fov && distance_from_player >= 0.5 && distance_from_player < depth {
                    let object_top = (screen_height / 2.0) - screen_height / distance_from_player;
                    let object_bottom = screen_height - object_top;
                    let object_height = object_bottom - object_top;
                    let object_aspect_ratio = 32.0/32.0;
                    let object_width = object_height / object_aspect_ratio;

                    let middle_of_object = (0.5 * (object_angle/(fov/2.0) as f64) + 0.5) * screen_width as f64;

                    for lx in 0..object_width as i32 {
                        for ly in 0..(object_height) as i32 {
                            //tempo
                            let val = 0;
                            let sample_x = lx as f32/object_width;
                            let sample_y = ly as f32/object_height;
                            let object_column = (middle_of_object as f32 + lx as f32- (object_width/2.0)) as i32;
                            let col = object_sprite_vec[object.index]
                                .as_ref()
                                .expect("cant get texture thingy idk why")
                                .get_pixel((sample_x * 31.0) as u32, ((sample_y+val as f32) * 31.0) as u32);

                            if object_column > 0 && object_column < screen_width as i32 {
                                buffer.set_pixel(object_column as u32,object_top as u32+ly as u32,col);
                            }
                        }
                    }
                }
            }
        }

        let mut buffer_texture = Texture2D::from_image(&buffer);
        buffer_texture.set_filter(FilterMode::Nearest);
        // buffer = images::nearest(screen_width as i32,screen_height as i32,(screen_width*SCALAR) as i32,(screen_height*SCALAR) as i32,&buffer);
        draw_texture_ex(&buffer_texture,0.0,0.0,WHITE,DrawTextureParams {
                dest_size: Some(vec2(screen_width*SCALAR, screen_height*SCALAR)),
                ..Default::default()
            },
        );
        // draw_text(&get_fps().to_string(),20.0,20.0,32.0,GREEN);

        next_frame().await
    }
}

