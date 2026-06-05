# BMP_editor

[![Repo Badge]][Repo]  [![License Badge]][License]  [![Deps Badge]][Deps]

A crate to create .bmp files in rust
- [Features](#features)
- [Usage](#usage)
- [Limitations](#limitations) 

## Features

- Create a grayscale image in .bmp format
- Create a color image in .bmp format
- A bmp file of any width and height can be created 
- Will add more in the future!

## Usage
### Grayscale image
```rust
// Create a BMP struct containing the file 
// Here the image_data is a Vec<u8>
// 0 represents black and 1 represents white eg: 0b00001111, 0b00000000
let bmp_file_grayscale = BMP::new_greyscale_image(image_data_grayscale, 640, 640);
// Create a file handle 
let file = fs::File::create(format!("{}.bmp", file_name)).unwrap();
// Write the bmp data into the file
bmp_file_grayscale.write_data(file).unwrap();
```
### Color image
``` rust
// The bit count here is 24
// The image_data_color is Vec<u8>
let bmp_file_color = BMP::new_color_image(200, 200, 24, image_data_color);
// Create a file handle
let file = fs::File::create(format!("{}.bmp", file_name)).unwrap();
// Write the bmp data into the file
bmp_file_grayscale.write_data(file).unwrap();
```
Full example program in the [main.rs](src/main.rs)

## Limitations

- Cannot create color image with table
- The image data provided must be padded (number of bits must be multiple of 32)

[Deps]: https://github.com/JoeChristo27/BMP_editor/blob/main/Cargo.toml
[Repo]: https://github.com/JoeChristo27/BMP_editor
[License]: https://github.com/JoeChristo27/BMP_editor/blob/main/LICENSE
[Repo Badge]:  https://img.shields.io/badge/github-BMP_editor-blue?logo=github
[License Badge]: https://img.shields.io/badge/Licencse-MIT-red
[Deps Badge]: https://img.shields.io/badge/Dependencies-None-green
