extern crate leptess;

use std::path::Path;
use leptess::leptonica;

#[test]
fn test_read_pix() {
    let path = Path::new("./tests/di.png");
    let img = leptonica::pix_read(path).unwrap();

    assert_eq!(442, img.get_w());
    assert_eq!(852, img.get_h());
}
