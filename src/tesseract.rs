//! Low level wrapper for Tesseract C API

use crate::leptonica::BoxGeometry;

use super::leptonica;
use std::os::raw::c_int;
use thiserror;

use std::ffi::{CStr, CString};

pub use capi::kMaxCredibleResolution as MAX_CREDIBLE_RESOLUTION;
pub use capi::kMinCredibleResolution as MIN_CREDIBLE_RESOLUTION;
use capi::{TessPageIteratorLevel, TessPageIteratorLevel_RIL_BLOCK};

#[derive(thiserror::Error, Debug, PartialEq)]
#[error("TessInitError{{{}}}", .code)]
pub struct TessInitError {
    pub code: i32,
}

#[derive(thiserror::Error, Debug, PartialEq)]
#[error("Tess set_variable returned false")]
pub struct TessSetVariableError();

#[derive(Debug)]
pub struct TessApi {
    pub raw: tesseract_plumbing::TessBaseApi,
}

impl TessApi {
    pub fn new<'a>(data_path: Option<&'a str>, lang: &'a str) -> Result<TessApi, TessInitError> {
        let data_path_cstr = data_path.map(|dp| CString::new(dp).unwrap());
        let lang = CString::new(lang).unwrap();

        let mut api = TessApi {
            raw: tesseract_plumbing::TessBaseApi::create(),
        };

        match api
            .raw
            .init_2(data_path_cstr.as_deref(), Some(lang.as_ref()))
        {
            Err(tesseract_plumbing::TessBaseApiInitError()) => Err(TessInitError { code: -1 }),
            Ok(()) => Ok(api),
        }
    }

    /// Provide an image for Tesseract to recognize.
    ///
    /// set_image clears all recognition results, and sets the rectangle to the full image, so it
    /// may be followed immediately by a `[Self::get_utf8_text]`, and it will automatically perform
    /// recognition.
    pub fn set_image(&mut self, img: &leptonica::Pix) {
        self.raw.set_image_2(&img.raw)
    }

    /// Get the dimensions of the currently loaded image, or None if no image is loaded.
    ///
    /// # Example
    /// ```rust
    /// let path = std::path::Path::new("tests/di.png");
    /// let img = leptess::leptonica::pix_read(&path).unwrap();
    ///
    /// let mut tes = leptess::tesseract::TessApi::new(Some("tests/tessdata"), "eng").unwrap();
    /// tes.set_image(&img);
    ///
    /// assert_eq!(tes.get_image_dimensions(), Some((442, 852)));
    /// ```
    pub fn get_image_dimensions(&self) -> Option<(u32, u32)> {
        let pix = self.raw.get_input_image()?;
        Some((pix.get_width() as u32, pix.get_height() as u32))
    }

    pub fn get_source_y_resolution(&self) -> i32 {
        self.raw.get_source_y_resolution()
    }

    /// Override image resolution.
    /// Can be used to suppress "Warning: Invalid resolution 0 dpi." output.
    pub fn set_source_resolution(&mut self, res: i32) {
        self.raw.set_source_resolution(res)
    }

    pub fn recognize(&mut self) -> i32 {
        match self.raw.recognize() {
            Ok(()) => 0,
            Err(tesseract_plumbing::TessBaseApiRecogniseError()) => -1,
        }
    }

    pub fn set_rectangle(&mut self, left: i32, top: i32, width: i32, height: i32) {
        self.raw.set_rectangle(left, top, width, height);
    }

    pub fn set_rectangle_from_box(&mut self, b: &leptonica::Box) {
        let BoxGeometry { x, y, w, h } = b.get_geometry();
        self.set_rectangle(x, y, w, h);
    }

    pub fn get_utf8_text(&mut self) -> Result<String, std::str::Utf8Error> {
        let text = self.raw.get_utf8_text().unwrap();
        let cstr: &CStr = text.as_ref();
        match cstr.to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn get_hocr_text(&mut self, page: c_int) -> Result<String, std::str::Utf8Error> {
        let text = self.raw.get_hocr_text(page).unwrap();
        let cstr: &CStr = text.as_ref();
        match cstr.to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn get_alto_text(&mut self, page: c_int) -> Result<String, std::str::Utf8Error> {
        let text = self.raw.get_alto_text(page).unwrap();
        let cstr: &CStr = text.as_ref();
        match cstr.to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn get_tsv_text(&mut self, page: c_int) -> Result<String, std::str::Utf8Error> {
        let text = self.raw.get_tsv_text(page).unwrap();
        let cstr: &CStr = text.as_ref();
        match cstr.to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn get_lstm_box_text(&mut self, page: c_int) -> Result<String, std::str::Utf8Error> {
        let text = self.raw.get_lstm_box_text(page).unwrap();
        let cstr: &CStr = text.as_ref();
        match cstr.to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn get_word_str_box_text(&mut self, page: c_int) -> Result<String, std::str::Utf8Error> {
        let text = self.raw.get_word_str_box_text(page).unwrap();
        let cstr: &CStr = text.as_ref();
        match cstr.to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn mean_text_conf(&self) -> i32 {
        self.raw.mean_text_conf()
    }

    pub fn get_regions(&self) -> Option<leptonica::Boxa> {
        self.get_component_images(TessPageIteratorLevel_RIL_BLOCK, false)
    }

    /// Get the given level kind of components (block, textline, word etc.) as a leptonica-style
    /// Boxa, in reading order.If text_only is true, then only text components are returned.
    pub fn get_component_images(
        &self,
        level: TessPageIteratorLevel,
        text_only: bool,
    ) -> Option<leptonica::Boxa> {
        match self
            .raw
            .get_component_images_1(level, if text_only { 1 } else { 0 })
        {
            Ok(boxa) => Some(leptonica::Boxa { raw: boxa }),
            Err(_) => None,
        }
    }
}
