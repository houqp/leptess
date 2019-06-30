Leptess
=======

[![CircleCI](https://circleci.com/gh/houqp/leptess.svg?style=svg)](https://circleci.com/gh/houqp/leptess)
[![Crates.io](https://img.shields.io/crates/v/leptess.svg)](https://crates.io/crates/leptess)
[![Docs](https://img.shields.io/badge/rust-docs-blue.svg)](https://docs.rs/leptess)

Productive and safe Rust bindings/wrappers for Tesseract and Leptonica.


Build dependencies
------------------

Make sure you have Leptonica and Tesseract installed.

For Ubuntu user:

```bash
sudo apt-get install libleptonica-dev libtesseract-dev
```

You will also need to install tesseract language data based on your OCR needs:

```bash
sudo apt-get install tesseract-ocr-eng
```


Usage
-----

```rust
let mut lt = leptess::LepTess::new(None, "eng").unwrap();
lt.set_image("path/to/page.bmp");
println!("{}", lt.get_utf8_text().unwrap());
```

For more examples, see [docs](https://docs.rs/leptess) and `examples` directory.


Development
-----------

Regenerate capi binding:

```
make gen
```

To run tests, you will need at Tesseract 4.x to match what we have in
`tests/tessdata/eng.traineddata`. See CircleCI config to see how to replicate
the setup.
