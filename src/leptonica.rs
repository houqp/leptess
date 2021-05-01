//! Low level wrapper for Leptonica C API
extern crate tesseract_plumbing;

use std::ffi::CString;
use std::path::Path;

use self::tesseract_plumbing::leptonica_plumbing;
use thiserror;

pub struct Pix {
    pub raw: leptonica_plumbing::Pix,
}

pub use self::leptonica_plumbing::Box;

impl Pix {
    pub fn get_w(&self) -> u32 {
        let lpix: &leptonica_plumbing::leptonica_sys::Pix = self.raw.as_ref();
        lpix.w
    }

    pub fn get_h(&self) -> u32 {
        let lpix: &leptonica_plumbing::leptonica_sys::Pix = self.raw.as_ref();
        lpix.h
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
    match leptonica_plumbing::Pix::read(&cs) {
        Err(leptonica_plumbing::PixReadError()) => Err(PixError::ReadFrom("file")),
        Ok(raw) => Ok(Pix { raw }),
    }
}

/// Like pix_read, but reads the image from memory instead of disk
pub fn pix_read_mem(img: &[u8]) -> Result<Pix, PixError> {
    match leptonica_plumbing::Pix::read_mem(img) {
        Err(leptonica_plumbing::PixReadMemError::ImageSizeConversion(source)) => {
            Err(PixError::MemoryTooLarge { source })
        }
        Err(leptonica_plumbing::PixReadMemError::NullPtr) => Err(PixError::ReadFrom("memory")),
        Ok(raw) => Ok(Pix { raw }),
    }
}

pub struct Boxa {
    pub raw: leptonica_plumbing::Boxa,
}

impl Boxa {
    pub fn get_n(&self) -> usize {
        let lboxa: &leptonica_plumbing::leptonica_sys::Boxa = self.raw.as_ref();
        lboxa.n as usize
    }

    pub fn get_box(&self, i: usize) -> Option<leptonica_plumbing::BorrowedBox> {
        self.raw.get(std::convert::TryInto::try_into(i).ok()?)
    }
}

impl<'a> IntoIterator for &'a Boxa {
    type Item = leptonica_plumbing::BorrowedBox<'a>;
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
    type Item = leptonica_plumbing::BorrowedBox<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }

        let re = self.boxa.get_box(self.index);
        self.index += 1;

        re
    }
}
