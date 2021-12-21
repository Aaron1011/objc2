use core::cmp::Ordering;

use objc2::{Encode, Encoding, RefEncode};

use crate::ffi;

#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NSComparisonResult {
    Ascending = ffi::NSComparisonResult_NSOrderedAscending,
    Same = ffi::NSComparisonResult_NSOrderedSame,
    Descending = ffi::NSComparisonResult_NSOrderedDescending,
}

impl Default for NSComparisonResult {
    fn default() -> Self {
        Self::Same
    }
}

unsafe impl Encode for NSComparisonResult {
    const ENCODING: Encoding<'static> = isize::ENCODING;
}

unsafe impl RefEncode for NSComparisonResult {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}

impl From<Ordering> for NSComparisonResult {
    fn from(order: Ordering) -> Self {
        match order {
            Ordering::Less => Self::Ascending,
            Ordering::Equal => Self::Same,
            Ordering::Greater => Self::Descending,
        }
    }
}

impl From<NSComparisonResult> for Ordering {
    fn from(comparison_result: NSComparisonResult) -> Self {
        match comparison_result {
            NSComparisonResult::Ascending => Self::Less,
            NSComparisonResult::Same => Self::Equal,
            NSComparisonResult::Descending => Self::Greater,
        }
    }
}
