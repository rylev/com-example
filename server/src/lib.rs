use common::{
    failed, ComInterface, ComPtr, IID_IUnknown, IUnknownVTable, RawIUnknown,
    CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, HRESULT, IID, LPVOID, NOERROR, REFCLSID, REFIID,
};
use std::os::raw::c_void;

pub const IID_ICAT: IID = IID {
    data1: 0xf5353c58,
    data2: 0xcfd9,
    data3: 0x4204,
    data4: [0x8d, 0x92, 0xd2, 0x74, 0xc7, 0x57, 0x8b, 0x53],
};
pub const IID_IANIMAL: IID = IID {
    data1: 0xeff8970e,
    data2: 0xc50f,
    data3: 0x45e0,
    data4: [0x92, 0x84, 0x29, 0x1c, 0xe5, 0xa6, 0xf7, 0x71],
};

// C5F45CBC-4439-418C-A9F9-05AC67525E43
pub const CLSID_CAT: IID = IID {
    data1: 0xC5F45CBC,
    data2: 0x4439,
    data3: 0x418C,
    data4: [0xA9, 0xF9, 0x05, 0xAC, 0x67, 0x52, 0x5E, 0x43],
};

#[repr(C)]
pub struct RawICat {
    vtable: *const ICatVTable,
}

impl RawICat {
    unsafe fn raw_ignore_humans(&mut self) -> HRESULT {
        ((*self.vtable).IgnoreHumans)(self as *mut RawICat)
    }
}

impl std::convert::AsRef<RawIUnknown> for RawICat {
    fn as_ref(&self) -> &RawIUnknown {
        unsafe { &*(self as *const RawICat as *const RawIUnknown) }
    }
}

impl std::convert::AsMut<RawIUnknown> for RawICat {
    fn as_mut(&mut self) -> &mut RawIUnknown {
        unsafe { &mut *(self as *mut RawICat as *mut RawIUnknown) }
    }
}

impl std::convert::AsRef<RawIAnimal> for RawICat {
    fn as_ref(&self) -> &RawIAnimal {
        unsafe { &*(self as *const RawICat as *const RawIAnimal) }
    }
}

impl std::convert::AsMut<RawIAnimal> for RawICat {
    fn as_mut(&mut self) -> &mut RawIAnimal {
        unsafe { &mut *(self as *mut RawICat as *mut RawIAnimal) }
    }
}

#[repr(C)]
pub struct ICat {
    inner: RawICat,
}

impl ComInterface for ICat {
    const IID: IID = IID_ICAT;
}
impl ICat {
    pub fn query_interface<T: ComInterface>(&mut self) -> Option<ComPtr<T>> {
        let inner: &mut RawIUnknown = self.inner.as_mut();
        inner.query_interface()
    }

    pub fn eat(&mut self) {
        let inner: &mut RawIAnimal = self.inner.as_mut();
        inner.eat()
    }

    pub fn ignore_humans(&mut self) {
        let _ = unsafe { self.inner.raw_ignore_humans() };
    }
}


#[allow(non_snake_case)]
#[repr(C)]
pub struct ICatVTable {
    iunknown: IUnknownVTable,
    // IAnimal
    pub Eat: unsafe extern "stdcall" fn(*mut RawICat) -> HRESULT,
    // ICat
    pub IgnoreHumans: unsafe extern "stdcall" fn(*mut RawICat) -> HRESULT,
}

#[repr(C)]
pub struct RawIAnimal {
    vtable: *const ICatVTable,
}

impl RawIAnimal {
    pub fn eat(&mut self) {
        let _ = unsafe { self.raw_eat() };
    }

    pub unsafe fn raw_eat(&mut self) -> HRESULT {
        ((*self.vtable).Eat)(self as *mut RawIAnimal as *mut RawICat)
    }
}

impl std::convert::AsRef<RawIUnknown> for RawIAnimal {
    fn as_ref(&self) -> &RawIUnknown {
        unsafe { &*(self as *const RawIAnimal as *const RawIUnknown) }
    }
}

impl std::convert::AsMut<RawIUnknown> for RawIAnimal {
    fn as_mut(&mut self) -> &mut RawIUnknown {
        unsafe { &mut *(self as *mut RawIAnimal as *mut RawIUnknown) }
    }
}

#[repr(C)]
pub struct IAnimal {
    inner: RawIAnimal,
}

impl IAnimal {
    pub fn eat(&mut self) {
        self.inner.eat()
    }

    pub fn query_interface<T: ComInterface>(&mut self) -> Option<ComPtr<T>> {
        let inner: &mut RawIUnknown = self.inner.as_mut();
        inner.query_interface()
    }
}
impl ComInterface for IAnimal {
    const IID: IID = IID_IANIMAL;
}

#[repr(C)]
pub struct Cat {
    // inner must always be first because Cat is actually an ICat with one extra field at the end
    inner: ICat,
    ref_count: u32,
}

impl Drop for Cat {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.inner.inner.vtable as *mut ICatVTable) };
    }
}

unsafe extern "stdcall" fn query_interface(
    this: *mut RawIUnknown,
    riid: *const IID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    println!("Querying interface...");
    if *riid == IID_IUnknown || *riid == IID_ICAT || *riid == IID_IANIMAL {
        *ppv = this as *mut c_void;
        (*this).raw_add_ref();
        NOERROR
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "stdcall" fn add_ref(this: *mut RawIUnknown) -> u32 {
    println!("Adding ref...");
    let this = this as *mut Cat;
    (*this).ref_count += 1;
    println!("Count now {}", (*this).ref_count);
    (*this).ref_count
}

// TODO: This could potentially be null or pointing to some invalid memory
unsafe extern "stdcall" fn release(this: *mut RawIUnknown) -> u32 {
    println!("Releasing...");
    let this = this as *mut Cat;
    (*this).ref_count -= 1;
    println!("Count now {}", (*this).ref_count);
    let count = (*this).ref_count;
    if count == 0 {
        println!("Count is 0. Freeing memory...");
        let _ = Box::from_raw(this);
    }
    count
}

unsafe extern "stdcall" fn ignore_humans(_this: *mut RawICat) -> HRESULT {
    println!("Ignoring...");
    NOERROR
}

unsafe extern "stdcall" fn eat(_this: *mut RawICat) -> HRESULT {
    println!("Eating...");
    NOERROR
}

impl Cat {
    fn new() -> Cat {
        println!("Allocating new Vtable...");
        let iunknown = IUnknownVTable {
            QueryInterface: query_interface,
            Release: release,
            AddRef: add_ref,
        };
        let vtable = Box::into_raw(Box::new(ICatVTable {
            iunknown,
            Eat: eat,
            IgnoreHumans: ignore_humans,
        }));
        let inner = RawICat { vtable };
        Cat {
            inner: ICat { inner },
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
        let hr = (*(cat as *mut RawIUnknown)).raw_query_interface(riid, ppv);
        if failed(hr) {
            println!("Querying new object failed... Deallocating object...");
            let _ = Box::from_raw(cat);
        }
        hr
    }
}