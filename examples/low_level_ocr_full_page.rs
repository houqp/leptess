extern crate leptess;

use leptess::{leptonica, tesseract};
use std::path::Path;

fn main() {
    let mut api = tesseract::TessApi::new(None, "eng").unwrap();

    let pix = leptonica::pix_read(Path::new("./tests/di.png")).unwrap();
    api.set_image(&pix);

    let text = api.get_utf8_text();
    println!("{}", text.unwrap());
}
