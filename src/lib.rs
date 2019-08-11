extern crate winapi;

use winapi::shared::guiddef::IID;

pub mod comptr;
pub mod iunknown;
pub mod iclassfactory;

pub use comptr::ComPtr;
pub use iunknown::{IUnknown, IUnknownMethods, IUnknownVPtr};
pub use iclassfactory::{IClassFactory};

pub trait ComInterface {
    const IID: IID;
}
