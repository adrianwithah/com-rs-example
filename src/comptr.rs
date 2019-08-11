use crate::ComInterface;

use super::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::os::raw::c_void;
use std::ptr::NonNull;

pub struct ComPtr<T: ComInterface + ?Sized> {
    ptr: NonNull<c_void>,
    phantom: PhantomData<T>,
}

impl<T: ComInterface + ?Sized> ComPtr<T> {
    /// NonNull<T> must be safely convertable to *mut RawIUnknown
    pub fn wrap(ptr: NonNull<c_void>) -> Self {
        ComPtr {
            ptr,
            phantom: PhantomData,
        }
    }

    pub fn into_raw(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }

    pub fn get_ptr(&self) -> NonNull<c_void> {
        self.ptr
    }
}
// impl<T: ComInterface> Clone for ComPtr<T> {
//     fn clone(&self) -> Self {
//         self.add_ref();
//         ComPtr { ptr: self.ptr }
//     }
// }

impl<T: ComInterface + ?Sized> Drop for ComPtr<T> {
    fn drop(&mut self) {
        println!("Dropped!");
        unsafe {
            (*(self as *const _ as *const ComPtr<IUnknown>)).release();
        }
    }
}


