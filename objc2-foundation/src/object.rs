use core::marker::PhantomData;

use objc2::rc::{Id, Owned, Shared};
use objc2::runtime::{Object, Class};
use objc2::Message;

use super::ffi;
use super::NSString;

pub unsafe trait INSObject: Message {
    fn class() -> &'static Class;

    fn raw(&self) -> &ffi::NSObject {
        unsafe { &*(self as *const Self as *const ffi::NSObject) }
    }

    fn hash_code(&self) -> usize {
        unsafe { self.raw().hash() }
    }

    fn is_equal<T: INSObject>(&self, other: &T) -> bool {
        let other = &**other.raw();
        unsafe { self.raw().isEqual_(other) }.is_true()
    }

    fn description(&self) -> Id<NSString, Shared> {
        unsafe {
            // TODO: Verify that description always returns a non-null string
            Id::retain(self.raw().description().unwrap().cast())
        }
    }

    fn is_kind_of(&self, cls: &Class) -> bool {
        unsafe { self.raw().isKindOfClass_(cls) }.is_true()
    }
}

object!(unsafe pub struct NSObject<> {
    p: PhantomData<Object>, // Temporary
});

/// ```compile_fail
/// use objc2_foundation::NSObject;
/// fn needs_sync<T: Sync>() {}
/// needs_sync::<NSObject>();
/// ```
/// ```compile_fail
/// use objc2_foundation::NSObject;
/// fn needs_send<T: Send>() {}
/// needs_send::<NSObject>();
/// ```
#[cfg(doctest)]
pub struct NSObjectNotSendNorSync;

impl NSObject {
    pub fn new() -> Id<Self, Owned> {
        unsafe { ffi::NSObject::new().unwrap() };
        todo!("unsafe Id::cast")
    }
}

#[cfg(test)]
mod tests {
    use super::{INSObject, NSObject};
    use crate::NSString;
    use alloc::format;

    #[test]
    fn test_equality() {
        let obj1 = NSObject::new();
        assert_eq!(obj1, obj1);

        let obj2 = NSObject::new();
        assert_ne!(obj1, obj2);
    }

    #[test]
    fn test_hash_code() {
        let obj = NSObject::new();
        assert_eq!(obj.hash_code(), obj.hash_code());
    }

    #[test]
    fn test_debug() {
        let obj = NSObject::new();
        let expected = format!("<NSObject: {:p}>", &*obj);
        assert_eq!(format!("{:?}", obj), format!("{:?}", expected));
    }

    #[test]
    fn test_is_kind_of() {
        let obj = NSObject::new();
        assert!(obj.is_kind_of(NSObject::class()));
        assert!(!obj.is_kind_of(NSString::class()));
    }
}
