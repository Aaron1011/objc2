//! Example of what I would like `bindgen` to be able to generate.

use core::mem::ManuallyDrop;

use objc2::ffi::NSUInteger;
use objc2::rc::{Autoreleased, Id, Unknown};
use objc2::runtime::{Bool, Class, Imp, Object, Protocol, Sel};
use objc2::{class, msg_send, Encoding, Message, RefEncode};

type NSInvocation = NSObject;
type NSMethodSignature = NSObject;
type NSString = NSObject;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NSZone {
    _unused: [u8; 0],
}

unsafe impl RefEncode for _NSZone {
    const ENCODING_REF: Encoding<'static> = Encoding::Unknown;
}

#[repr(transparent)]
pub struct NSObject(Object);

unsafe impl Message for NSObject {}
unsafe impl RefEncode for NSObject {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}

impl std::ops::Deref for NSObject {
    type Target = Object;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for NSObject {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl NSObject {
    // alloc family

    pub unsafe fn alloc() -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![class!(NSObject), alloc])
    }
    pub unsafe fn allocWithZone_(zone: *mut _NSZone) -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![class!(NSObject), allocWithZone: zone])
    }

    // init family

    pub unsafe fn init(this: Id<Self, Unknown>) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![this, init])
    }

    // TODO
    pub unsafe fn dealloc(&mut self) {
        msg_send![self, dealloc]
    }
    // TODO
    pub unsafe fn finalize(&self) {
        msg_send![self, finalize]
    }

    // copy family

    pub unsafe fn copy(&self) -> Option<Id<Object, Unknown>> {
        Id::new_null(msg_send![self, copy])
    }
    pub unsafe fn copyWithZone_(zone: *mut _NSZone) -> Option<Id<Object, Unknown>> {
        Id::new_null(msg_send![class!(NSObject), copyWithZone: zone])
    }
    pub unsafe fn mutableCopy(&self) -> Option<Id<Object, Unknown>> {
        Id::new_null(msg_send![self, mutableCopy])
    }
    pub unsafe fn mutableCopyWithZone_(zone: *mut _NSZone) -> Option<Id<Object, Unknown>> {
        Id::new_null(msg_send![class!(NSObject), mutableCopyWithZone: zone])
    }

    pub unsafe fn methodForSelector_(&self, aSelector: Sel) -> Option<Imp> {
        msg_send![self, methodForSelector: aSelector]
    }
    pub unsafe fn doesNotRecognizeSelector_(&self, aSelector: Sel) {
        msg_send![self, doesNotRecognizeSelector: aSelector]
    }
    pub unsafe fn forwardingTargetForSelector_<'a>(
        &self,
        aSelector: Sel,
    ) -> Option<Autoreleased<'a, Object, Unknown>> {
        Autoreleased::new(msg_send![self, forwardingTargetForSelector: aSelector])
    }
    pub unsafe fn forwardInvocation_(&self, anInvocation: *const NSInvocation) {
        msg_send![self, forwardInvocation: anInvocation]
    }
    pub unsafe fn methodSignatureForSelector_<'a>(
        &self,
        aSelector: Sel,
    ) -> Option<Autoreleased<'a, NSMethodSignature, Unknown>> {
        Autoreleased::new(msg_send![self, methodSignatureForSelector: aSelector])
    }
    pub unsafe fn allowsWeakReference(&self) -> Bool {
        msg_send![self, allowsWeakReference]
    }
    pub unsafe fn retainWeakReference(&self) -> Bool {
        msg_send![self, retainWeakReference]
    }
    pub unsafe fn load() {
        msg_send![class!(NSObject), load]
    }
    pub unsafe fn initialize() {
        msg_send![class!(NSObject), initialize]
    }
    pub unsafe fn new() -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![class!(NSObject), new])
    }
    pub unsafe fn instancesRespondToSelector_(aSelector: Sel) -> Bool {
        msg_send![class!(NSObject), instancesRespondToSelector: aSelector]
    }
    pub unsafe fn class_conformsToProtocol_(protocol: &Protocol) -> Bool {
        msg_send![class!(NSObject), conformsToProtocol: protocol]
    }
    pub unsafe fn instanceMethodForSelector_(aSelector: Sel) -> Option<Imp> {
        msg_send![class!(NSObject), instanceMethodForSelector: aSelector]
    }
    pub unsafe fn instanceMethodSignatureForSelector_<'a>(
        aSelector: Sel,
    ) -> Option<Autoreleased<'a, NSMethodSignature, Unknown>> {
        Autoreleased::new(msg_send![
            class!(NSObject),
            instanceMethodSignatureForSelector: aSelector,
        ])
    }
    pub unsafe fn isSubclassOfClass_(aClass: &Class) -> Bool {
        msg_send![class!(NSObject), isSubclassOfClass: aClass]
    }
    pub unsafe fn resolveClassMethod_(sel: Sel) -> Bool {
        msg_send![class!(NSObject), resolveClassMethod: sel]
    }
    pub unsafe fn resolveInstanceMethod_(sel: Sel) -> Bool {
        msg_send![class!(NSObject), resolveInstanceMethod: sel]
    }
    pub unsafe fn class_hash() -> NSUInteger {
        msg_send![class!(NSObject), hash]
    }
    pub unsafe fn class_superclass<'a>() -> &'a Class {
        msg_send![class!(NSObject), superclass]
    }
    pub unsafe fn class_class<'a>() -> Option<&'a Class> {
        msg_send![class!(NSObject), class]
    }
    pub unsafe fn class_description<'a>() -> Option<Autoreleased<'a, NSString, Unknown>> {
        Autoreleased::new(msg_send![class!(NSObject), description])
    }
    pub unsafe fn class_debugDescription<'a>() -> Option<Autoreleased<'a, NSString, Unknown>> {
        Autoreleased::new(msg_send![class!(NSObject), debugDescription])
    }

    pub unsafe fn isEqual_(&self, object: *const Object) -> Bool {
        msg_send![self, isEqual: object]
    }
    pub unsafe fn self_<'a>(&self) -> Option<Autoreleased<'a, Self, Unknown>> {
        Autoreleased::new(msg_send![self, self])
    }
    pub unsafe fn performSelector_<'a>(
        &self,
        aSelector: Sel,
    ) -> Option<Autoreleased<'a, Object, Unknown>> {
        Autoreleased::new(msg_send![self, performSelector: aSelector])
    }
    pub unsafe fn performSelector_withObject_<'a>(
        &self,
        aSelector: Sel,
        object: *const Object,
    ) -> Option<Autoreleased<'a, Object, Unknown>> {
        Autoreleased::new(msg_send![
            self,
            performSelector: aSelector,
            withObject: object
        ])
    }
    pub unsafe fn performSelector_withObject_withObject_<'a>(
        &self,
        aSelector: Sel,
        object1: *const Object,
        object2: *const Object,
    ) -> Option<Autoreleased<'a, Object, Unknown>> {
        Autoreleased::new(msg_send![
            self,
            performSelector: aSelector,
            withObject: object1,
            withObject: object2,
        ])
    }
    pub unsafe fn isProxy(&self) -> Bool {
        msg_send![self, isProxy]
    }
    pub unsafe fn isKindOfClass_(&self, aClass: &Class) -> Bool {
        msg_send![self, isKindOfClass: aClass]
    }
    pub unsafe fn isMemberOfClass_(&self, aClass: &Class) -> Bool {
        msg_send![self, isMemberOfClass: aClass]
    }
    pub unsafe fn conformsToProtocol_(&self, aProtocol: &Protocol) -> Bool {
        msg_send![self, conformsToProtocol: aProtocol]
    }
    pub unsafe fn respondsToSelector_(&self, aSelector: Sel) -> Bool {
        msg_send![self, respondsToSelector: aSelector]
    }
    pub unsafe fn retain(&self) -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![self, retain])
    }
    pub unsafe fn release(&self) {
        msg_send![self, release]
    }
    pub unsafe fn autorelease<'a>(&self) -> Option<Autoreleased<'a, Self, Unknown>> {
        Autoreleased::new(msg_send![self, autorelease])
    }
    pub unsafe fn retainCount(&self) -> NSUInteger {
        msg_send![self, retainCount]
    }
    pub unsafe fn zone(&self) -> *mut _NSZone {
        msg_send![self, zone]
    }
    pub unsafe fn hash(&self) -> NSUInteger {
        msg_send![self, hash]
    }
    pub unsafe fn class<'a>(&self) -> Option<&'a Class> {
        msg_send![self, class]
    }
    pub unsafe fn superclass<'a>(&self) -> Option<&'a Class> {
        msg_send![self, superclass]
    }
    pub unsafe fn description<'a>(&self) -> Option<Autoreleased<'a, NSString, Unknown>> {
        Autoreleased::new(msg_send![self, description])
    }
    pub unsafe fn debugDescription<'a>(&self) -> Option<Autoreleased<'a, NSString, Unknown>> {
        Autoreleased::new(msg_send![self, debugDescription])
    }
}