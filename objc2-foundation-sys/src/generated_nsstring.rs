//! Example of what I would like `bindgen` to be able to generate for NSString

use core::mem::ManuallyDrop;
use core::ptr::NonNull;

use std::os::raw::{c_char, c_int, c_longlong, c_schar, c_ulong, c_ushort, c_void};

use objc2::ffi::{NSInteger, NSUInteger};
use objc2::rc::{Id, Unknown};
use objc2::runtime::{Bool, Object};
use objc2::{class, msg_send, Encoding, Message, RefEncode};

use crate::NSObject;

pub type Autoreleased<T> = NonNull<T>;
pub type NSRange = [NSUInteger; 2];
pub type NSComparisonResult = NSInteger;

pub type NSArray = NSObject;
pub type NSCoder = NSObject;
pub type NSLocale = NSObject;
pub type NSError = NSObject;
pub type NSData = NSObject;
pub type NSDictionary = NSObject;
pub type NSCharacterSet = NSObject;
pub type NSURL = NSObject;

pub type unichar = c_ushort;

pub const NSStringCompareOptions_NSCaseInsensitiveSearch: NSStringCompareOptions = 1;
pub const NSStringCompareOptions_NSLiteralSearch: NSStringCompareOptions = 2;
pub const NSStringCompareOptions_NSBackwardsSearch: NSStringCompareOptions = 4;
pub const NSStringCompareOptions_NSAnchoredSearch: NSStringCompareOptions = 8;
pub const NSStringCompareOptions_NSNumericSearch: NSStringCompareOptions = 64;
pub const NSStringCompareOptions_NSDiacriticInsensitiveSearch: NSStringCompareOptions = 128;
pub const NSStringCompareOptions_NSWidthInsensitiveSearch: NSStringCompareOptions = 256;
pub const NSStringCompareOptions_NSForcedOrderingSearch: NSStringCompareOptions = 512;
pub const NSStringCompareOptions_NSRegularExpressionSearch: NSStringCompareOptions = 1024;
pub type NSStringCompareOptions = NSUInteger;
pub type NSStringEncoding = NSUInteger;
pub const NSASCIIStringEncoding: NSStringEncoding = 1;
pub const NSNEXTSTEPStringEncoding: NSStringEncoding = 2;
pub const NSJapaneseEUCStringEncoding: NSStringEncoding = 3;
pub const NSUTF8StringEncoding: NSStringEncoding = 4;
pub const NSISOLatin1StringEncoding: NSStringEncoding = 5;
pub const NSSymbolStringEncoding: NSStringEncoding = 6;
pub const NSNonLossyASCIIStringEncoding: NSStringEncoding = 7;
pub const NSShiftJISStringEncoding: NSStringEncoding = 8;
pub const NSISOLatin2StringEncoding: NSStringEncoding = 9;
pub const NSUnicodeStringEncoding: NSStringEncoding = 10;
pub const NSWindowsCP1251StringEncoding: NSStringEncoding = 11;
pub const NSWindowsCP1252StringEncoding: NSStringEncoding = 12;
pub const NSWindowsCP1253StringEncoding: NSStringEncoding = 13;
pub const NSWindowsCP1254StringEncoding: NSStringEncoding = 14;
pub const NSWindowsCP1250StringEncoding: NSStringEncoding = 15;
pub const NSISO2022JPStringEncoding: NSStringEncoding = 21;
pub const NSMacOSRomanStringEncoding: NSStringEncoding = 30;
pub const NSUTF16StringEncoding: NSStringEncoding = 10;
pub const NSUTF16BigEndianStringEncoding: NSStringEncoding = 2415919360;
pub const NSUTF16LittleEndianStringEncoding: NSStringEncoding = 2483028224;
pub const NSUTF32StringEncoding: NSStringEncoding = 2348810496;
pub const NSUTF32BigEndianStringEncoding: NSStringEncoding = 2550137088;
pub const NSUTF32LittleEndianStringEncoding: NSStringEncoding = 2617245952;
pub type _bindgen_ty_10 = NSStringEncoding;
pub const NSStringEncodingConversionOptions_NSStringEncodingConversionAllowLossy:
    NSStringEncodingConversionOptions = 1;
pub const NSStringEncodingConversionOptions_NSStringEncodingConversionExternalRepresentation:
    NSStringEncodingConversionOptions = 2;
pub type NSStringEncodingConversionOptions = NSUInteger;

