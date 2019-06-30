extern crate leptess;

use std::path::Path;
use leptess::{leptonica, tesseract};

fn main() {
    let mut api = tesseract::TessApi::new(None, "eng").unwrap();

    let mut pix = leptonica::pix_read(Path::new("path/page.bmp")).unwrap();
    api.set_image(&pix);

    // detect bounding boxes for words
    let boxes = api.get_component_images(
        leptess::capi::TessPageIteratorLevel_RIL_WORD,
        true,
    ).unwrap();

    println!("Found {} textline image components.", boxes.get_n());

    // run OCR on each word bounding box
    for b in boxes {
        api.set_rectangle(&b);
        let text = api.get_utf8_text().unwrap();
        let confi = api.mean_text_conf();
        println!(
            "{:?}, confidence: {}, text: {}",
            b.get_val(),
            confi,
            text
        );
    }

    api.destroy();
    pix.destroy();
}
