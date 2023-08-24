#[derive(Debug)]
enum Error {
    OpenFile(std::io::Error),
    DecodeFile(tiff::TiffError),
    GetDimensions(tiff::TiffError),
    GetColorType(tiff::TiffError),
    ReadImage(tiff::TiffError),
    UnknownClolorType(tiff::decoder::DecodingResult),
    GetPixel,
    CreateFile(std::io::Error),
    EncodeImage(tiff::TiffError),
    WriteImage(tiff::TiffError),
}

#[derive(Debug)]
struct Converter {
    load: &'static str,
    save: &'static str,
    from: (u8, u8, u8, u8),
    to: (u8, u8, u8, u8),
}

impl Converter {
    pub fn exec(&self) -> Result<(), Error> {
        dbg!(self);

        // decode file
        let file = std::fs::File::open(self.load).map_err(Error::OpenFile)?;
        let mut decoder = tiff::decoder::Decoder::new(file).map_err(Error::DecodeFile)?;

        // check
        let (width, height) = decoder.dimensions().map_err(Error::GetDimensions)?;
        // assert_eq!((width, height), (8, 8));
        let colortype = decoder.colortype().map_err(Error::GetColorType)?;
        assert_eq!(colortype, tiff::ColorType::CMYK(8));

        // get pixel
        let image = decoder.read_image().map_err(Error::ReadImage)?;

        // convert
        const PIXEL_SIZE: usize = 4;
        let result = match image {
            tiff::decoder::DecodingResult::U8(mut pixels) => {
                for idx in 0..pixels.len() / PIXEL_SIZE {
                    let i = idx * PIXEL_SIZE;
                    if *pixels.get(i).ok_or(Error::GetPixel)? == self.from.0
                        && *pixels.get(i + 1).ok_or(Error::GetPixel)? == self.from.1
                        && *pixels.get(i + 2).ok_or(Error::GetPixel)? == self.from.2
                        && *pixels.get(i + 3).ok_or(Error::GetPixel)? == self.from.3
                    {
                        *pixels.get_mut(i).ok_or(Error::GetPixel)? = self.to.0;
                        *pixels.get_mut(i + 1).ok_or(Error::GetPixel)? = self.to.1;
                        *pixels.get_mut(i + 2).ok_or(Error::GetPixel)? = self.to.2;
                        *pixels.get_mut(i + 3).ok_or(Error::GetPixel)? = self.to.3;
                    }
                }
                Ok(pixels)
            }
            miss_matched => Err(Error::UnknownClolorType(miss_matched)),
        }?;

        let mut file = std::fs::File::create(self.save).map_err(Error::CreateFile)?;
        tiff::encoder::TiffEncoder::new(&mut file)
            .map_err(Error::EncodeImage)?
            .write_image::<tiff::encoder::colortype::CMYK8>(width, height, &result)
            .map_err(Error::WriteImage)?;

        Ok(())
    }
}

fn main() {
    let converter = Converter {
        load: "./share/sample.tif",
        save: "./share/sample.edited.tif",
        from: (0xec, 0xe2, 0xe2, 0xcc),
        to: (0x28, 0x28, 0x28, 0xff),
    };

    match converter.exec() {
        Err(e) => panic!("{:?}", e),
        Ok(_) => println!("finish!"),
    }
}
