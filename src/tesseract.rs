use super::capi;
use super::leptonica;

use std::ffi::{CStr, CString};
use std::ptr;

pub struct TessApi {
    pub raw: *mut capi::TessBaseAPI,
}

impl Drop for TessApi {
    fn drop(&mut self) {
        unsafe {
            capi::TessBaseAPIEnd(self.raw);
            capi::TessBaseAPIDelete(self.raw);
        }
    }
}

impl TessApi {
    pub fn new() -> TessApi {
        TessApi {
            raw: unsafe { capi::TessBaseAPICreate() },
        }
    }

    pub fn init(&self, lang: &str) -> i32 {
        unsafe {
            capi::TessBaseAPIInit3(self.raw, ptr::null(), CString::new(lang).unwrap().as_ptr())
        }
    }

    pub fn set_image(&self, img: leptonica::Pix) {
        unsafe { capi::TessBaseAPISetImage2(self.raw, img.raw as *mut capi::Pix) }
    }

    pub fn recognize(&self) -> i32 {
        unsafe { capi::TessBaseAPIRecognize(self.raw, ptr::null_mut()) }
    }

    pub fn set_rectangle(&self, b: leptonica::BoxVal) {
        unsafe {
            capi::TessBaseAPISetRectangle(self.raw, b.x, b.y, b.w, b.h);
        }
    }

    pub fn get_utf8_text(&self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let re: Result<String, std::str::Utf8Error>;
            let sptr = capi::TessBaseAPIGetUTF8Text(self.raw);
            match CStr::from_ptr(sptr).to_str() {
                Ok(s) => {
                    re = Ok(s.to_string());
                },
                Err(e) => {
                    re = Err(e);
                },
            }
            capi::TessDeleteText(sptr);
            return re;
        }
    }

    pub fn mean_text_conf(&self) -> i32 {
        unsafe { capi::TessBaseAPIMeanTextConf(self.raw) }
    }

    pub fn get_regions(&self) -> Option<leptonica::Boxa> {
        unsafe {
            let boxes = capi::TessBaseAPIGetRegions(
                self.raw,
                ptr::null_mut(),
            );
            match boxes.as_ref() {
                Some(p) => Some(leptonica::Boxa { raw: p }),
                None => None,
            }
        }
    }

    pub fn get_component_images(
        &self,
        level: capi::TessPageIteratorLevel,
        text_only: bool,
    ) -> Option<leptonica::Boxa> {
        let mut text_only_val: i32 = 0;
        if text_only {
            text_only_val = 1;
        }

        unsafe {
            let boxes = capi::TessBaseAPIGetComponentImages(
                self.raw,
                level,
                text_only_val,
                ptr::null_mut(),
                ptr::null_mut(),
            );

            match boxes.as_ref() {
                Some(p) => Some(leptonica::Boxa { raw: p }),
                None => None,
            }
        }
    }
}
