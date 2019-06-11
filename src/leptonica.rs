use super::capi;

use std::ffi::CString;
use std::path::Path;

pub struct Pix {
    pub raw: *const capi::Pix,
}

impl Pix {
    pub fn get_w(&self) -> u32 {
        unsafe { (*self.raw).w }
    }
    pub fn get_h(&self) -> u32 {
        unsafe { (*self.raw).h }
    }
    pub fn destroy(&self) {
        unsafe { capi::pixDestroy(&mut (self.raw as *mut capi::Pix)) }
    }
}

pub fn pix_read(path: &Path) -> Option<Pix> {
    let s = path.to_str().unwrap();

    unsafe {
        let pix = capi::pixRead(CString::new(s).unwrap().as_ptr());
        match pix.as_ref() {
            Some(p) => Some(Pix { raw: p }),
            None => None,
        }
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
    pub raw: *const capi::Box,
}

impl Box {
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
}

pub struct Boxa {
    pub raw: *const capi::Boxa,
}

impl Boxa {
    pub fn get_n(&self) -> usize {
        unsafe { (*self.raw).n as usize }
    }

    pub fn get_box(&self, i: usize, flag: u32) -> Option<Box> {
        unsafe {
            let b = capi::boxaGetBox(self.raw as *mut capi::Boxa, i as i32, flag as i32);
            match b.as_ref() {
                Some(p) => Some(Box { raw: p }),
                None => None,
            }
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
