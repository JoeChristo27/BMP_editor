use std::fs;
use std::io;
use BMP_editor::BMP;

fn main() {
    println!("---------BMP Editor---------\nEnter file name:");

    let mut file_name = String::new();

    io::stdin().read_line(&mut file_name).unwrap();

    let file_name = file_name.trim();

    let mut image_data: Vec<u8> = Vec::new();

    let mut flag: u8 = 0b11111111;
    let mut counter = 0;
    let mut row: i32 = 0;

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

    let bmp_file = BMP::new_greyscale_image(image_data, 640, 640);

    let file = fs::File::create(format!("{}.bmp", file_name)).unwrap();

    bmp_file.write_data(file).unwrap();

}

