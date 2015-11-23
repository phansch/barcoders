//! This module provides types for generating image representations of barcodes.

extern crate image;

use image::GenericImage;
use image::ImageBuffer;
use std::fs::File;

const IMAGE_BAR_WIDTH: u32 = 1;

/// The GIF barcode generator type.
pub enum Image {
    GIF {
        /// The height of the barcode in pixels.
        height: u32,
        /// The X dimension. Specifies the width of the "narrow" bars. 
        /// For GIF, each will be ```self.xdim * IMAGE_BAR_WIDTH``` pixels wide.
        xdim: u32,
    },
    PNG {
        /// The height of the barcode in pixels.
        height: u32,
        /// The X dimension. Specifies the width of the "narrow" bars. 
        /// For PNG, each will be ```self.xdim * IMAGE_BAR_WIDTH``` pixels wide.
        xdim: u32,
    },
    JPEG {
        /// The height of the barcode in pixels.
        height: u32,
        /// The X dimension. Specifies the width of the "narrow" bars. 
        /// For JPEG, each will be ```self.xdim * IMAGE_BAR_WIDTH``` pixels wide.
        xdim: u32,
    },
}

impl Image {
    /// Returns a new GIF with default values.
    pub fn gif() -> Image {
        Image::GIF {
            height: 80,
            xdim: 1,
        }
    }

    /// Returns a new PNG with default values.
    pub fn png() -> Image {
        Image::PNG {
            height: 80,
            xdim: 1,
        }
    }

    /// Returns a new PNG with default values.
    pub fn jpeg() -> Image {
        Image::JPEG {
            height: 80,
            xdim: 1,
        }
    }

    /// Generates the given barcode. Returns a usize indicating the number of bytes written.
    pub fn generate(&self, barcode: &[u8], path: &mut File) -> Result<usize, &str> {
        let (xdim, height, format) = match *self {
            Image::GIF{height: h, xdim: x} => (x, h, image::GIF),
            Image::PNG{height: h, xdim: x} => (x, h, image::PNG),
            Image::JPEG{height: h, xdim: x} => (x, h, image::JPEG),
        };

        let width = (barcode.len() as u32) * (xdim * IMAGE_BAR_WIDTH);
        let mut buffer = ImageBuffer::new(width, height);
        let mut pos = 0;

        for y in 0..height {
            for &b in barcode {
                let size = xdim * IMAGE_BAR_WIDTH;

                if b == 0 {
                    for p in 0..size {
                        buffer.put_pixel(pos + p, y, image::Luma([255 as u8]));
                    }
                }

                pos += size;
            }

            pos = 0;
        }

        let buflen = buffer.len();

        match image::ImageLuma8(buffer).save(path, format) {
            Ok(_) => Ok(buflen),
            _ => Err("Could not encode image."),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate image;

    use ::sym::ean13::*;
    use ::sym::ean8::*;
    use ::sym::code39::*;
    use ::sym::ean_supp::*;
    use ::sym::tf::*;
    use ::generators::image::*;
    use std::fs::File;
    use std::path::Path;

    const TEST_DATA_BASE: &'static str = "./target/debug";

    fn open_file(name: &'static str) -> File {
        File::create(&Path::new(&format!("{}/{}", TEST_DATA_BASE, name)[..])).unwrap()
    }

    #[test]
    fn ean_13_as_gif() {
        let mut path = open_file("ean13.gif");

        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let gif = Image::gif();
        let generated = gif.generate(&ean13.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 7600);
    }

    #[test]
    fn ean_13_as_png() {
        let mut path = open_file("ean13.png");

        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 1,
        };
        let generated = png.generate(&ean13.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 9500);
    }

    #[test]
    fn ean_13_as_jpeg() {
        let mut path = open_file("ean13.jpg");

        let ean13 = EAN13::new("999988881234".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 100,
            xdim: 3,
        };
        let generated = jpeg.generate(&ean13.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 28500);
    }

    #[test]
    fn code39_as_png() {
        let mut path = open_file("code39.png");

        let code39 = Code39::new("ILOVEMEL".to_owned()).unwrap();
        let png = Image::PNG {
            height: 60,
            xdim: 1,
        };
        let generated = png.generate(&code39.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 7740);
    }

    #[test]
    fn code39_as_gif() {
        let mut path = open_file("code39.gif");

        let code39 = Code39::new("WIKIPEDIA".to_owned()).unwrap();
        let gif = Image::GIF {
            height: 60,
            xdim: 1,
        };
        let generated = gif.generate(&code39.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 8520);
    }

    #[test]
    fn code39_as_jpeg() {
        let mut path = open_file("code39.jpg");

        let code39 = Code39::new("SWAGLORDTHE3RD".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 160,
            xdim: 1,
        };
        let generated = jpeg.generate(&code39.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 33120);
    }

    #[test]
    fn ean8_as_png() {
        let mut path = open_file("ean8.png");

        let ean8 = EAN8::new("5512345".to_owned()).unwrap();
        let png = Image::PNG {
            height: 70,
            xdim: 2,
        };
        let generated = png.generate(&ean8.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 9380);
    }

    #[test]
    fn ean8_as_gif() {
        let mut path = open_file("ean8.gif");

        let ean8 = EAN8::new("9992227".to_owned()).unwrap();
        let gif = Image::GIF {
            height: 70,
            xdim: 2,
        };
        let generated = gif.generate(&ean8.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 9380);
    }

    #[test]
    fn ean8_as_jpeg() {
        let mut path = open_file("ean8.jpg");

        let ean8 = EAN8::new("9992227".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 70,
            xdim: 2,
        };
        let generated = jpeg.generate(&ean8.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 9380);
    }

    #[test]
    fn ean2_as_png() {
        let mut path = open_file("ean2.png");

        let ean2 = EANSUPP::new("94".to_owned()).unwrap();
        let png = Image::PNG {
            height: 70,
            xdim: 2,
        };
        let generated = png.generate(&ean2.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 2800);
    }

    #[test]
    fn ean5_as_gif() {
        let mut path = open_file("ean5.gif");

        let ean5 = EANSUPP::new("51234".to_owned()).unwrap();
        let gif = Image::GIF {
            height: 70,
            xdim: 2,
        };
        let generated = gif.generate(&ean5.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 6580);
    }

    #[test]
    fn ean5_as_jpeg() {
        let mut path = open_file("ean5.jpg");

        let ean5 = EANSUPP::new("51574".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 140,
            xdim: 5,
        };
        let generated = jpeg.generate(&ean5.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 32900);
    }

    #[test]
    fn itf_as_png() {
        let mut path = open_file("itf.png");

        let itf = TF::interleaved("1234567".to_owned()).unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 2,
        };
        let generated = png.generate(&itf.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 16000);
    }

    #[test]
    fn stf_as_png() {
        let mut path = open_file("stf.png");

        let stf = TF::standard("1234567".to_owned()).unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 2,
        };
        let generated = png.generate(&stf.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 22800);
    }

    #[test]
    fn itf_as_gif() {
        let mut path = open_file("itf.gif");

        let itf = TF::interleaved("98766543561".to_owned()).unwrap();
        let gif = Image::GIF {
            height: 130,
            xdim: 1,
        };
        let generated = gif.generate(&itf.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 15080);
    }

    #[test]
    fn itf_as_jpeg() {
        let mut path = open_file("itf.jpg");

        let itf = TF::interleaved("98766543561".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 130,
            xdim: 1,
        };
        let generated = jpeg.generate(&itf.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 15080);
    }
}
