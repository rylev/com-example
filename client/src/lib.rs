// import "unknwn.idl";
// [object, uuid(DF12E151-A29A-l1dO-8C2D-00BOC73925BA)]
// interface IAnimal : IUnknown {
//   HRESULT Eat(void);
// }
// [object, uuid(DF12E152-A29A-l1dO-8C2D-0080C73925BA)]
// interface ICat : IAnimal {
//   HRESULT ignoreHumans(void);
// }

use common::{
    failed, CoGetClassObject, CoInitializeEx, CoUninitialize, MyCat, CLSCTX_INPROC_SERVER,
    COINIT_APARTMENTTHREADED, HRESULT, IID, LPVOID, REFCLSID, REFIID,
};
use std::os::raw::c_void;

const CLSID_CAT: IID = IID {
    data1: 0xf5353c58,
    data2: 0xcfd9,
    data3: 0x4204,
    data4: [0x8d, 0x92, 0xd2, 0x74, 0xc7, 0x57, 0x8b, 0x53],
};
const CLSID_IANIMAL: IID = IID {
    data1: 0xeff8970e,
    data2: 0xc50f,
    data3: 0x45e0,
    data4: [0x92, 0x84, 0x29, 0x1c, 0xe5, 0xa6, 0xf7, 0x71],
};

extern "C" {
    static IID_IUnknown: IID;
}

pub trait IUnknown {
    unsafe fn query_interface(&self, iid: &IID, object: *mut *mut c_void) -> HRESULT;
    unsafe fn add_ref(&self) -> u32;
    unsafe fn release(&self) -> u32;
}

impl IUnknown for MyCat {
    unsafe fn query_interface(&self, iid: &IID, object: *mut *mut c_void) -> HRESULT {
        ((*self.iunknown_vtable).query_interface)(self, iid, object)
    }

    unsafe fn add_ref(&self) -> u32 {
        ((*self.iunknown_vtable).add_ref)(self)
    }

    unsafe fn release(&self) -> u32 {
        ((*self.iunknown_vtable).release)(self)
    }
}


pub trait ICat: IAnimal {
    unsafe fn ignore_humans(&self) -> HRESULT;
}

impl ICat for MyCat {
    unsafe fn ignore_humans(&self) -> HRESULT {
        ((*self.icat_vtable).ignore_humans)(self)
    }
}

pub trait IAnimal: IUnknown {
    unsafe fn eat(&self) -> HRESULT;
}

impl IAnimal for MyCat {
    unsafe fn eat(&self) -> HRESULT {
        ((*self.ianimal_vtable).eat)(self)
    }
}


pub fn eat_and_ignore_humans() {
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
            println!("Failed to get com class object");
            return;
        }
        if unknown.is_null() {
            println!("Pointer to IUnknown is null");
            return;
        }
        let unknown = std::ptr::NonNull::new_unchecked(unknown as *mut MyCat);
        let mut animal = std::ptr::null_mut::<c_void>();
        hr = unknown
            .as_ref()
            .query_interface(&CLSID_IANIMAL, &mut animal as *mut LPVOID);
        if failed(hr) {
            println!("Failed to get IAnimal interface");
            return;
        }
        if animal.is_null() {
            println!("Pointer to IAnimal is null");
            return;
        }
        let animal = std::ptr::NonNull::new_unchecked(animal as *mut MyCat);

        // TODO: this should not be possible
        hr = animal.as_ref().ignore_humans();
        if failed(hr) {
            println!("Failed to ignore humans");
            return;
        }
        animal.as_ref().release();

        CoUninitialize();
    };
}

use std::ptr::NonNull;

#[derive(Debug)]
pub struct ComPtr<T: IUnknown> {
    ptr: NonNull<T>,
}

impl<T: IUnknown> ComPtr<T> {
    pub fn new(ptr: NonNull<T>) -> ComPtr<T> {
        ComPtr { ptr }
    }
}

impl<T: IUnknown> std::ops::Deref for ComPtr<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.ptr.as_ptr() }
    }
}

impl<T: IUnknown> Clone for ComPtr<T> {
    fn clone(&self) -> ComPtr<T> {
        unsafe { (*self.ptr.as_ptr()).add_ref() };
        ComPtr { ptr: self.ptr }
    }
}

impl<T: IUnknown> Drop for ComPtr<T> {
    fn drop(&mut self) {
        unsafe { (*self.ptr.as_ptr()).release() };
    }
}