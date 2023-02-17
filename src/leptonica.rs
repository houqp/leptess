//! Low level wrapper for Leptonica C API
extern crate tesseract_plumbing;

use std::convert::TryInto;
use std::ffi::CString;
use std::fmt::Debug;
use std::path::Path;

use self::tesseract_plumbing::leptonica_plumbing;
use thiserror;

pub struct Pix {
    pub raw: leptonica_plumbing::memory::RefCounted<leptonica_plumbing::Pix>,
}

impl Pix {
    pub fn get_w(&self) -> u32 {
        self.raw.get_width().try_into().unwrap()
    }

    pub fn get_h(&self) -> u32 {
        self.raw.get_height().try_into().unwrap()
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
        Ok(raw) => Ok(Pix {
            raw: raw.to_ref_counted(),
        }),
    }
}

/// Like pix_read, but reads the image from memory instead of disk
pub fn pix_read_mem(img: &[u8]) -> Result<Pix, PixError> {
    match leptonica_plumbing::Pix::read_mem(img) {
        Err(leptonica_plumbing::PixReadMemError::ImageSizeConversion(source)) => {
            Err(PixError::MemoryTooLarge { source })
        }
        Err(leptonica_plumbing::PixReadMemError::NullPtr) => Err(PixError::ReadFrom("memory")),
        Ok(raw) => Ok(Pix {
            raw: raw.to_ref_counted(),
        }),
    }
}

pub struct Boxa {
    pub raw: leptonica_plumbing::memory::RefCountedExclusive<leptonica_plumbing::Boxa>,
}

impl Boxa {
    pub fn get_n(&self) -> usize {
        self.raw.get_count().try_into().unwrap()
    }

    pub fn get_box(
        &self,
        i: usize,
    ) -> Option<leptonica_plumbing::memory::RefCounted<leptonica_plumbing::Box>> {
        self.raw.get_box_cloned(i.try_into().ok()?)
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

pub struct Box {
    pub raw: leptonica_plumbing::memory::RefCounted<leptonica_plumbing::Box>,
}

#[derive(Debug, PartialEq)]
pub struct BoxGeometry {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Box {
    pub fn get_geometry(&self) -> BoxGeometry {
        let mut bg = BoxGeometry {
            x: -1,
            y: -1,
            w: -1,
            h: -1,
        };
        self.raw.get_geometry(
            Some(&mut bg.x),
            Some(&mut bg.y),
            Some(&mut bg.w),
            Some(&mut bg.h),
        );
        bg
    }
}

impl Debug for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Box")
            .field("geometry", &self.get_geometry())
            .finish()
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

        let re = self.boxa.get_box(self.index).map(|b| Box { raw: b });
        self.index += 1;

        re
    }
}
