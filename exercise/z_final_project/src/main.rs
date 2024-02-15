extern crate clap;
extern crate image;
extern crate rayon;

use clap::{Arg, App};
use image::{ImageError, ImageBuffer, RgbImage};
use std::process;
use rayon::prelude::*;
use num_complex::Complex;

fn blur(infile: &str, outfile: &str, blur_amount: f32) -> Result<(), ImageError> {
    let img = image::open(infile)?;
    let blurred = img.blur(blur_amount);
    blurred.save(outfile)?;
    Ok(())
}

fn brighten(infile: &str, outfile: &str, brightness_amount: i32) -> Result<(), ImageError> {
    let img = image::open(infile)?;
    let brightened = img.brighten(brightness_amount);
    brightened.save(outfile)?;
    Ok(())
}

fn crop(infile: &str, outfile: &str, x: u32, y: u32, width: u32, height: u32) -> Result<(), ImageError> {
    let img = image::open(infile)?;
    let cropped = img.crop_imm(x, y, width, height);
    cropped.save(outfile)?;
    Ok(())
}

fn rotate(infile: &str, outfile: &str, degrees: u32) -> Result<(), String> {
    let img = match image::open(infile) {
        Ok(img) => img,
        Err(e) => return Err(format!("Failed to open input file: {}", e)),
    };

    let rotated = match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => return Err(format!("Rotation by {} degrees is not supported.", degrees)),
    };

    match rotated.save(outfile) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to save output file: {}", e)),
    }
}

fn invert(infile: &str, outfile: &str) -> Result<(), ImageError> {
    let mut img = image::open(infile)?;
    img.invert();
    img.save(outfile)?;
    Ok(())
}

fn grayscale(infile: &str, outfile: &str) -> Result<(), ImageError> {
    let img = image::open(infile)?;
    let gray = img.grayscale();
    gray.save(outfile)?;
    Ok(())
}

fn fractal(outfile: &str) -> Result<(), ImageError> {
    let (width, height) = (9000, 9000);
    let mut imgbuf: RgbImage = ImageBuffer::new(width, height);

    imgbuf.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        let cx = y as f32 * 3.0 / width as f32 - 1.5;
        let cy = x as f32 * 3.0 / height as f32 - 1.5;
        let c = Complex::new(-0.4, 0.6);
        let mut z = Complex::new(cx, cy);

        let mut i = 0;
        for t in 0..255 {
            if z.norm_sqr() > 4.0 {
                break;
            }
            z = z * z + c;
            i = t;
        }

        let red = (i % 256) as u8;
        let green = ((i * 2) % 256) as u8;
        let blue = ((i * 5) % 256) as u8;

        *pixel = image::Rgb([red, green, blue]);
    });

    imgbuf.save(outfile)?;
    Ok(())
}

fn generate(outfile: &str, width: u32, height: u32, color: [u8; 3]) -> Result<(), ImageError> {
    let imgbuf: RgbImage = ImageBuffer::from_fn(width, height, |_, _| image::Rgb(color));
    imgbuf.save(outfile)?;
    Ok(())
}

fn parse_color(color_str: &str) -> [u8; 3] {
    let parts: Vec<&str> = color_str.split(',').collect();
    let r: u8 = parts.get(0).unwrap_or(&"0").parse().unwrap();
    let g: u8 = parts.get(1).unwrap_or(&"0").parse().unwrap();
    let b: u8 = parts.get(2).unwrap_or(&"0").parse().unwrap();
    [r, g, b]
}

fn main() {
    let matches = App::new("Image Processor")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Performs various image processing operations")
        .arg(Arg::with_name("operation")
            .help("The image operation to perform: blur, brighten, crop, rotate, invert, grayscale, fractal, generate")
            .required(true)
            .index(1))
        .arg(Arg::with_name("parameters")
            .help("Operation specific parameters")
            .required(false)
            .multiple(true))
        .get_matches();

    let operation = matches.value_of("operation").unwrap();
    let parameters: Vec<&str> = matches.values_of("parameters").unwrap_or_default().collect();

    match operation {
        "blur" => {
            if parameters.len() != 3 {
                eprintln!("Usage: blur <infile> <outfile> <blur_amount>");
                process::exit(1);
            }
            let blur_amount: f32 = parameters[2].parse().expect("Blur amount must be a number");
            let _ = blur(parameters[0], parameters[1], blur_amount);
        },
        "brighten" => {
            if parameters.len() != 3 {
                eprintln!("Usage: brighten <infile> <outfile> <brightness_amount>");
                process::exit(1);
            }
            let brightness_amount: i32 = parameters[2].parse().expect("Brightness amount must be a number");
            let _ = brighten(parameters[0], parameters[1], brightness_amount);
        },
        "crop" => {
            if parameters.len() != 6 {
                eprintln!("Usage: crop <infile> <outfile> <x> <y> <width> <height>");
                process::exit(1);
            }
            let x: u32 = parameters[2].parse().expect("x must be a number");
            let y: u32 = parameters[3].parse().expect("y must be a number");
            let width: u32 = parameters[4].parse().expect("width must be a number");
            let height: u32 = parameters[5].parse().expect("height must be a number");
            let _ = crop(parameters[0], parameters[1], x, y, width, height);
        },
        "rotate" => {
            if parameters.len() != 3 {
                eprintln!("Usage: rotate <infile> <outfile> <degrees>");
                process::exit(1);
            }
            let degrees: u32 = parameters[2].parse().expect("Degrees must be a number");
            let _ = rotate(parameters[0], parameters[1], degrees);
        },
        "invert" => {
            if parameters.len() != 2 {
                eprintln!("Usage: invert <infile> <outfile>");
                process::exit(1);
            }
            let _ = invert(parameters[0], parameters[1]);
        },
        "grayscale" => {
            if parameters.len() != 2 {
                eprintln!("Usage: grayscale <infile> <outfile>");
                process::exit(1);
            }
            let _ = grayscale(parameters[0], parameters[1]);
        },
        "fractal" => {
            if parameters.len() != 1 {
                eprintln!("Usage: fractal <outfile>");
                process::exit(1);
            }
            let _ = fractal(parameters[0]);
        },
        "generate" => {
            if parameters.len() < 1 || parameters.len() > 4 {
                eprintln!("Usage: generate <outfile> [width height color]");
                process::exit(1);
            }
            let width: u32 = parameters.get(1).unwrap_or(&"800").parse().unwrap();
            let height: u32 = parameters.get(2).unwrap_or(&"600").parse().unwrap();
            let color: [u8; 3] = parse_color(parameters.get(3).unwrap_or(&"255,255,255"));
            let _ = generate(parameters[0], width, height, color);
        },
        _ => {
            eprintln!("Unknown operation: '{}'", operation);
            process::exit(1);
        }
    }
}
