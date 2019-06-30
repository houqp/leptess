extern crate leptess;

use std::path::Path;
use leptess::{leptonica, tesseract};

fn main() {
    let mut api = tesseract::TessApi::new(None, "eng").unwrap();

    let mut pix = leptonica::pix_read(Path::new("path/page.bmp")).unwrap();
    api.set_image(&pix);

    let text = api.get_utf8_text();
    println!("{}", text.unwrap());

    api.destroy();
    pix.destroy();
}
