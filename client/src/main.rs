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
    COINIT_APARTMENTTHREADED, LPVOID, REFCLSID, REFIID,
};
use server::{IAnimal, IUnknown, CLSID_CAT, IID_IANIMAL};
use std::os::raw::c_void;

fn main() {
    unsafe {
        let mut hr = CoInitializeEx(std::ptr::null_mut::<c_void>(), COINIT_APARTMENTTHREADED);
        if failed(hr) {
            println!("Failed to initialize COM");
            return;
        }
        let mut unknown = std::ptr::null_mut::<c_void>();
        hr = CoGetClassObject(
            &CLSID_CAT as REFCLSID,
            CLSCTX_INPROC_SERVER,
            std::ptr::null_mut::<c_void>(),
            &IID_IUnknown as REFIID,
            &mut unknown as *mut LPVOID,
        );

        if failed(hr) {
            println!("Failed to get com class object {}", hr);
            return;
        }
        if unknown.is_null() {
            println!("Pointer to IUnknown is null");
            return;
        }
        println!("Got unknown.");

        let mut animal = std::ptr::null_mut::<c_void>();
        hr = (*(unknown as *mut IUnknown))
            .query_interface(&mut IID_IANIMAL, &mut animal as *mut LPVOID);

        if failed(hr) {
            println!("Failed to get IAnimal interface");
            return;
        }
        if animal.is_null() {
            println!("Pointer to IAnimal is null");
            return;
        }
        println!("Got animal.");
        (*(unknown as *mut IUnknown)).release();

        let animal = animal as *mut IAnimal;
        (*animal).eat();

        // This doesn't compile
        // hr = (*animal).ignore_humans();
        (*animal).release();

        CoUninitialize();
    };
}