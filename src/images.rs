use macroquad::texture::Image;
use macroquad::color::Color;
use macroquad::prelude::BLACK;


pub fn nearest(w1:i32,h1:i32,w2:i32,h2:i32,image:&Image) -> Image {
    let mut temp = Image::gen_image_color(w2 as u16,h2 as u16,BLACK);
    let x_ratio = ((w1<<16)/w2) as i32+1;
    let y_ratio = ((h1<<16)/h2) as i32+1;

    let mut x2;
    let mut y2;

    for i in 0..w2 {
        for j in 0..h2 {
            x2 = ((j*x_ratio)>>16);
            y2 = ((i*y_ratio)>>16);
            println!("{} {} {} {}",i as u32,j as u32,x2 as u32,y2 as u32);
            temp.set_pixel(i as u32,j as u32,image.get_pixel(x2 as u32,y2 as u32));
        }
    }
    return temp;
}