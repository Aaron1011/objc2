use std::os::raw::{c_char, c_int, c_uint};

use crate::{
    objc_ivar, objc_method, objc_object, objc_property, objc_property_attribute_t, objc_protocol,
    objc_selector, OpaqueData, BOOL, IMP,
};

/// An opaque type that represents an Objective-C class.
#[repr(C)]
pub struct objc_class {
    // `isa` field is deprecated and not available on GNUStep, so we don't
    // expose it here. Use `class_getSuperclass` instead.
    _priv: [u8; 0],
    _p: OpaqueData,
}

/// This is `c_char` in GNUStep's libobjc2 and `uint8_t` in Apple's objc4.
///
/// The pointer represents opaque data, and is definitely not just an integer,
/// so its signedness (i8 vs. u8) is not applicable.
///
/// So we just assign it here as a private alias to u8, to not document the
/// difference.
type ivar_layout_type = u8;

extern "C" {
    pub fn objc_getClass(name: *const c_char) -> *const objc_class;
    pub fn objc_getRequiredClass(name: *const c_char) -> *const objc_class;
    pub fn objc_lookUpClass(name: *const c_char) -> *const objc_class;
    pub fn objc_getMetaClass(name: *const c_char) -> *const objc_class;
    pub fn objc_copyClassList(out_len: *mut c_uint) -> *mut *const objc_class;
    pub fn objc_getClassList(buffer: *mut *const objc_class, buffer_len: c_int) -> c_int;

    pub fn objc_allocateClassPair(
        superclass: *const objc_class,
        name: *const c_char,
        extra_bytes: usize,
    ) -> *mut objc_class;
    #[cfg(apple)]
    pub fn objc_duplicateClass(
        original: *const objc_class,
        name: *const c_char,
        extra_bytes: usize,
    ) -> *mut objc_class;
    pub fn objc_disposeClassPair(cls: *mut objc_class);
    pub fn objc_registerClassPair(cls: *mut objc_class);

    pub fn class_addIvar(
        cls: *mut objc_class,
        name: *const c_char,
        size: usize,
        alignment: u8,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
        imp: IMP,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addProperty(
        cls: *mut objc_class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributes_count: c_uint,
    ) -> BOOL;
    pub fn class_addProtocol(cls: *mut objc_class, protocol: *const objc_protocol) -> BOOL;
    pub fn class_conformsToProtocol(cls: *const objc_class, protocol: *const objc_protocol)
        -> BOOL;
    pub fn class_copyIvarList(
        cls: *const objc_class,
        out_len: *mut c_uint,
    ) -> *mut *const objc_ivar;
    pub fn class_copyMethodList(
        cls: *const objc_class,
        out_len: *mut c_uint,
    ) -> *mut *const objc_method;
    pub fn class_copyPropertyList(
        cls: *const objc_class,
        out_len: *mut c_uint,
    ) -> *mut *const objc_property;
    pub fn class_copyProtocolList(
        cls: *const objc_class,
        out_len: *mut c_uint,
    ) -> *mut *const objc_protocol;

    pub fn class_createInstance(cls: *const objc_class, extra_bytes: usize) -> *mut objc_object;
    pub fn class_getClassMethod(
        cls: *const objc_class,
        name: *const objc_selector,
    ) -> *const objc_method;
    pub fn class_getClassVariable(cls: *const objc_class, name: *const c_char) -> *const objc_ivar;
    #[cfg(apple)]
    pub fn class_getImageName(cls: *const objc_class) -> *const c_char;
    pub fn class_getInstanceMethod(
        cls: *const objc_class,
        name: *const objc_selector,
    ) -> *const objc_method;
    pub fn class_getInstanceSize(cls: *const objc_class) -> usize;
    pub fn class_getInstanceVariable(
        cls: *const objc_class,
        name: *const c_char,
    ) -> *const objc_ivar;
    pub fn class_getIvarLayout(cls: *const objc_class) -> *const ivar_layout_type;
    pub fn class_getName(cls: *const objc_class) -> *const c_char;
    pub fn class_getProperty(cls: *const objc_class, name: *const c_char) -> *const objc_property;
    pub fn class_getSuperclass(cls: *const objc_class) -> *const objc_class;
    pub fn class_getVersion(cls: *const objc_class) -> c_int;
    #[cfg(apple)]
    pub fn class_getWeakIvarLayout(cls: *const objc_class) -> *const ivar_layout_type;
    pub fn class_isMetaClass(cls: *const objc_class) -> BOOL;
    pub fn class_replaceMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
        imp: IMP,
        types: *const c_char,
    ) -> IMP;
    pub fn class_replaceProperty(
        cls: *mut objc_class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributes_len: c_uint,
    );
    pub fn class_respondsToSelector(cls: *const objc_class, sel: *const objc_selector) -> BOOL;
    pub fn class_setIvarLayout(cls: *mut objc_class, layout: *const ivar_layout_type);
    pub fn class_setVersion(cls: *mut objc_class, version: c_int);
    #[cfg(apple)]
    pub fn class_setWeakIvarLayout(cls: *mut objc_class, layout: *const ivar_layout_type);

    // #[deprecated = "not recommended"]
    // pub fn class_setSuperclass
    // #[deprecated = "use class_getMethodImplementation instead"]
    // #[cfg(apple)]
    // pub fn class_lookupMethod
    // #[deprecated = "use class_respondsToSelector instead"]
    // #[cfg(apple)]
    // pub fn class_respondsToMethod
}
