// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use image::DynamicImage;
use std::path::PathBuf;

const OPEN_FILE_ERR: &str = "Failed to open INFILE.";
const WRITE_FILE_ERR: &str = "Failed to write to OUTFILE.";

/// Simple program to greet a person

#[derive(Parser, Debug, Clone, ValueEnum)]
enum RotateOptions {
    d90,
    d180,
    d270,
}

#[derive(Subcommand, Debug)]
enum CropValues {
    /// X coordinate, Y coordinate, Width, Height
    Crop {
        #[arg(long)]
        x: u32,
        #[arg(long)]
        y: u32,
        #[arg(long)]
        width: u32,
        #[arg(long)]
        height: u32,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input image file
    #[arg(short, long)]
    infile: PathBuf,

    /// Output image file
    #[arg(short, long, value_name = "output.png")]
    outfile: PathBuf,

    /// blur an image
    #[arg(long)]
    blur: Option<f32>,

    /// brighten an image
    #[arg(long)]
    brighten: Option<i32>,

    /// crops an image x: u32, y: u32, width: u32, height: u32
    #[command(subcommand)]
    cropCommand: Option<CropValues>,

    /// Rotate an image
    #[arg(long)]
    rotate: Option<RotateOptions>,

    /// Invert an image
    #[arg(long, action = ArgAction::SetTrue)]
    invert: Option<bool>,

    /// Grayscale an image
    #[arg(long, action = ArgAction::SetTrue)]
    grayscale: Option<bool>,

    /// Fractal an image
    #[arg(long, action = ArgAction::SetTrue)]
    fractal: Option<bool>,

    /// Generate an image
    #[arg(long, action = ArgAction::SetTrue)]
    generate: Option<bool>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {

    let cli = Cli::parse();
    println!("{:?}", cli);

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    let (infile, outfile) = (cli.infile, cli.outfile);

    if let Some(value) = cli.blur {
        blur(&infile, &outfile, value);
    }

    if let Some(value) = cli.brighten {
        brighten(&infile, &outfile, value);
    }

    if let Some(value) = cli.cropCommand {
        match value {
            CropValues::Crop {
                x,
                y,
                width,
                height,
            } => {
                crop(&infile, &outfile, x, y, width, height);
            }
        }
    }

    if let Some(value) = cli.rotate {
        rotate(&infile, &outfile, value);
    }

    if cli.invert.unwrap() {
        invert(&infile, &outfile);
    }

    if cli.grayscale.unwrap() {
        grayscale(&infile, &outfile);
    }

    if cli.fractal.unwrap() {
        fractal(&outfile);
    }

    if cli.generate.unwrap() {
        generate(&outfile);
    }
}

fn blur(infile: &PathBuf, outfile: &PathBuf, value: f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect(OPEN_FILE_ERR);
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(value);
    // Here's how you save an image to a file.
    img2.save(outfile).expect(WRITE_FILE_ERR);
}

fn brighten(infile: &PathBuf, outfile: &PathBuf, value: i32) {
    // See blur() for an example of how to open / save an image.
    let img = image::open(infile).expect(OPEN_FILE_ERR);

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(value);

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect(WRITE_FILE_ERR);
}

fn crop(infile: &PathBuf, outfile: &PathBuf, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect(OPEN_FILE_ERR);

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.
    let img2 = img.crop(x, y, width, height);

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    img2.save(outfile).expect(WRITE_FILE_ERR);
}

fn rotate(infile: &PathBuf, outfile: &PathBuf, value: RotateOptions) {
    let img = image::open(infile).expect(OPEN_FILE_ERR);
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!
    let img2 = match value {
        RotateOptions::d90 => img.rotate90(),
        RotateOptions::d180 => img.rotate180(),
        RotateOptions::d270 => img.rotate270(),
    };
    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    img2.save(outfile).expect(WRITE_FILE_ERR);
}

fn invert(infile: &PathBuf, outfile: &PathBuf) {
    let mut img = image::open(infile).expect(OPEN_FILE_ERR);

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();
    img.save(outfile).expect(WRITE_FILE_ERR);
}

fn grayscale(infile: &PathBuf, outfile: &PathBuf) {
    let img = image::open(infile).expect(OPEN_FILE_ERR);

    // .grayscale() takes no arguments. It returns a new image.
    let img2 = img.grayscale();
    img2.save(outfile).expect(WRITE_FILE_ERR);
}

fn generate(outfile: &PathBuf) {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 1000;
    let height = 1000;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    for (x,y, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;
        let green = (0.3 * (x^y) as f32) as u8;

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }
    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
    imgbuf.save(outfile).expect(WRITE_FILE_ERR);
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: &PathBuf) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
