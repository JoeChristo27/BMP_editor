use std::fs::File;
use std::io::Write;

/// This struct contains the file structure of the BMP file
pub struct BMP {
    pub header: BmpHeader,
    pub info_header: DIBHeader,
    pub color_table: ColorTable,
    pub image_data: Vec<u8>,

}


impl BMP {
    /// Used to create a new grayscale image.
    /// Image data provided must be padded
    pub fn new_greyscale_image(width: u32, height: u32, image_data: Vec<u8>) -> BMP {

        let info_header = DIBHeader {
            size: 40,
            width, 
            height, 
            planes: 1,
            bit_count: 1, // Only black or white
            compression: 0,
            image_size: 0,
            x_pixels_per_metre: 1,
            y_pixels_per_metre: 1,
            colors_used: 0,
            colors_important: 0,

        };

        let color_table = ColorTable {
        entries: vec![0,0,0,0,
                        255,255,255,0], // We only have one color that is black or white
        };

        let header = BmpHeader {
            signature: String::from("BM"),
            file_size: (54 + color_table.entries.len() + image_data.len()) as u32, // TODO: SET THIS TO HEADER SIZE + DATA SIZE
            reserved: 0,
            data_offset: 54 + color_table.entries.len() as u32, // TODO: SET THIS TO HEADER SIZE (including color table)
        };

        BMP {
            header,
            info_header,
            color_table,
            image_data
        }

    }
    
    /// This function is used to create a color image with no color table.
    /// The image data provided must be padded.
    /// The minimum bit count is 24.
    // TODO: Add a way to include the color table here
    pub fn new_color_image(width: u32, height: u32, bit_count: u16, image_data: Vec<u8>) -> Result<BMP, String> {

        if bit_count > 8 {
            Ok(Self::new_color_image_without_color_table(width, height, bit_count, image_data))
        }
        else {
            Err(String::from("The bit count must be greater than 8"))
        }

    }

    // TODO: Get a color table as argument and use it
    pub fn new_color_image_with_color_table(width: u32, height: u32, bit_count: u16, image_data: Vec<u8>) -> BMP {

        let info_header = DIBHeader {
            size: 40,
            width, 
            height, 
            planes: 1,
            bit_count, // Only black or white
            compression: 0,
            image_size: 0, // Value is zero when there is no compression
            x_pixels_per_metre: 1,
            y_pixels_per_metre: 1,
            colors_used: 0,
            colors_important: 0,

        };

        // Change this color table from the parameter
        let color_table = ColorTable {
        entries: vec![0,0,0,0,
                        255,255,255,0],
        };

        let header = BmpHeader {
            signature: String::from("BM"),
            file_size: (54 + image_data.len() + color_table.entries.len()) as u32, // SET THIS TO HEADER SIZE + DATA SIZE
            reserved: 0,
            data_offset: (54 + color_table.entries.len()) as u32, // SET THIS TO HEADER SIZE (including color table)
        };

        BMP {
            header,
            info_header,
            color_table: ColorTable { entries: vec![] },
            image_data
        }

    }

 
    pub fn new_color_image_without_color_table(width: u32, height: u32, bit_count: u16, image_data: Vec<u8>) -> BMP {

        let info_header = DIBHeader {
            size: 40,
            width, 
            height, 
            planes: 1,
            bit_count, // Only black or white
            compression: 0,
            image_size: 0,
            x_pixels_per_metre: 1,
            y_pixels_per_metre: 1,
            colors_used: 0,
            colors_important: 0,

        };

        // Color table not required
        // let color_table = ColorTable {
        // entries: vec![0,0,0,0,
        //                 255,255,255,0], // We only have one color that is black or white
        // };

        let header = BmpHeader {
            signature: String::from("BM"),
            file_size: (54 + image_data.len()) as u32, // SET THIS TO HEADER SIZE + DATA SIZE
            reserved: 0,
            data_offset: 54, // SET THIS TO HEADER SIZE (including color table)
        };

        BMP {
            header,
            info_header,
            color_table: ColorTable { entries: vec![] },
            image_data
        }

    }

    /// This function is used to write the bmp file to a file on the computer
    pub fn write_data(&self, mut file: File) -> Result<(), std::io::Error> { 
        file.write(self.header.signature.as_bytes())?;
        file.write(&self.header.file_size.to_le_bytes())?;
        file.write(&self.header.reserved.to_le_bytes())?;
        file.write(&self.header.data_offset.to_le_bytes())?;
        file.write(&self.info_header.size.to_le_bytes())?;
        file.write(&self.info_header.width.to_le_bytes())?;
        file.write(&self.info_header.height.to_le_bytes())?;
        file.write(&self.info_header.planes.to_le_bytes())?;
        file.write(&self.info_header.bit_count.to_le_bytes())?;
        file.write(&self.info_header.compression.to_le_bytes())?;
        file.write(&self.info_header.image_size.to_le_bytes())?;
        file.write(&self.info_header.x_pixels_per_metre.to_le_bytes())?;
        file.write(&self.info_header.y_pixels_per_metre.to_le_bytes())?;
        file.write(&self.info_header.colors_used.to_le_bytes())?;
        file.write(&self.info_header.colors_important.to_le_bytes())?;
        file.write(&self.color_table.entries)?;
        file.write(&self.image_data)?;

        Ok(())
    }
}

/// The first part of the bmp file. (Total size: 14 bytes)
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

/// BITMAPINFOHEADER (Total Size: 40 bytes)
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

/// The color table used when bit count <= 8
pub struct ColorTable {
    /// Vector containing the colors
    pub entries: Vec<u8>,
}
