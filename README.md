# Leptess

[![Test](https://github.com/houqp/leptess/actions/workflows/test.yml/badge.svg)](https://github.com/houqp/leptess/actions/workflows/test.yml)
[![Crates.io](https://img.shields.io/crates/v/leptess.svg)](https://crates.io/crates/leptess)
[![Docs](https://img.shields.io/badge/rust-docs-blue.svg)](https://houqp.github.io/leptess/leptess/index.html)

Productive and safe Rust bindings/wrappers for Tesseract and Leptonica.

## Build dependencies

Make sure you have clang, Leptonica and Tesseract installed.

Tesseract should be version 4.0.0 or above.

### Ubuntu

```bash
sudo apt-get install libleptonica-dev libtesseract-dev clang
```

You will also need to install tesseract language data based on your OCR needs:

```bash
sudo apt-get install tesseract-ocr-eng
```

### Mac

```bash
brew install tesseract leptonica pkg-config
```

### Windows

On Windows, this library uses Microsoft's [vcpkg](https://github.com/microsoft/vcpkg) to provide tesseract.

Please install [vcpkg](https://github.com/microsoft/vcpkg) and **set up user wide integration** or [vcpkg crate](https://crates.io/crates/vcpkg) won't be able to find the library.

To install tesseract:

```cmd
REM from the vcpkg directory

REM 32 bit
.\vcpkg install tesseract:x86-windows

REM 64 bit
.\vcpkg install tesseract:x64-windows
```

To run the tests configure vcpkg-crate to find the tesseract library:

```cmd
SET VCPKGRS_DYNAMIC=true
cargo test
```

## Usage

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

## Development

To run tests, you will need at Tesseract 4.x or 5.x to match what we have in
`tests/tessdata/eng.traineddata`. See GitHub config actions to see how to replicate
the setup.
