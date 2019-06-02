extern crate bindgen;

use std::env;

fn main() {
    println!("cargo:rustc-link-lib=lept");
    println!("cargo:rustc-link-lib=tesseract");

    match env::var("GEN_BINDING") {
        Ok(_) => {
            // The bindgen::Builder is the main entry point
            // to bindgen, and lets you build up options for
            // the resulting bindings.
            let bindings = bindgen::Builder::default()
                // The input header we would like to generate
                // bindings for.
                .header("wrapper.h")
                // Finish the builder and generate the bindings.
                .generate()
                // Unwrap the Result and panic on failure.
                .expect("Unable to generate bindings");

            // Write the bindings to the $OUT_DIR/bindings.rs file.
            bindings
                .write_to_file("src/capi/leptonica-tesseract.rs")
                .expect("Couldn't write bindings!");
        }
        _ => {}
    }
}
