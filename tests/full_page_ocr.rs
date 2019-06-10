extern crate leptess;

use std::path::Path;
use leptess::{tesseract, leptonica};

#[test]
fn test_get_text() {
    let path = Path::new("./tests/di.png");
    let img = leptonica::pix_read(path).unwrap();

    let api = tesseract::TessApi::new();
    api.init("eng");
    api.set_image(&img);
    let text = api.get_utf8_text().unwrap();

    let mut lines = text.lines();
    assert_eq!("We hold these truths to be self-evident, that all men", lines.nth(14).unwrap());
    assert_eq!("are created equal, that they are endowed by their", lines.nth(0).unwrap());

    api.destroy();
    img.destroy();
}


#[test]
fn test_ocr_iterate_word() {
    let path = Path::new("./tests/di.png");
    let img = leptonica::pix_read(path).unwrap();

    let api = tesseract::TessApi::new();
    api.init("eng");
    api.set_image(&img);

    let boxes = api.get_component_images(
        leptess::capi::TessPageIteratorLevel_RIL_WORD,
        true,
    ).unwrap();

    for b in &boxes {
        api.set_rectangle(&b);
        let text = api.get_utf8_text().unwrap();

        assert_eq!(leptonica::BoxVal{
            x: 118,
            y: 5,
            w: 17,
            h: 11,
        }, b.get_val());
        assert_eq!("IN\n", text);

        break;
    }

    let mut iter = boxes.into_iter();
    let b = iter.nth(5).unwrap();
    api.set_rectangle(&b);
    assert_eq!("The unanimous Declaration\n", api.get_utf8_text().unwrap());

    let b = iter.nth(14).unwrap();
    api.set_rectangle(&b);
    assert_eq!("people\n", api.get_utf8_text().unwrap());

    api.destroy();
    img.destroy();
}

