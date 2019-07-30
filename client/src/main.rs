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
    failed, CoGetClassObject, CoInitializeEx, CoUninitialize, IAnimal, IID_IUnknown, IUnknown,
    CLSCTX_INPROC_SERVER, CLSID_CAT, COINIT_APARTMENTTHREADED, IID_IANIMAL, LPVOID, REFCLSID,
    REFIID, comptr::ComPtr, IID_ICAT, ICat
};
use std::os::raw::c_void;
use com_client::eat;

fn main() {
    unsafe {
        // COM Library Initialisation
        let hr = CoInitializeEx(std::ptr::null_mut::<c_void>(), COINIT_APARTMENTTHREADED);
        if failed(hr) {
            println!("Failed to initialize COM Library!");
            return;
        }

        eat();

        // Uninitialise COM Library
        CoUninitialize();
    };
}
