use super::*;
use winapi::shared::guiddef::IID;
use winapi::shared::guiddef::REFIID;
use winapi::shared::ntdef::HRESULT;
use winapi::shared::minwindef::BOOL;

use std::os::raw::c_void;
use std::marker::PhantomData;

#[allow(non_upper_case_globals)]
pub const IID_ICLASS_FACTORY: IID = IID {
    Data1: 0x01u32,
    Data2: 0u16,
    Data3: 0u16,
    Data4: [0xC0, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0x46u8],
};

#[allow(non_snake_case)]
#[repr(C)]
pub struct IClassFactoryMethods {
    pub CreateInstance: unsafe extern "stdcall" fn(
        *mut IClassFactoryVPtr,
        *mut IUnknownVPtr,
        REFIID,
        *mut *mut c_void,
    ) -> HRESULT,
    pub LockServer: unsafe extern "stdcall" fn(BOOL) -> HRESULT,
}
#[repr(C)]
pub struct IClassFactoryVTable(pub IUnknownMethods, pub IClassFactoryMethods);

pub type IClassFactoryVPtr = *const IClassFactoryVTable;

pub trait IClassFactory: IUnknown {
    fn create_instance(&mut self, aggr: *mut IUnknownVPtr, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT;
    fn lock_server(&self, increment: BOOL) -> HRESULT;
}

impl IClassFactory for ComPtr<IClassFactory> {
    fn create_instance(&mut self, aggr: *mut IUnknownVPtr, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT {
        let itf_ptr = self.into_raw() as *mut IClassFactoryVPtr;
        unsafe { ((**itf_ptr).1.CreateInstance)(itf_ptr, aggr, riid, ppv) }
    }

    fn lock_server(&self, increment: BOOL) -> HRESULT {
        let itf_ptr = self.into_raw() as *mut IClassFactoryVPtr;
        unsafe { ((**itf_ptr).1.LockServer)(increment) }
    }
}


impl ComInterface for IClassFactory {
    const IID: IID = IID_ICLASS_FACTORY;
}

impl From<ComPtr<IClassFactory>> for ComPtr<IUnknown> {
    fn from(comptr: ComPtr<IClassFactory>) -> ComPtr<IUnknown> {
        println!("Wrapped!");
        ComPtr::wrap(comptr.get_ptr())
        // ComPtr {
        //     ptr: comptr.get_ptr(),
        //     phantom: PhantomData
        // }
    }
}
