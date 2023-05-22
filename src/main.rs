use std::{io, path::PathBuf};
use image::{io::Reader as ImageReader, GenericImageView, GenericImage, DynamicImage};
use structopt::{StructOpt};
use colored::Colorize;

#[derive(StructOpt, Debug)]
#[structopt(name = "stegomage")]
struct Opt {
    /// Use this to decode an image
    #[structopt(short, long)]
    decode: bool,

    /// Use this to encode an image
    #[structopt(short, long)]
    encode: bool,

    /// The image to encode/decode
    #[structopt(short, long, parse(from_os_str))]
    image: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    if !(opt.decode ^ opt.encode) {
        _error("Please provide only 'one' flag to decode or encode the image");
        return;
    }
    if !opt.image.exists() {
        _error("Please provide a valid path for the image");
        return;
    }

    if opt.decode {
        // TODO: handle this
    } else if opt.encode {
        // TODO: handle this
    }
    

    println!("Let's get encoding!");
    println!("enter the phrase that you want to embed.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("AAAAAAAA");

    input = input.trim().to_string();
    println!("len of your input is {}", input.len());

    let input_in_binary = get_binary_str(&input);

    println!("\"{}\" in binary is {}", input, input_in_binary);

    let mut img = ImageReader::open("new.png").unwrap().decode().unwrap();
    
    encode_image(&mut img, &input_in_binary);

    img.save("new.png").expect("could not save image!");
}

fn get_binary_str(input: &String) -> String {
    let mut input_in_binary = "".to_string();

    for character in input.clone().into_bytes() {
        input_in_binary += &format!("0{:b}", character);
    }

    input_in_binary
}

fn encode_image(img: &mut DynamicImage, binary_message: &String) {
    let (x,y): (u32, u32) = img.dimensions();
    let mut i = 0;
    let mut j = 0;

    'main: while i < x {
        while j < y {
            let binary_index = (i * y) + j;
            if binary_index >= u32::try_from(binary_message.len()).unwrap() {
                break 'main;
            }

            let mut pixel = img.get_pixel(i, j);
            if binary_message.clone().into_bytes()[usize::try_from(binary_index).unwrap()] == b'1' {
                if pixel.0[0] % 2 == 0 {
                    pixel.0[0] += 1;
                }
            } else {
                if pixel.0[0] % 2 == 1 {
                    pixel.0[0] -= 1;
                }
            }

            img.put_pixel(i, j, pixel);

            j += 1;
        }
        i += 1;
    }
}

fn decode_image(img: &DynamicImage, message_lenght: u32) -> String {
    let (x,y): (u32, u32) = img.dimensions();
    let mut i = 0;
    let mut j = 0;
    let mut output_in_binary = "".to_string();

    'main: while i < x {
        while j < y {
            if (i * y) + j >= message_lenght * 8 {
                break 'main;
            }

            let pixel = img.get_pixel(i, j);
            output_in_binary += &format!("{}", (pixel.0[0] % 2).to_string());


            j += 1;
        }
        i += 1;
    }

    return output_in_binary;
}

fn _error(msg: &str) {
    println!("{} {}\n", "error:".red().bold(), msg);
}