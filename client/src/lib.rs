use common::{
    IAnimal, IID_IUnknown, IUnknown, CLSID_CAT, IID_IANIMAL, LPVOID, REFCLSID,
    REFIID, comptr::ComPtr, IID_ICAT, ICat, CLSCTX_INPROC_SERVER
};

use std::os::raw::c_void;

pub unsafe fn eat() {
    let mut animal_ptr = ComPtr::<IAnimal>::create_instance(
        &CLSID_CAT,
        std::ptr::null_mut::<c_void>() as *mut IUnknown,
        CLSCTX_INPROC_SERVER,
        &mut IID_IANIMAL as REFIID
    );

    animal_ptr.eat();
}