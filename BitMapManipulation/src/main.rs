#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};
use std::fs;

static output_folder:&'static str = "img_out";

fn main() {
    fs::create_dir_all(output_folder);

    create_first_image("img_01");
    create_second_image("img_02");
    create_third_image("img_03");
    create_heart("img_04");
}

fn create_first_image(file_name: &str){
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
    let _ = img.save(format!("{output_folder}/{file_name}"));
}

fn create_second_image(file_name: &str){
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
    let _ = img.save(format!("{output_folder}/{file_name}"));
}


fn create_third_image(file_name: &str){
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
    let _ = img.save(format!("{output_folder}/{file_name}"));
}

fn create_heart(file_name: &str){
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
    let _ = img.save(format!("{output_folder}/{file_name}"));
}