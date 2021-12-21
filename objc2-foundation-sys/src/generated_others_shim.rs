//! Other quick examples

use core::mem::ManuallyDrop;
use core::ptr::NonNull;

use std::os::raw::{c_char, c_int, c_longlong, c_schar, c_ulong, c_ushort, c_void};

use objc2::ffi::{NSInteger, NSUInteger};
use objc2::rc::{Id, Unknown};
use objc2::runtime::{Bool, Object};
use objc2::{class, msg_send, Encoding, Message, RefEncode};

use crate::{Autoreleased, NSCoder, NSObject, NSRange, NSString};

#[repr(transparent)]
pub struct NSArray(NSObject);
impl core::ops::Deref for NSArray {
    type Target = NSObject;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for NSArray {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl Message for NSArray {}
unsafe impl RefEncode for NSArray {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}
impl NSArray {
    pub unsafe fn alloc() -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![class!(NSArray), alloc])
    }
}
impl NSArray {
    pub unsafe fn objectAtIndex_(&self, index: NSUInteger) -> Autoreleased<Object> {
        msg_send![self, objectAtIndex: index]
    }
    pub unsafe fn init(this: Id<Self, Unknown>) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![this, init])
    }
    pub unsafe fn initWithObjects_count_(
        this: Id<Self, Unknown>,
        objects: NonNull<*mut Object>,
        cnt: NSUInteger,
    ) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![this, initWithObjects: objects, count: cnt])
    }
    pub unsafe fn initWithCoder_(&self, coder: NonNull<NSCoder>) -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![self, initWithCoder: coder])
    }
    pub unsafe fn count(&self) -> NSUInteger {
        msg_send![self, count]
    }
    pub unsafe fn firstObject(&self) -> Option<Autoreleased<Object>> {
        msg_send![self, firstObject]
    }
    pub unsafe fn lastObject(&self) -> Option<Autoreleased<Object>> {
        msg_send![self, lastObject]
    }
    pub unsafe fn getObjects_range_(&self, objects: NonNull<NonNull<Object>>, range: NSRange) {
        msg_send![self, getObjects: objects, range: range]
    }
}
