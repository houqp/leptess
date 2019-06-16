extern crate bindgen;

use std::env;

fn main() {
    println!("cargo:rustc-link-lib=lept");
    println!("cargo:rustc-link-lib=tesseract");

    match env::var("GEN_BINDING") {
        Ok(_) => {
            let prefix_list = vec![
                "Tess", "lept",
                "reco",
                "pix", "Pix", "L_", "l_", "CC", "cc",
                "FPix", "fpix", "DPix", "dpix",
                "Box", "box", "div", "lldiv", "ecv", "fcv",
                "array", "string", "sarray", "nbytes", "fnbytes", "path",
                "Pt", "pt", "cid",
                "numa", "gray", "lstack", "lqueue", "lheap", "strcode", "strto",
                "jb", "JB", "sa",
                "gplot", "GPLOT",
                "morph", "fmorph", "fhm",
                "bmf", "pms", "seal", "sel", "SEL", "cmap", "wshed",
                "bar", "bbuff", "tiff",
                "make", "modify", "gen", "convert", "append", "split", "apply", "generate", "display", "run", "create", "concatenate", "parse", "add", "list", "next",
                "SPLIT", "IFF", "REMOVE",
                "read", "write", "get", "set", "find", "locate", "compare", "fget", "show",
                "encode", "decode", "extract", "adjacent", "raster", "reg", "rch", "project", "dewarp", "kernel", "reformat", "threshold",
                "bi", "quad", "line", "log", "isPng", "affine", "gauss", "RGBA",
            ];
            let sym_prefix_match_regex = vec![
                "(".to_string(), prefix_list.join("|"), ").+".to_string(),
            ].join("");

            // The bindgen::Builder is the main entry point
            // to bindgen, and lets you build up options for
            // the resulting bindings.
            let bindings = bindgen::Builder::default()
                // The input header we would like to generate
                // bindings for.
                .header("wrapper.h")
                .whitelist_function(&sym_prefix_match_regex)
                .whitelist_type(&sym_prefix_match_regex)
                .whitelist_var(&sym_prefix_match_regex)
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
