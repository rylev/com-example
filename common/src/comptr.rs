// An issue with having T be Human is that I am never
// actually possessing the entire Human struct, just
// an interface pointer.
use super::*;
use std::os::raw::c_void;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct ComPtr<T> {
    // Interface pointer for interface T as a box.
    raw_ptr: *mut T
}

impl<T> Deref for ComPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {&*(self.raw_ptr)}
    }
}

impl<T> DerefMut for ComPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *(self.raw_ptr)}
    }
}

impl<T> ComPtr<T> {
    pub unsafe fn create_instance(rclsid: REFCLSID, pUnkOuter: *mut IUnknown, dwClsContext: DWORD, riid: REFIID) -> Self {
        let mut p = std::ptr::null_mut::<c_void>();
        // let hr = CoCreateInstance(rclsid, std::ptr::null_mut() as *mut IUnknown, dwClsContext, riid, &mut p);

        let hr = CoGetClassObject(rclsid, CLSCTX_INPROC_SERVER, std::ptr::null_mut::<c_void>(), riid, &mut p as *mut LPVOID);
        if failed(hr) {
            panic!("Failed to create instance!");
        }
        let raw_ptr = p as *mut T;

        ComPtr {
            raw_ptr
        }
    }
}

impl<T> Drop for ComPtr<T> {
    fn drop(&mut self) {
        unsafe {(*(self.raw_ptr as *mut IUnknown)).release();}
    }
}

