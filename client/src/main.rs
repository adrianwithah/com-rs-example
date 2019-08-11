use winapi::um::combaseapi::{CoCreateInstance, CoInitializeEx, CoUninitialize};
use winapi::shared::guiddef::IID;
use winapi::shared::guiddef::REFCLSID;
use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;
use winapi::shared::minwindef::DWORD;
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::LPVOID;
use winapi::um::winnt::HRESULT;
use com::{ComInterface, ComPtr, IUnknown};
use interfaces::{CLSID_CAT_CLASS, IID_IANIMAL, IAnimal};

// use winapi::ctypes::*;
use std::os::raw::c_void;

pub const COINIT_APARTMENTTHREADED: DWORD = 0x2;

fn main() {
    let result = initialize_ex();

    if let Err(hr) = result {
        println!("Failed to initialize COM Library: {}", hr);
        return;
    }

    run();
    uninitialize();
}

fn run() {
    let result = create_instance::<IUnknown>(&CLSID_CAT_CLASS);
    let mut p_unk = match result {
        Ok(p_unk) => p_unk,
        Err(hr) => {
            println!("Failed to get com class object {:x}", hr as u32);
            return;
        }
    };

    let mut itf_ptr = std::ptr::null_mut::<c_void>();
    p_unk.query_interface(&IID_IANIMAL, &mut itf_ptr as *mut *mut c_void);
    let mut p_animal : ComPtr<IAnimal> = ComPtr::wrap(std::ptr::NonNull::new(itf_ptr).unwrap());
    p_animal.eat();
}

fn create_instance<T: ComInterface + ?Sized>(clsid: &IID) -> Result<ComPtr<T>, HRESULT> {
    let mut instance = std::ptr::null_mut::<c_void>();
    let hr = unsafe {
        CoCreateInstance(
            clsid as REFCLSID,
            std::ptr::null_mut(),
            CLSCTX_INPROC_SERVER,
            &T::IID as REFIID,
            &mut instance as *mut LPVOID,
        )
    };
    if failed(hr) {
        return Err(hr);
    }

    Ok(ComPtr::wrap(
        std::ptr::NonNull::new(instance).unwrap(),
    ))
}

// TODO: accept threading options
fn initialize_ex() -> Result<(), HRESULT> {
    let hr = unsafe { CoInitializeEx(std::ptr::null_mut::<c_void>(), COINIT_APARTMENTTHREADED) };
    if failed(hr) {
        // TODO: https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize
        // A thread must call CoUninitialize once for each successful call it has made to the
        // CoInitialize or CoInitializeEx function, including any call that returns S_FALSE.
        return Err(hr);
    }
    Ok(())
}

fn uninitialize() {
    unsafe { CoUninitialize() }
}

pub fn failed(result: HRESULT) -> bool {
    result < 0
}
