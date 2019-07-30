use common::{
    failed, ICat, ICatVTable, IID_IUnknown, CLASS_E_CLASSNOTAVAILABLE, CLSID_CAT, E_NOINTERFACE,
    HRESULT, IID, IID_IANIMAL, IID_ICAT, LPVOID, NOERROR, REFCLSID, REFIID,
};
use std::os::raw::c_void;

#[repr(C)]
pub struct Cat {
    // vtable must always be valid and non-null
    inner: ICat,
    ref_count: u32,
}

impl Drop for Cat {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.inner.vtable as *mut ICatVTable) };
    }
}

extern "stdcall" fn query_interface(
    this: *mut ICat,
    riid: *const IID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    println!("Querying interface...");
    unsafe {
        if *riid == IID_IUnknown || *riid == IID_ICAT || *riid == IID_IANIMAL {
            *ppv = this as *mut c_void;
            ((*(*(this as *mut Cat)).inner.vtable).AddRef)(this);
            NOERROR
        } else {
            E_NOINTERFACE
        }
    }
}

extern "stdcall" fn add_ref(this: *mut ICat) -> u32 {
    println!("Adding ref...");
    let this = this as *mut Cat;
    unsafe {
        (*this).ref_count += 1;
        println!("Count now {}", (*this).ref_count);
        (*this).ref_count
    }
}

// TODO: This could potentially be null or pointing to some invalid memory
extern "stdcall" fn release(this: *mut ICat) -> u32 {
    println!("Releasing...");
    let this = this as *mut Cat;
    unsafe {
        (*this).ref_count -= 1;
        println!("Count now {}", (*this).ref_count);
        let count = (*this).ref_count;
        if count == 0 {
            println!("Count is 0. Freeing memory...");
            let _ = Box::from_raw(this);
        }
        count
    }
}

extern "stdcall" fn ignore_humans(this: *mut ICat) -> HRESULT {
    println!("Ignoring...");
    NOERROR
}

extern "stdcall" fn eat(this: *mut ICat) -> HRESULT {
    println!("Eating...");
    NOERROR
}

impl Cat {
    fn new() -> Cat {
        println!("Allocating new Vtable...");
        let vtable = Box::into_raw(Box::new(ICatVTable {
            QueryInterface: query_interface,
            Release: release,
            AddRef: add_ref,
            Eat: eat,
            IgnoreHumans: ignore_humans,
        }));
        Cat {
            inner: ICat { vtable },
            ref_count: 0,
        }
    }
}

#[no_mangle]
extern "stdcall" fn DllGetClassObject(rclsid: REFCLSID, riid: REFIID, ppv: *mut LPVOID) -> HRESULT {
    unsafe {
        if *rclsid != CLSID_CAT {
            return CLASS_E_CLASSNOTAVAILABLE;
        }
        println!("Allocating new object...");
        let cat = Box::into_raw(Box::new(Cat::new()));
        let hr = ((*(*cat).inner.vtable).QueryInterface)(cat as *mut ICat, riid, ppv);
        if failed(hr) {
            println!("Querying new object failed... Deallocating object...");
            let _ = Box::from_raw(cat);
        }
        hr
    }
}