use image::{DynamicImage, GenericImageView};
use rand_distr::{Normal, Distribution, num_traits::Pow};

use crate::args::ConvoluteCommand;

#[derive(Clone)]
pub struct DensePixel{
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl DensePixel {
    pub fn max(self, other: &DensePixel) -> DensePixel {
        return DensePixel::new(self.r.max(other.r), self.g.max(other.g), self.b.max(other.b))
    }
    pub fn min(&mut self, other: &DensePixel) -> DensePixel{
        return DensePixel::new(self.r.min(other.r), self.g.min(other.g), self.b.min(other.b))
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self{
        return Self{r,g,b};
    }

    pub fn from_i32(r: i32, g: i32, b: i32) -> Self{
        return Self{r: f64::from(r),g: f64::from(g),b: f64::from(b)};
    }

    pub fn from_single_i32(value: i32) -> Self{
        return Self { r: f64::from(value), g: f64::from(value), b: f64::from(value) }
    }

    pub fn from_minmax_limit(value: &DensePixel, min: &DensePixel, max: &DensePixel, limit: f64) -> Self{
        return Self { 
            r: (value.r + min.r.abs()) / (min.r.abs() + max.r) * limit,  
            g: (value.g + min.g.abs()) / (min.g.abs() + max.g) * limit, 
            b: (value.b + min.b.abs()) / (min.b.abs() + max.b) * limit 
        }
    }
}

pub fn normal_map_2d(heigth:u32, width:u32, std_dev: f64) -> Vec<Vec<i32>>{
    let normal = Normal::new(2f64, std_dev).unwrap();
    let mut normal_distribution_map : Vec<Vec<i32>> = Vec::new();
    for _ in 0..heigth {
        let mut distribution_row = Vec::new();
        for _ in 0..width{
            distribution_row.push(normal.sample(&mut rand::thread_rng()) as i32);
        }
        normal_distribution_map.push(distribution_row);
    }
    return normal_distribution_map;
}

pub fn parse_convolution_command(cmd: &ConvoluteCommand) -> Option<Vec<Vec<f32>>>{
    let kernel_dim = cmd.kernel.len() as u32;
    match cmd.dimension * cmd.dimension {
        dim if kernel_dim == dim as u32 => {
            let mut kernel = Vec::new();
            for i in 0..cmd.dimension{
                let mut kernel_row = Vec::new();
                for j in 0..cmd.dimension{
                    kernel_row.push(cmd.kernel[j + i*cmd.dimension]);
                }
                kernel.push(kernel_row);
            }
            return Some(kernel);
        },
        _ if kernel_dim == 1 => {
            let mut kernel = Vec::new();
            for _ in 0..cmd.dimension{
                let mut kernel_row = Vec::new();
                for _ in 0..cmd.dimension{
                    kernel_row.push(cmd.kernel[0]);
                }
                kernel.push(kernel_row);
            }
            return Some(kernel);
        },
        _ if kernel_dim == cmd.dimension as u32 => {
            let mut kernel = Vec::new();
            for _ in 0..cmd.dimension{
                let mut kernel_row = Vec::new();
                for j in 0..cmd.dimension{
                    kernel_row.push(cmd.kernel[j]);
                }
                kernel.push(kernel_row);
            }
            return Some(kernel);
        },
        _ => None
    }
}
/*pub fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}*/

pub fn kernel_blur_gaussian() -> Vec<Vec<f32>>{
    return vec![
        vec![1.0, 4.0, 7.0, 4.0, 1.0], 
        vec![4.0, 1.6, 26.0, 16.0, 4.0], 
        vec![7.0, 26.0, 41.0, 26.0, 7.0],
        vec![4.0, 1.0, 26.0, 16.0, 4.0],
        vec![1.0, 4.0, 7.0, 4.0, 1.0]
        ];
}

pub fn kernel_sobel_vertical_5() -> Vec<Vec<f32>> {
    return vec![
        vec![2.0, 1.0, 0.0, -1.0, -2.0], 
        vec![2.0, 1.0, 0.0, -1.0, -2.0], 
        vec![4.0, 2.0, 0.0, -2.0, -4.0],
        vec![2.0, 1.0, 0.0, -1.0, -2.0],
        vec![2.0, 1.0, 0.0, -1.0, -2.0]];
}

pub fn kernel_sobel_horizontal_5() -> Vec<Vec<f32>> {
    return vec![
        vec![2.0, 2.0, 4.0, 2.0, 2.0], 
        vec![1.0, 1.0, 2.0, 1.0, 1.0], 
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![-1.0, -1.0, -2.0, -1.0, -1.0],
        vec![-2.0, -2.0, -4.0, -2.0, -2.0]];
}
pub fn kernel_sobel_horizontal_3() -> Vec<Vec<f32>> {
     return vec![
        vec![1.0, 2.0, 1.0], 
        vec![0.0, 0.0, 0.0], 
        vec![-1.0, -2.0, -1.0]];
    }
pub fn kernel_sobel_vertical_3() -> Vec<Vec<f32>> {
    return vec![
        vec![1.0, 0.0, -1.0], 
        vec![2.0, 0.0, -2.0], 
        vec![1.0, 0.0, -1.0]];
}

pub fn psnr(original_image: &DynamicImage, processed_image: &DynamicImage) -> f64{
    let dimensions = original_image.dimensions();

    let max_intensity: f64 = match original_image {
        DynamicImage::ImageLuma8(_) => (u8::MAX as f64).pow(2),
        DynamicImage::ImageLumaA8(_) => (u8::MAX as f64).pow(2), 
        DynamicImage::ImageRgb8(_) => (u8::MAX as f64).pow(2),
        DynamicImage::ImageRgba8(_) => (u8::MAX as f64).pow(2), 
        DynamicImage::ImageLuma16(_) => (u16::MAX as f64).pow(2),
        DynamicImage::ImageLumaA16(_) => (u16::MAX as f64).pow(2),
        DynamicImage::ImageRgb16(_) => (u16::MAX as f64).pow(2),
        DynamicImage::ImageRgba16(_) => (u16::MAX as f64).pow(2),
        DynamicImage::ImageRgb32F(_) => (u32::MAX as f64).pow(2),
        DynamicImage::ImageRgba32F(_) => (u32::MAX as f64).pow(2),
        _ => 0.0,
    };

    let avarage_factor = 1.0 / (dimensions.0 * dimensions.1) as f64;

    let mut sum = 0f64;
    for (x, y, o_pixel) in original_image.pixels(){
        let p_pixel = processed_image.get_pixel(x, y);
        sum += (o_pixel[0] as f64 - p_pixel[0] as f64).pow(2);
    }
    
    let mse = sum * avarage_factor;
    let ratio = max_intensity / mse;


    return 10.0 * ratio.log10();
}