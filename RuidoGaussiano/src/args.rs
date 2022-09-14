use std::{str::FromStr};

use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Arguments{
    /// Specify wich image you want to work with
    pub image_folder: String,
    /// Specify a folder to output the operation
    #[clap(short, long)]
    pub output_folder: Option<String>,
    #[clap(subcommand)]
    pub operation: OperationType,
    /// Displays the image instead of saving it
    #[clap(long, action)]
    pub show_image: bool
}

#[derive(Debug, Subcommand)]
pub enum OperationType{
    /// Apply a gaussian noise to the image
    GaussianNoise(GaussianNoiseCommand),
    /// Creates a histogram of the image
    Histogram(HistogramCommand),
    /// Applies a convolution operator based on supplied Kernel. Kernel must be an odd-sized 2N Vector (1x1, 3x3, 5x5...)
    Convolute(ConvoluteCommand),
    /// Tries to do an edge detection
    EdgeDetection(EdgeDetectionCommand)
}

#[derive(Debug, Args)]
pub struct GaussianNoiseCommand{
    /// The standard deviation used to generate the normal distribution
    #[clap(short, long)]
    pub std_dev: u16,
    /// Should the image be converted to grayscale
    #[clap(short, long, default_value_t=false, action)]
    pub grayscale: bool
}

#[derive(Debug, Args)]
pub struct HistogramCommand{
    #[clap(short, long, default_value_t=false, action)]
    pub grayscale: bool
}

#[derive(Debug, Args)]
pub struct ConvoluteCommand{
    #[clap(short, long)]
    pub dimension: usize,
    #[clap(short, long, required=true, use_value_delimiter=true, value_delimiter=',', allow_hyphen_values=true)]
    pub kernel: Vec<f32>
}

#[derive(Debug, Args)]
pub struct EdgeDetectionCommand{
    #[clap(short, long)]
    pub std_dev: Option<u16>,
    #[clap(short, long)]
    pub kernel_size: Option<SobelKernelSize>
}

#[derive(Debug, Clone)]
pub enum SobelKernelSize{
    Small,
    Medium
}
impl FromStr for SobelKernelSize {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "3" => Ok(SobelKernelSize::Small),
            "5"=> Ok(SobelKernelSize::Medium),
            _ => Err("Invalid kernel size (use 3 or 5)")
        }
    }
}

