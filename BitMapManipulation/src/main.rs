#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

fn main() {
    create_first_image();
    create_second_image();
    create_third_image();
    create_heart();
}

fn create_first_image(){
    let resolution = 256;
    let mut img = Image::new(resolution, resolution);

    for y in 0..resolution / 2 {
        for x in 0..resolution / 2{
            img.set_pixel(x, y, px!(255, 255, 255));
            img.set_pixel(resolution/2 + x, y, px!(0, 0, 0));
            img.set_pixel(x, resolution/2+y, px!(0, 0, 0));
            img.set_pixel(resolution/2+x, resolution/2+y, px!(255, 255, 255));
        }
    }
    let _ = img.save("img.bmp");
}

fn create_second_image(){
    let resolution = 256;
    let mut img = Image::new(resolution, resolution);

    for y in 0..resolution / 2 {
        for x in 0..resolution / 2{
            img.set_pixel(x, y, px!(255, 255, 255));
            img.set_pixel(resolution/2+x, resolution/2+y, px!(255, 255, 255));
            
            let mut color = 0;
            if x==y {
                color = 255;
            }
            img.set_pixel(resolution/2 + x, y, px!(color, color, color));
            img.set_pixel(x, resolution/2+y, px!(color, color, color));
        }
    }
    let _ = img.save("img2.bmp");
}


fn create_third_image(){
    let resolution = 256;
    let mut img = Image::new(resolution, resolution);

    for y in 0..resolution / 2 {
        for x in 0..resolution / 2{
            let mut color2 = 255;
            if y%2==0{
                color2 = 0;
            }
            img.set_pixel(x, y, px!(color2, color2, color2));
            img.set_pixel(resolution/2+x, resolution/2+y, px!(color2, color2, color2));
            
            let mut color = 0;
            if x%2==0 {
                color = 255;
            }
            img.set_pixel(resolution/2 + x, y, px!(color, color, color));
            img.set_pixel(x, resolution/2+y, px!(color, color, color));
        }
    }
    let _ = img.save("img3.bmp");
}

fn create_heart(){
    let resolution = 256;
    let mut img = Image::new(resolution, resolution);
    //0..255
    //0..255
    for y in 0..resolution {
        for x in 0..resolution {
            let mapped_x = (x as f32)/((resolution as f32)/4f32);
            let circle = ((1f32 - ((mapped_x-1f32)*(mapped_x-1f32))) as f32).sqrt();
            let new_y = (0.5f32 + ((1f32-circle) * ((resolution as f32)/2f32))) as u32;
            
            if y > new_y{
                img.set_pixel(x,y, px!(255,0,0));
            }else{
                img.set_pixel(x,y, px!(0,0,0));
            }
        }
    }
    let _ = img.save("heart.bmp");
}