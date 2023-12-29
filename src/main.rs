use opencv::core;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <image>", args[0]);
        return;
    }

    // Read image
    let image = imgcodecs::imread(args[1].as_str(), imgcodecs::IMREAD_COLOR).unwrap();

    // Convert to grayscale
    let mut gray_image = Mat::default();
    imgproc::cvt_color(&image, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0).unwrap();

    // Desired width or height
    let desired_width = 150;
    let desired_height = 150;

    // Calculate scaling factor while maintaining aspect ratio
    let scale_w = desired_width as f64 / image.cols() as f64;
    let scale_h = desired_height as f64 / image.rows() as f64;
    let scale = scale_w.min(scale_h);

    let new_width = (image.cols() as f64 * scale) as i32;
    let new_height = (image.rows() as f64 * scale) as i32;

    // Resize image
    let mut resized = Mat::default();
    // let scale = 1.0;
    imgproc::resize(
        &gray_image,
        &mut resized,
        core::Size::new(new_width, new_height),
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )
    .unwrap();

    // ASCII Converstion
    // let ascii_chars = "@%#*+=-:. "; // More chars for finer details
    let ascii_chars = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
    let mut ascii_art = String::new();

    for i in 0..resized.rows() {
        for j in 0..resized.cols() {
            let pixel = resized.at_2d::<u8>(i, j).unwrap();
            let ascii_index = *pixel as usize * ascii_chars.len() / 256;
            ascii_art.push(ascii_chars.chars().nth(ascii_index).unwrap());
        }
        ascii_art.push('\n');
    }

    // Save ASCII art to file
    let mut file = File::create(format!(
        "{}.txt",
        args[1].split('.').collect::<Vec<&str>>()[0]
    ))
    .unwrap();
    file.write_all(ascii_art.as_bytes()).unwrap();

    // Print ASCII art to console
    println!("{}", ascii_art);

    println!("Done!");
}
