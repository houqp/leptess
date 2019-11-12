Leptess
=======

[![CircleCI](https://circleci.com/gh/houqp/leptess.svg?style=svg)](https://circleci.com/gh/houqp/leptess)
[![Crates.io](https://img.shields.io/crates/v/leptess.svg)](https://crates.io/crates/leptess)
[![Docs](https://img.shields.io/badge/rust-docs-blue.svg)](https://houqp.github.io/leptess/leptess/index.html)

Productive and safe Rust bindings/wrappers for Tesseract and Leptonica.


Build dependencies
------------------

Make sure you have clang, Leptonica and Tesseract installed.

For Ubuntu user:

```bash
sudo apt-get install libleptonica-dev libtesseract-dev clang
```

You will also need to install tesseract language data based on your OCR needs:

```bash
sudo apt-get install tesseract-ocr-eng
```

For mac user:

```bash
brew install tesseract leptonica
```


Usage
-----

```rust
let mut lt = leptess::LepTess::new(None, "eng").unwrap();
lt.set_image("path/to/page.bmp");
println!("{}", lt.get_utf8_text().unwrap());
```

For more examples, see [docs](https://houqp.github.io/leptess/leptess/index.html) and `examples` directory.

To run demos in `examples` directory, try:

```bash
cargo run --example low_level_ocr_full_page
```

Development
-----------

To run tests, you will need at Tesseract 4.x to match what we have in
`tests/tessdata/eng.traineddata`. See CircleCI config to see how to replicate
the setup.
