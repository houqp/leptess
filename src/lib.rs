//! Productive and safe Rust bindings/wrappers for Leptonica and Tesseract.
//!
//! Overview
//! --------
//!
//! It comes with a high level wrapper [LepTess](struct.LepTess.html) focusing on productivity and
//! memory safty:
//!
//! ```rust,no_run
//! let mut lt = leptess::LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
//! lt.set_image("./tests/di.png");
//! println!("{}", lt.get_utf8_text().unwrap());
//! ```
//!
//! It also exposes low level bindings to for flexibitliy and close mirroring of upstream C APIs:
//!
//! ```rust,no_run
//! use std::path::Path;
//! let mut api = leptess::tesseract::TessApi::new(Some("./tests/tessdata"), "eng").unwrap();
//! let mut pix = leptess::leptonica::pix_read(Path::new("./tests/di.png")).unwrap();
//! api.set_image(&pix);
//!
//! let text = api.get_utf8_text();
//! println!("{}", text.unwrap());
//! ```
//!
//! Raw unsafe C API [bindings](capi/index.html) are auto-generated using bindgen at compile time.
//!
//! Build dependencies
//! ------------------
//!
//! Make sure you have Leptonica and Tesseract installed.
//!
//! For Ubuntu user:
//!
//! ```bash
//! sudo apt-get install libleptonica-dev libtesseract-dev clang
//! ```
//!
//! You will also need to install tesseract language data based on your OCR needs:
//!
//! ```bash
//! sudo apt-get install tesseract-ocr-eng
//! ```

extern crate thiserror;

pub mod capi;
pub mod leptonica;
pub mod tesseract;

use std::os::raw::c_int;
use std::path::Path;

/// High level wrapper for Tesseract and Leptonica
///
/// # Examples
///
/// ## Full page OCR
///
/// ```
/// let mut lt = leptess::LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
/// lt.set_image("./tests/di.png");
/// println!("{}", lt.get_utf8_text().unwrap());
/// ```
///
/// ## OCR on a specific region of the image
///
/// ```
/// # let mut lt = leptess::LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
/// # lt.set_image("./tests/di.png");
/// lt.set_rectangle(&leptess::leptonica::Box::new(
///     10, 10, 200, 60,
/// ).unwrap());
/// println!("{}", lt.get_utf8_text().unwrap());
/// ```
///
/// ## Iterate bounding boxes for each recognized word
///
/// ```
/// # let mut lt = leptess::LepTess::new(Some("./tests/tessdata"), "eng").unwrap();
/// # lt.set_image("./tests/di.png");
/// let boxes = lt.get_component_boxes(
///     leptess::capi::TessPageIteratorLevel_RIL_WORD,
///     true,
/// ).unwrap();
///
/// for b in &boxes {
///     println!("{:?}", b.get_val());
/// }
/// ```

pub struct LepTess {
    tess_api: tesseract::TessApi,
}

impl LepTess {
    pub fn new(data_path: Option<&str>, lang: &str) -> Result<LepTess, tesseract::TessInitError> {
        Ok(LepTess {
            tess_api: tesseract::TessApi::new(data_path, lang)?,
        })
    }

    /// Set image to use for OCR.
    pub fn set_image(&mut self, img_uri: impl AsRef<Path>) -> Result<(), leptonica::PixError> {
        // TODO: support more uri scheme, default to file://
        let pix = leptonica::pix_read(img_uri.as_ref())?;
        self.tess_api.set_image(&pix);
        Ok(())
    }

    pub fn set_image_from_mem(&mut self, img: &[u8]) -> Result<(), leptonica::PixError> {
        let pix = leptonica::pix_read_mem(img)?;
        self.tess_api.set_image(&pix);
        Ok(())
    }

    pub fn get_source_y_resolution(&mut self) -> i32 {
        self.tess_api.get_source_y_resolution()
    }

    pub fn get_image_dimensions(&self) -> Option<(u32, u32)> {
        self.tess_api.get_image_dimensions()
    }

    /// Override image resolution.
    /// Can be used to suppress "Warning: Invalid resolution 0 dpi." output.
    pub fn set_source_resolution(&mut self, res: i32) {
        self.tess_api.set_source_resolution(res)
    }

    /// Override image resolution if not detected
    pub fn set_fallback_source_resolution(&mut self, res: i32) {
        let resolution = self.get_source_y_resolution();
        if !(tesseract::MIN_CREDIBLE_RESOLUTION..=tesseract::MAX_CREDIBLE_RESOLUTION)
            .contains(&resolution)
        {
            self.tess_api.set_source_resolution(res)
        }
    }

    pub fn recognize(&self) -> i32 {
        self.tess_api.recognize()
    }

    /// Restrict OCR to a specific region of the image.
    pub fn set_rectangle(&mut self, b: &leptonica::Box) {
        self.tess_api.set_rectangle(b)
    }

    /// Extract text from current selected region of the image. By default, it is the full page.
    /// But it can be changed through [set_rectangle](struct.LepTess.html#method.set_rectangle)
    /// api.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    /// lt.set_image("./tests/di.png");
    /// println!("{}", lt.get_utf8_text().unwrap());
    /// ```
    pub fn get_utf8_text(&self) -> Result<String, std::str::Utf8Error> {
        self.tess_api.get_utf8_text()
    }

    /// Extract text from image as HTML with bounding box attributes.
    pub fn get_hocr_text(&self, page: c_int) -> Result<String, std::str::Utf8Error> {
        self.tess_api.get_hocr_text(page)
    }

    pub fn mean_text_conf(&self) -> i32 {
        self.tess_api.mean_text_conf()
    }

    pub fn get_regions(&self) -> Option<leptonica::Boxa> {
        self.tess_api.get_regions()
    }

    /// Get the given level kind of components (block, textline, word etc.) as a leptonica-style
    /// Boxa, in reading order.If text_only is true, then only text components are returned.
    ///
    /// # Example
    ///
    /// ## Get word bounding boxes
    ///
    /// ```no_run
    /// let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    /// lt.set_image("./tests/di.png");
    /// let boxes = lt.get_component_boxes(
    ///     leptess::capi::TessPageIteratorLevel_RIL_WORD,
    ///     true,
    /// ).unwrap();
    ///
    /// for b in &boxes {
    ///     println!("{:?}", b.get_val());
    /// }
    /// ```
    pub fn get_component_boxes(
        &self,
        level: capi::TessPageIteratorLevel,
        text_only: bool,
    ) -> Option<leptonica::Boxa> {
        self.tess_api.get_component_images(level, text_only)
    }
}
