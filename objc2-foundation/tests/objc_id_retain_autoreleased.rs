use std::ffi::c_void;
use std::ptr::NonNull;

use objc2::msg_send;
use objc2::rc::{autoreleasepool, Id, Shared};
use objc2_foundation::{INSObject, NSData};

fn retain_count(obj: &NSData) -> usize {
    unsafe { msg_send![obj, retainCount] }
}

fn create_data(bytes: &[u8]) -> Id<NSData, Shared> {
    let bytes_ptr = bytes.as_ptr() as *const c_void;
    unsafe {
        // All code between the `msg_send!` and the `retain_autoreleased` must
        // be able to be optimized away for this to work.
        let obj: *mut NSData = msg_send![
            NSData::class(),
            dataWithBytes: bytes_ptr,
            length: bytes.len(),
        ];
        Id::retain_autoreleased(NonNull::new_unchecked(obj))
    }
}

#[test]
fn test_retain_autoreleased() {
    #[cfg(gnustep)]
    unsafe {
        objc2::__gnustep_hack::get_class_to_force_linkage()
    };

    autoreleasepool(|_| {
        let data = create_data(b"12");
        // The autorelease-return-mechanism has to "warm up" somehow? At least
        // for some reason the first time this is used it fails.
        assert_eq!(retain_count(&data), 2);

        // When compiled in release mode / with optimizations enabled,
        // subsequent usage of `retain_autoreleased` will succeed in retaining
        // the autoreleased value!
        let expected_retain_count = if cfg!(debug_assertions) { 2 } else { 1 };

        let data = create_data(b"34");
        assert_eq!(retain_count(&data), expected_retain_count);

        let data = create_data(b"56");
        assert_eq!(retain_count(&data), expected_retain_count);

        // Here we manually clean up the autorelease, so it will always be 1.
        let data = autoreleasepool(|_| create_data(b"78"));
        assert_eq!(retain_count(&data), 1);
    });
}
