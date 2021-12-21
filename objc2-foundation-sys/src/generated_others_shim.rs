//! Other quick examples

use core::mem::ManuallyDrop;
use core::ptr::NonNull;

use std::os::raw::c_void;

use objc2::ffi::{NSInteger, NSUInteger};
use objc2::rc::{Id, Unknown};
use objc2::runtime::Object;
use objc2::{class, msg_send, Encoding, Message, RefEncode};

use crate::{Autoreleased, NSCoder, NSObject, NSRange};

pub const NSComparisonResult_NSOrderedAscending: NSComparisonResult = -1;
pub const NSComparisonResult_NSOrderedSame: NSComparisonResult = 0;
pub const NSComparisonResult_NSOrderedDescending: NSComparisonResult = 1;
pub type NSComparisonResult = NSInteger;

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
    pub unsafe fn initWithCoder_(
        this: Id<Self, Unknown>,
        coder: NonNull<NSCoder>,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![this, initWithCoder: coder])
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

#[repr(transparent)]
pub struct NSData(NSObject);
impl core::ops::Deref for NSData {
    type Target = NSObject;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for NSData {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl Message for NSData {}
unsafe impl RefEncode for NSData {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}
impl NSData {
    pub unsafe fn alloc() -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![class!(NSData), alloc])
    }
}
impl NSData {
    pub unsafe fn length(&self) -> NSUInteger {
        msg_send![self, length]
    }
    pub unsafe fn bytes(&self) -> *const c_void {
        msg_send![self, bytes]
    }
}
impl NSData {
    pub unsafe fn initWithBytes_length_(
        this: Id<Self, Unknown>,
        bytes: *const c_void,
        length: NSUInteger,
    ) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![this, initWithBytes: bytes, length: length])
    }
    pub unsafe fn initWithBytesNoCopy_length_deallocator_(
        this: Id<Self, Unknown>,
        bytes: *mut c_void,
        length: NSUInteger,
        deallocator: *mut c_void,
    ) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![
            this,
            initWithBytesNoCopy: bytes,
            length: length,
            deallocator: deallocator,
        ])
    }
}
