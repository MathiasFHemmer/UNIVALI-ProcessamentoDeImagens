use crate::{hmath, args::SobelKernelSize};

use image::{DynamicImage, GenericImageView, Rgb};
use hmath::{normal_map_2d, DensePixel, kernel_blur_gaussian};

pub fn white_noise_generator(original: &image::DynamicImage, std_dev: f64, gray: bool) -> DynamicImage{
    let image_to_use = match gray { true => original.grayscale(), false => original.clone() };

    let (w, h) = image_to_use.dimensions();
    let mut gaussian_image = image::RgbImage::new(w,h);

    let normal_distribution_map = normal_map_2d(w, h, std_dev);
    let mut max_values = DensePixel::from_single_i32(i32::min_value());
    let mut min_values = DensePixel::from_single_i32(i32::max_value());
    
    let mut new_pixel = Vec::new();
    for y in 0..h {
        let mut new_pixel_row  = Vec::new();
        for x in 0..w{
            let pixel = image_to_use.get_pixel(x, y);
            let noise = normal_distribution_map[x as usize][y as usize];
            
            let dense_pixel = DensePixel::from_i32(pixel[0] as i32 + noise, pixel[1] as i32 + noise, pixel[2] as i32 + noise);
            max_values = max_values.max(&dense_pixel);
            min_values = min_values.min(&dense_pixel);

            new_pixel_row.push(dense_pixel);
        }
        new_pixel.push(new_pixel_row);
    }

    for y in 0..h {
        for x in 0..w{
            let noise = &new_pixel[y as usize][x as usize];
            let mapped_pixel = DensePixel::from_minmax_limit(noise, &min_values, &max_values, 255.0);
            gaussian_image.put_pixel(x, y, image::Rgb([mapped_pixel.r as u8, mapped_pixel.r as u8, mapped_pixel.r as u8]));
        }
    }   
    return DynamicImage::ImageRgb8(gaussian_image);
}

pub fn convolute(mask: Vec<Vec<f32>>, image: &DynamicImage) -> DynamicImage{
    let img_dim = image.dimensions();
    
    let mut total = 0.0;
    for x in mask.iter(){
        for y in x.iter(){
            total += *y;
        }
    }

    let mut convoluted_image = image::RgbImage::new(img_dim.0,img_dim.1);

    for (px, py, _) in image.pixels(){
        let mut sum= 0.0;
        for (y, row) in mask.iter().enumerate(){
            for (x, val) in row.iter().enumerate(){
                let px_index_x = px as i32 + (x as i32 - (row.len() as i32 / 2));
                let px_index_y = py as i32 + (y as i32 - (mask.len() as i32 / 2));

                let mut pixel = image::Rgba([0, 0, 0, 0]);
                if px_index_x >= 0 && px_index_x < img_dim.0 as i32 && px_index_y >= 0 && px_index_y < img_dim.1 as i32{
                    pixel = image.get_pixel(px_index_x as u32, px_index_y as u32);
                }
                sum += pixel[0] as f32 * *val;
            }
        }

        let mut avarage = (sum / mask.len() as f32).abs() as u8;
        if total > 0.0 {
            avarage = (sum / total).abs() as u8;
        }

        let convoluted_pixel = Rgb([avarage as u8, avarage as u8, avarage as u8]);
        convoluted_image.put_pixel(px, py, convoluted_pixel);
    }
    return image::DynamicImage::ImageRgb8(convoluted_image);
}

pub fn edge_detection(image: &DynamicImage, std_dev: f64, kernel_size: SobelKernelSize) -> DynamicImage{
    let grayscale_image = image.grayscale();

    let noised_image = white_noise_generator(&grayscale_image, std_dev, true);

    let blur_convolution = kernel_blur_gaussian();    
    let vertical_edge_mask = match kernel_size {
        SobelKernelSize::Small => hmath::kernel_sobel_vertical_3(),
        SobelKernelSize::Medium => hmath::kernel_sobel_vertical_5(),
    };
    let horizontal_edge_mask = match kernel_size {
        SobelKernelSize::Small => hmath::kernel_sobel_horizontal_3(),
        SobelKernelSize::Medium => hmath::kernel_sobel_horizontal_5(),
    };
    
    let blurred_image = convolute(blur_convolution, &noised_image);
    let horizontal_edges = convolute(horizontal_edge_mask, &blurred_image);
    let vertical_edges = convolute(vertical_edge_mask, &blurred_image);

    let dimension = blurred_image.dimensions();
    let mut edge_image = image::RgbImage::new(dimension.0,dimension.1);

    for x in 0..dimension.0{
        for y in 0..dimension.1{
            let horizontal_pixel = horizontal_edges.get_pixel(x, y);
            let vertical_pixel = vertical_edges.get_pixel(x, y);

            let r = horizontal_pixel[0].saturating_add(vertical_pixel[0]);
            let g = horizontal_pixel[1].saturating_add(vertical_pixel[1]);
            let b = horizontal_pixel[2].saturating_add(vertical_pixel[2]);

            edge_image.put_pixel(x, y, Rgb([r,g,b]));
        }
    }
    return image::DynamicImage::ImageRgb8(edge_image);
}