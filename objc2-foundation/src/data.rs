#[cfg(feature = "block")]
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut, Range};
use core::slice;
use core::{ffi::c_void, ptr::NonNull};

use super::{ffi, INSCopying, INSMutableCopying, INSObject, NSRange};
use objc2::msg_send;
use objc2::rc::{Id, Owned, Ownership, Shared};

pub unsafe trait INSData: INSObject {
    type Ownership: Ownership;

    unsafe_def_fn!(fn new -> Self::Ownership);

    fn r(&self) -> &ffi::NSData {
        unsafe { &*(self as *const Self as *const ffi::NSData) }
    }

    fn len(&self) -> usize {
        unsafe { self.r().length() }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn bytes(&self) -> &[u8] {
        let ptr = unsafe { self.r().bytes() } as *const u8;
        // The bytes pointer may be null for length zero
        if ptr.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(ptr, self.len()) }
        }
    }

    fn with_bytes(bytes: &[u8]) -> Id<Self, Self::Ownership> {
        let bytes_ptr = bytes.as_ptr() as *const c_void;
        unsafe {
            let obj = ffi::NSData::alloc().unwrap();
            let obj = ffi::NSData::initWithBytes_length_(obj, bytes_ptr, bytes.len());
            obj.cast().into_ownership()
        }
    }

    #[cfg(feature = "block")]
    fn from_vec(bytes: Vec<u8>) -> Id<Self, Self::Ownership> {
        use core::mem::ManuallyDrop;

        use block2::{Block, ConcreteBlock};

        let capacity = bytes.capacity();

        let dealloc = ConcreteBlock::new(move |bytes: *mut c_void, len: usize| unsafe {
            // Recreate the Vec and let it drop
            let _ = Vec::from_raw_parts(bytes as *mut u8, len, capacity);
        });
        let dealloc = dealloc.copy();
        let dealloc: &Block<(*mut c_void, usize), ()> = &dealloc;

        // GNUStep's NSData `initWithBytesNoCopy:length:deallocator:` has a
        // bug; it forgets to assign the input buffer and length to the
        // instance before it swizzles to NSDataWithDeallocatorBlock.
        // See https://github.com/gnustep/libs-base/pull/213
        // So we just use NSDataWithDeallocatorBlock directly.
        #[cfg(gnustep)]
        let obj = {
            let cls = Self::class();
            let cls = if cls == objc2::class!(NSData) {
                objc2::class!(NSDataWithDeallocatorBlock)
            } else {
                cls
            };
            unsafe { Id::new_null(msg_send![cls, alloc]) }.unwrap()
        };
        #[cfg(not(gnustep))]
        let obj = unsafe { ffi::NSData::alloc() }.unwrap();

        let mut bytes = ManuallyDrop::new(bytes);

        unsafe {
            let obj = ffi::NSData::initWithBytesNoCopy_length_deallocator_(
                obj,
                bytes.as_mut_ptr().cast(),
                bytes.len(),
                dealloc as *const _ as *mut c_void,
            );
            obj.cast().into_ownership()
        }
    }
}

object!(unsafe pub struct NSData);

// TODO: SAFETY
unsafe impl Sync for NSData {}
unsafe impl Send for NSData {}

unsafe impl INSData for NSData {
    type Ownership = Shared;
}

unsafe impl INSCopying for NSData {
    type Ownership = Shared;
    type Output = NSData;
}

unsafe impl INSMutableCopying for NSData {
    type Output = NSMutableData;
}

impl Deref for NSData {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.bytes()
    }
}

pub unsafe trait INSMutableData: INSData {
    fn bytes_mut(&mut self) -> &mut [u8] {
        let ptr: *mut c_void = unsafe { msg_send![self, mutableBytes] };
        // The bytes pointer may be null for length zero
        if ptr.is_null() {
            &mut []
        } else {
            unsafe { slice::from_raw_parts_mut(ptr as *mut u8, self.len()) }
        }
    }

    /// Expands with zeroes, or truncates the buffer.
    fn set_len(&mut self, len: usize) {
        unsafe { msg_send![self, setLength: len] }
    }

    fn append(&mut self, bytes: &[u8]) {
        let bytes_ptr = bytes.as_ptr() as *const c_void;
        unsafe { msg_send![self, appendBytes: bytes_ptr, length: bytes.len()] }
    }

    fn replace_range(&mut self, range: Range<usize>, bytes: &[u8]) {
        let range = NSRange::from(range);
        let ptr = bytes.as_ptr() as *const c_void;
        unsafe {
            msg_send![
                self,
                replaceBytesInRange: range,
                withBytes: ptr,
                length: bytes.len(),
            ]
        }
    }

    fn set_bytes(&mut self, bytes: &[u8]) {
        let len = self.len();
        self.replace_range(0..len, bytes);
    }
}

object!(unsafe pub struct NSMutableData);

// TODO: SAFETY
unsafe impl Sync for NSMutableData {}
unsafe impl Send for NSMutableData {}

unsafe impl INSData for NSMutableData {
    type Ownership = Owned;
}

unsafe impl INSMutableData for NSMutableData {}

unsafe impl INSCopying for NSMutableData {
    type Ownership = Shared;
    type Output = NSData;
}

unsafe impl INSMutableCopying for NSMutableData {
    type Output = NSMutableData;
}

impl Deref for NSMutableData {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.bytes()
    }
}

impl DerefMut for NSMutableData {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.bytes_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::{INSData, INSMutableData, NSData, NSMutableData};
    #[cfg(feature = "block")]
    use alloc::vec;

    #[test]
    fn test_bytes() {
        let bytes = [3, 7, 16, 52, 112, 19];
        let data = NSData::with_bytes(&bytes);
        assert_eq!(data.len(), bytes.len());
        assert_eq!(data.bytes(), bytes);
    }

    #[test]
    fn test_no_bytes() {
        let data = NSData::new();
        assert!(Some(data.bytes()).is_some());
    }

    #[test]
    fn test_bytes_mut() {
        let mut data = NSMutableData::with_bytes(&[7, 16]);
        data.bytes_mut()[0] = 3;
        assert_eq!(data.bytes(), [3, 16]);
    }

    #[test]
    fn test_set_len() {
        let mut data = NSMutableData::with_bytes(&[7, 16]);
        data.set_len(4);
        assert_eq!(data.len(), 4);
        assert_eq!(data.bytes(), [7, 16, 0, 0]);

        data.set_len(1);
        assert_eq!(data.len(), 1);
        assert_eq!(data.bytes(), [7]);
    }

    #[test]
    fn test_append() {
        let mut data = NSMutableData::with_bytes(&[7, 16]);
        data.append(&[3, 52]);
        assert_eq!(data.len(), 4);
        assert_eq!(data.bytes(), [7, 16, 3, 52]);
    }

    #[test]
    fn test_replace() {
        let mut data = NSMutableData::with_bytes(&[7, 16]);
        data.replace_range(0..0, &[3]);
        assert_eq!(data.bytes(), [3, 7, 16]);

        data.replace_range(1..2, &[52, 13]);
        assert_eq!(data.bytes(), [3, 52, 13, 16]);

        data.replace_range(2..4, &[6]);
        assert_eq!(data.bytes(), [3, 52, 6]);

        data.set_bytes(&[8, 17]);
        assert_eq!(data.bytes(), [8, 17]);
    }

    #[cfg(feature = "block")]
    #[test]
    fn test_from_vec() {
        let bytes = vec![3, 7, 16];
        let bytes_ptr = bytes.as_ptr();

        let data = NSData::from_vec(bytes);
        assert_eq!(data.bytes().as_ptr(), bytes_ptr);
    }
}
