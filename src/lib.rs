#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![no_std]

extern crate libc;
#[cfg(target_os = "windows")]
extern crate winapi;

use core::cmp::{Eq, PartialEq};
use core::hash::{Hash, Hasher};
use core::fmt;
use core::mem;
use core::ptr;
use core::str;
use libc::*;

pub type UInt8 = c_uchar;
pub type SInt8 = c_char;
pub type UInt16 = c_ushort;
pub type SInt16 = c_short;
pub type UInt32 = c_uint;
pub type SInt32 = c_int;

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct wide {
    pub hi: SInt32,
    pub lo: UInt32,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UnsignedWide {
    pub hi: UInt32,
    pub lo: UInt32,
}

#[cfg(not(target_endian = "big"))]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct wide {
    pub lo: UInt32,
    pub hi: SInt32,
}

#[cfg(not(target_endian = "big"))]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UnsignedWide {
    pub lo: UInt32,
    pub hi: UInt32,
}

// _LP64_
#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
pub type SInt64 = winapi::__int64;

#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
pub type UInt64 = winapi::__int64;

#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
pub type SInt64 = c_ulonglong;

#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
pub type UInt64 = c_longlong;

pub type Fixed = SInt32;
pub type FixedPtr = *mut Fixed;
pub type Fract = SInt32;
pub type FractPtr = *mut Fract;
pub type UnsignedFixed = UInt32;
pub type UnsignedFixedPtr = *mut UnsignedFixed;
pub type ShortFixed = SInt16;
pub type ShortFixedPtr = *mut ShortFixed;

