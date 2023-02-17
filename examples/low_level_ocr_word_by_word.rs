extern crate leptess;

use leptess::{leptonica, tesseract};
use std::path::Path;

fn main() {
    let mut api = tesseract::TessApi::new(None, "eng").unwrap();

    let pix = leptonica::pix_read(Path::new("./tests/di.png")).unwrap();
    api.set_image(&pix);

    // detect bounding boxes for words
    let boxes = api
        .get_component_images(leptess::capi::TessPageIteratorLevel_RIL_WORD, true)
        .unwrap();

    println!("Found {} textline image components.", boxes.get_n());

    // run OCR on each word bounding box
    for b in &boxes {
        api.set_rectangle_from_box(&b);
        let text = api.get_utf8_text().unwrap();
        let confi = api.mean_text_conf();
        println!("{:?}, confidence: {}, text: {}", b, confi, text);
    }
}
