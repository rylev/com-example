// These are all defined in [winapi](https://github.com/retep998/winapi-rs)

use std::os::raw::c_void;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct IID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub type REFCLSID = *const IID;
pub type REFIID = *const IID;

pub type HRESULT = c_long;
pub fn failed(result: HRESULT) -> bool {
    result < 0
}
pub const E_NOINTERFACE: HRESULT = -0x7FFFBFFE;
pub const NOERROR: HRESULT = 0x0;

#[allow(non_camel_case_types)]
pub type c_long = i32;
#[allow(non_camel_case_types)]
pub type c_ulong = u32;
pub type LPVOID = *mut c_void;
pub type DWORD = c_ulong;

pub const COINIT_APARTMENTTHREADED: DWORD = 0x2;
pub const CLSCTX_INPROC_SERVER: DWORD = 0x1;


extern "system" {
    // https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex
    // Initializes the COM library for use by the calling thread, sets the thread's concurrency model,
    // and creates a new apartment for the thread if one is required.
    pub fn CoInitializeEx(pvReserved: LPVOID, dwCoInit: DWORD) -> HRESULT;

    // https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cogetclassobject
    // Provides a pointer to an interface on a class object associated with a specified CLSID.
    // CoGetClassObject locates, and if necessary, dynamically loads the executable code required to do this.
    pub fn CoGetClassObject(
        rclsid: REFCLSID,
        dwClsContext: DWORD,
        pvReserved: LPVOID,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;

    // https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize
    // Closes the COM library on the current thread, unloads all DLLs loaded by the thread, frees any
    // other resources that the thread maintains, and forces all RPC connections on the thread to close.
    pub fn CoUninitialize() -> ();
}

extern "C" {
    pub static IID_IUnknown: IID;
}

pub const IID_CAT: IID = IID {
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


#[repr(C)]
pub struct ICat {
    vtable: *const ICatVTable,
}
#[repr(C)]
pub struct IAnimal {
    vtable: *const ICatVTable,
}
#[repr(C)]
pub struct IUnknown {
    vtable: *const ICatVTable,
}

#[repr(C)]
pub struct ICatVTable {
    ignore_humans: extern "stdcall" fn(*const ICat) -> HRESULT,
    eat: extern "stdcall" fn(*const ICat) -> HRESULT,
    query_interface: extern "stdcall" fn(*const ICat, *const IID, *mut *mut c_void) -> HRESULT,
    add_ref: extern "stdcall" fn(*const ICat) -> u32,
    release: extern "stdcall" fn(*const ICat) -> u32,
}

impl ICat {
    pub unsafe fn ignore_humans(&self) -> HRESULT {
        ((*self.vtable).ignore_humans)(self)
    }
    pub unsafe fn eat(&self) -> HRESULT {
        ((*self.vtable).eat)(self)
    }
    pub unsafe fn query_interface(&self, riid: *const IID, ppv: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).query_interface)(self, riid, ppv)
    }
    pub unsafe fn add_ref(&self) -> u32 {
        ((*self.vtable).add_ref)(self)
    }
    pub unsafe fn release(&self) -> u32 {
        ((*self.vtable).release)(self)
    }
}
impl IAnimal {
    pub unsafe fn eat(&self) -> HRESULT {
        ((*self.vtable).eat)(self as *const IAnimal as *const ICat)
    }
    pub unsafe fn query_interface(&self, riid: *const IID, ppv: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).query_interface)(self as *const IAnimal as *const ICat, riid, ppv)
    }
    pub unsafe fn add_ref(&self) -> u32 {
        ((*self.vtable).add_ref)(self as *const IAnimal as *const ICat)
    }
    pub unsafe fn release(&self) -> u32 {
        ((*self.vtable).release)(self as *const IAnimal as *const ICat)
    }
}

impl IUnknown {
    pub unsafe fn query_interface(&self, riid: *const IID, ppv: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).query_interface)(self as *const IUnknown as *const ICat, riid, ppv)
    }
    pub unsafe fn add_ref(&self) -> u32 {
        ((*self.vtable).add_ref)(self as *const IUnknown as *const ICat)
    }
    pub unsafe fn release(&self) -> u32 {
        ((*self.vtable).release)(self as *const IUnknown as *const ICat)
    }
}