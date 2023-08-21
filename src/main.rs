#[allow(unused_assignments)]

use std::f32::consts::PI;
use macroquad::prelude::*;
const MAPW:i32 = 8;
const MAPH:i32 = 8;
const SIZE:i32 = 64;

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
    return a * (PI/180.0);
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

fn main() {
    //window is 1024 512
    let mut px: f32 = 150.0;
    let mut py: f32 = 400.0;
    let mut pa: f32 = 90.0;
    let mut pdx: f32 = deg_rad(pa).cos();
    let mut pdy: f32 = -deg_rad(pa).sin();

    loop {
        if is_key_down(KeyCode::A){ 
            pa+=5.0; 
            pa=fix_angle(pa); 
            pdx=(deg_rad(pa)).cos(); 
            pdy=-(deg_rad(pa)).sin();
        } 	
        if is_key_down(KeyCode::D){ 
            pa-=5.0; pa=fix_angle(pa); 
            pdx=(deg_rad(pa)).cos(); 
            pdy=-(deg_rad(pa)).sin();
        } 
        if is_key_down(KeyCode::W){ 
            px+=pdx*5.0; 
            py+=pdy*5.0;
        }
        if is_key_down(KeyCode::S){ 
            px-=pdx*5.0; 
            py-=pdy*5.0;
        }

        let mut r: i32 = 0;
        let mut mx: i32 = 0;
        let mut my: i32 = 0;
        let mut mp: i32 = 0;
        let mut dof: i32 = 0;
        let mut side: i32 = 0;

        let mut vx: f32 = 0.0;
        let mut vy: f32 = 0.0;
        let mut rx: f32 = 0.0;
        let mut ry: f32 = 0.0;
        let mut ra: f32 = fix_angle(pa+30.0);
        let mut xo: f32 = 0.0;
        let mut yo: f32 = 0.0;
        let mut disV: f32 = 0.0;
        let mut disH: f32 = 0.0;

        while r < 60 {
            disV = 100000.0;
            let mut tan: f32 = deg_rad(ra).tan();

            if deg_rad(ra).cos() > 0.0001 {
                rx = (((px as i32 >> 6)<<6)+64) as f32;
                ry = (px-rx)*tan*py;
                xo = 64.0;
                yo = -xo*tan;
            }
            else if deg_rad(ra).cos() < -0.0001 {
                rx = ((px as i32 >> 6)<<6) as f32-0.0001;
                ry = (px-rx)*tan*py;
                xo = -64.0;
                yo = -xo*tan;
            } else {
                rx=px;
                ry=py;
                dof=8;
            }


            while dof < 8 {
                mx = rx as i32 >> 6;
                my = ry as i32 >> 6;
                mp = my *MAPW + mx;

                if mp>0 && mp<MAPW*MAPH && MAP[mp as usize]==1 { 
                    dof=8; 
                    disV=(deg_rad(ra)).cos()*(rx-px)-(deg_rad(ra)).sin()*(ry-py);
                }
                else{ 
                    rx+=xo; 
                    ry+=yo; 
                    dof+=1;
                }
            }
            vx = rx;
            vy = ry;

            dof = 0;
            disH = 100000.0;

            tan = 1.0/tan;

            if (deg_rad(ra)).sin() as f32 > 0.001 { 
                ry=((py as i32>>6)<<6) as f32; 
                rx=(py-ry)*tan+px;
                yo=-64.0; xo=-yo*tan;
            }//looking up
            else if (deg_rad(ra)).sin()< -0.001 { 
                ry=(((py as i32>>6)<<6)+64) as f32;
                rx=(py-ry)*tan+px; yo= 64.0; 
                xo=-yo*tan;
            }//looking down
            else{ 
                rx=px; 
                ry=py; 
                dof=8;
            }

            while dof > 8 {
                mx=(rx as i32)>>6; 
                my=(ry as i32)>>6; 
                mp=my*MAPW+mx;                          
                if(mp>0 && mp<MAPW*MAPH && MAP[mp as usize]==1){ 
                    dof=8; 
                    disH=(deg_rad(ra)).cos()*(rx-px)-(deg_rad(ra)).sin()*(ry-py);}//hit         
                else{ 
                    rx+=xo; 
                    ry+=yo; 
                    dof+=1;
                }

                //set color to (0.0,0.8,0,0)
                if disV<disH { 
                    rx=vx; 
                    ry=vy; 
                    disH=disV; 
                    //glColor3f(0,0.6,0);
                }

                //optionslly draw line px py rx ry for vidualixation


                let ca: i32=fix_angle(pa-ra) as i32; 
                disH=disH*(deg_rad(ca as f32).cos());                            //fix fisheye 
                let mut lineH: i32 = ((SIZE*320) as f32/(disH)) as i32; 
                if lineH>320 { 
                    lineH=320;
                }        //line height and limit
                let lineOff = 160 - (lineH>>1);

                draw_line((r*8+530) as f32,lineOff as f32,(r*8+530)as f32,(lineOff+lineH) as f32,8.0,RED);

                ra=fix_angle(ra-1.0);
            }
            
            r= 1;
        }
    }
}


fn distance(a:Vec2,b:Vec2,angle:f32) -> f32 {
    return deg_rad(angle).cos()*(b.x-a.x)-(deg_rad(angle).sin())*(b.y-a.y);
}