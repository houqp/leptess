Leptess
=======

Rust binding for Tesseract and Leptonica.


Build dependencies
------------------

Make sure you have Leptonica and Tesseract installed.

For Ubuntu user:

```bash
sudo apt-get install libleptonica-dev libtesseract-dev
```


Usage
-----

Minimal example:

```rust
let api = tesseract::TessApi::new();
api.init("eng");

let pix = leptonica::pix_read(Path::new("path/page.bmp")).unwrap();
api.set_image(pix);

println!("{}", api.get_utf8_text());
```

For more examples, see `examples` directory.


Development
-----------

Regenerate capi binding:

```
make gen
```
