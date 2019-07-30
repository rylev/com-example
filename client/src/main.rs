// import "unknwn.idl";
// [object, uuid(DF12E151-A29A-l1dO-8C2D-00BOC73925BA)]
// interface IAnimal : IUnknown {
//   HRESULT Eat(void);
// }
// [object, uuid(DF12E152-A29A-l1dO-8C2D-0080C73925BA)]
// interface ICat : IAnimal {
//   HRESULT IgnoreHumans(void);
// }

use common::{
    failed, CoGetClassObject, CoInitializeEx, CoUninitialize, IID_IUnknown, CLSCTX_INPROC_SERVER,
    COINIT_APARTMENTTHREADED, HRESULT, IID, LPVOID, REFCLSID, REFIID, ComPtr
};
use server::{IAnimal, IUnknown, CLSID_CAT, IID_IANIMAL};
use std::os::raw::c_void;

fn main() {
    let result = initialize_ex();

    if let Err(hr) = result {
        println!("Failed to initialize COM Library: {}", hr);
        return;
    }

    let result = get_class_object(&CLSID_CAT);
    let unknown = match result {
        Ok(unknown) => unknown,
        Err(hr) => {
            println!("Failed to get com class object {}", hr);
            return;
        }
    };
    println!("Got unknown.");
    let result = unknown.query_interface::<IAnimal>();
    // hr = (*(unknown as *mut IUnknown))
    //     .query_interface(&mut IID_IANIMAL, &mut animal as *mut LPVOID);

    // if failed(hr) {
    //     println!("Failed to get IAnimal interface");
    //     return;
    // }
    // if animal.is_null() {
    //     println!("Pointer to IAnimal is null");
    //     return;
    // }
    // println!("Got animal.");
    // (*(unknown as *mut IUnknown)).release();

    // let animal = animal as *mut IAnimal;
    // (*animal).eat();

    // // This doesn't compile
    // // hr = (*animal).ignore_humans();
    // (*animal).release();

    uninitialize();
}


// TODO: accept threading options
fn initialize_ex() -> Result<(), HRESULT> {
    let hr = unsafe { CoInitializeEx(std::ptr::null_mut::<c_void>(), COINIT_APARTMENTTHREADED) };
    if failed(hr) {
        // TODO: https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize
        // A thread must call CoUninitialize once for each successful call it has made to the
        // CoInitialize or CoInitializeEx function, including any call that returns S_FALSE.
        return Err(hr);
    }
    Ok(())
}

// TODO: accept server options
fn get_class_object(iid: &IID) -> Result<ComPtr<IUnknown>, HRESULT> {
    let mut unknown = std::ptr::null_mut::<c_void>();
    let hr = unsafe {
        CoGetClassObject(
            iid as REFCLSID,
            CLSCTX_INPROC_SERVER,
            std::ptr::null_mut::<c_void>(),
            &IID_IUnknown as REFIID,
            &mut unknown as *mut LPVOID,
        )
    };
    if failed(hr) {
        return Err(hr);
    }

    Ok(ComPtr::new(unknown as *const IUnknown))
}

fn uninitialize() {
    unsafe { CoUninitialize() }
}