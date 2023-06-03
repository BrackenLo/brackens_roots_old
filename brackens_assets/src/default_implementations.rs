//===============================================================

use std::{fs::File, io::Read};

use super::{Asset, AssetLoadable};
use glyph_brush::ab_glyph::FontArc;
use image::DynamicImage;
use log::error;

//===============================================================

impl Asset for DynamicImage {
    fn asset_name() -> &'static str {
        "DynamicImage"
    }
}

impl AssetLoadable<()> for DynamicImage {
    fn load_from_file(path: String, data: ()) -> Self {
        match image::open(path) {
            Ok(val) => val,
            Err(e) => {
                error!("Error: Could not open image - {}", e);
                Self::load_default(data)
            }
        }
    }

    fn load_default(_: ()) -> Self {
        DynamicImage::default()
    }
}

//===============================================================

impl Asset for FontArc {
    fn asset_name() -> &'static str {
        "Font"
    }
}
impl AssetLoadable<()> for FontArc {
    fn load_from_file(path: String, data: ()) -> Self {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                error!("Error: Unable to load Font Asset - {}", e);
                return Self::load_default(data);
            }
        };

        let mut buffer = vec![];
        if let Err(e) = file.read_to_end(&mut buffer) {
            error!("Error: Unable to read font file - {}", e);
            return Self::load_default(data);
        }

        match FontArc::try_from_vec(buffer) {
            Ok(font) => font,
            Err(e) => {
                error!("Error: Unable to read font from file - {}", e);
                Self::load_default(data)
            }
        }
    }

    fn load_default(_: ()) -> Self {
        panic!("Error: Unable to load font");
    }
}

//===============================================================
