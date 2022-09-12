#[macro_use]
extern crate bmp;
use std::{fs, env::{args}};

use bmp::{Image, Pixel, open};

static output_folder:&'static str = "img_out";

enum FirstLevelOperations{
    SumRGBValues,
    Invalid
}

fn main() {
    let mut operation = FirstLevelOperations::Invalid;

    let mut args = args().collect::<Vec<String>>();

    if(args.len() >= 1){
        args.remove(0);
        match args[0].as_str() {
            "sum_rgb_values" => operation = FirstLevelOperations::SumRGBValues,
            _ => {
                    print!("No argument was provide!\n");
                    print!("Available commands:\n");
                    print!("sum_rgb_values 'path/to/img' 'path/to/img' [output/path]\n");
                }
        }

        args.remove(0);
    }else{
        print!("No argument was provide!\n");
        print!("Available commands:\n");
        print!("sum_rgb_values 'path/to/img' 'path/to/img' [output/path]\n");
    }

    match operation {
        FirstLevelOperations::SumRGBValues => sum_rgb_values_handle(args),
        _ => {}
    }
}

fn create_first_image(file_name: &str, resolution:i32){
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
            if x + y == (resolution/2) - 1{
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

fn sum_rgb_values_handle(args: Vec<String>){
    let mut out_file = output_folder;
    let input_file_01 : &str;
    let input_file_02 : &str;
    
    if args.len() >=3 {
        input_file_01 = args[0].as_str();
        input_file_02 = args[1].as_str();
        out_file = args[2].as_str();
    }else if args.len() >=2 {
        input_file_01 = args[0].as_str();
        input_file_02 = args[1].as_str();
    }else{
        print!("Missing image directory to search for!");
        return ;
    }
    let mut directory = out_file.split("/").collect::<Vec<&str>>();
    directory.pop();
    let dir = directory.join("/");
    fs::create_dir_all(dir);
    sum_rgb_values(input_file_01, input_file_02, out_file);
}


fn sum_rgb_values(file_1: &str, file_2: &str, out_file: &str){
    let img1 = open(file_1).unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
     });
    let img2 = open(file_2).unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });
    
    let res_x = img1.get_width();
    let res_y = img1.get_height();
    
    let mut out = Image::new(res_x, res_y);
    for y in 0..res_y{
        for x in 0..res_x{
            let px1 = img1.get_pixel(x, y);
            let px2 = img2.get_pixel(x, y);

            let new_pixel = sum_pixels(px1, px2);
            out.set_pixel(x,y, new_pixel);
        }
    }
    let mut directory_and_file = out_file.to_string();
    if out_file.ends_with(".bmp") == false{
        directory_and_file += ".bmp";
    };

    out.save(directory_and_file).unwrap_or_else(|e| {panic!("Failed to open: {}", e)});
}

fn sum_pixels(p: Pixel, q:Pixel) -> Pixel{  
    let r = p.r.saturating_add(q.r);
    let g = p.g.saturating_add(q.g);
    let b = p.b.saturating_add(q.b);
    let pixel = px!(r,g,b);
    return pixel;
}

fn evaluate_neighbor_4(matrix: &mut Vec<Vec<u8>> ) -> u32{
    for y in 0..matrix.len(){
        for x in 0..matrix[y].len(){
            
        }
    }

    return 32;
}