pub type Float32 = c_float;
pub type Float64 = c_double;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Float80 {
    pub exp: SInt16,
    pub man: [UInt16; 4],
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Float96 {
    pub exp: [SInt16; 2],
    pub man: [UInt16; 4],
}

pub type Ptr = *mut c_char;
pub type Handle = *mut Ptr;
pub type size = c_long;

pub type OSErr = SInt16;
pub type OSStatus = SInt32;
pub type LogicalAddress = *mut c_void;
pub type ConstLogicalAddress = *const c_void;
pub type PhysicalAddress = *mut c_void;
pub type BytePtr = *mut UInt8;
pub type ByteCount = c_ulong;
pub type ByteOffset = c_ulong;
pub type Duration = SInt32;
pub type AbsoluteTime = UnsignedWide;
pub type OptionBits = UInt32;
pub type ItemCount = c_ulong;
pub type PBVersion = UInt32;
pub type ScriptCode = SInt16;
pub type LangCode = SInt16;
pub type RegionCode = SInt16;
pub type FourCharCode = UInt32;
pub type OSType = FourCharCode;
pub type ResType = FourCharCode;
pub type OSTypePtr = *mut OSType;
pub type ResTypePtr = *mut ResType;

pub type Boolean = c_uchar;

pub type ProcPtr = unsafe extern "C" fn(c_long);
pub type Register68kProcPtr = unsafe extern "C" fn();

pub type UniversalProcPtr = ProcPtr;

pub type ProcHandle = *mut ProcPtr;
pub type UniversalProcHandle = *mut UniversalProcPtr;

pub type PRefCon = *mut c_void;

#[cfg(target_pointer_width = "64")]
pub type URefCon = *mut c_void;

#[cfg(target_pointer_width = "64")]
pub type SRefCon = *mut c_void;

#[cfg(not(target_pointer_width = "64"))]
pub type URefCon = UInt32;

#[cfg(not(target_pointer_width = "64"))]
pub type SRefCon = SInt32;

pub const kNoErr: OSErr = 0;
pub const kNilOptions: OptionBits = 0;
pub const kInvalidId: u32 = 0;
pub const kVariableLengthArray: u32 = 1;
pub const kUnknownType: u32 = 0x3F3F3F3F;

pub type UnicodeScalarValue = UInt32;
pub type UTF32Char = UInt32;
pub type UniChar = UInt16;
pub type UTF16Char = UInt16;
pub type UTF8Char = UInt8;
pub type UniCharPtr = *mut UniChar;
pub type UniCharCount = c_ulong;
pub type UniCharCountPtr = *mut UniCharCount;
pub type Str255 = [c_uchar; 256];
pub type Str63 = [c_uchar; 64];
pub type Str31 = [c_uchar; 32];
pub type Str27 = [c_uchar; 28];
pub type Str15 = [c_uchar; 16];

pub type StrField32 = [c_uchar; 34];

pub type StrFileName = Str63;
pub type StringPtr = *mut c_uchar;
pub type StringHandle = *mut StringPtr;
pub type ConstStringPtr = *const c_uchar;
pub type ConstStr255Param = *const c_uchar;
pub type ConstStr63Param = *const c_uchar;
pub type ConstStr31Param = *const c_uchar;
pub type ConstStr27Param = *const c_uchar;
pub type ConstStr15Param = *const c_uchar;
pub type ConstStrFileNameParam = *const ConstStr63Param;

#[inline]
pub unsafe extern "C" fn StrLength(string: ConstStr255Param) -> c_uchar {
    *string
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ProcessSerialNumber {
    pub highLongOfPSN: UInt32,
    pub lowLongOfPSN: UInt32,
}

pub type ProcessSerialNumberPtr = *mut ProcessSerialNumber;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point {
    pub v: c_short,
    pub h: c_short,
}

pub type PointPtr = *mut Point;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Rect {
    pub top: c_short,
    pub left: c_short,
    pub bottom: c_short,
    pub right: c_short,
}

pub type RectPtr = *mut Rect;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FixedRect {
    pub top: Fixed,
    pub left: Fixed,
    pub bottom: Fixed,
    pub right: Fixed,
}

pub type FixedRectPtr = *mut FixedRect;

pub type CharParameter = c_short;

pub const normal: Style = 0;
pub const bold: Style = 1;
pub const italic: Style = 2;
pub const underline: Style = 4;
pub const outline: Style = 8;
pub const shadow: Style = 0x10;
pub const condense: Style = 0x20;
pub const extend: Style = 0x40;

pub type Style = c_uchar;
pub type StyleParameter = c_short;
pub type StyleField = Style;

pub type TimeValue = SInt32;
pub type TimeScale = SInt32;
pub type CompTimeValue = wide;
pub type TimeBase = *mut TimeBaseRecord;

#[repr(C)]
pub struct TimeBaseRecord {
    _priv: c_void
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TimeRecord {
    pub value: CompTimeValue,
    pub scale: TimeScale,
    pub base: TimeBase,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct NumVersion {
    pub majorRev: UInt8,
    pub minorAndBugRef: UInt8,
    pub stage: UInt8,
    pub nonRelRev: UInt8,
}

#[cfg(not(target_endian = "big"))]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct NumVersion {
    pub nonRelRev: UInt8,
    pub stage: UInt8,
    pub minorAndBugRef: UInt8,
    pub majorRev: UInt8,
}

pub const developStage: UInt32 = 0x20;
pub const alphaStage: UInt32 = 0x40;
pub const betaStage: UInt32 = 0x60;
pub const finalStage: UInt32 = 0x80;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct NumVersionVariant {
    pub whole: UInt32,
}

pub type NumVersionVariantPtr = *mut NumVersionVariant;
pub type NumVersionVariantHandle = *mut NumVersionVariantPtr;

#[repr(C)]
pub struct VersRec {
    pub numericVersion: NumVersion,
    pub countryCode: c_short,
    pub shortVersion: Str255,
    pub reserved: Str255,
}

// Manual implementation of various traits on VersRec as the Str255 members prevent auto-derive

impl Clone for VersRec {
    fn clone(&self) -> Self {
        let mut clone = VersRec::default();
        clone.numericVersion = self.numericVersion;
        clone.countryCode = self.countryCode;
        unsafe {
            ptr::copy_nonoverlapping(&self.shortVersion, &mut clone.shortVersion, 1);
            ptr::copy_nonoverlapping(&self.reserved, &mut clone.reserved, 1);
        }
        clone
    }
}

impl Copy for VersRec { }

impl Default for VersRec {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Eq for VersRec { }

impl fmt::Debug for VersRec {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("VersRec")
            .field("numericVersion", &self.numericVersion)
            .field("countryCode", &self.countryCode)
            .field("shortVersion", &str::from_utf8(&self.shortVersion).unwrap())
            .field("reserved", &str::from_utf8(&self.reserved).unwrap())
            .finish()
    }
}

impl Hash for VersRec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(unsafe { mem::transmute(self.numericVersion) });
        state.write_i16(self.countryCode);
        state.write(&self.shortVersion);
        state.write(&self.reserved);
    }
}

impl PartialEq for VersRec {
    fn eq(&self, other: &Self) -> bool {
        #[inline]
        fn ptr<T>(slice: &[T]) -> *const c_void {
            slice.as_ptr() as *const _
        }

        unsafe {
            self.numericVersion == other.numericVersion &&
            self.countryCode == other.countryCode &&
            memcmp(ptr(&self.shortVersion), ptr(&other.shortVersion), mem::size_of::<Str255>()) == 0 &&
            memcmp(ptr(&self.reserved), ptr(&other.reserved), mem::size_of::<Str255>()) == 0
        }
    }
}

pub type VersRecPtr = *mut VersRec;
pub type VersRecHndl = *mut VersRecPtr;

pub type Byte = UInt8;
pub type SignedByte = SInt8;
pub type WidePtr = *mut wide;
pub type UnsignedWidePtr = *mut UnsignedWide;
pub type extended80 = Float80;
pub type extended96 = Float96;
pub type VHSelect = SInt8;

#[cfg(target_os = "macos")]
#[cfg_attr(target_os = "macos", link(name = "CoreServices", kind = "framework"))]
extern "C" {
    pub fn Debugger();
    pub fn DebugStr(debuggerMsg: ConstStr255Param);
    pub fn SysBreak();
    pub fn SysBreakStr(debuggerMsg: ConstStr255Param);
    pub fn SysBreakFunc(debuggerMsg: ConstStr255Param);
}

#[cfg(test)]
mod versrec_manual_derive_tests {
    use super::*;
    #[test]
    fn test_clone_and_eq() {
        let mut rec0 = VersRec::default();
        rec0.countryCode = 14;

        let random_string = b"Hello, world!A)R\xE2Q$";
        for i in 0..random_string.len() {
            rec0.shortVersion[i] = random_string[i];
        }

        let rec1 = rec0.clone();
        assert_eq!(rec0, rec1);
    }
}
