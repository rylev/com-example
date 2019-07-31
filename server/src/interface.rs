use common::{ComInterface, ComPtr, IUnknownVTable, RawIUnknown, HRESULT, IID};

// ICat

pub const IID_ICAT: IID = IID {
    data1: 0xf5353c58,
    data2: 0xcfd9,
    data3: 0x4204,
    data4: [0x8d, 0x92, 0xd2, 0x74, 0xc7, 0x57, 0x8b, 0x53],
};

pub const CLSID_CAT: IID = IID {
    data1: 0xC5F45CBC,
    data2: 0x4439,
    data3: 0x418C,
    data4: [0xA9, 0xF9, 0x05, 0xAC, 0x67, 0x52, 0x5E, 0x43],
};

#[repr(C)]
pub struct ICat {
    pub(crate) inner: RawICat,
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

impl ComInterface for ICat {
    const IID: IID = IID_ICAT;
}

#[repr(C)]
pub(crate) struct RawICat {
    pub(crate) vtable: *const ICatVTable,
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

#[allow(non_snake_case)]
#[repr(C)]
pub struct ICatVTable {
    pub(crate) iunknown: IUnknownVTable,
    pub(crate) Eat: unsafe extern "stdcall" fn(*mut RawICat) -> HRESULT,
    pub(crate) IgnoreHumans: unsafe extern "stdcall" fn(*mut RawICat) -> HRESULT,
}

// IAnimal

pub const IID_IANIMAL: IID = IID {
    data1: 0xeff8970e,
    data2: 0xc50f,
    data3: 0x45e0,
    data4: [0x92, 0x84, 0x29, 0x1c, 0xe5, 0xa6, 0xf7, 0x71],
};


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
pub(crate) struct RawIAnimal {
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

