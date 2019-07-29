use common::{MyCat, HRESULT, IID};
use std::os::raw::c_void;

#[no_mangle]
#[link_name = "QueryInterface"]
pub extern "stdcall" fn query_interface(
    this: *const MyCat,
    riid: &IID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    unimplemented!()
}

#[no_mangle]
#[link_name = "AddRef"]
extern "stdcall" fn add_ref(this: *mut MyCat) -> u32 {
    println!("Adding ref");
    unsafe {
        (*this).ref_count += 1;
        (*this).ref_count
    }
}

#[no_mangle]
#[link_name = "Release"]
extern "stdcall" fn release(this: *mut MyCat) -> u32 {
    unsafe {
        (*this).ref_count -= 1;
        let count = (*this).ref_count;
        if count == 0 {
           let _ = Box::from_raw(this);
        }
        count
    }
}