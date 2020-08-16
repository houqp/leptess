//! Low level wrapper for Leptonica C API

use super::capi;

use std::convert::TryInto;
use std::ffi::CString;
use std::path::Path;

use thiserror;

pub struct Pix {
    pub raw: *mut capi::Pix,
}

impl Pix {
    pub fn get_w(&self) -> u32 {
        unsafe { (*self.raw).w }
    }
    pub fn get_h(&self) -> u32 {
        unsafe { (*self.raw).h }
    }
}

impl Drop for Pix {
    fn drop(&mut self) {
        unsafe {
            capi::pixDestroy(&mut self.raw);
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PixError {
    #[error("Failed to read image from {}", .0)]
    ReadFrom(&'static str),

    #[error("Path is not a valid utf8 string")]
    InvalidUtf8Path,

    #[error("Path contains invalid nul byte: {}", .source)]
    InvalidPathNulByte {
        #[from]
        source: std::ffi::NulError,
    },

    // this happens when usize has bit size 128 but leptonica only takes u64 address pointer
    #[error(transparent)]
    MemoryTooLarge {
        #[from]
        source: std::num::TryFromIntError,
    },
}

/// Read an image from a local file.
///
/// The provided path must be valid UTF-8.
pub fn pix_read(path: &Path) -> Result<Pix, PixError> {
    let s = path.to_str().ok_or(PixError::InvalidUtf8Path)?;
    let cs = CString::new(s)?;
    let pix = unsafe { capi::pixRead(cs.as_ptr()) };
    // on read errors, leptonica sets pointer to null and prints error message to stderr, so there
    // is no easy to capture and return the actual message programatically
    if pix.is_null() {
        Err(PixError::ReadFrom("file"))
    } else {
        Ok(Pix { raw: pix })
    }
}

/// Like pix_read, but redas the image from memory instead of disk
pub fn pix_read_mem(img: &[u8]) -> Result<Pix, PixError> {
    let pix = unsafe { capi::pixReadMem(img.as_ptr(), img.len().try_into()?) };
    if pix.is_null() {
        Err(PixError::ReadFrom("memory"))
    } else {
        Ok(Pix { raw: pix })
    }
}

#[derive(Debug, PartialEq)]
pub struct BoxVal {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub struct Box {
    pub raw: *mut capi::Box,
}

impl Drop for Box {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Box {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Option<Box> {
        unsafe {
            let p = capi::boxCreateValid(x, y, width, height);
            if p.is_null() {
                None
            } else {
                Some(Box { raw: p })
            }
        }
    }

    pub fn get_val(&self) -> BoxVal {
        unsafe {
            let v = *self.raw;
            BoxVal {
                x: v.x,
                y: v.y,
                w: v.w,
                h: v.h,
            }
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            capi::boxDestroy(&mut self.raw);
        }
    }
}

pub struct Boxa {
    pub raw: *mut capi::Boxa,
}

impl Drop for Boxa {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Boxa {
    pub fn get_n(&self) -> usize {
        unsafe { (*self.raw).n as usize }
    }

    pub fn get_box(&self, i: usize, flag: i32) -> Option<Box> {
        unsafe {
            let b = capi::boxaGetBox(self.raw, i as i32, flag);
            if b.is_null() {
                return None;
            }
            Some(Box { raw: b })
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            capi::boxaDestroy(&mut self.raw);
        }
    }
}

impl IntoIterator for Boxa {
    type Item = Box;
    type IntoIter = BoxaIterator;

    fn into_iter(self) -> Self::IntoIter {
        let count = self.get_n();
        BoxaIterator {
            boxa: self,
            index: 0,
            count,
        }
    }
}

pub struct BoxaIterator {
    boxa: Boxa,
    index: usize,
    count: usize,
}

impl Iterator for BoxaIterator {
    type Item = Box;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }

        let re = self.boxa.get_box(self.index, capi::L_CLONE as i32);
        self.index += 1;

        re
    }
}

impl<'a> IntoIterator for &'a Boxa {
    type Item = Box;
    type IntoIter = BoxaRefIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let count = self.get_n();
        BoxaRefIterator {
            boxa: self,
            index: 0,
            count,
        }
    }
}

pub struct BoxaRefIterator<'a> {
    boxa: &'a Boxa,
    index: usize,
    count: usize,
}

impl<'a> Iterator for BoxaRefIterator<'a> {
    type Item = Box;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }

        let re = self.boxa.get_box(self.index, capi::L_CLONE as i32);
        self.index += 1;

        re
    }
}