#[repr(transparent)]
pub struct NSString(NSObject);
impl core::ops::Deref for NSString {
    type Target = NSObject;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for NSString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl Message for NSString {}
unsafe impl RefEncode for NSString {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}
impl NSString {
    pub unsafe fn alloc() -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![class!(NSString), alloc])
    }
}
impl NSString {
    pub unsafe fn characterAtIndex_(&self, index: NSUInteger) -> unichar {
        msg_send![self, characterAtIndex: index]
    }
    // Marked nonnull
    pub unsafe fn init(this: Id<Self, Unknown>) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![this, init])
    }
    pub unsafe fn initWithCoder_(
        this: Id<Self, Unknown>,
        coder: &NSCoder,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![this, initWithCoder: coder])
    }
    pub unsafe fn length(&self) -> NSUInteger {
        msg_send![self, length]
    }
}
#[doc = " NSStringExtensionMethods"]
impl NSString {
    pub unsafe fn substringFromIndex_(&self, from: NSUInteger) -> Autoreleased<NSString> {
        msg_send![self, substringFromIndex: from]
    }
    pub unsafe fn substringToIndex_(&self, to: NSUInteger) -> Autoreleased<NSString> {
        msg_send![self, substringToIndex: to]
    }
    pub unsafe fn substringWithRange_(&self, range: NSRange) -> Autoreleased<NSString> {
        msg_send![self, substringWithRange: range]
    }
    pub unsafe fn getCharacters_range_(&self, buffer: NonNull<c_ushort>, range: NSRange) {
        msg_send![self, getCharacters: buffer, range: range]
    }
    pub unsafe fn compare_(&self, string: NonNull<NSString>) -> NSComparisonResult {
        msg_send![self, compare: string]
    }
    pub unsafe fn compare_options_(
        &self,
        string: NonNull<NSString>,
        mask: NSStringCompareOptions,
    ) -> NSComparisonResult {
        msg_send![self, compare: string, options: mask]
    }
    pub unsafe fn compare_options_range_(
        &self,
        string: NonNull<NSString>,
        mask: NSStringCompareOptions,
        rangeOfReceiverToCompare: NSRange,
    ) -> NSComparisonResult {
        msg_send![
            self,
            compare: string,
            options: mask,
            range: rangeOfReceiverToCompare
        ]
    }
    pub unsafe fn compare_options_range_locale_(
        &self,
        string: NonNull<NSString>,
        mask: NSStringCompareOptions,
        rangeOfReceiverToCompare: NSRange,
        locale: *mut Object,
    ) -> NSComparisonResult {
        msg_send![
            self,
            compare: string,
            options: mask,
            range: rangeOfReceiverToCompare,
            locale: locale
        ]
    }
    pub unsafe fn caseInsensitiveCompare_(&self, string: NonNull<NSString>) -> NSComparisonResult {
        msg_send![self, caseInsensitiveCompare: string]
    }
    pub unsafe fn localizedCompare_(&self, string: NonNull<NSString>) -> NSComparisonResult {
        msg_send![self, localizedCompare: string]
    }
    pub unsafe fn localizedCaseInsensitiveCompare_(
        &self,
        string: NonNull<NSString>,
    ) -> NSComparisonResult {
        msg_send![self, localizedCaseInsensitiveCompare: string]
    }
    pub unsafe fn localizedStandardCompare_(
        &self,
        string: NonNull<NSString>,
    ) -> NSComparisonResult {
        msg_send![self, localizedStandardCompare: string]
    }
    pub unsafe fn isEqualToString_(&self, aString: NonNull<NSString>) -> Bool {
        msg_send![self, isEqualToString: aString]
    }
    pub unsafe fn hasPrefix_(&self, str_: NonNull<NSString>) -> Bool {
        msg_send![self, hasPrefix: str_]
    }
    pub unsafe fn hasSuffix_(&self, str_: NonNull<NSString>) -> Bool {
        msg_send![self, hasSuffix: str_]
    }
    pub unsafe fn commonPrefixWithString_options_(
        &self,
        str_: NonNull<NSString>,
        mask: NSStringCompareOptions,
    ) -> Autoreleased<NSString> {
        msg_send![self, commonPrefixWithString: str_, options: mask]
    }
    pub unsafe fn containsString_(&self, str_: NonNull<NSString>) -> Bool {
        msg_send![self, containsString: str_]
    }
    pub unsafe fn localizedCaseInsensitiveContainsString_(&self, str_: NonNull<NSString>) -> Bool {
        msg_send![self, localizedCaseInsensitiveContainsString: str_]
    }
    pub unsafe fn localizedStandardContainsString_(&self, str_: NonNull<NSString>) -> Bool {
        msg_send![self, localizedStandardContainsString: str_]
    }
    pub unsafe fn localizedStandardRangeOfString_(&self, str_: NonNull<NSString>) -> NSRange {
        msg_send![self, localizedStandardRangeOfString: str_]
    }
    pub unsafe fn rangeOfString_(&self, searchString: NonNull<NSString>) -> NSRange {
        msg_send![self, rangeOfString: searchString]
    }
    pub unsafe fn rangeOfString_options_(
        &self,
        searchString: NonNull<NSString>,
        mask: NSStringCompareOptions,
    ) -> NSRange {
        msg_send![self, rangeOfString: searchString, options: mask]
    }
    pub unsafe fn rangeOfString_options_range_(
        &self,
        searchString: NonNull<NSString>,
        mask: NSStringCompareOptions,
        rangeOfReceiverToSearch: NSRange,
    ) -> NSRange {
        msg_send![
            self,
            rangeOfString: searchString,
            options: mask,
            range: rangeOfReceiverToSearch
        ]
    }
    pub unsafe fn rangeOfString_options_range_locale_(
        &self,
        searchString: NonNull<NSString>,
        mask: NSStringCompareOptions,
        rangeOfReceiverToSearch: NSRange,
        locale: *mut NSLocale,
    ) -> NSRange {
        msg_send![
            self,
            rangeOfString: searchString,
            options: mask,
            range: rangeOfReceiverToSearch,
            locale: locale
        ]
    }
    pub unsafe fn rangeOfCharacterFromSet_(&self, searchSet: NonNull<NSCharacterSet>) -> NSRange {
        msg_send![self, rangeOfCharacterFromSet: searchSet]
    }
    pub unsafe fn rangeOfCharacterFromSet_options_(
        &self,
        searchSet: NonNull<NSCharacterSet>,
        mask: NSStringCompareOptions,
    ) -> NSRange {
        msg_send![self, rangeOfCharacterFromSet: searchSet, options: mask]
    }
    pub unsafe fn rangeOfCharacterFromSet_options_range_(
        &self,
        searchSet: NonNull<NSCharacterSet>,
        mask: NSStringCompareOptions,
        rangeOfReceiverToSearch: NSRange,
    ) -> NSRange {
        msg_send![
            self,
            rangeOfCharacterFromSet: searchSet,
            options: mask,
            range: rangeOfReceiverToSearch
        ]
    }
    pub unsafe fn rangeOfComposedCharacterSequenceAtIndex_(&self, index: NSUInteger) -> NSRange {
        msg_send![self, rangeOfComposedCharacterSequenceAtIndex: index]
    }
    pub unsafe fn rangeOfComposedCharacterSequencesForRange_(&self, range: NSRange) -> NSRange {
        msg_send![self, rangeOfComposedCharacterSequencesForRange: range]
    }
    pub unsafe fn stringByAppendingString_(
        &self,
        aString: NonNull<NSString>,
    ) -> Autoreleased<NSString> {
        msg_send![self, stringByAppendingString: aString]
    }
    pub unsafe fn stringByAppendingFormat_(
        &self,
        format: NonNull<NSString>,
    ) -> Autoreleased<NSString> {
        msg_send![self, stringByAppendingFormat: format]
    }
    pub unsafe fn uppercaseStringWithLocale_(
        &self,
        locale: *mut NSLocale,
    ) -> Autoreleased<NSString> {
        msg_send![self, uppercaseStringWithLocale: locale]
    }
    pub unsafe fn lowercaseStringWithLocale_(
        &self,
        locale: *mut NSLocale,
    ) -> Autoreleased<NSString> {
        msg_send![self, lowercaseStringWithLocale: locale]
    }
    pub unsafe fn capitalizedStringWithLocale_(
        &self,
        locale: *mut NSLocale,
    ) -> Autoreleased<NSString> {
        msg_send![self, capitalizedStringWithLocale: locale]
    }
    pub unsafe fn getLineStart_end_contentsEnd_forRange_(
        &self,
        startPtr: *mut c_ulong,
        lineEndPtr: *mut c_ulong,
        contentsEndPtr: *mut c_ulong,
        range: NSRange,
    ) {
        msg_send![
            self,
            getLineStart: startPtr,
            end: lineEndPtr,
            contentsEnd: contentsEndPtr,
            forRange: range
        ]
    }
    pub unsafe fn lineRangeForRange_(&self, range: NSRange) -> NSRange {
        msg_send![self, lineRangeForRange: range]
    }
    pub unsafe fn getParagraphStart_end_contentsEnd_forRange_(
        &self,
        startPtr: *mut c_ulong,
        parEndPtr: *mut c_ulong,
        contentsEndPtr: *mut c_ulong,
        range: NSRange,
    ) {
        msg_send![
            self,
            getParagraphStart: startPtr,
            end: parEndPtr,
            contentsEnd: contentsEndPtr,
            forRange: range
        ]
    }
    pub unsafe fn paragraphRangeForRange_(&self, range: NSRange) -> NSRange {
        msg_send![self, paragraphRangeForRange: range]
    }
    pub unsafe fn enumerateSubstringsInRange_options_usingBlock_(
        &self,
        range: NSRange,
        opts: NSStringEnumerationOptions,
        block: *mut c_void,
    ) {
        msg_send![
            self,
            enumerateSubstringsInRange: range,
            options: opts,
            usingBlock: block
        ]
    }
    pub unsafe fn enumerateLinesUsingBlock_(&self, block: *mut c_void) {
        msg_send![self, enumerateLinesUsingBlock: block]
    }
    pub unsafe fn dataUsingEncoding_allowLossyConversion_(
        &self,
        encoding: NSStringEncoding,
        lossy: Bool,
    ) -> Autoreleased<NSData> {
        msg_send![
            self,
            dataUsingEncoding: encoding,
            allowLossyConversion: lossy
        ]
    }
    pub unsafe fn dataUsingEncoding_(&self, encoding: NSStringEncoding) -> Autoreleased<NSData> {
        msg_send![self, dataUsingEncoding: encoding]
    }
    pub unsafe fn canBeConvertedToEncoding_(&self, encoding: NSStringEncoding) -> Bool {
        msg_send![self, canBeConvertedToEncoding: encoding]
    }
    pub unsafe fn cStringUsingEncoding_(&self, encoding: NSStringEncoding) -> *const c_char {
        msg_send![self, cStringUsingEncoding: encoding]
    }
    pub unsafe fn getCString_maxLength_encoding_(
        &self,
        buffer: *mut c_char,
        maxBufferCount: NSUInteger,
        encoding: NSStringEncoding,
    ) -> Bool {
        msg_send![
            self,
            getCString: buffer,
            maxLength: maxBufferCount,
            encoding: encoding
        ]
    }
    pub unsafe fn getBytes_maxLength_usedLength_encoding_options_range_remainingRange_(
        &self,
        buffer: *mut c_void,
        maxBufferCount: NSUInteger,
        usedBufferCount: *mut c_ulong,
        encoding: NSStringEncoding,
        options: NSStringEncodingConversionOptions,
        range: NSRange,
        leftover: *mut NSRange,
    ) -> Bool {
        msg_send![
            self,
            getBytes: buffer,
            maxLength: maxBufferCount,
            usedLength: usedBufferCount,
            encoding: encoding,
            options: options,
            range: range,
            remainingRange: leftover,
        ]
    }
    pub unsafe fn maximumLengthOfBytesUsingEncoding_(&self, enc: NSStringEncoding) -> NSUInteger {
        msg_send![self, maximumLengthOfBytesUsingEncoding: enc]
    }
    pub unsafe fn lengthOfBytesUsingEncoding_(&self, enc: NSStringEncoding) -> NSUInteger {
        msg_send![self, lengthOfBytesUsingEncoding: enc]
    }
    pub unsafe fn componentsSeparatedByString_(
        &self,
        separator: NonNull<NSString>,
    ) -> Autoreleased<NSArray> {
        msg_send![self, componentsSeparatedByString: separator]
    }
    pub unsafe fn componentsSeparatedByCharactersInSet_(
        &self,
        separator: NonNull<NSCharacterSet>,
    ) -> Autoreleased<NSArray> {
        msg_send![self, componentsSeparatedByCharactersInSet: separator]
    }
    pub unsafe fn stringByTrimmingCharactersInSet_(
        &self,
        set: NonNull<NSCharacterSet>,
    ) -> Autoreleased<NSString> {
        msg_send![self, stringByTrimmingCharactersInSet: set]
    }
    pub unsafe fn stringByPaddingToLength_withString_startingAtIndex_(
        &self,
        newLength: NSUInteger,
        padString: NonNull<NSString>,
        padIndex: NSUInteger,
    ) -> Autoreleased<NSString> {
        msg_send![
            self,
            stringByPaddingToLength: newLength,
            withString: padString,
            startingAtIndex: padIndex
        ]
    }
    pub unsafe fn stringByFoldingWithOptions_locale_(
        &self,
        options: NSStringCompareOptions,
        locale: *mut NSLocale,
    ) -> Autoreleased<NSString> {
        msg_send![self, stringByFoldingWithOptions: options, locale: locale]
    }
    pub unsafe fn stringByReplacingOccurrencesOfString_withString_options_range_(
        &self,
        target: NonNull<NSString>,
        replacement: NonNull<NSString>,
        options: NSStringCompareOptions,
        searchRange: NSRange,
    ) -> Autoreleased<NSString> {
        msg_send![
            self,
            stringByReplacingOccurrencesOfString: target,
            withString: replacement,
            options: options,
            range: searchRange
        ]
    }
    pub unsafe fn stringByReplacingOccurrencesOfString_withString_(
        &self,
        target: NonNull<NSString>,
        replacement: NonNull<NSString>,
    ) -> Autoreleased<NSString> {
        msg_send![
            self,
            stringByReplacingOccurrencesOfString: target,
            withString: replacement
        ]
    }
    pub unsafe fn stringByReplacingCharactersInRange_withString_(
        &self,
        range: NSRange,
        replacement: NonNull<NSString>,
    ) -> Autoreleased<NSString> {
        msg_send![
            self,
            stringByReplacingCharactersInRange: range,
            withString: replacement
        ]
    }
    pub unsafe fn stringByApplyingTransform_reverse_(
        &self,
        transform: NonNull<NSString>,
        reverse: Bool,
    ) -> Option<Autoreleased<NSString>> {
        msg_send![self, stringByApplyingTransform: transform, reverse: reverse]
    }
    pub unsafe fn writeToURL_atomically_encoding_error_(
        &self,
        url: NonNull<NSURL>,
        useAuxiliaryFile: Bool,
        enc: NSStringEncoding,
        error: *mut NSError,
    ) -> Bool {
        msg_send![
            self,
            writeToURL: url,
            atomically: useAuxiliaryFile,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn writeToFile_atomically_encoding_error_(
        &self,
        path: NonNull<NSString>,
        useAuxiliaryFile: Bool,
        enc: NSStringEncoding,
        error: *mut NSError,
    ) -> Bool {
        msg_send![
            self,
            writeToFile: path,
            atomically: useAuxiliaryFile,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn initWithCharactersNoCopy_length_freeWhenDone_(
        this: Id<Self, Unknown>,
        characters: *mut c_ushort,
        length: NSUInteger,
        freeBuffer: Bool,
    ) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![
            this,
            initWithCharactersNoCopy: characters,
            length: length,
            freeWhenDone: freeBuffer,
        ])
    }
    pub unsafe fn initWithCharacters_length_(
        this: Id<Self, Unknown>,
        characters: *const c_ushort,
        length: NSUInteger,
    ) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![
            this,
            initWithCharacters: characters,
            length: length,
        ])
    }
    pub unsafe fn initWithUTF8String_(
        this: Id<Self, Unknown>,
        nullTerminatedCString: *const c_char,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![this, initWithUTF8String: nullTerminatedCString])
    }
    pub unsafe fn initWithString_(
        this: Id<Self, Unknown>,
        aString: NonNull<NSString>,
    ) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![this, initWithString: aString])
    }
    pub unsafe fn initWithFormat_(
        this: Id<Self, Unknown>,
        format: NonNull<NSString>,
    ) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![this, initWithFormat: format])
    }
    // pub unsafe fn initWithFormat_arguments_(
    //     this: Id<Self, Unknown>,
    //     format: NonNull<NSString>,
    //     argList: *mut __va_list_tag,
    // ) -> Id<Self, Unknown> {
    //     let this = ManuallyDrop::new(this);
    //     Id::new(msg_send![this, initWithFormat: format, arguments: argList])
    // }
    pub unsafe fn initWithFormat_locale_(
        this: Id<Self, Unknown>,
        format: NonNull<NSString>,
        locale: *mut Object,
    ) -> Id<Self, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![this, initWithFormat: format, locale: locale])
    }
    // pub unsafe fn initWithFormat_locale_arguments_(
    //     this: Id<Self, Unknown>,
    //     format: NonNull<NSString>,
    //     locale: *mut Object,
    //     argList: *mut __va_list_tag,
    // ) -> Id<Self, Unknown> {
    //     let this = ManuallyDrop::new(this);
    //     Id::new(msg_send![
    //         this,
    //         initWithFormat: format,
    //         locale: locale,
    //         arguments: argList
    //     ])
    // }
    pub unsafe fn initWithData_encoding_(
        this: Id<Self, Unknown>,
        data: NonNull<NSData>,
        encoding: NSStringEncoding,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![this, initWithData: data, encoding: encoding])
    }
    pub unsafe fn initWithBytes_length_encoding_(
        this: Id<Self, Unknown>,
        bytes: *const c_void,
        len: NSUInteger,
        encoding: NSStringEncoding,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![
            this,
            initWithBytes: bytes,
            length: len,
            encoding: encoding
        ])
    }
    pub unsafe fn initWithBytesNoCopy_length_encoding_freeWhenDone_(
        this: Id<Self, Unknown>,
        bytes: *mut c_void,
        len: NSUInteger,
        encoding: NSStringEncoding,
        freeBuffer: Bool,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![
            this,
            initWithBytesNoCopy: bytes,
            length: len,
            encoding: encoding,
            freeWhenDone: freeBuffer
        ])
    }
    pub unsafe fn initWithCString_encoding_(
        this: Id<Self, Unknown>,
        nullTerminatedCString: *const c_char,
        encoding: NSStringEncoding,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![
            this,
            initWithCString: nullTerminatedCString,
            encoding: encoding
        ])
    }
    pub unsafe fn initWithContentsOfURL_encoding_error_(
        this: Id<Self, Unknown>,
        url: NonNull<NSURL>,
        enc: NSStringEncoding,
        error: *mut NSError,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![
            this,
            initWithContentsOfURL: url,
            encoding: enc,
            error: error
        ])
    }
    pub unsafe fn initWithContentsOfFile_encoding_error_(
        this: Id<Self, Unknown>,
        path: NonNull<NSString>,
        enc: NSStringEncoding,
        error: *mut NSError,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![
            this,
            initWithContentsOfFile: path,
            encoding: enc,
            error: error
        ])
    }
    pub unsafe fn initWithContentsOfURL_usedEncoding_error_(
        this: Id<Self, Unknown>,
        url: NonNull<NSURL>,
        enc: *mut c_ulong,
        error: *mut NSError,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![
            this,
            initWithContentsOfURL: url,
            usedEncoding: enc,
            error: error
        ])
    }
    pub unsafe fn initWithContentsOfFile_usedEncoding_error_(
        this: Id<Self, Unknown>,
        path: NonNull<NSString>,
        enc: *mut c_ulong,
        error: *mut NSError,
    ) -> Option<Id<Self, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![
            this,
            initWithContentsOfFile: path,
            usedEncoding: enc,
            error: error
        ])
    }
    pub unsafe fn doubleValue(&self) -> f64 {
        msg_send![self, doubleValue]
    }
    pub unsafe fn floatValue(&self) -> f32 {
        msg_send![self, floatValue]
    }
    pub unsafe fn intValue(&self) -> c_int {
        msg_send![self, intValue]
    }
    pub unsafe fn integerValue(&self) -> NSInteger {
        msg_send![self, integerValue]
    }
    pub unsafe fn longLongValue(&self) -> c_longlong {
        msg_send![self, longLongValue]
    }
    pub unsafe fn boolValue(&self) -> Bool {
        msg_send![self, boolValue]
    }
    pub unsafe fn uppercaseString(&self) -> Autoreleased<NSString> {
        msg_send![self, uppercaseString]
    }
    pub unsafe fn lowercaseString(&self) -> Autoreleased<NSString> {
        msg_send![self, lowercaseString]
    }
    pub unsafe fn capitalizedString(&self) -> Autoreleased<NSString> {
        msg_send![self, capitalizedString]
    }
    pub unsafe fn localizedUppercaseString(&self) -> Autoreleased<NSString> {
        msg_send![self, localizedUppercaseString]
    }
    pub unsafe fn localizedLowercaseString(&self) -> Autoreleased<NSString> {
        msg_send![self, localizedLowercaseString]
    }
    pub unsafe fn localizedCapitalizedString(&self) -> Autoreleased<NSString> {
        msg_send![self, localizedCapitalizedString]
    }
    pub unsafe fn UTF8String(&self) -> *const c_char {
        msg_send![self, UTF8String]
    }
    pub unsafe fn fastestEncoding(&self) -> NSStringEncoding {
        msg_send![self, fastestEncoding]
    }
    pub unsafe fn smallestEncoding(&self) -> NSStringEncoding {
        msg_send![self, smallestEncoding]
    }
    pub unsafe fn decomposedStringWithCanonicalMapping(&self) -> Autoreleased<NSString> {
        msg_send![self, decomposedStringWithCanonicalMapping]
    }
    pub unsafe fn precomposedStringWithCanonicalMapping(&self) -> Autoreleased<NSString> {
        msg_send![self, precomposedStringWithCanonicalMapping]
    }
    pub unsafe fn decomposedStringWithCompatibilityMapping(&self) -> Autoreleased<NSString> {
        msg_send![self, decomposedStringWithCompatibilityMapping]
    }
    pub unsafe fn precomposedStringWithCompatibilityMapping(&self) -> Autoreleased<NSString> {
        msg_send![self, precomposedStringWithCompatibilityMapping]
    }
    pub unsafe fn description(&self) -> Autoreleased<NSString> {
        msg_send![self, description]
    }
    pub unsafe fn hash(&self) -> NSUInteger {
        msg_send![self, hash]
    }
    pub unsafe fn localizedNameOfStringEncoding_(
        encoding: NSStringEncoding,
    ) -> Autoreleased<NSString> {
        msg_send![class!(NSString), localizedNameOfStringEncoding: encoding]
    }
    pub unsafe fn string() -> Autoreleased<Self> {
        msg_send![class!(NSString), string]
    }
    pub unsafe fn stringWithString_(string: NonNull<NSString>) -> Autoreleased<Self> {
        msg_send![class!(NSString), stringWithString: string]
    }
    pub unsafe fn stringWithCharacters_length_(
        characters: *const c_ushort,
        length: NSUInteger,
    ) -> Autoreleased<Self> {
        msg_send![
            class!(NSString),
            stringWithCharacters: characters,
            length: length
        ]
    }
    pub unsafe fn stringWithUTF8String_(
        nullTerminatedCString: *const c_char,
    ) -> Option<Autoreleased<Self>> {
        msg_send![
            class!(NSString),
            stringWithUTF8String: nullTerminatedCString,
        ]
    }
    pub unsafe fn stringWithFormat_(format: NonNull<NSString>) -> Autoreleased<Self> {
        msg_send![class!(NSString), stringWithFormat: format]
    }
    pub unsafe fn localizedStringWithFormat_(format: NonNull<NSString>) -> Autoreleased<Self> {
        msg_send![class!(NSString), localizedStringWithFormat: format]
    }
    pub unsafe fn stringWithCString_encoding_(
        cString: *const c_char,
        enc: NSStringEncoding,
    ) -> Option<Autoreleased<Self>> {
        msg_send![class!(NSString), stringWithCString: cString, encoding: enc]
    }
    pub unsafe fn stringWithContentsOfURL_encoding_error_(
        url: NonNull<NSURL>,
        enc: NSStringEncoding,
        error: *mut NSError,
    ) -> Option<Autoreleased<Self>> {
        msg_send![
            class!(NSString),
            stringWithContentsOfURL: url,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn stringWithContentsOfFile_encoding_error_(
        path: NonNull<NSString>,
        enc: NSStringEncoding,
        error: *mut NSError,
    ) -> Option<Autoreleased<Self>> {
        msg_send![
            class!(NSString),
            stringWithContentsOfFile: path,
            encoding: enc,
            error: error
        ]
    }
    pub unsafe fn stringWithContentsOfURL_usedEncoding_error_(
        url: NonNull<NSURL>,
        enc: *mut c_ulong,
        error: *mut NSError,
    ) -> Option<Autoreleased<Self>> {
        msg_send![
            class!(NSString),
            stringWithContentsOfURL: url,
            usedEncoding: enc,
            error: error
        ]
    }
    pub unsafe fn stringWithContentsOfFile_usedEncoding_error_(
        path: NonNull<NSString>,
        enc: *mut c_ulong,
        error: *mut NSError,
    ) -> Option<Autoreleased<Self>> {
        msg_send![
            class!(NSString),
            stringWithContentsOfFile: path,
            usedEncoding: enc,
            error: error
        ]
    }
    pub unsafe fn availableStringEncodings() -> *const c_ulong {
        msg_send![class!(NSString), availableStringEncodings]
    }
    pub unsafe fn defaultCStringEncoding() -> NSStringEncoding {
        msg_send![class!(NSString), defaultCStringEncoding]
    }
}
pub const NSStringEnumerationOptions_NSStringEnumerationByLines: NSStringEnumerationOptions = 0;
pub const NSStringEnumerationOptions_NSStringEnumerationByParagraphs: NSStringEnumerationOptions =
    1;
pub const NSStringEnumerationOptions_NSStringEnumerationByComposedCharacterSequences:
    NSStringEnumerationOptions = 2;
pub const NSStringEnumerationOptions_NSStringEnumerationByWords: NSStringEnumerationOptions = 3;
pub const NSStringEnumerationOptions_NSStringEnumerationBySentences: NSStringEnumerationOptions = 4;
pub const NSStringEnumerationOptions_NSStringEnumerationReverse: NSStringEnumerationOptions = 256;
pub const NSStringEnumerationOptions_NSStringEnumerationSubstringNotRequired:
    NSStringEnumerationOptions = 512;
pub const NSStringEnumerationOptions_NSStringEnumerationLocalized: NSStringEnumerationOptions =
    1024;
pub type NSStringEnumerationOptions = NSUInteger;
pub type NSStringTransform = NSString;
extern "C" {
    pub static mut NSStringTransformLatinToKatakana: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformLatinToHiragana: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformLatinToHangul: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformLatinToArabic: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformLatinToHebrew: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformLatinToThai: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformLatinToCyrillic: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformLatinToGreek: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformToLatin: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformMandarinToLatin: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformHiraganaToKatakana: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformFullwidthToHalfwidth: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformToXMLHex: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformToUnicodeName: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformStripCombiningMarks: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringTransformStripDiacritics: NonNull<NSString>;
}
pub type NSStringEncodingDetectionOptionsKey = NSString;
#[doc = " NSStringEncodingDetection"]
impl NSString {
    pub unsafe fn stringEncodingForData_encodingOptions_convertedString_usedLossyConversion_(
        data: NonNull<NSData>,
        opts: *mut NSDictionary,
        string: NonNull<NSString>,
        usedLossyConversion: *mut c_schar,
    ) -> NSStringEncoding {
        msg_send![
            class!(NSString),
            stringEncodingForData: data,
            encodingOptions: opts,
            convertedString: string,
            usedLossyConversion: usedLossyConversion
        ]
    }
}
extern "C" {
    pub static mut NSStringEncodingDetectionSuggestedEncodingsKey: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringEncodingDetectionDisallowedEncodingsKey: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringEncodingDetectionUseOnlySuggestedEncodingsKey: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringEncodingDetectionAllowLossyKey: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringEncodingDetectionFromWindowsKey: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringEncodingDetectionLossySubstitutionKey: NonNull<NSString>;
}
extern "C" {
    pub static mut NSStringEncodingDetectionLikelyLanguageKey: NonNull<NSString>;
}
#[doc = " NSItemProvider"]
impl NSString {}
#[repr(transparent)]
pub struct NSMutableString(NSString);
impl core::ops::Deref for NSMutableString {
    type Target = NSString;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for NSMutableString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl Message for NSMutableString {}
unsafe impl RefEncode for NSMutableString {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}
impl NSMutableString {
    pub unsafe fn alloc() -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![class!(NSString), alloc])
    }
}
impl NSMutableString {
    pub unsafe fn replaceCharactersInRange_withString_(
        &self,
        range: NSRange,
        aString: NonNull<NSString>,
    ) {
        msg_send![self, replaceCharactersInRange: range, withString: aString]
    }
}
#[doc = " NSMutableStringExtensionMethods"]
impl NSMutableString {
    pub unsafe fn insertString_atIndex_(&self, aString: NonNull<NSString>, loc: NSUInteger) {
        msg_send![self, insertString: aString, atIndex: loc]
    }
    pub unsafe fn deleteCharactersInRange_(&self, range: NSRange) {
        msg_send![self, deleteCharactersInRange: range]
    }
    pub unsafe fn appendString_(&self, aString: NonNull<NSString>) {
        msg_send![self, appendString: aString]
    }
    pub unsafe fn appendFormat_(&self, format: NonNull<NSString>) {
        msg_send![self, appendFormat: format]
    }
    pub unsafe fn setString_(&self, aString: NonNull<NSString>) {
        msg_send![self, setString: aString]
    }
    pub unsafe fn replaceOccurrencesOfString_withString_options_range_(
        &self,
        target: *const NSString,
        replacement: *const NSString,
        options: NSStringCompareOptions,
        searchRange: NSRange,
    ) -> NSUInteger {
        msg_send![
            self,
            replaceOccurrencesOfString: target,
            withString: replacement,
            options: options,
            range: searchRange
        ]
    }
    pub unsafe fn applyTransform_reverse_range_updatedRange_(
        &self,
        transform: *const NSString,
        reverse: Bool,
        range: NSRange,
        resultingRange: *mut NSRange,
    ) -> Bool {
        msg_send![
            self,
            applyTransform: transform,
            reverse: reverse,
            range: range,
            updatedRange: resultingRange
        ]
    }
    pub unsafe fn initWithCapacity_(
        this: Id<Self, Unknown>,
        capacity: NSUInteger,
    ) -> Id<NSMutableString, Unknown> {
        let this = ManuallyDrop::new(this);
        Id::new(msg_send![this, initWithCapacity: capacity])
    }
    pub unsafe fn stringWithCapacity_(capacity: NSUInteger) -> Autoreleased<NSMutableString> {
        msg_send![class!(NSMutableString), stringWithCapacity: capacity]
    }
}
extern "C" {
    pub static mut NSCharacterConversionException: NonNull<NSString>;
}
extern "C" {
    pub static mut NSParseErrorException: NonNull<NSString>;
}
#[doc = " NSExtendedStringPropertyListParsing"]
impl NSString {
    pub unsafe fn propertyList(&self) -> Autoreleased<Object> {
        msg_send![self, propertyList]
    }
    pub unsafe fn propertyListFromStringsFileFormat(&self) -> Option<Autoreleased<NSDictionary>> {
        msg_send![self, propertyListFromStringsFileFormat]
    }
}
#[doc = " NSStringDeprecated"]
impl NSString {
    pub unsafe fn cString(&self) -> *const c_char {
        msg_send![self, cString]
    }
    pub unsafe fn lossyCString(&self) -> *const c_char {
        msg_send![self, lossyCString]
    }
    pub unsafe fn cStringLength(&self) -> NSUInteger {
        msg_send![self, cStringLength]
    }
    pub unsafe fn getCString_(&self, bytes: *mut c_char) {
        msg_send![self, getCString: bytes]
    }
    pub unsafe fn getCString_maxLength_(&self, bytes: *mut c_char, maxLength: NSUInteger) {
        msg_send![self, getCString: bytes, maxLength: maxLength]
    }
    pub unsafe fn getCString_maxLength_range_remainingRange_(
        &self,
        bytes: *mut c_char,
        maxLength: NSUInteger,
        aRange: NSRange,
        leftoverRange: *mut NSRange,
    ) {
        msg_send![
            self,
            getCString: bytes,
            maxLength: maxLength,
            range: aRange,
            remainingRange: leftoverRange
        ]
    }
    pub unsafe fn writeToFile_atomically_(
        &self,
        path: NonNull<NSString>,
        useAuxiliaryFile: Bool,
    ) -> Bool {
        msg_send![self, writeToFile: path, atomically: useAuxiliaryFile]
    }
    pub unsafe fn writeToURL_atomically_(&self, url: NonNull<NSURL>, atomically: Bool) -> Bool {
        msg_send![self, writeToURL: url, atomically: atomically]
    }
    pub unsafe fn initWithContentsOfFile_(
        this: Id<Self, Unknown>,
        path: NonNull<NSString>,
    ) -> Option<Id<Object, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![this, initWithContentsOfFile: path])
    }
    pub unsafe fn initWithContentsOfURL_(
        this: Id<Self, Unknown>,
        url: NonNull<NSURL>,
    ) -> Option<Id<Object, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![this, initWithContentsOfURL: url])
    }
    pub unsafe fn initWithCStringNoCopy_length_freeWhenDone_(
        this: Id<Self, Unknown>,
        bytes: *mut c_char,
        length: NSUInteger,
        freeBuffer: Bool,
    ) -> Option<Id<Object, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![
            this,
            initWithCStringNoCopy: bytes,
            length: length,
            freeWhenDone: freeBuffer
        ])
    }
    pub unsafe fn initWithCString_length_(
        this: Id<Self, Unknown>,
        bytes: *const c_char,
        length: NSUInteger,
    ) -> Option<Id<Object, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![this, initWithCString: bytes, length: length])
    }
    pub unsafe fn initWithCString_(
        this: Id<Self, Unknown>,
        bytes: *const c_char,
    ) -> Option<Id<Object, Unknown>> {
        let this = ManuallyDrop::new(this);
        Id::new_null(msg_send![this, initWithCString: bytes])
    }
    pub unsafe fn getCharacters_(&self, buffer: *mut c_ushort) {
        msg_send![self, getCharacters: buffer]
    }
    pub unsafe fn stringWithContentsOfFile_(
        path: NonNull<NSString>,
    ) -> Option<Autoreleased<Object>> {
        msg_send![class!(NSString), stringWithContentsOfFile: path]
    }
    pub unsafe fn stringWithContentsOfURL_(url: NonNull<NSURL>) -> Option<Autoreleased<Object>> {
        msg_send![class!(NSString), stringWithContentsOfURL: url]
    }
    pub unsafe fn stringWithCString_length_(
        bytes: *const c_char,
        length: NSUInteger,
    ) -> Option<Autoreleased<Object>> {
        msg_send![class!(NSString), stringWithCString: bytes, length: length]
    }
    pub unsafe fn stringWithCString_(bytes: *const c_char) -> Option<Autoreleased<Object>> {
        msg_send![class!(NSString), stringWithCString: bytes]
    }
}
pub const NSProprietaryStringEncoding: NSStringEncoding = 65536;
pub type _bindgen_ty_11 = NSStringEncoding;
#[repr(transparent)]
pub struct NSSimpleCString(NSString);
impl core::ops::Deref for NSSimpleCString {
    type Target = NSString;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for NSSimpleCString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl Message for NSSimpleCString {}
unsafe impl RefEncode for NSSimpleCString {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}
impl NSSimpleCString {
    pub unsafe fn alloc() -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![class!(NSString), alloc])
    }
}
impl NSSimpleCString {}
#[repr(transparent)]
pub struct NSConstantString(NSSimpleCString);
impl core::ops::Deref for NSConstantString {
    type Target = NSSimpleCString;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for NSConstantString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl NSConstantString {
    pub unsafe fn alloc() -> Option<Id<Self, Unknown>> {
        Id::new_null(msg_send![class!(NSString), alloc])
    }
}
unsafe impl Message for NSConstantString {}
unsafe impl RefEncode for NSConstantString {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}
impl NSConstantString {}
