
mod implementation;
mod interface;
use common::{failed, RawIUnknown, CLASS_E_CLASSNOTAVAILABLE, HRESULT, LPVOID, REFCLSID, REFIID};
use implementation::Cat;

pub use interface::{IAnimal, ICat, CLSID_CAT};

#[no_mangle]
extern "stdcall" fn DllGetClassObject(rclsid: REFCLSID, riid: REFIID, ppv: *mut LPVOID) -> HRESULT {
    unsafe {
        if *rclsid != CLSID_CAT {
            return CLASS_E_CLASSNOTAVAILABLE;
        }
        println!("Allocating new object...");
        let cat = Box::into_raw(Box::new(Cat::new()));
        let hr = (*(cat as *mut RawIUnknown)).raw_query_interface(riid, ppv);
        if failed(hr) {
            println!("Querying new object failed... Deallocating object...");
            let _ = Box::from_raw(cat);
        }
        hr
    }
}