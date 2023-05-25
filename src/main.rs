// TODO: Add key and encryption
use std::{io, path::PathBuf, process::exit};
use image::{io::Reader as ImageReader, GenericImageView, GenericImage, DynamicImage};
use structopt::{StructOpt};
use colored::Colorize;

const STOP_WORD: &str = "$$$$";



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
        exit(1);
    }
    if !opt.image.exists() {
        _error("Please provide a valid path for the image");
        exit(1);
    }

    if opt.decode {
        decode(&opt);
    } else if opt.encode {
        encode(&opt);
    }
}

fn decode(opt: &Opt) {
    let img = ImageReader::open(opt.image.to_str().unwrap()).unwrap().decode().unwrap();
    let result = decode_image(&img);

    println!("results: {result}");
}

fn encode(opt: &Opt) {
    let mut img = ImageReader::open(opt.image.to_str().unwrap().to_string()).unwrap().decode().unwrap();
    
    println!("enter the phrase you want to embed:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("AAAAAAAA");
    input = input.trim().to_string();

    if u32::try_from(input.len() + STOP_WORD.len()).unwrap() > (img.dimensions().0 * img.dimensions().1) {
        _error("This message is too long for this image. Use a bigger image and try again");
        exit(1);
    }
    
    input += STOP_WORD;
    println!("{input}");
    let input_in_binary = str_to_binary(&input);
    println!("{input_in_binary}");
    encode_image(&mut img, &input_in_binary);

    let mut saving_path =  String::from("encoded_");
    saving_path += opt.image.file_name().unwrap().to_str().unwrap().clone();

    img.save(&saving_path).expect("could not save image!");
    println!("encoded image successfully as: ./{}", saving_path);
}

fn str_to_binary(input: &String) -> String {
    let mut input_in_binary = "".to_string();

    for character in input.clone().into_bytes() {
        // TODO: Clean this padding
        let mut temp = format!("0{:b}", character);
        let i:usize = 0;
        while i < 8 - temp.len() {
            temp = "0".to_owned() + &temp;
        }

        input_in_binary += &temp;
    }

    input_in_binary
}

fn binary_to_bytes(input: &String) -> Vec<u8> {
    assert!(input.len() % 8 == 0);
    let mut bytes: Vec<u8> = Vec::new();
    let mut i = 0;

    while i < input.len() {
        bytes.push(u8::from_str_radix(&input[i..i+8], 2).unwrap());
        i += 8;
    }

    bytes
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

fn decode_image(img: &DynamicImage) -> String {
    let (x,y): (u32, u32) = img.dimensions();
    let mut i = 0;
    let mut j = 0;
    let mut output_in_binary = "".to_string();

    while i < x {
        while j < y {
            let pixel = img.get_pixel(i, j);
            output_in_binary += &format!("{}", (pixel.0[0] % 2).to_string());

            j += 1;
        }
        i += 1;
    }

    // TODO: Clean this padding
    let i:usize = 0;
    while i < output_in_binary.len() % 8 {
        output_in_binary += "0";
    }

    let stop_word_in_binary = str_to_binary(&String::from(STOP_WORD));

    if output_in_binary.find(stop_word_in_binary.as_str()) == None {
        _error("Could not find an embedded message in the image");
        exit(1);
    }
    output_in_binary = output_in_binary[..output_in_binary.find(stop_word_in_binary.as_str()).unwrap()].to_string();

    let ascii = String::from_utf8(binary_to_bytes(&output_in_binary)).unwrap();

    ascii
}

fn _error(msg: &str) {
    println!("{} {}\n", "error:".red().bold(), msg);
}