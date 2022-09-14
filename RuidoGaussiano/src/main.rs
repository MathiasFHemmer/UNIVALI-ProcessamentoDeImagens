mod args;
mod hmath;
mod imgops;

use std::{collections::HashMap, path::PathBuf};
use image::{GenericImageView, DynamicImage};
use clap::Parser;
use args::{Arguments, GaussianNoiseCommand, ConvoluteCommand, EdgeDetectionCommand, SobelKernelSize, HistogramCommand};
use hmath::{parse_convolution_command, psnr};
use imgops::{edge_detection, convolute};
use show_image::{create_window, event};
use plotters::prelude::*;

const DEFAULT_IMAGE_OUTPUT_FOLDER: &str = "output_image";
const DEFAULT_IMAGE_OUTPUT_NAME: &str = "out";

#[show_image::main]
fn main() {

    let args = Arguments::parse();
    let operation = &args.operation;

    match operation {
        args::OperationType::GaussianNoise(cmd) => gaussian_noise_command_handler(&args, &cmd),
        args::OperationType::Histogram(cmd) => histogram_command_handler(&args, &cmd),
        args::OperationType::Convolute(cmd) => convolute_command_handler(&args, &cmd),
        args::OperationType::EdgeDetection(cmd) => edge_detection_command_handler(&args, &cmd)
    }
    return;
}

fn get_image_output_folder(args: &Arguments) -> PathBuf{
    let out = args.output_folder.clone();

    match out {
        Some(path) => {
            match path.contains("/") || path.contains("\\") {
                true => PathBuf::from(path),
                false => PathBuf::from(format!("{}/{}", DEFAULT_IMAGE_OUTPUT_FOLDER, path)),
            }
        },
        None => PathBuf::from(format!("{}/{}.png", DEFAULT_IMAGE_OUTPUT_FOLDER, DEFAULT_IMAGE_OUTPUT_NAME))
    }
}

fn gaussian_noise_command_handler(args: &Arguments, cmd: &GaussianNoiseCommand){
    let image_folder = &args.image_folder;
    let img = image::open(image_folder).unwrap();
    
    let std_dev = cmd.std_dev as f64;
    let img_out = imgops::white_noise_generator(&img, std_dev, cmd.grayscale);

    let psnr = psnr(&img, &img_out);
    println!("PSNR: {}", psnr);

    match args.show_image {
        true => show_image(img, img_out),
        false => img_out.save(get_image_output_folder(&args)).unwrap(),
    }
}

fn histogram_command_handler(args: &Arguments, cmd: &HistogramCommand){
    let folder = get_image_output_folder(args);
    let root = BitMapBackend::new(&folder, (640, 480)).into_drawing_area();

    let image_folder = &args.image_folder;
    let img = image::open(image_folder).expect("File not found!");

    let img = match cmd.grayscale {
        true => img.grayscale(),
        false => img,
    };

    let data = create_histogram_data(&img);

    let max = data.iter()
    .fold(HashMap::<u8, usize>::new(), |mut m, x| {
        *m.entry(*x).or_default() += 1;
        m
    })
    .into_iter()
    .max_by_key(|(_, v)| *v)
    .map(|(_, v)| v)
    .unwrap();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(25)
        .y_label_area_size(25)
        .margin(5)
        .caption("Histograma", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..255u32).into_segmented(), 0u32..((max + max/10) as u32)).unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw().unwrap();

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x| (*x as u32, 1))),
    ).unwrap();

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    match args.show_image {
        true => {
            let image = image::open(&folder).unwrap();
            show_image(img, image);
            std::fs::remove_file(&folder).unwrap();
            ()
        },
        false => (),
    }
}

fn create_histogram_data(image: &DynamicImage) -> Vec<u8>{
    let mut histogram = Vec::new();

    for (_, _ , pixel) in image.pixels(){
        histogram.push(pixel[0]);
    }

    return histogram;
}

fn convolute_command_handler(args: &Arguments, cmd: &ConvoluteCommand){
    let kernel = parse_convolution_command(&cmd).unwrap();
    let image_folder = &args.image_folder;
    let img = image::open(image_folder).unwrap();

    let convolutade_image = convolute(kernel, &img);

    match args.show_image {
        true => show_image(img, convolutade_image),
        false => convolutade_image.save(get_image_output_folder(&args)).unwrap(),
    }
}

fn edge_detection_command_handler(args: &Arguments, cmd: &EdgeDetectionCommand){
    let image_folder = &args.image_folder;
    let image = image::open(image_folder).unwrap();
    
    let std_dev = cmd.std_dev.unwrap_or(16) as f64;
    let kernel = cmd.kernel_size.clone().unwrap_or(SobelKernelSize::Small);
    let image_output = edge_detection(&image, std_dev, kernel);

    match args.show_image {
        true => show_image(image, image_output),
        false => image_output.save(get_image_output_folder(&args)).unwrap(),
    }
}

fn show_image(original: DynamicImage, image: DynamicImage){
    let original_window = create_window("Imagem Original", Default::default()).unwrap();
    let processed_window = create_window("Imagem Processada", Default::default()).unwrap();
    
    original_window.set_image("v1", original).unwrap();
    processed_window.set_image("v2", image).unwrap();

    for event in original_window.event_channel().unwrap() {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }
    ();
}
