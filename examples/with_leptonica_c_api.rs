extern crate leptess;

use leptess::{capi, tesseract};
use std::ffi::CString;

fn main() {
    let tessdata_path = Some("./tests/tessdata");
    let mut api = tesseract::TessApi::new(tessdata_path, "eng").unwrap();

    let image_path = CString::new("./tests/di.png").unwrap();
    let mut pix = leptonica_plumbing::Pix::read(&image_path).unwrap();

    let pix_scaled_c = unsafe { capi::pixScaleSmooth(pix.as_mut(), 2.0, 2.0).as_mut() }
        .expect("pointer should not be null");
    let pix_scaled_safe = unsafe { leptonica_plumbing::Pix::new_from_pointer(pix_scaled_c) };
    api.set_image(&pix_scaled_safe);

    println!("Text: {}", api.get_utf8_text().unwrap());
}
