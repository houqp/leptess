extern crate leptess;

use leptess::leptonica;
use std::path::Path;

#[test]
fn test_read_pix() {
    let path = Path::new("./tests/di.png");
    let img = leptonica::pix_read(path).unwrap();

    assert_eq!(442, img.get_w());
    assert_eq!(852, img.get_h());

    let bytes = std::fs::read("./tests/di.png").unwrap();
    let img = leptonica::pix_read_mem(&bytes).unwrap();

    assert_eq!(442, img.get_w());
    assert_eq!(852, img.get_h());
}
