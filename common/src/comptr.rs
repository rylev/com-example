// An issue with having T be Human is that I am never
// actually possessing the entire Human struct, just
// an interface pointer.
use super::*;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct ComPtr<'a, T: 'a> {
    // Interface pointer for interface T as a box.
    raw_ptr: &'a mut T,
}

impl<T> Deref for ComPtr<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.raw_ptr
    }
}

impl<T> DerefMut for ComPtr<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.raw_ptr
    }
}

impl<T> ComPtr<'_, T> {
    pub unsafe fn new(raw_ptr: *const T) -> Self {
        let raw_ptr = &mut *(raw_ptr as *mut T);

        ComPtr { raw_ptr }
    }
}

impl<T> Drop for ComPtr<'_, T> {
    fn drop(&mut self) {
        unsafe {
            (*(self.raw_ptr as *mut T as *mut IUnknown)).raw_release();
        }
    }
}

