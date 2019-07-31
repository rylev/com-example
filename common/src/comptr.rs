// An issue with having T be Human is that I am never
// actually possessing the entire Human struct, just
// an interface pointer.
use super::*;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct ComPtr<T> {
    raw_ptr: *mut T,
}

impl<T> Deref for ComPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(!self.raw_ptr.is_null());
        unsafe { &*self.raw_ptr }
    }
}

impl<T> DerefMut for ComPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.raw_ptr.is_null());
        unsafe { &mut *self.raw_ptr }
    }
}

impl<T> ComPtr<T> {
    /// *mut T must be safely convertable to *mut RawIUnknown
    pub unsafe fn new(raw_ptr: *mut T) -> Self {
        ComPtr { raw_ptr }
    }
}

impl<T> Drop for ComPtr<T> {
    fn drop(&mut self) {
        unsafe {
            (*(self.raw_ptr as *mut RawIUnknown)).raw_release();
        }
    }
}