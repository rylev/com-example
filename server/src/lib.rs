use common::{
    ICatVTable, IID_IUnknown, E_NOINTERFACE, HRESULT, IID, IID_CAT, IID_IANIMAL, NOERROR,
};
use std::os::raw::c_void;

#[repr(C)]
pub struct ICat {
    vtable: *const ICatVTable,
    ref_count: u32,
}

#[no_mangle]
#[link_name = "QueryInterface"]
pub extern "stdcall" fn query_interface(
    this: *const ICat,
    riid: *const IID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    unsafe {
        if *riid == IID_IUnknown || *riid == IID_CAT || *riid == IID_IANIMAL {
            *ppv = this as *mut c_void;
            NOERROR
        } else {
            E_NOINTERFACE
        }
    }
}

#[no_mangle]
#[link_name = "AddRef"]
extern "stdcall" fn add_ref(this: *mut ICat) -> u32 {
    println!("Adding ref");
    unsafe {
        (*this).ref_count += 1;
        (*this).ref_count
    }
}

#[no_mangle]
#[link_name = "Release"]
extern "stdcall" fn release(this: *mut ICat) -> u32 {
    unsafe {
        (*this).ref_count -= 1;
        let count = (*this).ref_count;
        if count == 0 {
            let _ = Box::from_raw(this);
        }
        count
    }
}