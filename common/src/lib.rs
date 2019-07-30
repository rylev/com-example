// These are all defined in [winapi](https://github.com/retep998/winapi-rs)
mod comptr;

pub use comptr::ComPtr;
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
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = -0x7FFBFEEF;

#[allow(non_camel_case_types)]
pub type c_long = i32;
#[allow(non_camel_case_types)]
pub type c_ulong = u32;
pub type LPVOID = *mut c_void;
pub type LPUNKNOWN = *mut IUnknown;
pub type DWORD = c_ulong;

pub const COINIT_APARTMENTTHREADED: DWORD = 0x2;
pub const CLSCTX_INPROC_SERVER: DWORD = 0x1;

#[link(name = "ole32")]
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

    pub fn CoCreateInstance(
        rclsid: REFCLSID,
        pUnkOuter: LPUNKNOWN,
        dwClsContext: DWORD,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
}

pub trait ComInterface {
    const IID: IID;
}

pub const IID_IUnknown: IID = IID {
    data1: 0u32,
    data2: 0u16,
    data3: 0u16,
    data4: [192u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 70u8],
};

#[allow(non_snake_case)]
#[repr(C)]
pub struct IUnknownVTable {
    pub QueryInterface:
        unsafe extern "stdcall" fn(*mut IUnknown, *const IID, *mut *mut c_void) -> HRESULT,
    pub AddRef: unsafe extern "stdcall" fn(*mut IUnknown) -> u32,
    pub Release: unsafe extern "stdcall" fn(*mut IUnknown) -> u32,
}

#[repr(C)]
pub struct IUnknown {
    pub vtable: *const IUnknownVTable,
}

impl IUnknown {
    pub fn query_interface<T: ComInterface>(&mut self) -> Result<ComPtr<T>, HRESULT> {
        let ppv = std::ptr::null_mut::<*mut c_void>();
        let hr = unsafe { self.raw_query_interface(&T::IID as *const IID, ppv) };
        if failed(hr) {
            return Err(hr);
        }
        Ok(unsafe { ComPtr::new(*ppv as *const T) })
    }

    pub unsafe fn raw_query_interface(
        &mut self,
        riid: *const IID,
        ppv: *mut *mut c_void,
    ) -> HRESULT {
        ((*self.vtable).QueryInterface)(self as *mut IUnknown, riid, ppv)
    }
    pub unsafe fn raw_add_ref(&mut self) -> u32 {
        ((*self.vtable).AddRef)(self as *mut IUnknown)
    }
    pub unsafe fn raw_release(&mut self) -> u32 {
        ((*self.vtable).Release)(self as *mut IUnknown)
    }
}

impl ComInterface for IUnknown {
    const IID: IID = IID_IUnknown;
}