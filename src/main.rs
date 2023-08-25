#![allow(unused_assignments)]

use std::f32::consts::PI;
use macroquad::prelude::*;
const MAPW:i32 = 8;
const MAPH:i32 = 8;
const SIZE:i32 = 64;
const PI2:f32 = PI/2.0;
const PI3:f32 = 3.0*PI/2.0;
const DR:f32 = 0.01745329;

const MAP: &'static [i32] = &[
    1,1,1,1,1,1,1,1,
    1,0,1,0,0,0,0,1,
    1,0,1,0,0,0,0,1,
    1,0,1,0,0,0,0,1,
    1,0,0,0,0,0,0,1,
    1,0,0,0,0,1,0,1,
    1,0,0,0,0,0,0,1,
    1,1,1,1,1,1,1,1,
];

fn deg_rad(a:f32) -> f32 {
    return a * PI/180.0;
}
fn fix_angle(mut a:f32) -> f32{
    if a > 359.0 {
        a-=360.0;
    }
    if a < 0.0 {
        a+=360.0;
    }

    return a;
}

fn draw_map_2d() {
    let mut xo;
    let mut yo;
    let mut c = Color::new(0.0,0.0,0.0,0.0);

    for y in 0..MAPH{
        for x in 0..MAPW {
            c = Color::new(0.0,0.0,0.0,1.0);
            if MAP[(y*MAPW + x) as usize] == 1 {
                c = Color::new(1.0,1.0,1.0,1.0);
            }
            xo = x*SIZE;
            yo = y*SIZE;

            draw_rectangle(xo as f32,yo as f32,(SIZE-1) as f32,(SIZE-1) as f32,c);
                        
        }
    }
}

fn draw_rays_3d(pa:f32,px:f32,py:f32) {
    let (mut r, mut mx,mut my,mut mp,mut dof,mut dis_f): (i32,i32,i32,i32,i32,i32) = (0,0,0,0,0,0); // i32
    let (mut rx,mut ry,mut ra,mut xo,mut yo): (f32,f32,f32,f32,f32) = (0.0,0.0,0.0,0.0,0.0); // f32
    ra=pa-DR*30.0;
    if ra <0.0 {ra+=2.0*PI}
    if ra>2.0*PI {ra-=2.0*PI}


    r = 0;

    while r < 60 {

        //check hor lines
                
        dof = 0;
        let mut dis_h = 100000.0;
        let mut hx = px;
        let mut hy = py;
        let atan = -1.0/ra.tan();
        if ra>PI {ry = ((py as i32>>6)<<6) as f32 - 0.0001; rx=(py-ry)*atan+px;yo=-64.0;xo=-yo*atan}
        if ra<PI {ry = ((py as i32>>6)<<6) as f32 + 64.0; rx=(py-ry)*atan+px;yo= 64.0;xo=-yo*atan}

        if ra == 0.0 || ra == PI {rx =px;ry=py;dof=8}

        while dof<8 {
            mx = rx as i32>>6; my = ry as i32 >>6; mp=my*MAPW+mx;
            
            if mp<MAPW*MAPH && mp>0&&MAP[mp as usize] ==1 {hx=rx;hy=ry;dis_h=distance(Vec2::new(px,py),Vec2::new(hx,hy),ra);dof=8}
            else {rx += xo; ry += yo; dof += 1}
        }


        
        dof = 0;
        let mut dis_v = 100000.0;
        let mut vx = px;
        let mut vy = py;
        let ntan = -ra.tan();
        if ra>PI2 && ra<PI3 {rx = ((px as i32>>6)<<6) as f32 - 0.0001; ry=(px-rx)*ntan+py;xo=-64.0;yo=-xo*ntan}
        if ra<PI2 || ra>PI3 {rx = ((px as i32>>6)<<6) as f32 + 64.0; ry=(px-rx)*ntan+py;xo= 64.0;yo=-xo*ntan}

        if ra == 0.0 || ra == PI {rx =px;ry=py;dof=8}

        while dof<8 {
            mx = rx as i32>>6; my = ry as i32 >>6; mp=my*MAPW+mx;
            
            if mp<MAPW*MAPH && mp>0&&MAP[mp as usize] ==1 {vx=rx;vy=ry;dis_v=distance(Vec2::new(px,py),Vec2::new(vx,vy),ra);dof=8}
            else {rx += xo; ry += yo; dof += 1}
        }

        if dis_v<dis_h {rx=vx;ry=vy;dis_f=dis_v as i32}
        if dis_h<dis_v {rx=hx;ry=hy;dis_f=dis_h as i32}

        draw_line(px,py,rx,ry,5.0,RED);
        if dis_f == 0 {dis_f = 1}

        let mut line_h = (SIZE as f32*320.0)/dis_f as f32; if line_h> 320.0 {line_h=320.0}
        draw_line(r as f32*8.0+530.0,0.0,r as f32*8.0+530.0,line_h as f32,8.0,RED);
        
        ra += DR;
        if ra>PI {ry = ((py as i32>>6)<<6) as f32 - 0.0001; rx=(py-ry)*atan+px;yo=-64.0;xo=-yo*atan}
        if ra<PI {ry = ((py as i32>>6)<<6) as f32 + 64.0; rx=(py-ry)*atan+px;yo= 64.0;xo=-yo*atan}
        r += 1;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: ":3".to_owned(),
        fullscreen: false,
        window_height: 510,
        window_width: 1024,
        ..Default::default()
    }
}

fn draw_player(x:f32,y:f32,dx:f32,dy:f32,a:f32) {
    draw_circle(x,y,8.0,YELLOW);
    draw_line(x,y,x+dx*5.0,y+dy*5.0,3.0,YELLOW);
    draw_rays_3d(a,x,y);
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut px = 300.0;
    let mut py = 300.0;
    let mut pdx = 0.0;
    let mut pdy = 0.0;
    let mut pa = 0.0;
    loop {
        clear_background(GRAY);

        if is_key_down(KeyCode::A) {pa -= 0.1; if pa < 0.0 {pa += 2.0*PI} pdx = pa.cos() * 5.0;pdy=pa.sin()*5.0}
        if is_key_down(KeyCode::D) {pa += 0.1; if pa > 2.0*PI {pa -= 2.0*PI} pdx = pa.cos() * 5.0;pdy=pa.sin()*5.0}
        if is_key_down(KeyCode::W) {px += pdx; py += pdy;}
        if is_key_down(KeyCode::S) {px -= pdx; py -= pdy;}



        draw_map_2d();       
        draw_player(px,py,pdx,pdy,pa);
    
        next_frame().await
    }
}


fn distance(a:Vec2,b:Vec2,angle:f32) -> f32 {
    return deg_rad(angle).cos()*(b.x-a.x)-(deg_rad(angle).sin())*(b.y-a.y);
}