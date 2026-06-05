use std::fs;
use BMP_editor::BMP;

fn create_image_data_grayscale() -> Vec<u8> {
    
    let mut flag: u8 = 0b11111111;
    let mut counter = 0;
    let mut row: i32 = 0;
    let mut image_data = Vec::new();
    
    // Program to create a chess board pattern
    // It is not working at the moment
    // TODO: fix the below code
    for _i in 0..=640 {
        for _j in 0..=640 {
            image_data.push(flag);
            counter += 1;

            if counter >= 10  {
                counter = 0;
                flag = !flag;

            }
        }
        row += 1;
        if row >= 80 {
            flag = 0b00000000;
            row = 0;
        }
    }
    image_data
}

fn create_image_data_color() -> Vec<u8> {
    let mut image_data = Vec::new();

    for i in 0..=200 {
        for j in 0..=200 {
            image_data.push(i);
            image_data.push(j);
            image_data.push(255);
        }
    }


    image_data
}

fn main() {
    println!("---------BMP Editor---------\n");

    // Creating a grayscale image

    let image_data_grayscale: Vec<u8> = create_image_data_grayscale();
    let bmp_file_grayscale = BMP::new_greyscale_image(image_data_grayscale, 640, 640);

    let file_grayscale = fs::File::create("grayscale.bmp").unwrap();
    bmp_file_grayscale.write_data(file_grayscale).unwrap();

    // Creating a color image

    // The bmp file uses BGR format for colors
    // Here the bit count is 8 bit
    // Extra zero to make it 32 bit
    let image_data_color: Vec<u8> = create_image_data_color();
    let bmp_file_color = BMP::new_color_image(200, 200, 24, image_data_color).unwrap();
    let file_color = fs::File::create("color.bmp").unwrap();
    bmp_file_color.write_data(file_color).unwrap();
}

