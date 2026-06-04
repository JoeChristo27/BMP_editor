use std::fs::File;
use std::io::Write;

pub struct BMP {
    header: BmpHeader,
    info_header: DIBHeader,
    color_table: ColorTable,
    image_data: Vec<u8>,

}


impl BMP {
    pub fn new_greyscale_image(image_data: Vec<u8>, width: u32, height: u32) -> BMP {

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

struct BmpHeader {
    pub signature: String,
    pub file_size: u32,
    pub reserved: u32,
    pub data_offset: u32,
}

struct DIBHeader {
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub planes: u16, // always 1
    pub bit_count: u16,
    pub compression: u32,
    pub image_size: u32,
    pub x_pixels_per_metre: u32,
    pub y_pixels_per_metre: u32,
    pub colors_used: u32,
    pub colors_important: u32,
}

struct ColorTable {
    pub entries: Vec<u8>,
}
