extern crate image;
extern crate leptess;

use std::io::Cursor;

use leptess::LepTess;

#[test]
fn test_read_pix() {
    let img = image::open("./tests/di.png").unwrap();
    let mut tiff_buffer = Vec::new();
    img.write_to(
        &mut Cursor::new(&mut tiff_buffer),
        image::ImageOutputFormat::Tiff,
    )
    .unwrap();

    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();

    lt.set_image_from_mem(&tiff_buffer).unwrap();

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
