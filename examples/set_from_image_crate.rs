extern crate image;
extern crate leptess;

use std::io::Cursor;

use leptess::LepTess;

fn main() {
    let img = image::open("./tests/di.png").unwrap();
    let mut tiff_buffer = Vec::new();
    img.write_to(
        &mut Cursor::new(&mut tiff_buffer),
        image::ImageOutputFormat::Tiff,
    )
    .unwrap();

    let mut lt = LepTess::new(Some("./tests/tessdata"), "eng").unwrap();

    lt.set_image_from_mem(&tiff_buffer).unwrap();
    println!("{}", lt.get_utf8_text().unwrap());
}
