use common::{
    failed, IID_IUnknown, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, HRESULT, IID, LPVOID, NOERROR,
    REFCLSID, REFIID,
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
pub struct ICat {
    parent: IAnimal,
    vtable: *const ICatVTable,
}
impl Drop for ICat {
    fn drop(&mut self) {
        println!("Dropping ICat...");
        let _ = unsafe { Box::from_raw(self.vtable as *mut ICatVTable) };
    }
}
#[repr(C)]
pub struct IAnimal {
    parent: IUnknown,
    vtable: *const IAnimalVTable,
}
impl Drop for IAnimal {
    fn drop(&mut self) {
        println!("Dropping IAnimal...");
        let _ = unsafe { Box::from_raw(self.vtable as *mut IAnimalVTable) };
    }
}
#[repr(C)]
pub struct IUnknown {
    vtable: *const IUnknownVTable,
}
impl Drop for IUnknown {
    fn drop(&mut self) {
        println!("Dropping IUnknown...");
        let _ = unsafe { Box::from_raw(self.vtable as *mut IUnknownVTable) };
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct IUnknownVTable {
    pub QueryInterface: extern "stdcall" fn(*mut ICat, *const IID, *mut *mut c_void) -> HRESULT,
    pub AddRef: extern "stdcall" fn(*mut ICat) -> u32,
    pub Release: extern "stdcall" fn(*mut ICat) -> u32,
}
#[allow(non_snake_case)]
#[repr(C)]
pub struct ICatVTable {
    pub parent: IAnimalVTable,
    pub IgnoreHumans: extern "stdcall" fn(*mut ICat) -> HRESULT,
}
#[allow(non_snake_case)]
#[repr(C)]
pub struct IAnimalVTable {
    pub parent: IUnknownVTable,
    pub Eat: extern "stdcall" fn(*mut ICat) -> HRESULT,
}

impl ICat {
    pub unsafe fn ignore_humans(&mut self) -> HRESULT {
        ((*self.vtable).IgnoreHumans)(self)
    }
    pub unsafe fn eat(&mut self) -> HRESULT {
        ((*self.parent.vtable).Eat)(self)
    }
    pub unsafe fn query_interface(&mut self, riid: *const IID, ppv: *mut *mut c_void) -> HRESULT {
        ((*self.parent.parent.vtable).QueryInterface)(self, riid, ppv)
    }
    pub unsafe fn add_ref(&mut self) -> u32 {
        ((*self.parent.parent.vtable).AddRef)(self)
    }
    pub unsafe fn release(&mut self) -> u32 {
        ((*self.parent.parent.vtable).Release)(self)
    }
}
impl IAnimal {
    pub unsafe fn eat(&mut self) -> HRESULT {
        ((*self.vtable).Eat)(self as *mut IAnimal as *mut ICat)
    }
    pub unsafe fn query_interface(&mut self, riid: *const IID, ppv: *mut *mut c_void) -> HRESULT {
        ((*self.parent.vtable).QueryInterface)(self as *mut IAnimal as *mut ICat, riid, ppv)
    }
    pub unsafe fn add_ref(&mut self) -> u32 {
        ((*self.parent.vtable).AddRef)(self as *mut IAnimal as *mut ICat)
    }
    pub unsafe fn release(&mut self) -> u32 {
        ((*self.parent.vtable).Release)(self as *mut IAnimal as *mut ICat)
    }
}

impl IUnknown {
    pub unsafe fn query_interface(&mut self, riid: *const IID, ppv: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).QueryInterface)(self as *mut IUnknown as *mut ICat, riid, ppv)
    }
    pub unsafe fn add_ref(&mut self) -> u32 {
        ((*self.vtable).AddRef)(self as *mut IUnknown as *mut ICat)
    }
    pub unsafe fn release(&mut self) -> u32 {
        ((*self.vtable).Release)(self as *mut IUnknown as *mut ICat)
    }
}

#[repr(C)]
struct Cat {
    inner: ICat,
    ref_count: u32,
}

impl Drop for Cat {
    fn drop(&mut self) {
        println!("Dropping ICat...");
        let _ = unsafe { Box::from_raw(self.inner.vtable as *mut ICatVTable) };
    }
}

extern "stdcall" fn query_interface(
    this: *mut ICat,
    riid: *const IID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    println!("Querying interface...");
    unsafe {
        if *riid == IID_IUnknown || *riid == IID_ICAT || *riid == IID_IANIMAL {
            (*this).add_ref();
            NOERROR
        } else {
            E_NOINTERFACE
        }
    }
}

extern "stdcall" fn add_ref(this: *mut ICat) -> u32 {
    println!("Adding ref...");
    let this = this as *mut Cat;
    unsafe {
        (*this).ref_count += 1;
        println!("Count now {}", (*this).ref_count);
        (*this).ref_count
    }
}

// TODO: This could potentially be null or pointing to some invalid memory
extern "stdcall" fn release(this: *mut ICat) -> u32 {
    println!("Releasing...");
    let this = this as *mut Cat;
    unsafe {
        (*this).ref_count -= 1;
        println!("Count now {}", (*this).ref_count);
        let count = (*this).ref_count;
        if count == 0 {
            println!("Count is 0. Freeing memory...");
            let _ = Box::from_raw(this);
        }
        count
    }
}

extern "stdcall" fn ignore_humans(this: *mut ICat) -> HRESULT {
    println!("Ignoring...");
    NOERROR
}

extern "stdcall" fn eat(this: *mut ICat) -> HRESULT {
    println!("Eating...");
    NOERROR
}

impl Cat {
    fn new() -> Cat {
        println!("Allocating new Vtable...");
        let vtable = IUnknownVTable {
            QueryInterface: query_interface,
            Release: release,
            AddRef: add_ref,
        };
        let vtable = IAnimalVTable {
            parent: vtable,
            Eat: eat,
        };
        let vtable = Box::into_raw(Box::new(ICatVTable {
            parent: vtable,
            IgnoreHumans: ignore_humans,
        }));
        let parent = IUnknown {
            vtable: vtable as *mut IUnknownVTable,
        };
        let parent = IAnimal {
            parent,
            vtable: vtable as *mut IAnimalVTable,
        };
        Cat {
            inner: ICat { parent, vtable },
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
        let cat = Box::into_raw(Box::new(Cat::new())) as *mut ICat;
        let hr = (*cat).query_interface(riid, ppv);
        if failed(hr) {
            println!("Querying new object failed... Deallocating object...");
            let _ = Box::from_raw(cat);
        }
        hr
    }
}