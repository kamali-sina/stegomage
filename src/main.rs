use std::io;
// use image::{io::Reader as ImageReader};

fn main() {
    println!("Let's get encoding!");
    println!("enter the phrase that you want to embed.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("AAAAAAAA");

    input = input.trim().to_string();

    // let img = ImageReader::open("myimage.png").expect("marg").decode().expect("marg");

    let input_in_binary = get_binary_str(&input);

    println!("\"{}\" in binary is {}", input, input_in_binary);

}

fn get_binary_str(input: &String) -> String {
    let mut input_in_binary = "".to_string();

    for character in input.clone().into_bytes() {
        input_in_binary += &format!("0{:b}", character);
    }

    input_in_binary
}