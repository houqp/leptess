extern crate leptess;

use leptess::{leptonica, tesseract};

fn main() {
    let api = tesseract::TessApi::new();
    if api.init("eng") != 0 {
        panic!("Error initializing tesseract");
    }

    let pix = leptonica::pix_read(Path::new("path/page.bmp")).unwrap();
    api.set_image(pix);

    let text = api.get_utf8_text();
    println!("{}", text);
}
