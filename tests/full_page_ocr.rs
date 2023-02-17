extern crate leptess;
extern crate regex;

use leptess::{
    leptonica::{self, BoxGeometry},
    tesseract, LepTess, Variable,
};
use regex::Regex;
use std::path::Path;

#[test]
fn test_source_resolution() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    lt.set_image("./tests/di.png").unwrap();
    assert_eq!(lt.get_source_y_resolution(), 0);
}

#[test]
fn test_get_text() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    lt.set_image("./tests/di.png").unwrap();

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
fn test_get_hocr_text() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    lt.set_image("./tests/di.png").unwrap();
    let text = lt.get_hocr_text(0).unwrap();

    assert!(text.contains("<div class='ocr_page'"));
    assert!(text.contains("self-evident,"));
}

#[test]
fn test_get_alto_text() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    lt.set_image("./tests/di.png").unwrap();
    let text = lt.get_alto_text(0).unwrap();

    let re = Regex::new(r#"<Page WIDTH="([0-9])+" HEIGHT="([0-9])+" PHYSICAL_IMG_NR="([0-9])+" ID="page_([0-9])+">"#).unwrap();
    assert!(re.is_match(&text));
    assert!(text.contains("CONTENT=\"Declaration\"/>"));
}

#[test]
fn test_get_tsv_text() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    lt.set_image("./tests/di.png").unwrap();
    let text = lt.get_tsv_text(0).unwrap();

    let re = Regex::new(r"([-0-9]+\t){11}.*").unwrap();
    assert!(re.is_match(&text));
    assert!(text.contains("Declaration"));
}

#[test]
fn test_get_lstm_box_text() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    lt.set_image("./tests/di.png").unwrap();
    let text = lt.get_lstm_box_text(0).unwrap();

    let re = Regex::new(r".?( [0-9]+){5}").unwrap();
    assert!(re.is_match(&text));
}

#[test]
fn test_get_word_str_box_text() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    lt.set_image("./tests/di.png").unwrap();
    let text = lt.get_word_str_box_text(0).unwrap();

    assert!(text.contains("WordStr"));
    assert!(text.contains("becomes necessary for one people to"));
    assert!(text.contains("to throw off such Government, and to provide new"));
}

#[test]
fn test_ocr_iterate_word() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    lt.set_image("./tests/di.png").unwrap();

    let boxes = lt
        .get_component_boxes(leptess::capi::TessPageIteratorLevel_RIL_WORD, true)
        .unwrap();

    for b in &boxes {
        lt.set_rectangle_from_box(&b);
        let text = lt.get_utf8_text().unwrap();

        assert_eq!(
            BoxGeometry {
                x: 118,
                y: 5,
                w: 17,
                h: 11
            },
            b.get_geometry()
        );
        assert_eq!("IN\n", text);

        break;
    }

    let mut iter = boxes.into_iter();
    let b = iter.nth(5).unwrap();
    lt.set_rectangle_from_box(&b);
    assert_eq!("The unanimous Declaration\n", lt.get_utf8_text().unwrap());

    let b = iter.nth(14).unwrap();
    lt.set_rectangle_from_box(&b);
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
        api.set_rectangle_from_box(&b);
        let text = api.get_utf8_text().unwrap();

        assert_eq!(
            BoxGeometry {
                x: 118,
                y: 5,
                w: 17,
                h: 11
            },
            b.get_geometry()
        );
        assert_eq!("IN\n", text);

        break;
    }

    let mut iter = boxes.into_iter();
    let b = iter.nth(5).unwrap();
    api.set_rectangle_from_box(&b);
    assert_eq!("The unanimous Declaration\n", api.get_utf8_text().unwrap());

    let b = iter.nth(14).unwrap();
    api.set_rectangle_from_box(&b);
    assert_eq!("people\n", api.get_utf8_text().unwrap());
}

#[test]
fn test_low_lvl_invalid_data_path() {
    let re = tesseract::TessApi::new(Some("tests_foo"), "eng");
    assert_eq!(Some(tesseract::TessInitError { code: -1 }), re.err());
}

#[cfg(not(windows))]
#[test]
fn test_low_lvl_read_data_path_from_env() {
    std::env::set_var("TESSDATA_PREFIX", "./tests/tessdata");
    tesseract::TessApi::new(None, "eng").unwrap();
}

#[test]
fn test_set_variable() {
    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
    lt.set_image("./tests/di.png").unwrap();
    lt.set_variable(Variable::TesseditCharBlacklist, "aeiou")
        .unwrap();

    let text = lt.get_utf8_text().unwrap();

    let mut lines = text.lines();
    assert_eq!(
        "W hld ths trths t b slf-vdnt, tht ll mn",
        lines.nth(14).unwrap()
    );
}
