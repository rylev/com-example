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

#[repr(C)]
pub struct ICat {
    pub ref_count: u32,
    pub iunknown_vtable: *const IUnknownVTable,
    pub ianimal_vtable: *const IAnimalVTable,
    pub icat_vtable: *const ICatVTable,
}

#[repr(C)]
pub struct IUnknownVTable {
    pub query_interface: extern "stdcall" fn(*const MyCat, &IID, *mut *mut c_void) -> HRESULT,
    pub add_ref: extern "stdcall" fn(*const MyCat) -> u32,
    pub release: extern "stdcall" fn(*const MyCat) -> u32,
}

#[repr(C)]
pub struct ICatVTable {
    pub ignore_humans: extern "stdcall" fn(*const MyCat) -> HRESULT,
}

#[repr(C)]
pub struct IAnimalVTable {
    pub eat: extern "stdcall" fn(*const MyCat) -> HRESULT,
}
