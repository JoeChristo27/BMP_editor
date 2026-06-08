This File contains all the struct fields with some explanation to make creating bmp files easier.
For more info refer [wikipedia](https://en.wikipedia.org/wiki/BMP_file_format#File_structure)

# Struct Fields
This struct contains the file structure of the BMP file
``` rust
pub struct BMP {
    pub header: BmpHeader,
    pub info_header: DIBHeader,
    pub color_table: ColorTable,
    pub image_data: Vec<u8>,

}
```
 The first part of the bmp file. (Total size: 14 bytes)
``` rust
pub struct BmpHeader {
    /// Signature is always 'BM' in ASCII
    pub signature: String,
    /// The file size: 40 + 14 + color table size + image data size
    pub file_size: u32,
    /// If creating image manually it is always zero
    pub reserved: u32,
    /// The starting address of the image data: 40 + 14 + color table size
    pub data_offset: u32,
}
```
BITMAPINFOHEADER (Total Size: 40 bytes)
``` rust
pub struct DIBHeader {
    /// The size of thi header
    pub size: u32,
    /// The bitmap width // TODO: Make it signed
    pub width: u32,
    /// The bitmap height // TODO: Make it signed
    pub height: u32,
    /// The number of color planes (must be 1)
    pub planes: u16, // always 1
    /// The number of bits per pixel
    pub bit_count: u16,
    /// Compression method (only 0 supported for now)
    pub compression: u32,
    /// value is 0 for compression value 0 
    pub image_size: u32,
    /// The horizontal resolution of the image
    pub x_pixels_per_metre: u32,
    /// The vertical resolution of the image
    pub y_pixels_per_metre: u32,
    /// The number of colors in the palette. Use 0 to default to 2^n
    pub colors_used: u32,
    /// Number of colors used 0 when every color is important
    pub colors_important: u32,
}
```
The color table used when bit count <= 8
``` rust
pub struct ColorTable {
    /// Vector containing the colors
    pub entries: Vec<u8>,
}
```