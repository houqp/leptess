//! Low level wrapper for Leptonica C API

use super::capi;

use std::ffi::CString;
use std::path::Path;

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
    pub fn destroy(&mut self) {
        unsafe { capi::pixDestroy(&mut self.raw) }
    }
}

pub fn pix_read(path: &Path) -> Option<Pix> {
    let s = path.to_str().unwrap();

    unsafe {
        let pix = capi::pixRead(CString::new(s).unwrap().as_ptr());
        if pix.is_null() {
            return None;
        }

        return Some(Pix { raw: pix });
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
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Option<Box> {
        unsafe {
            let p = capi::boxCreateValid(x, y, w, h);
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

    pub fn get_box(&self, i: usize, flag: u32) -> Option<Box> {
        unsafe {
            let b = capi::boxaGetBox(self.raw, i as i32, flag as i32);
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
            count: count,
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

        let re = self.boxa.get_box(self.index, capi::L_CLONE);
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
            count: count,
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

        let re = self.boxa.get_box(self.index, capi::L_CLONE);
        self.index += 1;

        re
    }
}
