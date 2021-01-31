//! Low level wrapper for Tesseract C API

use super::capi;
use super::leptonica;
use std::os::raw::c_int;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

pub use capi::kMaxCredibleResolution as MAX_CREDIBLE_RESOLUTION;
pub use capi::kMinCredibleResolution as MIN_CREDIBLE_RESOLUTION;

#[derive(Debug, PartialEq)]
pub struct TessInitError {
    pub code: i32,
}

impl std::error::Error for TessInitError {}

impl std::fmt::Display for TessInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TessInitError{{{}}}", self.code)
    }
}

#[derive(Debug, PartialEq)]
pub struct TessApi {
    pub raw: *mut capi::TessBaseAPI,
    data_path_cptr: *mut c_char,
}

impl Drop for TessApi {
    fn drop(&mut self) {
        unsafe {
            capi::TessBaseAPIEnd(self.raw);
            capi::TessBaseAPIDelete(self.raw);

            if !self.data_path_cptr.is_null() {
                // free data_path_cptr, drop trait will take care of it
                CString::from_raw(self.data_path_cptr);
            }
        }
    }
}

impl TessApi {
    pub fn new<'a>(data_path: Option<&'a str>, lang: &'a str) -> Result<TessApi, TessInitError> {
        let data_path_cptr;
        let data_path_cstr;
        let lang = CString::new(lang).unwrap();
        match data_path {
            Some(dstr) => {
                data_path_cstr = CString::new(dstr).unwrap();
                data_path_cptr = data_path_cstr.into_raw();
            }
            None => {
                data_path_cptr = ptr::null_mut();
            }
        }

        let api = TessApi {
            raw: unsafe { capi::TessBaseAPICreate() },
            data_path_cptr,
        };

        unsafe {
            let re = capi::TessBaseAPIInit3(api.raw, api.data_path_cptr, lang.as_ptr());

            if re == 0 {
                Ok(api)
            } else {
                Err(TessInitError { code: re })
            }
        }
    }

    /// Provide an image for Tesseract to recognize.
    ///
    /// set_image clears all recognition results, and sets the rectangle to the full image, so it
    /// may be followed immediately by a `[Self::get_utf8_text]`, and it will automatically perform
    /// recognition.
    pub fn set_image(&mut self, img: &leptonica::Pix) {
        // "Tesseract takes its own copy of the image, so it need not persist until after Recognize"
        unsafe { capi::TessBaseAPISetImage2(self.raw, img.raw as *mut capi::Pix) }
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
        unsafe {
            let pix = capi::TessBaseAPIGetInputImage(self.raw);
            if pix.is_null() {
                return None;
            }

            Some(((*pix).w as u32, (*pix).h as u32))
        }
    }

    pub fn get_source_y_resolution(&mut self) -> i32 {
        unsafe { capi::TessBaseAPIGetSourceYResolution(self.raw) }
    }

    /// Override image resolution.
    /// Can be used to suppress "Warning: Invalid resolution 0 dpi." output.
    pub fn set_source_resolution(&mut self, res: i32) {
        unsafe { capi::TessBaseAPISetSourceResolution(self.raw, res) }
    }

    pub fn recognize(&self) -> i32 {
        unsafe { capi::TessBaseAPIRecognize(self.raw, ptr::null_mut()) }
    }

    pub fn set_rectangle(&mut self, b: &leptonica::Box) {
        let v = b.get_val();
        unsafe {
            capi::TessBaseAPISetRectangle(self.raw, v.x, v.y, v.w, v.h);
        }
    }

    pub fn get_utf8_text(&self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let re: Result<String, std::str::Utf8Error>;
            let sptr = capi::TessBaseAPIGetUTF8Text(self.raw);
            match CStr::from_ptr(sptr).to_str() {
                Ok(s) => {
                    re = Ok(s.to_string());
                }
                Err(e) => {
                    re = Err(e);
                }
            }
            capi::TessDeleteText(sptr);
            re
        }
    }

    pub fn get_hocr_text(&self, page: c_int) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let sptr = capi::TessBaseAPIGetHOCRText(self.raw, page);
            let re = match CStr::from_ptr(sptr).to_str() {
                Ok(s) => Ok(s.to_string()),
                Err(e) => Err(e),
            };
            capi::TessDeleteText(sptr);
            re
        }
    }

    pub fn mean_text_conf(&self) -> i32 {
        unsafe { capi::TessBaseAPIMeanTextConf(self.raw) }
    }

    pub fn get_regions(&self) -> Option<leptonica::Boxa> {
        unsafe {
            let boxes = capi::TessBaseAPIGetRegions(self.raw, ptr::null_mut());
            if boxes.is_null() {
                None
            } else {
                Some(leptonica::Boxa { raw: boxes })
            }
        }
    }

    /// Get the given level kind of components (block, textline, word etc.) as a leptonica-style
    /// Boxa, in reading order.If text_only is true, then only text components are returned.
    pub fn get_component_images(
        &self,
        level: capi::TessPageIteratorLevel,
        text_only: bool,
    ) -> Option<leptonica::Boxa> {
        let text_only_val: i32 = if text_only { 1 } else { 0 };
        unsafe {
            let boxes = capi::TessBaseAPIGetComponentImages(
                self.raw,
                level,
                text_only_val,
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if boxes.is_null() {
                None
            } else {
                Some(leptonica::Boxa { raw: boxes })
            }
        }
    }
}
