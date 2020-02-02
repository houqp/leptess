extern crate leptess;

use leptess::{leptonica, tesseract, LepTess};
use std::path::Path;

#[test]
fn test_source_resolution() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    assert_eq!(lt.set_image("./tests/di.png"), true);
    assert_eq!(lt.get_source_y_resolution(), 0);
}

#[test]
fn test_get_text() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    assert_eq!(lt.set_image("./tests/di.png"), true);

    let text = lt.get_utf8_text().unwrap();

    let mut lines = text.lines();
    assert_eq!(
        "We hold these truths to be self-evident, that all men",
        lines.nth(14).unwrap()
    );
    assert_eq!(
        "are created equal, that they are endowed by their",
        lines.nth(0).unwrap()
    );
}

#[test]
fn test_ocr_iterate_word() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    assert_eq!(lt.set_image("./tests/di.png"), true);

    let boxes = lt
        .get_component_boxes(leptess::capi::TessPageIteratorLevel_RIL_WORD, true)
        .unwrap();

    for b in &boxes {
        lt.set_rectangle(&b);
        let text = lt.get_utf8_text().unwrap();

        assert_eq!(
            leptonica::BoxVal {
                x: 118,
                y: 5,
                w: 17,
                h: 11,
            },
            b.get_val()
        );
        assert_eq!("IN\n", text);

        break;
    }

    let mut iter = boxes.into_iter();
    let b = iter.nth(5).unwrap();
    lt.set_rectangle(&b);
    assert_eq!("The unanimous Declaration\n", lt.get_utf8_text().unwrap());

    let b = iter.nth(14).unwrap();
    lt.set_rectangle(&b);
    assert_eq!("people\n", lt.get_utf8_text().unwrap());
}

#[test]
fn test_low_lvl_get_text() {
    let path = Path::new("./tests/di.png");
    let img = leptonica::pix_read(path).unwrap();

    let mut api = tesseract::TessApi::new(Some("./tests/tessdata"), "eng").unwrap();
    api.set_image(&img);

    let text = api.get_utf8_text().unwrap();

    let mut lines = text.lines();
    assert_eq!(
        "We hold these truths to be self-evident, that all men",
        lines.nth(14).unwrap()
    );
    assert_eq!(
        "are created equal, that they are endowed by their",
        lines.nth(0).unwrap()
    );
}

#[test]
fn test_low_lvl_ocr_iterate_word() {
    let path = Path::new("./tests/di.png");
    let img = leptonica::pix_read(path).unwrap();

    let mut api = tesseract::TessApi::new(Some("./tests/tessdata"), "eng").unwrap();
    api.set_image(&img);

    let boxes = api
        .get_component_images(leptess::capi::TessPageIteratorLevel_RIL_WORD, true)
        .unwrap();

    for b in &boxes {
        api.set_rectangle(&b);
        let text = api.get_utf8_text().unwrap();

        assert_eq!(
            leptonica::BoxVal {
                x: 118,
                y: 5,
                w: 17,
                h: 11,
            },
            b.get_val()
        );
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
}

#[test]
fn test_low_lvl_invalid_data_path() {
    let re = tesseract::TessApi::new(Some("tests_foo"), "eng");
    assert_eq!(Err(tesseract::TessInitError { code: -1 }), re);
}

#[cfg(not(windows))]
#[test]
fn test_low_lvl_read_data_path_from_env() {
    std::env::set_var("TESSDATA_PREFIX", "./tests/tessdata");
    tesseract::TessApi::new(None, "eng").unwrap();
}
