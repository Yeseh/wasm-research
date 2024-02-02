#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(unused_unsafe, clippy::all)]
/// Ask some data from the host
pub fn get_name() -> wit_bindgen::rt::string::String {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
    unsafe {
        #[repr(align(4))]
        struct RetArea([u8; 8]);
        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32) {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        wit_import(ptr0);
        let l1 = *((ptr0 + 0) as *const i32);
        let l2 = *((ptr0 + 4) as *const i32);
        let len3 = l2 as usize;
        let bytes3 = Vec::from_raw_parts(l1 as *mut _, len3, len3);
        wit_bindgen::rt::string_lift(bytes3)
    }
}
#[allow(unused_unsafe, clippy::all)]
/// Ask the host to print a message
pub fn print(msg: &str) {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
    unsafe {
        let vec0 = msg;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32) {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        wit_import(ptr0, len0);
    }
}
const _: () = {
    #[doc(hidden)]
    #[export_name = "hello"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_hello() {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        <_GuestImpl as Guest>::hello();
    }
};
use Hello as _GuestImpl;
pub trait Guest {
    /// Export the run method
    fn hello();
}
pub mod wasi {
    pub mod clocks {
        #[allow(clippy::all)]
        pub mod wall_clock {
            /// A time and date in seconds plus nanoseconds.
            #[repr(C)]
            pub struct Datetime {
                pub seconds: u64,
                pub nanoseconds: u32,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Datetime {
                #[inline]
                fn clone(&self) -> Datetime {
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Datetime {}
            impl ::core::fmt::Debug for Datetime {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Datetime")
                        .field("seconds", &self.seconds)
                        .field("nanoseconds", &self.nanoseconds)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Read the current value of the clock.
            ///
            /// This clock is not monotonic, therefore calling this function repeatedly
            /// will not necessarily produce a sequence of non-decreasing values.
            ///
            /// The returned timestamps represent the number of seconds since
            /// 1970-01-01T00:00:00Z, also known as [POSIX's Seconds Since the Epoch],
            /// also known as [Unix Time].
            ///
            /// The nanoseconds field of the output is always less than 1000000000.
            ///
            /// [POSIX's Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16
            /// [Unix Time]: https://en.wikipedia.org/wiki/Unix_time
            pub fn now() -> Datetime {
                #[allow(unused_imports)]
                use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([u8; 16]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    let ptr0 = ret_area.as_mut_ptr() as i32;
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                    wit_import(ptr0);
                    let l1 = *((ptr0 + 0) as *const i64);
                    let l2 = *((ptr0 + 8) as *const i32);
                    Datetime {
                        seconds: l1 as u64,
                        nanoseconds: l2 as u32,
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Query the resolution of the clock.
            ///
            /// The nanoseconds field of the output is always less than 1000000000.
            pub fn resolution() -> Datetime {
                #[allow(unused_imports)]
                use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([u8; 16]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    let ptr0 = ret_area.as_mut_ptr() as i32;
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                    wit_import(ptr0);
                    let l1 = *((ptr0 + 0) as *const i64);
                    let l2 = *((ptr0 + 8) as *const i32);
                    Datetime {
                        seconds: l1 as u64,
                        nanoseconds: l2 as u32,
                    }
                }
            }
        }
    }
    pub mod filesystem {
        #[allow(clippy::all)]
        pub mod types {
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type Error = super::super::super::wasi::io::streams::Error;
            pub type Datetime = super::super::super::wasi::clocks::wall_clock::Datetime;
            /// File size or length of a region within a file.
            pub type Filesize = u64;
            /// The type of a filesystem object referenced by a descriptor.
            ///
            /// Note: This was called `filetype` in earlier versions of WASI.
            #[repr(u8)]
            pub enum DescriptorType {
                /// The type of the descriptor or file is unknown or is different from
                /// any of the other types specified.
                Unknown,
                /// The descriptor refers to a block device inode.
                BlockDevice,
                /// The descriptor refers to a character device inode.
                CharacterDevice,
                /// The descriptor refers to a directory inode.
                Directory,
                /// The descriptor refers to a named pipe.
                Fifo,
                /// The file refers to a symbolic link inode.
                SymbolicLink,
                /// The descriptor refers to a regular file inode.
                RegularFile,
                /// The descriptor refers to a socket.
                Socket,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DescriptorType {
                #[inline]
                fn clone(&self) -> DescriptorType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for DescriptorType {}
            #[automatically_derived]
            impl ::core::cmp::Eq for DescriptorType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for DescriptorType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for DescriptorType {
                #[inline]
                fn eq(&self, other: &DescriptorType) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl ::core::fmt::Debug for DescriptorType {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        DescriptorType::Unknown => {
                            f.debug_tuple("DescriptorType::Unknown").finish()
                        }
                        DescriptorType::BlockDevice => {
                            f.debug_tuple("DescriptorType::BlockDevice").finish()
                        }
                        DescriptorType::CharacterDevice => {
                            f.debug_tuple("DescriptorType::CharacterDevice").finish()
                        }
                        DescriptorType::Directory => {
                            f.debug_tuple("DescriptorType::Directory").finish()
                        }
                        DescriptorType::Fifo => {
                            f.debug_tuple("DescriptorType::Fifo").finish()
                        }
                        DescriptorType::SymbolicLink => {
                            f.debug_tuple("DescriptorType::SymbolicLink").finish()
                        }
                        DescriptorType::RegularFile => {
                            f.debug_tuple("DescriptorType::RegularFile").finish()
                        }
                        DescriptorType::Socket => {
                            f.debug_tuple("DescriptorType::Socket").finish()
                        }
                    }
                }
            }
            impl DescriptorType {
                pub(crate) unsafe fn _lift(val: u8) -> DescriptorType {
                    if !true {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => DescriptorType::Unknown,
                        1 => DescriptorType::BlockDevice,
                        2 => DescriptorType::CharacterDevice,
                        3 => DescriptorType::Directory,
                        4 => DescriptorType::Fifo,
                        5 => DescriptorType::SymbolicLink,
                        6 => DescriptorType::RegularFile,
                        7 => DescriptorType::Socket,
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("invalid enum discriminant"),
                            );
                        }
                    }
                }
            }
            /// Descriptor flags.
            ///
            /// Note: This was called `fdflags` in earlier versions of WASI.
            pub struct DescriptorFlags(
                <DescriptorFlags as ::bitflags::__private::PublicFlags>::Internal,
            );
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for DescriptorFlags {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for DescriptorFlags {
                #[inline]
                fn eq(&self, other: &DescriptorFlags) -> bool {
                    self.0 == other.0
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for DescriptorFlags {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <DescriptorFlags as ::bitflags::__private::PublicFlags>::Internal,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for DescriptorFlags {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &DescriptorFlags,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for DescriptorFlags {
                #[inline]
                fn cmp(&self, other: &DescriptorFlags) -> ::core::cmp::Ordering {
                    ::core::cmp::Ord::cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for DescriptorFlags {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.0, state)
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for DescriptorFlags {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DescriptorFlags",
                        &&self.0,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DescriptorFlags {
                #[inline]
                fn clone(&self) -> DescriptorFlags {
                    let _: ::core::clone::AssertParamIsClone<
                        <DescriptorFlags as ::bitflags::__private::PublicFlags>::Internal,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for DescriptorFlags {}
            impl DescriptorFlags {
                /// Read mode: Data can be read.
                #[allow(deprecated, non_upper_case_globals)]
                pub const READ: Self = Self::from_bits_retain(1 << 0);
                /// Write mode: Data can be written to.
                #[allow(deprecated, non_upper_case_globals)]
                pub const WRITE: Self = Self::from_bits_retain(1 << 1);
                /// Request that writes be performed according to synchronized I/O file
                /// integrity completion. The data stored in the file and the file's
                /// metadata are synchronized. This is similar to `O_SYNC` in POSIX.
                ///
                /// The precise semantics of this operation have not yet been defined for
                /// WASI. At this time, it should be interpreted as a request, and not a
                /// requirement.
                #[allow(deprecated, non_upper_case_globals)]
                pub const FILE_INTEGRITY_SYNC: Self = Self::from_bits_retain(1 << 2);
                /// Request that writes be performed according to synchronized I/O data
                /// integrity completion. Only the data stored in the file is
                /// synchronized. This is similar to `O_DSYNC` in POSIX.
                ///
                /// The precise semantics of this operation have not yet been defined for
                /// WASI. At this time, it should be interpreted as a request, and not a
                /// requirement.
                #[allow(deprecated, non_upper_case_globals)]
                pub const DATA_INTEGRITY_SYNC: Self = Self::from_bits_retain(1 << 3);
                /// Requests that reads be performed at the same level of integrety
                /// requested for writes. This is similar to `O_RSYNC` in POSIX.
                ///
                /// The precise semantics of this operation have not yet been defined for
                /// WASI. At this time, it should be interpreted as a request, and not a
                /// requirement.
                #[allow(deprecated, non_upper_case_globals)]
                pub const REQUESTED_WRITE_SYNC: Self = Self::from_bits_retain(1 << 4);
                /// Mutating directories mode: Directory contents may be mutated.
                ///
                /// When this flag is unset on a descriptor, operations using the
                /// descriptor which would create, rename, delete, modify the data or
                /// metadata of filesystem objects, or obtain another handle which
                /// would permit any of those, shall fail with `error-code::read-only` if
                /// they would otherwise succeed.
                ///
                /// This may only be set on directories.
                #[allow(deprecated, non_upper_case_globals)]
                pub const MUTATE_DIRECTORY: Self = Self::from_bits_retain(1 << 5);
            }
            impl ::bitflags::Flags for DescriptorFlags {
                const FLAGS: &'static [::bitflags::Flag<DescriptorFlags>] = &[
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("READ", DescriptorFlags::READ)
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("WRITE", DescriptorFlags::WRITE)
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new(
                            "FILE_INTEGRITY_SYNC",
                            DescriptorFlags::FILE_INTEGRITY_SYNC,
                        )
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new(
                            "DATA_INTEGRITY_SYNC",
                            DescriptorFlags::DATA_INTEGRITY_SYNC,
                        )
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new(
                            "REQUESTED_WRITE_SYNC",
                            DescriptorFlags::REQUESTED_WRITE_SYNC,
                        )
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new(
                            "MUTATE_DIRECTORY",
                            DescriptorFlags::MUTATE_DIRECTORY,
                        )
                    },
                ];
                type Bits = u8;
                fn bits(&self) -> u8 {
                    DescriptorFlags::bits(self)
                }
                fn from_bits_retain(bits: u8) -> DescriptorFlags {
                    DescriptorFlags::from_bits_retain(bits)
                }
            }
            #[allow(
                dead_code,
                deprecated,
                unused_doc_comments,
                unused_attributes,
                unused_mut,
                unused_imports,
                non_upper_case_globals,
                clippy::assign_op_pattern,
                clippy::indexing_slicing,
                clippy::same_name_method,
                clippy::iter_without_into_iter,
            )]
            const _: () = {
                #[repr(transparent)]
                pub struct InternalBitFlags(u8);
                #[automatically_derived]
                impl ::core::clone::Clone for InternalBitFlags {
                    #[inline]
                    fn clone(&self) -> InternalBitFlags {
                        let _: ::core::clone::AssertParamIsClone<u8>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for InternalBitFlags {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for InternalBitFlags {
                    #[inline]
                    fn eq(&self, other: &InternalBitFlags) -> bool {
                        self.0 == other.0
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for InternalBitFlags {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::PartialOrd for InternalBitFlags {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &InternalBitFlags,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Ord for InternalBitFlags {
                    #[inline]
                    fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }
                #[automatically_derived]
                impl ::core::hash::Hash for InternalBitFlags {
                    #[inline]
                    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                        ::core::hash::Hash::hash(&self.0, state)
                    }
                }
                impl ::bitflags::__private::PublicFlags for DescriptorFlags {
                    type Primitive = u8;
                    type Internal = InternalBitFlags;
                }
                impl ::bitflags::__private::core::default::Default for InternalBitFlags {
                    #[inline]
                    fn default() -> Self {
                        InternalBitFlags::empty()
                    }
                }
                impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        if self.is_empty() {
                            f.write_fmt(
                                format_args!("{0:#x}", <u8 as ::bitflags::Bits>::EMPTY),
                            )
                        } else {
                            ::bitflags::__private::core::fmt::Display::fmt(self, f)
                        }
                    }
                }
                impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        ::bitflags::parser::to_writer(&DescriptorFlags(*self), f)
                    }
                }
                impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
                    type Err = ::bitflags::parser::ParseError;
                    fn from_str(
                        s: &str,
                    ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                        ::bitflags::parser::from_str::<DescriptorFlags>(s)
                            .map(|flags| flags.0)
                    }
                }
                impl ::bitflags::__private::core::convert::AsRef<u8>
                for InternalBitFlags {
                    fn as_ref(&self) -> &u8 {
                        &self.0
                    }
                }
                impl ::bitflags::__private::core::convert::From<u8>
                for InternalBitFlags {
                    fn from(bits: u8) -> Self {
                        Self::from_bits_retain(bits)
                    }
                }
                #[allow(dead_code, deprecated, unused_attributes)]
                impl InternalBitFlags {
                    /// Get a flags value with all bits unset.
                    #[inline]
                    pub const fn empty() -> Self {
                        { Self(<u8 as ::bitflags::Bits>::EMPTY) }
                    }
                    /// Get a flags value with all known bits set.
                    #[inline]
                    pub const fn all() -> Self {
                        {
                            let mut truncated = <u8 as ::bitflags::Bits>::EMPTY;
                            let mut i = 0;
                            {
                                {
                                    let flag = <DescriptorFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag = <DescriptorFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag = <DescriptorFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag = <DescriptorFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag = <DescriptorFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag = <DescriptorFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            let _ = i;
                            Self::from_bits_retain(truncated)
                        }
                    }
                    /// Get the underlying bits value.
                    ///
                    /// The returned value is exactly the bits set in this flags value.
                    #[inline]
                    pub const fn bits(&self) -> u8 {
                        let f = self;
                        { f.0 }
                    }
                    /// Convert from a bits value.
                    ///
                    /// This method will return `None` if any unknown bits are set.
                    #[inline]
                    pub const fn from_bits(
                        bits: u8,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let bits = bits;
                        {
                            let truncated = Self::from_bits_truncate(bits).0;
                            if truncated == bits {
                                ::bitflags::__private::core::option::Option::Some(
                                    Self(bits),
                                )
                            } else {
                                ::bitflags::__private::core::option::Option::None
                            }
                        }
                    }
                    /// Convert from a bits value, unsetting any unknown bits.
                    #[inline]
                    pub const fn from_bits_truncate(bits: u8) -> Self {
                        let bits = bits;
                        { Self(bits & Self::all().bits()) }
                    }
                    /// Convert from a bits value exactly.
                    #[inline]
                    pub const fn from_bits_retain(bits: u8) -> Self {
                        let bits = bits;
                        { Self(bits) }
                    }
                    /// Get a flags value with the bits of a flag with the given name set.
                    ///
                    /// This method will return `None` if `name` is empty or doesn't
                    /// correspond to any named flag.
                    #[inline]
                    pub fn from_name(
                        name: &str,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let name = name;
                        {
                            {
                                if name == "READ" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(DescriptorFlags::READ.bits()),
                                    );
                                }
                            };
                            {
                                if name == "WRITE" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(DescriptorFlags::WRITE.bits()),
                                    );
                                }
                            };
                            {
                                if name == "FILE_INTEGRITY_SYNC" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(DescriptorFlags::FILE_INTEGRITY_SYNC.bits()),
                                    );
                                }
                            };
                            {
                                if name == "DATA_INTEGRITY_SYNC" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(DescriptorFlags::DATA_INTEGRITY_SYNC.bits()),
                                    );
                                }
                            };
                            {
                                if name == "REQUESTED_WRITE_SYNC" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(DescriptorFlags::REQUESTED_WRITE_SYNC.bits()),
                                    );
                                }
                            };
                            {
                                if name == "MUTATE_DIRECTORY" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(DescriptorFlags::MUTATE_DIRECTORY.bits()),
                                    );
                                }
                            };
                            let _ = name;
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                    /// Whether all bits in this flags value are unset.
                    #[inline]
                    pub const fn is_empty(&self) -> bool {
                        let f = self;
                        { f.bits() == <u8 as ::bitflags::Bits>::EMPTY }
                    }
                    /// Whether all known bits in this flags value are set.
                    #[inline]
                    pub const fn is_all(&self) -> bool {
                        let f = self;
                        { Self::all().bits() | f.bits() == f.bits() }
                    }
                    /// Whether any set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn intersects(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.bits() & other.bits() != <u8 as ::bitflags::Bits>::EMPTY }
                    }
                    /// Whether all set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn contains(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.bits() & other.bits() == other.bits() }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    pub fn insert(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits()).union(other);
                        }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `remove` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    pub fn remove(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits()).difference(other);
                        }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    pub fn toggle(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits())
                                .symmetric_difference(other);
                        }
                    }
                    /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                    #[inline]
                    pub fn set(&mut self, other: Self, value: bool) {
                        let f = self;
                        let other = other;
                        let value = value;
                        {
                            if value {
                                f.insert(other);
                            } else {
                                f.remove(other);
                            }
                        }
                    }
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn intersection(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() & other.bits()) }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn union(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() | other.bits()) }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    #[must_use]
                    pub const fn difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() & !other.bits()) }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn symmetric_difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() ^ other.bits()) }
                    }
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    #[must_use]
                    pub const fn complement(self) -> Self {
                        let f = self;
                        { Self::from_bits_truncate(!f.bits()) }
                    }
                }
                impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor(self, other: InternalBitFlags) -> Self {
                        self.union(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor_assign(&mut self, other: Self) {
                        self.insert(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor(self, other: Self) -> Self {
                        self.symmetric_difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitXorAssign
                for InternalBitFlags {
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor_assign(&mut self, other: Self) {
                        self.toggle(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand(self, other: Self) -> Self {
                        self.intersection(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitAndAssign
                for InternalBitFlags {
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand_assign(&mut self, other: Self) {
                        *self = Self::from_bits_retain(self.bits()).intersection(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
                    type Output = Self;
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub(self, other: Self) -> Self {
                        self.difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub_assign(&mut self, other: Self) {
                        self.remove(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    fn not(self) -> Self {
                        self.complement()
                    }
                }
                impl ::bitflags::__private::core::iter::Extend<InternalBitFlags>
                for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn extend<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(&mut self, iterator: T) {
                        for item in iterator {
                            self.insert(item)
                        }
                    }
                }
                impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags>
                for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn from_iter<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(iterator: T) -> Self {
                        use ::bitflags::__private::core::iter::Extend;
                        let mut result = Self::empty();
                        result.extend(iterator);
                        result
                    }
                }
                impl InternalBitFlags {
                    /// Yield a set of contained flags values.
                    ///
                    /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                    /// will be yielded together as a final flags value.
                    #[inline]
                    pub const fn iter(&self) -> ::bitflags::iter::Iter<DescriptorFlags> {
                        ::bitflags::iter::Iter::__private_const_new(
                            <DescriptorFlags as ::bitflags::Flags>::FLAGS,
                            DescriptorFlags::from_bits_retain(self.bits()),
                            DescriptorFlags::from_bits_retain(self.bits()),
                        )
                    }
                    /// Yield a set of contained named flags values.
                    ///
                    /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                    #[inline]
                    pub const fn iter_names(
                        &self,
                    ) -> ::bitflags::iter::IterNames<DescriptorFlags> {
                        ::bitflags::iter::IterNames::__private_const_new(
                            <DescriptorFlags as ::bitflags::Flags>::FLAGS,
                            DescriptorFlags::from_bits_retain(self.bits()),
                            DescriptorFlags::from_bits_retain(self.bits()),
                        )
                    }
                }
                impl ::bitflags::__private::core::iter::IntoIterator
                for InternalBitFlags {
                    type Item = DescriptorFlags;
                    type IntoIter = ::bitflags::iter::Iter<DescriptorFlags>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
                impl InternalBitFlags {
                    /// Returns a mutable reference to the raw value of the flags currently stored.
                    #[inline]
                    pub fn bits_mut(&mut self) -> &mut u8 {
                        &mut self.0
                    }
                }
                #[allow(dead_code, deprecated, unused_attributes)]
                impl DescriptorFlags {
                    /// Get a flags value with all bits unset.
                    #[inline]
                    pub const fn empty() -> Self {
                        { Self(InternalBitFlags::empty()) }
                    }
                    /// Get a flags value with all known bits set.
                    #[inline]
                    pub const fn all() -> Self {
                        { Self(InternalBitFlags::all()) }
                    }
                    /// Get the underlying bits value.
                    ///
                    /// The returned value is exactly the bits set in this flags value.
                    #[inline]
                    pub const fn bits(&self) -> u8 {
                        let f = self;
                        { f.0.bits() }
                    }
                    /// Convert from a bits value.
                    ///
                    /// This method will return `None` if any unknown bits are set.
                    #[inline]
                    pub const fn from_bits(
                        bits: u8,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let bits = bits;
                        {
                            match InternalBitFlags::from_bits(bits) {
                                ::bitflags::__private::core::option::Option::Some(bits) => {
                                    ::bitflags::__private::core::option::Option::Some(
                                        Self(bits),
                                    )
                                }
                                ::bitflags::__private::core::option::Option::None => {
                                    ::bitflags::__private::core::option::Option::None
                                }
                            }
                        }
                    }
                    /// Convert from a bits value, unsetting any unknown bits.
                    #[inline]
                    pub const fn from_bits_truncate(bits: u8) -> Self {
                        let bits = bits;
                        { Self(InternalBitFlags::from_bits_truncate(bits)) }
                    }
                    /// Convert from a bits value exactly.
                    #[inline]
                    pub const fn from_bits_retain(bits: u8) -> Self {
                        let bits = bits;
                        { Self(InternalBitFlags::from_bits_retain(bits)) }
                    }
                    /// Get a flags value with the bits of a flag with the given name set.
                    ///
                    /// This method will return `None` if `name` is empty or doesn't
                    /// correspond to any named flag.
                    #[inline]
                    pub fn from_name(
                        name: &str,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let name = name;
                        {
                            match InternalBitFlags::from_name(name) {
                                ::bitflags::__private::core::option::Option::Some(bits) => {
                                    ::bitflags::__private::core::option::Option::Some(
                                        Self(bits),
                                    )
                                }
                                ::bitflags::__private::core::option::Option::None => {
                                    ::bitflags::__private::core::option::Option::None
                                }
                            }
                        }
                    }
                    /// Whether all bits in this flags value are unset.
                    #[inline]
                    pub const fn is_empty(&self) -> bool {
                        let f = self;
                        { f.0.is_empty() }
                    }
                    /// Whether all known bits in this flags value are set.
                    #[inline]
                    pub const fn is_all(&self) -> bool {
                        let f = self;
                        { f.0.is_all() }
                    }
                    /// Whether any set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn intersects(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.0.intersects(other.0) }
                    }
                    /// Whether all set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn contains(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.0.contains(other.0) }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    pub fn insert(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        { f.0.insert(other.0) }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `remove` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    pub fn remove(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        { f.0.remove(other.0) }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    pub fn toggle(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        { f.0.toggle(other.0) }
                    }
                    /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                    #[inline]
                    pub fn set(&mut self, other: Self, value: bool) {
                        let f = self;
                        let other = other;
                        let value = value;
                        { f.0.set(other.0, value) }
                    }
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn intersection(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.intersection(other.0)) }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn union(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.union(other.0)) }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    #[must_use]
                    pub const fn difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.difference(other.0)) }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn symmetric_difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.symmetric_difference(other.0)) }
                    }
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    #[must_use]
                    pub const fn complement(self) -> Self {
                        let f = self;
                        { Self(f.0.complement()) }
                    }
                }
                impl ::bitflags::__private::core::fmt::Binary for DescriptorFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::Octal for DescriptorFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::LowerHex for DescriptorFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::UpperHex for DescriptorFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOr for DescriptorFlags {
                    type Output = Self;
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor(self, other: DescriptorFlags) -> Self {
                        self.union(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOrAssign for DescriptorFlags {
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor_assign(&mut self, other: Self) {
                        self.insert(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitXor for DescriptorFlags {
                    type Output = Self;
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor(self, other: Self) -> Self {
                        self.symmetric_difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitXorAssign for DescriptorFlags {
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor_assign(&mut self, other: Self) {
                        self.toggle(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitAnd for DescriptorFlags {
                    type Output = Self;
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand(self, other: Self) -> Self {
                        self.intersection(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitAndAssign for DescriptorFlags {
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand_assign(&mut self, other: Self) {
                        *self = Self::from_bits_retain(self.bits()).intersection(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Sub for DescriptorFlags {
                    type Output = Self;
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub(self, other: Self) -> Self {
                        self.difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::SubAssign for DescriptorFlags {
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub_assign(&mut self, other: Self) {
                        self.remove(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Not for DescriptorFlags {
                    type Output = Self;
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    fn not(self) -> Self {
                        self.complement()
                    }
                }
                impl ::bitflags::__private::core::iter::Extend<DescriptorFlags>
                for DescriptorFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn extend<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(&mut self, iterator: T) {
                        for item in iterator {
                            self.insert(item)
                        }
                    }
                }
                impl ::bitflags::__private::core::iter::FromIterator<DescriptorFlags>
                for DescriptorFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn from_iter<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(iterator: T) -> Self {
                        use ::bitflags::__private::core::iter::Extend;
                        let mut result = Self::empty();
                        result.extend(iterator);
                        result
                    }
                }
                impl DescriptorFlags {
                    /// Yield a set of contained flags values.
                    ///
                    /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                    /// will be yielded together as a final flags value.
                    #[inline]
                    pub const fn iter(&self) -> ::bitflags::iter::Iter<DescriptorFlags> {
                        ::bitflags::iter::Iter::__private_const_new(
                            <DescriptorFlags as ::bitflags::Flags>::FLAGS,
                            DescriptorFlags::from_bits_retain(self.bits()),
                            DescriptorFlags::from_bits_retain(self.bits()),
                        )
                    }
                    /// Yield a set of contained named flags values.
                    ///
                    /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                    #[inline]
                    pub const fn iter_names(
                        &self,
                    ) -> ::bitflags::iter::IterNames<DescriptorFlags> {
                        ::bitflags::iter::IterNames::__private_const_new(
                            <DescriptorFlags as ::bitflags::Flags>::FLAGS,
                            DescriptorFlags::from_bits_retain(self.bits()),
                            DescriptorFlags::from_bits_retain(self.bits()),
                        )
                    }
                }
                impl ::bitflags::__private::core::iter::IntoIterator
                for DescriptorFlags {
                    type Item = DescriptorFlags;
                    type IntoIter = ::bitflags::iter::Iter<DescriptorFlags>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
            };
            /// Flags determining the method of how paths are resolved.
            pub struct PathFlags(
                <PathFlags as ::bitflags::__private::PublicFlags>::Internal,
            );
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for PathFlags {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for PathFlags {
                #[inline]
                fn eq(&self, other: &PathFlags) -> bool {
                    self.0 == other.0
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for PathFlags {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <PathFlags as ::bitflags::__private::PublicFlags>::Internal,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for PathFlags {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &PathFlags,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for PathFlags {
                #[inline]
                fn cmp(&self, other: &PathFlags) -> ::core::cmp::Ordering {
                    ::core::cmp::Ord::cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for PathFlags {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.0, state)
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PathFlags {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PathFlags",
                        &&self.0,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for PathFlags {
                #[inline]
                fn clone(&self) -> PathFlags {
                    let _: ::core::clone::AssertParamIsClone<
                        <PathFlags as ::bitflags::__private::PublicFlags>::Internal,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for PathFlags {}
            impl PathFlags {
                /// As long as the resolved path corresponds to a symbolic link, it is
                /// expanded.
                #[allow(deprecated, non_upper_case_globals)]
                pub const SYMLINK_FOLLOW: Self = Self::from_bits_retain(1 << 0);
            }
            impl ::bitflags::Flags for PathFlags {
                const FLAGS: &'static [::bitflags::Flag<PathFlags>] = &[
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new(
                            "SYMLINK_FOLLOW",
                            PathFlags::SYMLINK_FOLLOW,
                        )
                    },
                ];
                type Bits = u8;
                fn bits(&self) -> u8 {
                    PathFlags::bits(self)
                }
                fn from_bits_retain(bits: u8) -> PathFlags {
                    PathFlags::from_bits_retain(bits)
                }
            }
            #[allow(
                dead_code,
                deprecated,
                unused_doc_comments,
                unused_attributes,
                unused_mut,
                unused_imports,
                non_upper_case_globals,
                clippy::assign_op_pattern,
                clippy::indexing_slicing,
                clippy::same_name_method,
                clippy::iter_without_into_iter,
            )]
            const _: () = {
                #[repr(transparent)]
                pub struct InternalBitFlags(u8);
                #[automatically_derived]
                impl ::core::clone::Clone for InternalBitFlags {
                    #[inline]
                    fn clone(&self) -> InternalBitFlags {
                        let _: ::core::clone::AssertParamIsClone<u8>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for InternalBitFlags {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for InternalBitFlags {
                    #[inline]
                    fn eq(&self, other: &InternalBitFlags) -> bool {
                        self.0 == other.0
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for InternalBitFlags {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::PartialOrd for InternalBitFlags {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &InternalBitFlags,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Ord for InternalBitFlags {
                    #[inline]
                    fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }
                #[automatically_derived]
                impl ::core::hash::Hash for InternalBitFlags {
                    #[inline]
                    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                        ::core::hash::Hash::hash(&self.0, state)
                    }
                }
                impl ::bitflags::__private::PublicFlags for PathFlags {
                    type Primitive = u8;
                    type Internal = InternalBitFlags;
                }
                impl ::bitflags::__private::core::default::Default for InternalBitFlags {
                    #[inline]
                    fn default() -> Self {
                        InternalBitFlags::empty()
                    }
                }
                impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        if self.is_empty() {
                            f.write_fmt(
                                format_args!("{0:#x}", <u8 as ::bitflags::Bits>::EMPTY),
                            )
                        } else {
                            ::bitflags::__private::core::fmt::Display::fmt(self, f)
                        }
                    }
                }
                impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        ::bitflags::parser::to_writer(&PathFlags(*self), f)
                    }
                }
                impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
                    type Err = ::bitflags::parser::ParseError;
                    fn from_str(
                        s: &str,
                    ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                        ::bitflags::parser::from_str::<PathFlags>(s).map(|flags| flags.0)
                    }
                }
                impl ::bitflags::__private::core::convert::AsRef<u8>
                for InternalBitFlags {
                    fn as_ref(&self) -> &u8 {
                        &self.0
                    }
                }
                impl ::bitflags::__private::core::convert::From<u8>
                for InternalBitFlags {
                    fn from(bits: u8) -> Self {
                        Self::from_bits_retain(bits)
                    }
                }
                #[allow(dead_code, deprecated, unused_attributes)]
                impl InternalBitFlags {
                    /// Get a flags value with all bits unset.
                    #[inline]
                    pub const fn empty() -> Self {
                        { Self(<u8 as ::bitflags::Bits>::EMPTY) }
                    }
                    /// Get a flags value with all known bits set.
                    #[inline]
                    pub const fn all() -> Self {
                        {
                            let mut truncated = <u8 as ::bitflags::Bits>::EMPTY;
                            let mut i = 0;
                            {
                                {
                                    let flag = <PathFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            let _ = i;
                            Self::from_bits_retain(truncated)
                        }
                    }
                    /// Get the underlying bits value.
                    ///
                    /// The returned value is exactly the bits set in this flags value.
                    #[inline]
                    pub const fn bits(&self) -> u8 {
                        let f = self;
                        { f.0 }
                    }
                    /// Convert from a bits value.
                    ///
                    /// This method will return `None` if any unknown bits are set.
                    #[inline]
                    pub const fn from_bits(
                        bits: u8,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let bits = bits;
                        {
                            let truncated = Self::from_bits_truncate(bits).0;
                            if truncated == bits {
                                ::bitflags::__private::core::option::Option::Some(
                                    Self(bits),
                                )
                            } else {
                                ::bitflags::__private::core::option::Option::None
                            }
                        }
                    }
                    /// Convert from a bits value, unsetting any unknown bits.
                    #[inline]
                    pub const fn from_bits_truncate(bits: u8) -> Self {
                        let bits = bits;
                        { Self(bits & Self::all().bits()) }
                    }
                    /// Convert from a bits value exactly.
                    #[inline]
                    pub const fn from_bits_retain(bits: u8) -> Self {
                        let bits = bits;
                        { Self(bits) }
                    }
                    /// Get a flags value with the bits of a flag with the given name set.
                    ///
                    /// This method will return `None` if `name` is empty or doesn't
                    /// correspond to any named flag.
                    #[inline]
                    pub fn from_name(
                        name: &str,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let name = name;
                        {
                            {
                                if name == "SYMLINK_FOLLOW" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(PathFlags::SYMLINK_FOLLOW.bits()),
                                    );
                                }
                            };
                            let _ = name;
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                    /// Whether all bits in this flags value are unset.
                    #[inline]
                    pub const fn is_empty(&self) -> bool {
                        let f = self;
                        { f.bits() == <u8 as ::bitflags::Bits>::EMPTY }
                    }
                    /// Whether all known bits in this flags value are set.
                    #[inline]
                    pub const fn is_all(&self) -> bool {
                        let f = self;
                        { Self::all().bits() | f.bits() == f.bits() }
                    }
                    /// Whether any set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn intersects(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.bits() & other.bits() != <u8 as ::bitflags::Bits>::EMPTY }
                    }
                    /// Whether all set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn contains(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.bits() & other.bits() == other.bits() }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    pub fn insert(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits()).union(other);
                        }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `remove` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    pub fn remove(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits()).difference(other);
                        }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    pub fn toggle(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits())
                                .symmetric_difference(other);
                        }
                    }
                    /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                    #[inline]
                    pub fn set(&mut self, other: Self, value: bool) {
                        let f = self;
                        let other = other;
                        let value = value;
                        {
                            if value {
                                f.insert(other);
                            } else {
                                f.remove(other);
                            }
                        }
                    }
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn intersection(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() & other.bits()) }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn union(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() | other.bits()) }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    #[must_use]
                    pub const fn difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() & !other.bits()) }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn symmetric_difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() ^ other.bits()) }
                    }
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    #[must_use]
                    pub const fn complement(self) -> Self {
                        let f = self;
                        { Self::from_bits_truncate(!f.bits()) }
                    }
                }
                impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor(self, other: InternalBitFlags) -> Self {
                        self.union(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor_assign(&mut self, other: Self) {
                        self.insert(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor(self, other: Self) -> Self {
                        self.symmetric_difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitXorAssign
                for InternalBitFlags {
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor_assign(&mut self, other: Self) {
                        self.toggle(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand(self, other: Self) -> Self {
                        self.intersection(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitAndAssign
                for InternalBitFlags {
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand_assign(&mut self, other: Self) {
                        *self = Self::from_bits_retain(self.bits()).intersection(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
                    type Output = Self;
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub(self, other: Self) -> Self {
                        self.difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub_assign(&mut self, other: Self) {
                        self.remove(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    fn not(self) -> Self {
                        self.complement()
                    }
                }
                impl ::bitflags::__private::core::iter::Extend<InternalBitFlags>
                for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn extend<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(&mut self, iterator: T) {
                        for item in iterator {
                            self.insert(item)
                        }
                    }
                }
                impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags>
                for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn from_iter<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(iterator: T) -> Self {
                        use ::bitflags::__private::core::iter::Extend;
                        let mut result = Self::empty();
                        result.extend(iterator);
                        result
                    }
                }
                impl InternalBitFlags {
                    /// Yield a set of contained flags values.
                    ///
                    /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                    /// will be yielded together as a final flags value.
                    #[inline]
                    pub const fn iter(&self) -> ::bitflags::iter::Iter<PathFlags> {
                        ::bitflags::iter::Iter::__private_const_new(
                            <PathFlags as ::bitflags::Flags>::FLAGS,
                            PathFlags::from_bits_retain(self.bits()),
                            PathFlags::from_bits_retain(self.bits()),
                        )
                    }
                    /// Yield a set of contained named flags values.
                    ///
                    /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                    #[inline]
                    pub const fn iter_names(
                        &self,
                    ) -> ::bitflags::iter::IterNames<PathFlags> {
                        ::bitflags::iter::IterNames::__private_const_new(
                            <PathFlags as ::bitflags::Flags>::FLAGS,
                            PathFlags::from_bits_retain(self.bits()),
                            PathFlags::from_bits_retain(self.bits()),
                        )
                    }
                }
                impl ::bitflags::__private::core::iter::IntoIterator
                for InternalBitFlags {
                    type Item = PathFlags;
                    type IntoIter = ::bitflags::iter::Iter<PathFlags>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
                impl InternalBitFlags {
                    /// Returns a mutable reference to the raw value of the flags currently stored.
                    #[inline]
                    pub fn bits_mut(&mut self) -> &mut u8 {
                        &mut self.0
                    }
                }
                #[allow(dead_code, deprecated, unused_attributes)]
                impl PathFlags {
                    /// Get a flags value with all bits unset.
                    #[inline]
                    pub const fn empty() -> Self {
                        { Self(InternalBitFlags::empty()) }
                    }
                    /// Get a flags value with all known bits set.
                    #[inline]
                    pub const fn all() -> Self {
                        { Self(InternalBitFlags::all()) }
                    }
                    /// Get the underlying bits value.
                    ///
                    /// The returned value is exactly the bits set in this flags value.
                    #[inline]
                    pub const fn bits(&self) -> u8 {
                        let f = self;
                        { f.0.bits() }
                    }
                    /// Convert from a bits value.
                    ///
                    /// This method will return `None` if any unknown bits are set.
                    #[inline]
                    pub const fn from_bits(
                        bits: u8,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let bits = bits;
                        {
                            match InternalBitFlags::from_bits(bits) {
                                ::bitflags::__private::core::option::Option::Some(bits) => {
                                    ::bitflags::__private::core::option::Option::Some(
                                        Self(bits),
                                    )
                                }
                                ::bitflags::__private::core::option::Option::None => {
                                    ::bitflags::__private::core::option::Option::None
                                }
                            }
                        }
                    }
                    /// Convert from a bits value, unsetting any unknown bits.
                    #[inline]
                    pub const fn from_bits_truncate(bits: u8) -> Self {
                        let bits = bits;
                        { Self(InternalBitFlags::from_bits_truncate(bits)) }
                    }
                    /// Convert from a bits value exactly.
                    #[inline]
                    pub const fn from_bits_retain(bits: u8) -> Self {
                        let bits = bits;
                        { Self(InternalBitFlags::from_bits_retain(bits)) }
                    }
                    /// Get a flags value with the bits of a flag with the given name set.
                    ///
                    /// This method will return `None` if `name` is empty or doesn't
                    /// correspond to any named flag.
                    #[inline]
                    pub fn from_name(
                        name: &str,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let name = name;
                        {
                            match InternalBitFlags::from_name(name) {
                                ::bitflags::__private::core::option::Option::Some(bits) => {
                                    ::bitflags::__private::core::option::Option::Some(
                                        Self(bits),
                                    )
                                }
                                ::bitflags::__private::core::option::Option::None => {
                                    ::bitflags::__private::core::option::Option::None
                                }
                            }
                        }
                    }
                    /// Whether all bits in this flags value are unset.
                    #[inline]
                    pub const fn is_empty(&self) -> bool {
                        let f = self;
                        { f.0.is_empty() }
                    }
                    /// Whether all known bits in this flags value are set.
                    #[inline]
                    pub const fn is_all(&self) -> bool {
                        let f = self;
                        { f.0.is_all() }
                    }
                    /// Whether any set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn intersects(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.0.intersects(other.0) }
                    }
                    /// Whether all set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn contains(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.0.contains(other.0) }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    pub fn insert(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        { f.0.insert(other.0) }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `remove` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    pub fn remove(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        { f.0.remove(other.0) }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    pub fn toggle(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        { f.0.toggle(other.0) }
                    }
                    /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                    #[inline]
                    pub fn set(&mut self, other: Self, value: bool) {
                        let f = self;
                        let other = other;
                        let value = value;
                        { f.0.set(other.0, value) }
                    }
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn intersection(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.intersection(other.0)) }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn union(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.union(other.0)) }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    #[must_use]
                    pub const fn difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.difference(other.0)) }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn symmetric_difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.symmetric_difference(other.0)) }
                    }
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    #[must_use]
                    pub const fn complement(self) -> Self {
                        let f = self;
                        { Self(f.0.complement()) }
                    }
                }
                impl ::bitflags::__private::core::fmt::Binary for PathFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::Octal for PathFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::LowerHex for PathFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::UpperHex for PathFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOr for PathFlags {
                    type Output = Self;
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor(self, other: PathFlags) -> Self {
                        self.union(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOrAssign for PathFlags {
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor_assign(&mut self, other: Self) {
                        self.insert(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitXor for PathFlags {
                    type Output = Self;
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor(self, other: Self) -> Self {
                        self.symmetric_difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitXorAssign for PathFlags {
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor_assign(&mut self, other: Self) {
                        self.toggle(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitAnd for PathFlags {
                    type Output = Self;
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand(self, other: Self) -> Self {
                        self.intersection(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitAndAssign for PathFlags {
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand_assign(&mut self, other: Self) {
                        *self = Self::from_bits_retain(self.bits()).intersection(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Sub for PathFlags {
                    type Output = Self;
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub(self, other: Self) -> Self {
                        self.difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::SubAssign for PathFlags {
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub_assign(&mut self, other: Self) {
                        self.remove(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Not for PathFlags {
                    type Output = Self;
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    fn not(self) -> Self {
                        self.complement()
                    }
                }
                impl ::bitflags::__private::core::iter::Extend<PathFlags> for PathFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn extend<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(&mut self, iterator: T) {
                        for item in iterator {
                            self.insert(item)
                        }
                    }
                }
                impl ::bitflags::__private::core::iter::FromIterator<PathFlags>
                for PathFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn from_iter<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(iterator: T) -> Self {
                        use ::bitflags::__private::core::iter::Extend;
                        let mut result = Self::empty();
                        result.extend(iterator);
                        result
                    }
                }
                impl PathFlags {
                    /// Yield a set of contained flags values.
                    ///
                    /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                    /// will be yielded together as a final flags value.
                    #[inline]
                    pub const fn iter(&self) -> ::bitflags::iter::Iter<PathFlags> {
                        ::bitflags::iter::Iter::__private_const_new(
                            <PathFlags as ::bitflags::Flags>::FLAGS,
                            PathFlags::from_bits_retain(self.bits()),
                            PathFlags::from_bits_retain(self.bits()),
                        )
                    }
                    /// Yield a set of contained named flags values.
                    ///
                    /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                    #[inline]
                    pub const fn iter_names(
                        &self,
                    ) -> ::bitflags::iter::IterNames<PathFlags> {
                        ::bitflags::iter::IterNames::__private_const_new(
                            <PathFlags as ::bitflags::Flags>::FLAGS,
                            PathFlags::from_bits_retain(self.bits()),
                            PathFlags::from_bits_retain(self.bits()),
                        )
                    }
                }
                impl ::bitflags::__private::core::iter::IntoIterator for PathFlags {
                    type Item = PathFlags;
                    type IntoIter = ::bitflags::iter::Iter<PathFlags>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
            };
            /// Open flags used by `open-at`.
            pub struct OpenFlags(
                <OpenFlags as ::bitflags::__private::PublicFlags>::Internal,
            );
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for OpenFlags {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for OpenFlags {
                #[inline]
                fn eq(&self, other: &OpenFlags) -> bool {
                    self.0 == other.0
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for OpenFlags {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <OpenFlags as ::bitflags::__private::PublicFlags>::Internal,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for OpenFlags {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &OpenFlags,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for OpenFlags {
                #[inline]
                fn cmp(&self, other: &OpenFlags) -> ::core::cmp::Ordering {
                    ::core::cmp::Ord::cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for OpenFlags {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.0, state)
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for OpenFlags {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "OpenFlags",
                        &&self.0,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for OpenFlags {
                #[inline]
                fn clone(&self) -> OpenFlags {
                    let _: ::core::clone::AssertParamIsClone<
                        <OpenFlags as ::bitflags::__private::PublicFlags>::Internal,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for OpenFlags {}
            impl OpenFlags {
                /// Create file if it does not exist, similar to `O_CREAT` in POSIX.
                #[allow(deprecated, non_upper_case_globals)]
                pub const CREATE: Self = Self::from_bits_retain(1 << 0);
                /// Fail if not a directory, similar to `O_DIRECTORY` in POSIX.
                #[allow(deprecated, non_upper_case_globals)]
                pub const DIRECTORY: Self = Self::from_bits_retain(1 << 1);
                /// Fail if file already exists, similar to `O_EXCL` in POSIX.
                #[allow(deprecated, non_upper_case_globals)]
                pub const EXCLUSIVE: Self = Self::from_bits_retain(1 << 2);
                /// Truncate file to size 0, similar to `O_TRUNC` in POSIX.
                #[allow(deprecated, non_upper_case_globals)]
                pub const TRUNCATE: Self = Self::from_bits_retain(1 << 3);
            }
            impl ::bitflags::Flags for OpenFlags {
                const FLAGS: &'static [::bitflags::Flag<OpenFlags>] = &[
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("CREATE", OpenFlags::CREATE)
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("DIRECTORY", OpenFlags::DIRECTORY)
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("EXCLUSIVE", OpenFlags::EXCLUSIVE)
                    },
                    {
                        #[allow(deprecated, non_upper_case_globals)]
                        ::bitflags::Flag::new("TRUNCATE", OpenFlags::TRUNCATE)
                    },
                ];
                type Bits = u8;
                fn bits(&self) -> u8 {
                    OpenFlags::bits(self)
                }
                fn from_bits_retain(bits: u8) -> OpenFlags {
                    OpenFlags::from_bits_retain(bits)
                }
            }
            #[allow(
                dead_code,
                deprecated,
                unused_doc_comments,
                unused_attributes,
                unused_mut,
                unused_imports,
                non_upper_case_globals,
                clippy::assign_op_pattern,
                clippy::indexing_slicing,
                clippy::same_name_method,
                clippy::iter_without_into_iter,
            )]
            const _: () = {
                #[repr(transparent)]
                pub struct InternalBitFlags(u8);
                #[automatically_derived]
                impl ::core::clone::Clone for InternalBitFlags {
                    #[inline]
                    fn clone(&self) -> InternalBitFlags {
                        let _: ::core::clone::AssertParamIsClone<u8>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for InternalBitFlags {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for InternalBitFlags {
                    #[inline]
                    fn eq(&self, other: &InternalBitFlags) -> bool {
                        self.0 == other.0
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for InternalBitFlags {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::PartialOrd for InternalBitFlags {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &InternalBitFlags,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Ord for InternalBitFlags {
                    #[inline]
                    fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }
                #[automatically_derived]
                impl ::core::hash::Hash for InternalBitFlags {
                    #[inline]
                    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                        ::core::hash::Hash::hash(&self.0, state)
                    }
                }
                impl ::bitflags::__private::PublicFlags for OpenFlags {
                    type Primitive = u8;
                    type Internal = InternalBitFlags;
                }
                impl ::bitflags::__private::core::default::Default for InternalBitFlags {
                    #[inline]
                    fn default() -> Self {
                        InternalBitFlags::empty()
                    }
                }
                impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        if self.is_empty() {
                            f.write_fmt(
                                format_args!("{0:#x}", <u8 as ::bitflags::Bits>::EMPTY),
                            )
                        } else {
                            ::bitflags::__private::core::fmt::Display::fmt(self, f)
                        }
                    }
                }
                impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        ::bitflags::parser::to_writer(&OpenFlags(*self), f)
                    }
                }
                impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
                    type Err = ::bitflags::parser::ParseError;
                    fn from_str(
                        s: &str,
                    ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                        ::bitflags::parser::from_str::<OpenFlags>(s).map(|flags| flags.0)
                    }
                }
                impl ::bitflags::__private::core::convert::AsRef<u8>
                for InternalBitFlags {
                    fn as_ref(&self) -> &u8 {
                        &self.0
                    }
                }
                impl ::bitflags::__private::core::convert::From<u8>
                for InternalBitFlags {
                    fn from(bits: u8) -> Self {
                        Self::from_bits_retain(bits)
                    }
                }
                #[allow(dead_code, deprecated, unused_attributes)]
                impl InternalBitFlags {
                    /// Get a flags value with all bits unset.
                    #[inline]
                    pub const fn empty() -> Self {
                        { Self(<u8 as ::bitflags::Bits>::EMPTY) }
                    }
                    /// Get a flags value with all known bits set.
                    #[inline]
                    pub const fn all() -> Self {
                        {
                            let mut truncated = <u8 as ::bitflags::Bits>::EMPTY;
                            let mut i = 0;
                            {
                                {
                                    let flag = <OpenFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag = <OpenFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag = <OpenFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            {
                                {
                                    let flag = <OpenFlags as ::bitflags::Flags>::FLAGS[i]
                                        .value()
                                        .bits();
                                    truncated = truncated | flag;
                                    i += 1;
                                }
                            };
                            let _ = i;
                            Self::from_bits_retain(truncated)
                        }
                    }
                    /// Get the underlying bits value.
                    ///
                    /// The returned value is exactly the bits set in this flags value.
                    #[inline]
                    pub const fn bits(&self) -> u8 {
                        let f = self;
                        { f.0 }
                    }
                    /// Convert from a bits value.
                    ///
                    /// This method will return `None` if any unknown bits are set.
                    #[inline]
                    pub const fn from_bits(
                        bits: u8,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let bits = bits;
                        {
                            let truncated = Self::from_bits_truncate(bits).0;
                            if truncated == bits {
                                ::bitflags::__private::core::option::Option::Some(
                                    Self(bits),
                                )
                            } else {
                                ::bitflags::__private::core::option::Option::None
                            }
                        }
                    }
                    /// Convert from a bits value, unsetting any unknown bits.
                    #[inline]
                    pub const fn from_bits_truncate(bits: u8) -> Self {
                        let bits = bits;
                        { Self(bits & Self::all().bits()) }
                    }
                    /// Convert from a bits value exactly.
                    #[inline]
                    pub const fn from_bits_retain(bits: u8) -> Self {
                        let bits = bits;
                        { Self(bits) }
                    }
                    /// Get a flags value with the bits of a flag with the given name set.
                    ///
                    /// This method will return `None` if `name` is empty or doesn't
                    /// correspond to any named flag.
                    #[inline]
                    pub fn from_name(
                        name: &str,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let name = name;
                        {
                            {
                                if name == "CREATE" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(OpenFlags::CREATE.bits()),
                                    );
                                }
                            };
                            {
                                if name == "DIRECTORY" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(OpenFlags::DIRECTORY.bits()),
                                    );
                                }
                            };
                            {
                                if name == "EXCLUSIVE" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(OpenFlags::EXCLUSIVE.bits()),
                                    );
                                }
                            };
                            {
                                if name == "TRUNCATE" {
                                    return ::bitflags::__private::core::option::Option::Some(
                                        Self(OpenFlags::TRUNCATE.bits()),
                                    );
                                }
                            };
                            let _ = name;
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                    /// Whether all bits in this flags value are unset.
                    #[inline]
                    pub const fn is_empty(&self) -> bool {
                        let f = self;
                        { f.bits() == <u8 as ::bitflags::Bits>::EMPTY }
                    }
                    /// Whether all known bits in this flags value are set.
                    #[inline]
                    pub const fn is_all(&self) -> bool {
                        let f = self;
                        { Self::all().bits() | f.bits() == f.bits() }
                    }
                    /// Whether any set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn intersects(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.bits() & other.bits() != <u8 as ::bitflags::Bits>::EMPTY }
                    }
                    /// Whether all set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn contains(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.bits() & other.bits() == other.bits() }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    pub fn insert(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits()).union(other);
                        }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `remove` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    pub fn remove(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits()).difference(other);
                        }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    pub fn toggle(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        {
                            *f = Self::from_bits_retain(f.bits())
                                .symmetric_difference(other);
                        }
                    }
                    /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                    #[inline]
                    pub fn set(&mut self, other: Self, value: bool) {
                        let f = self;
                        let other = other;
                        let value = value;
                        {
                            if value {
                                f.insert(other);
                            } else {
                                f.remove(other);
                            }
                        }
                    }
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn intersection(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() & other.bits()) }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn union(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() | other.bits()) }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    #[must_use]
                    pub const fn difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() & !other.bits()) }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn symmetric_difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self::from_bits_retain(f.bits() ^ other.bits()) }
                    }
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    #[must_use]
                    pub const fn complement(self) -> Self {
                        let f = self;
                        { Self::from_bits_truncate(!f.bits()) }
                    }
                }
                impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor(self, other: InternalBitFlags) -> Self {
                        self.union(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor_assign(&mut self, other: Self) {
                        self.insert(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor(self, other: Self) -> Self {
                        self.symmetric_difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitXorAssign
                for InternalBitFlags {
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor_assign(&mut self, other: Self) {
                        self.toggle(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand(self, other: Self) -> Self {
                        self.intersection(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitAndAssign
                for InternalBitFlags {
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand_assign(&mut self, other: Self) {
                        *self = Self::from_bits_retain(self.bits()).intersection(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
                    type Output = Self;
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub(self, other: Self) -> Self {
                        self.difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub_assign(&mut self, other: Self) {
                        self.remove(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
                    type Output = Self;
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    fn not(self) -> Self {
                        self.complement()
                    }
                }
                impl ::bitflags::__private::core::iter::Extend<InternalBitFlags>
                for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn extend<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(&mut self, iterator: T) {
                        for item in iterator {
                            self.insert(item)
                        }
                    }
                }
                impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags>
                for InternalBitFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn from_iter<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(iterator: T) -> Self {
                        use ::bitflags::__private::core::iter::Extend;
                        let mut result = Self::empty();
                        result.extend(iterator);
                        result
                    }
                }
                impl InternalBitFlags {
                    /// Yield a set of contained flags values.
                    ///
                    /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                    /// will be yielded together as a final flags value.
                    #[inline]
                    pub const fn iter(&self) -> ::bitflags::iter::Iter<OpenFlags> {
                        ::bitflags::iter::Iter::__private_const_new(
                            <OpenFlags as ::bitflags::Flags>::FLAGS,
                            OpenFlags::from_bits_retain(self.bits()),
                            OpenFlags::from_bits_retain(self.bits()),
                        )
                    }
                    /// Yield a set of contained named flags values.
                    ///
                    /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                    #[inline]
                    pub const fn iter_names(
                        &self,
                    ) -> ::bitflags::iter::IterNames<OpenFlags> {
                        ::bitflags::iter::IterNames::__private_const_new(
                            <OpenFlags as ::bitflags::Flags>::FLAGS,
                            OpenFlags::from_bits_retain(self.bits()),
                            OpenFlags::from_bits_retain(self.bits()),
                        )
                    }
                }
                impl ::bitflags::__private::core::iter::IntoIterator
                for InternalBitFlags {
                    type Item = OpenFlags;
                    type IntoIter = ::bitflags::iter::Iter<OpenFlags>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
                impl InternalBitFlags {
                    /// Returns a mutable reference to the raw value of the flags currently stored.
                    #[inline]
                    pub fn bits_mut(&mut self) -> &mut u8 {
                        &mut self.0
                    }
                }
                #[allow(dead_code, deprecated, unused_attributes)]
                impl OpenFlags {
                    /// Get a flags value with all bits unset.
                    #[inline]
                    pub const fn empty() -> Self {
                        { Self(InternalBitFlags::empty()) }
                    }
                    /// Get a flags value with all known bits set.
                    #[inline]
                    pub const fn all() -> Self {
                        { Self(InternalBitFlags::all()) }
                    }
                    /// Get the underlying bits value.
                    ///
                    /// The returned value is exactly the bits set in this flags value.
                    #[inline]
                    pub const fn bits(&self) -> u8 {
                        let f = self;
                        { f.0.bits() }
                    }
                    /// Convert from a bits value.
                    ///
                    /// This method will return `None` if any unknown bits are set.
                    #[inline]
                    pub const fn from_bits(
                        bits: u8,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let bits = bits;
                        {
                            match InternalBitFlags::from_bits(bits) {
                                ::bitflags::__private::core::option::Option::Some(bits) => {
                                    ::bitflags::__private::core::option::Option::Some(
                                        Self(bits),
                                    )
                                }
                                ::bitflags::__private::core::option::Option::None => {
                                    ::bitflags::__private::core::option::Option::None
                                }
                            }
                        }
                    }
                    /// Convert from a bits value, unsetting any unknown bits.
                    #[inline]
                    pub const fn from_bits_truncate(bits: u8) -> Self {
                        let bits = bits;
                        { Self(InternalBitFlags::from_bits_truncate(bits)) }
                    }
                    /// Convert from a bits value exactly.
                    #[inline]
                    pub const fn from_bits_retain(bits: u8) -> Self {
                        let bits = bits;
                        { Self(InternalBitFlags::from_bits_retain(bits)) }
                    }
                    /// Get a flags value with the bits of a flag with the given name set.
                    ///
                    /// This method will return `None` if `name` is empty or doesn't
                    /// correspond to any named flag.
                    #[inline]
                    pub fn from_name(
                        name: &str,
                    ) -> ::bitflags::__private::core::option::Option<Self> {
                        let name = name;
                        {
                            match InternalBitFlags::from_name(name) {
                                ::bitflags::__private::core::option::Option::Some(bits) => {
                                    ::bitflags::__private::core::option::Option::Some(
                                        Self(bits),
                                    )
                                }
                                ::bitflags::__private::core::option::Option::None => {
                                    ::bitflags::__private::core::option::Option::None
                                }
                            }
                        }
                    }
                    /// Whether all bits in this flags value are unset.
                    #[inline]
                    pub const fn is_empty(&self) -> bool {
                        let f = self;
                        { f.0.is_empty() }
                    }
                    /// Whether all known bits in this flags value are set.
                    #[inline]
                    pub const fn is_all(&self) -> bool {
                        let f = self;
                        { f.0.is_all() }
                    }
                    /// Whether any set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn intersects(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.0.intersects(other.0) }
                    }
                    /// Whether all set bits in a source flags value are also set in a target flags value.
                    #[inline]
                    pub const fn contains(&self, other: Self) -> bool {
                        let f = self;
                        let other = other;
                        { f.0.contains(other.0) }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    pub fn insert(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        { f.0.insert(other.0) }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `remove` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    pub fn remove(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        { f.0.remove(other.0) }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    pub fn toggle(&mut self, other: Self) {
                        let f = self;
                        let other = other;
                        { f.0.toggle(other.0) }
                    }
                    /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                    #[inline]
                    pub fn set(&mut self, other: Self, value: bool) {
                        let f = self;
                        let other = other;
                        let value = value;
                        { f.0.set(other.0, value) }
                    }
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn intersection(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.intersection(other.0)) }
                    }
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn union(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.union(other.0)) }
                    }
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    #[must_use]
                    pub const fn difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.difference(other.0)) }
                    }
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    #[must_use]
                    pub const fn symmetric_difference(self, other: Self) -> Self {
                        let f = self;
                        let other = other;
                        { Self(f.0.symmetric_difference(other.0)) }
                    }
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    #[must_use]
                    pub const fn complement(self) -> Self {
                        let f = self;
                        { Self(f.0.complement()) }
                    }
                }
                impl ::bitflags::__private::core::fmt::Binary for OpenFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::Octal for OpenFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::LowerHex for OpenFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::fmt::UpperHex for OpenFlags {
                    fn fmt(
                        &self,
                        f: &mut ::bitflags::__private::core::fmt::Formatter,
                    ) -> ::bitflags::__private::core::fmt::Result {
                        let inner = self.0;
                        ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOr for OpenFlags {
                    type Output = Self;
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor(self, other: OpenFlags) -> Self {
                        self.union(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitOrAssign for OpenFlags {
                    /// The bitwise or (`|`) of the bits in two flags values.
                    #[inline]
                    fn bitor_assign(&mut self, other: Self) {
                        self.insert(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitXor for OpenFlags {
                    type Output = Self;
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor(self, other: Self) -> Self {
                        self.symmetric_difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitXorAssign for OpenFlags {
                    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                    #[inline]
                    fn bitxor_assign(&mut self, other: Self) {
                        self.toggle(other);
                    }
                }
                impl ::bitflags::__private::core::ops::BitAnd for OpenFlags {
                    type Output = Self;
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand(self, other: Self) -> Self {
                        self.intersection(other)
                    }
                }
                impl ::bitflags::__private::core::ops::BitAndAssign for OpenFlags {
                    /// The bitwise and (`&`) of the bits in two flags values.
                    #[inline]
                    fn bitand_assign(&mut self, other: Self) {
                        *self = Self::from_bits_retain(self.bits()).intersection(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Sub for OpenFlags {
                    type Output = Self;
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub(self, other: Self) -> Self {
                        self.difference(other)
                    }
                }
                impl ::bitflags::__private::core::ops::SubAssign for OpenFlags {
                    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                    ///
                    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                    /// `difference` won't truncate `other`, but the `!` operator will.
                    #[inline]
                    fn sub_assign(&mut self, other: Self) {
                        self.remove(other);
                    }
                }
                impl ::bitflags::__private::core::ops::Not for OpenFlags {
                    type Output = Self;
                    /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                    #[inline]
                    fn not(self) -> Self {
                        self.complement()
                    }
                }
                impl ::bitflags::__private::core::iter::Extend<OpenFlags> for OpenFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn extend<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(&mut self, iterator: T) {
                        for item in iterator {
                            self.insert(item)
                        }
                    }
                }
                impl ::bitflags::__private::core::iter::FromIterator<OpenFlags>
                for OpenFlags {
                    /// The bitwise or (`|`) of the bits in each flags value.
                    fn from_iter<
                        T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                    >(iterator: T) -> Self {
                        use ::bitflags::__private::core::iter::Extend;
                        let mut result = Self::empty();
                        result.extend(iterator);
                        result
                    }
                }
                impl OpenFlags {
                    /// Yield a set of contained flags values.
                    ///
                    /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                    /// will be yielded together as a final flags value.
                    #[inline]
                    pub const fn iter(&self) -> ::bitflags::iter::Iter<OpenFlags> {
                        ::bitflags::iter::Iter::__private_const_new(
                            <OpenFlags as ::bitflags::Flags>::FLAGS,
                            OpenFlags::from_bits_retain(self.bits()),
                            OpenFlags::from_bits_retain(self.bits()),
                        )
                    }
                    /// Yield a set of contained named flags values.
                    ///
                    /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                    #[inline]
                    pub const fn iter_names(
                        &self,
                    ) -> ::bitflags::iter::IterNames<OpenFlags> {
                        ::bitflags::iter::IterNames::__private_const_new(
                            <OpenFlags as ::bitflags::Flags>::FLAGS,
                            OpenFlags::from_bits_retain(self.bits()),
                            OpenFlags::from_bits_retain(self.bits()),
                        )
                    }
                }
                impl ::bitflags::__private::core::iter::IntoIterator for OpenFlags {
                    type Item = OpenFlags;
                    type IntoIter = ::bitflags::iter::Iter<OpenFlags>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
            };
            /// Number of hard links to an inode.
            pub type LinkCount = u64;
            /// File attributes.
            ///
            /// Note: This was called `filestat` in earlier versions of WASI.
            #[repr(C)]
            pub struct DescriptorStat {
                /// File type.
                pub type_: DescriptorType,
                /// Number of hard links to the file.
                pub link_count: LinkCount,
                /// For regular files, the file size in bytes. For symbolic links, the
                /// length in bytes of the pathname contained in the symbolic link.
                pub size: Filesize,
                /// Last data access timestamp.
                ///
                /// If the `option` is none, the platform doesn't maintain an access
                /// timestamp for this file.
                pub data_access_timestamp: Option<Datetime>,
                /// Last data modification timestamp.
                ///
                /// If the `option` is none, the platform doesn't maintain a
                /// modification timestamp for this file.
                pub data_modification_timestamp: Option<Datetime>,
                /// Last file status-change timestamp.
                ///
                /// If the `option` is none, the platform doesn't maintain a
                /// status-change timestamp for this file.
                pub status_change_timestamp: Option<Datetime>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DescriptorStat {
                #[inline]
                fn clone(&self) -> DescriptorStat {
                    let _: ::core::clone::AssertParamIsClone<DescriptorType>;
                    let _: ::core::clone::AssertParamIsClone<LinkCount>;
                    let _: ::core::clone::AssertParamIsClone<Filesize>;
                    let _: ::core::clone::AssertParamIsClone<Option<Datetime>>;
                    let _: ::core::clone::AssertParamIsClone<Option<Datetime>>;
                    let _: ::core::clone::AssertParamIsClone<Option<Datetime>>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for DescriptorStat {}
            impl ::core::fmt::Debug for DescriptorStat {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("DescriptorStat")
                        .field("type", &self.type_)
                        .field("link-count", &self.link_count)
                        .field("size", &self.size)
                        .field("data-access-timestamp", &self.data_access_timestamp)
                        .field(
                            "data-modification-timestamp",
                            &self.data_modification_timestamp,
                        )
                        .field("status-change-timestamp", &self.status_change_timestamp)
                        .finish()
                }
            }
            /// When setting a timestamp, this gives the value to set it to.
            pub enum NewTimestamp {
                /// Leave the timestamp set to its previous value.
                NoChange,
                /// Set the timestamp to the current time of the system clock associated
                /// with the filesystem.
                Now,
                /// Set the timestamp to the given value.
                Timestamp(Datetime),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for NewTimestamp {
                #[inline]
                fn clone(&self) -> NewTimestamp {
                    let _: ::core::clone::AssertParamIsClone<Datetime>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for NewTimestamp {}
            impl ::core::fmt::Debug for NewTimestamp {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        NewTimestamp::NoChange => {
                            f.debug_tuple("NewTimestamp::NoChange").finish()
                        }
                        NewTimestamp::Now => f.debug_tuple("NewTimestamp::Now").finish(),
                        NewTimestamp::Timestamp(e) => {
                            f.debug_tuple("NewTimestamp::Timestamp").field(e).finish()
                        }
                    }
                }
            }
            /// A directory entry.
            pub struct DirectoryEntry {
                /// The type of the file referred to by this directory entry.
                pub type_: DescriptorType,
                /// The name of the object.
                pub name: wit_bindgen::rt::string::String,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DirectoryEntry {
                #[inline]
                fn clone(&self) -> DirectoryEntry {
                    DirectoryEntry {
                        type_: ::core::clone::Clone::clone(&self.type_),
                        name: ::core::clone::Clone::clone(&self.name),
                    }
                }
            }
            impl ::core::fmt::Debug for DirectoryEntry {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("DirectoryEntry")
                        .field("type", &self.type_)
                        .field("name", &self.name)
                        .finish()
                }
            }
            /// Error codes returned by functions, similar to `errno` in POSIX.
            /// Not all of these error codes are returned by the functions provided by this
            /// API; some are used in higher-level library layers, and others are provided
            /// merely for alignment with POSIX.
            #[repr(u8)]
            pub enum ErrorCode {
                /// Permission denied, similar to `EACCES` in POSIX.
                Access,
                /// Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX.
                WouldBlock,
                /// Connection already in progress, similar to `EALREADY` in POSIX.
                Already,
                /// Bad descriptor, similar to `EBADF` in POSIX.
                BadDescriptor,
                /// Device or resource busy, similar to `EBUSY` in POSIX.
                Busy,
                /// Resource deadlock would occur, similar to `EDEADLK` in POSIX.
                Deadlock,
                /// Storage quota exceeded, similar to `EDQUOT` in POSIX.
                Quota,
                /// File exists, similar to `EEXIST` in POSIX.
                Exist,
                /// File too large, similar to `EFBIG` in POSIX.
                FileTooLarge,
                /// Illegal byte sequence, similar to `EILSEQ` in POSIX.
                IllegalByteSequence,
                /// Operation in progress, similar to `EINPROGRESS` in POSIX.
                InProgress,
                /// Interrupted function, similar to `EINTR` in POSIX.
                Interrupted,
                /// Invalid argument, similar to `EINVAL` in POSIX.
                Invalid,
                /// I/O error, similar to `EIO` in POSIX.
                Io,
                /// Is a directory, similar to `EISDIR` in POSIX.
                IsDirectory,
                /// Too many levels of symbolic links, similar to `ELOOP` in POSIX.
                Loop,
                /// Too many links, similar to `EMLINK` in POSIX.
                TooManyLinks,
                /// Message too large, similar to `EMSGSIZE` in POSIX.
                MessageSize,
                /// Filename too long, similar to `ENAMETOOLONG` in POSIX.
                NameTooLong,
                /// No such device, similar to `ENODEV` in POSIX.
                NoDevice,
                /// No such file or directory, similar to `ENOENT` in POSIX.
                NoEntry,
                /// No locks available, similar to `ENOLCK` in POSIX.
                NoLock,
                /// Not enough space, similar to `ENOMEM` in POSIX.
                InsufficientMemory,
                /// No space left on device, similar to `ENOSPC` in POSIX.
                InsufficientSpace,
                /// Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX.
                NotDirectory,
                /// Directory not empty, similar to `ENOTEMPTY` in POSIX.
                NotEmpty,
                /// State not recoverable, similar to `ENOTRECOVERABLE` in POSIX.
                NotRecoverable,
                /// Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX.
                Unsupported,
                /// Inappropriate I/O control operation, similar to `ENOTTY` in POSIX.
                NoTty,
                /// No such device or address, similar to `ENXIO` in POSIX.
                NoSuchDevice,
                /// Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX.
                Overflow,
                /// Operation not permitted, similar to `EPERM` in POSIX.
                NotPermitted,
                /// Broken pipe, similar to `EPIPE` in POSIX.
                Pipe,
                /// Read-only file system, similar to `EROFS` in POSIX.
                ReadOnly,
                /// Invalid seek, similar to `ESPIPE` in POSIX.
                InvalidSeek,
                /// Text file busy, similar to `ETXTBSY` in POSIX.
                TextFileBusy,
                /// Cross-device link, similar to `EXDEV` in POSIX.
                CrossDevice,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ErrorCode {
                #[inline]
                fn clone(&self) -> ErrorCode {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ErrorCode {}
            #[automatically_derived]
            impl ::core::cmp::Eq for ErrorCode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ErrorCode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ErrorCode {
                #[inline]
                fn eq(&self, other: &ErrorCode) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl ErrorCode {
                pub fn name(&self) -> &'static str {
                    match self {
                        ErrorCode::Access => "access",
                        ErrorCode::WouldBlock => "would-block",
                        ErrorCode::Already => "already",
                        ErrorCode::BadDescriptor => "bad-descriptor",
                        ErrorCode::Busy => "busy",
                        ErrorCode::Deadlock => "deadlock",
                        ErrorCode::Quota => "quota",
                        ErrorCode::Exist => "exist",
                        ErrorCode::FileTooLarge => "file-too-large",
                        ErrorCode::IllegalByteSequence => "illegal-byte-sequence",
                        ErrorCode::InProgress => "in-progress",
                        ErrorCode::Interrupted => "interrupted",
                        ErrorCode::Invalid => "invalid",
                        ErrorCode::Io => "io",
                        ErrorCode::IsDirectory => "is-directory",
                        ErrorCode::Loop => "loop",
                        ErrorCode::TooManyLinks => "too-many-links",
                        ErrorCode::MessageSize => "message-size",
                        ErrorCode::NameTooLong => "name-too-long",
                        ErrorCode::NoDevice => "no-device",
                        ErrorCode::NoEntry => "no-entry",
                        ErrorCode::NoLock => "no-lock",
                        ErrorCode::InsufficientMemory => "insufficient-memory",
                        ErrorCode::InsufficientSpace => "insufficient-space",
                        ErrorCode::NotDirectory => "not-directory",
                        ErrorCode::NotEmpty => "not-empty",
                        ErrorCode::NotRecoverable => "not-recoverable",
                        ErrorCode::Unsupported => "unsupported",
                        ErrorCode::NoTty => "no-tty",
                        ErrorCode::NoSuchDevice => "no-such-device",
                        ErrorCode::Overflow => "overflow",
                        ErrorCode::NotPermitted => "not-permitted",
                        ErrorCode::Pipe => "pipe",
                        ErrorCode::ReadOnly => "read-only",
                        ErrorCode::InvalidSeek => "invalid-seek",
                        ErrorCode::TextFileBusy => "text-file-busy",
                        ErrorCode::CrossDevice => "cross-device",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        ErrorCode::Access => {
                            "Permission denied, similar to `EACCES` in POSIX."
                        }
                        ErrorCode::WouldBlock => {
                            "Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX."
                        }
                        ErrorCode::Already => {
                            "Connection already in progress, similar to `EALREADY` in POSIX."
                        }
                        ErrorCode::BadDescriptor => {
                            "Bad descriptor, similar to `EBADF` in POSIX."
                        }
                        ErrorCode::Busy => {
                            "Device or resource busy, similar to `EBUSY` in POSIX."
                        }
                        ErrorCode::Deadlock => {
                            "Resource deadlock would occur, similar to `EDEADLK` in POSIX."
                        }
                        ErrorCode::Quota => {
                            "Storage quota exceeded, similar to `EDQUOT` in POSIX."
                        }
                        ErrorCode::Exist => "File exists, similar to `EEXIST` in POSIX.",
                        ErrorCode::FileTooLarge => {
                            "File too large, similar to `EFBIG` in POSIX."
                        }
                        ErrorCode::IllegalByteSequence => {
                            "Illegal byte sequence, similar to `EILSEQ` in POSIX."
                        }
                        ErrorCode::InProgress => {
                            "Operation in progress, similar to `EINPROGRESS` in POSIX."
                        }
                        ErrorCode::Interrupted => {
                            "Interrupted function, similar to `EINTR` in POSIX."
                        }
                        ErrorCode::Invalid => {
                            "Invalid argument, similar to `EINVAL` in POSIX."
                        }
                        ErrorCode::Io => "I/O error, similar to `EIO` in POSIX.",
                        ErrorCode::IsDirectory => {
                            "Is a directory, similar to `EISDIR` in POSIX."
                        }
                        ErrorCode::Loop => {
                            "Too many levels of symbolic links, similar to `ELOOP` in POSIX."
                        }
                        ErrorCode::TooManyLinks => {
                            "Too many links, similar to `EMLINK` in POSIX."
                        }
                        ErrorCode::MessageSize => {
                            "Message too large, similar to `EMSGSIZE` in POSIX."
                        }
                        ErrorCode::NameTooLong => {
                            "Filename too long, similar to `ENAMETOOLONG` in POSIX."
                        }
                        ErrorCode::NoDevice => {
                            "No such device, similar to `ENODEV` in POSIX."
                        }
                        ErrorCode::NoEntry => {
                            "No such file or directory, similar to `ENOENT` in POSIX."
                        }
                        ErrorCode::NoLock => {
                            "No locks available, similar to `ENOLCK` in POSIX."
                        }
                        ErrorCode::InsufficientMemory => {
                            "Not enough space, similar to `ENOMEM` in POSIX."
                        }
                        ErrorCode::InsufficientSpace => {
                            "No space left on device, similar to `ENOSPC` in POSIX."
                        }
                        ErrorCode::NotDirectory => {
                            "Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX."
                        }
                        ErrorCode::NotEmpty => {
                            "Directory not empty, similar to `ENOTEMPTY` in POSIX."
                        }
                        ErrorCode::NotRecoverable => {
                            "State not recoverable, similar to `ENOTRECOVERABLE` in POSIX."
                        }
                        ErrorCode::Unsupported => {
                            "Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX."
                        }
                        ErrorCode::NoTty => {
                            "Inappropriate I/O control operation, similar to `ENOTTY` in POSIX."
                        }
                        ErrorCode::NoSuchDevice => {
                            "No such device or address, similar to `ENXIO` in POSIX."
                        }
                        ErrorCode::Overflow => {
                            "Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX."
                        }
                        ErrorCode::NotPermitted => {
                            "Operation not permitted, similar to `EPERM` in POSIX."
                        }
                        ErrorCode::Pipe => "Broken pipe, similar to `EPIPE` in POSIX.",
                        ErrorCode::ReadOnly => {
                            "Read-only file system, similar to `EROFS` in POSIX."
                        }
                        ErrorCode::InvalidSeek => {
                            "Invalid seek, similar to `ESPIPE` in POSIX."
                        }
                        ErrorCode::TextFileBusy => {
                            "Text file busy, similar to `ETXTBSY` in POSIX."
                        }
                        ErrorCode::CrossDevice => {
                            "Cross-device link, similar to `EXDEV` in POSIX."
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ErrorCode")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.write_fmt(
                        format_args!("{0} (error {1})", self.name(), *self as i32),
                    )
                }
            }
            impl std::error::Error for ErrorCode {}
            impl ErrorCode {
                pub(crate) unsafe fn _lift(val: u8) -> ErrorCode {
                    if !true {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ErrorCode::Access,
                        1 => ErrorCode::WouldBlock,
                        2 => ErrorCode::Already,
                        3 => ErrorCode::BadDescriptor,
                        4 => ErrorCode::Busy,
                        5 => ErrorCode::Deadlock,
                        6 => ErrorCode::Quota,
                        7 => ErrorCode::Exist,
                        8 => ErrorCode::FileTooLarge,
                        9 => ErrorCode::IllegalByteSequence,
                        10 => ErrorCode::InProgress,
                        11 => ErrorCode::Interrupted,
                        12 => ErrorCode::Invalid,
                        13 => ErrorCode::Io,
                        14 => ErrorCode::IsDirectory,
                        15 => ErrorCode::Loop,
                        16 => ErrorCode::TooManyLinks,
                        17 => ErrorCode::MessageSize,
                        18 => ErrorCode::NameTooLong,
                        19 => ErrorCode::NoDevice,
                        20 => ErrorCode::NoEntry,
                        21 => ErrorCode::NoLock,
                        22 => ErrorCode::InsufficientMemory,
                        23 => ErrorCode::InsufficientSpace,
                        24 => ErrorCode::NotDirectory,
                        25 => ErrorCode::NotEmpty,
                        26 => ErrorCode::NotRecoverable,
                        27 => ErrorCode::Unsupported,
                        28 => ErrorCode::NoTty,
                        29 => ErrorCode::NoSuchDevice,
                        30 => ErrorCode::Overflow,
                        31 => ErrorCode::NotPermitted,
                        32 => ErrorCode::Pipe,
                        33 => ErrorCode::ReadOnly,
                        34 => ErrorCode::InvalidSeek,
                        35 => ErrorCode::TextFileBusy,
                        36 => ErrorCode::CrossDevice,
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("invalid enum discriminant"),
                            );
                        }
                    }
                }
            }
            /// File or memory access pattern advisory information.
            #[repr(u8)]
            pub enum Advice {
                /// The application has no advice to give on its behavior with respect
                /// to the specified data.
                Normal,
                /// The application expects to access the specified data sequentially
                /// from lower offsets to higher offsets.
                Sequential,
                /// The application expects to access the specified data in a random
                /// order.
                Random,
                /// The application expects to access the specified data in the near
                /// future.
                WillNeed,
                /// The application expects that it will not access the specified data
                /// in the near future.
                DontNeed,
                /// The application expects to access the specified data once and then
                /// not reuse it thereafter.
                NoReuse,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Advice {
                #[inline]
                fn clone(&self) -> Advice {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Advice {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Advice {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Advice {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Advice {
                #[inline]
                fn eq(&self, other: &Advice) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl ::core::fmt::Debug for Advice {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Advice::Normal => f.debug_tuple("Advice::Normal").finish(),
                        Advice::Sequential => {
                            f.debug_tuple("Advice::Sequential").finish()
                        }
                        Advice::Random => f.debug_tuple("Advice::Random").finish(),
                        Advice::WillNeed => f.debug_tuple("Advice::WillNeed").finish(),
                        Advice::DontNeed => f.debug_tuple("Advice::DontNeed").finish(),
                        Advice::NoReuse => f.debug_tuple("Advice::NoReuse").finish(),
                    }
                }
            }
            impl Advice {
                pub(crate) unsafe fn _lift(val: u8) -> Advice {
                    if !true {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => Advice::Normal,
                        1 => Advice::Sequential,
                        2 => Advice::Random,
                        3 => Advice::WillNeed,
                        4 => Advice::DontNeed,
                        5 => Advice::NoReuse,
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("invalid enum discriminant"),
                            );
                        }
                    }
                }
            }
            /// A 128-bit hash value, split into parts because wasm doesn't have a
            /// 128-bit integer type.
            #[repr(C)]
            pub struct MetadataHashValue {
                /// 64 bits of a 128-bit hash value.
                pub lower: u64,
                /// Another 64 bits of a 128-bit hash value.
                pub upper: u64,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for MetadataHashValue {
                #[inline]
                fn clone(&self) -> MetadataHashValue {
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for MetadataHashValue {}
            impl ::core::fmt::Debug for MetadataHashValue {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("MetadataHashValue")
                        .field("lower", &self.lower)
                        .field("upper", &self.upper)
                        .finish()
                }
            }
            /// A descriptor is a reference to a filesystem object, which may be a file,
            /// directory, named pipe, special file, or other object on which filesystem
            /// calls may be made.
            #[repr(transparent)]
            pub struct Descriptor {
                handle: wit_bindgen::rt::Resource<Descriptor>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Descriptor {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Descriptor",
                        "handle",
                        &&self.handle,
                    )
                }
            }
            impl Descriptor {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: wit_bindgen::rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn into_handle(self) -> u32 {
                    wit_bindgen::rt::Resource::into_handle(self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    wit_bindgen::rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl wit_bindgen::rt::WasmResource for Descriptor {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    ::core::panicking::panic("internal error: entered unreachable code");
                }
            }
            /// A stream of directory entries.
            #[repr(transparent)]
            pub struct DirectoryEntryStream {
                handle: wit_bindgen::rt::Resource<DirectoryEntryStream>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for DirectoryEntryStream {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "DirectoryEntryStream",
                        "handle",
                        &&self.handle,
                    )
                }
            }
            impl DirectoryEntryStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: wit_bindgen::rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn into_handle(self) -> u32 {
                    wit_bindgen::rt::Resource::into_handle(self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    wit_bindgen::rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl wit_bindgen::rt::WasmResource for DirectoryEntryStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    ::core::panicking::panic("internal error: entered unreachable code");
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a stream for reading from a file, if available.
                ///
                /// May fail with an error-code describing why the file cannot be read.
                ///
                /// Multiple read, write, and append streams may be active on the same open
                /// file and they do not interfere with each other.
                ///
                /// Note: This allows using `read-stream`, which is similar to `read` in POSIX.
                pub fn read_via_stream(
                    &self,
                    offset: Filesize,
                ) -> Result<InputStream, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 8]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(offset),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 4) as *const i32);
                                    super::super::super::wasi::io::streams::InputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 4) as *const u8));
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a stream for writing to a file, if available.
                ///
                /// May fail with an error-code describing why the file cannot be written.
                ///
                /// Note: This allows using `write-stream`, which is similar to `write` in
                /// POSIX.
                pub fn write_via_stream(
                    &self,
                    offset: Filesize,
                ) -> Result<OutputStream, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 8]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(offset),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 4) as *const i32);
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 4) as *const u8));
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a stream for appending to a file, if available.
                ///
                /// May fail with an error-code describing why the file cannot be appended.
                ///
                /// Note: This allows using `write-stream`, which is similar to `write` with
                /// `O_APPEND` in in POSIX.
                pub fn append_via_stream(&self) -> Result<OutputStream, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 8]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 4) as *const i32);
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 4) as *const u8));
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Provide file advisory information on a descriptor.
                ///
                /// This is similar to `posix_fadvise` in POSIX.
                pub fn advise(
                    &self,
                    offset: Filesize,
                    length: Filesize,
                    advice: Advice,
                ) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(offset),
                            wit_bindgen::rt::as_i64(length),
                            advice.clone() as i32,
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 1) as *const u8));
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Synchronize the data of a file to disk.
                ///
                /// This function succeeds with no effect if the file descriptor is not
                /// opened for writing.
                ///
                /// Note: This is similar to `fdatasync` in POSIX.
                pub fn sync_data(&self) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 1) as *const u8));
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Get flags associated with a descriptor.
                ///
                /// Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.
                ///
                /// Note: This returns the value that was the `fs_flags` value returned
                /// from `fdstat_get` in earlier versions of WASI.
                pub fn get_flags(&self) -> Result<DescriptorFlags, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 1) as *const u8));
                                    DescriptorFlags::empty()
                                        | DescriptorFlags::from_bits_retain(((l2 as u8) << 0) as _)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 1) as *const u8));
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the dynamic type of a descriptor.
                ///
                /// Note: This returns the same value as the `type` field of the `fd-stat`
                /// returned by `stat`, `stat-at` and similar.
                ///
                /// Note: This returns similar flags to the `st_mode & S_IFMT` value provided
                /// by `fstat` in POSIX.
                ///
                /// Note: This returns the value that was the `fs_filetype` value returned
                /// from `fdstat_get` in earlier versions of WASI.
                pub fn get_type(&self) -> Result<DescriptorType, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 1) as *const u8));
                                    DescriptorType::_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 1) as *const u8));
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Adjust the size of an open file. If this increases the file's size, the
                /// extra bytes are filled with zeros.
                ///
                /// Note: This was called `fd_filestat_set_size` in earlier versions of WASI.
                pub fn set_size(&self, size: Filesize) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(size),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 1) as *const u8));
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Adjust the timestamps of an open file or directory.
                ///
                /// Note: This is similar to `futimens` in POSIX.
                ///
                /// Note: This was called `fd_filestat_set_times` in earlier versions of WASI.
                pub fn set_times(
                    &self,
                    data_access_timestamp: NewTimestamp,
                    data_modification_timestamp: NewTimestamp,
                ) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let (result1_0, result1_1, result1_2) = match data_access_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds0,
                                    nanoseconds: nanoseconds0,
                                } = e;
                                (
                                    2i32,
                                    wit_bindgen::rt::as_i64(seconds0),
                                    wit_bindgen::rt::as_i32(nanoseconds0),
                                )
                            }
                        };
                        let (result3_0, result3_1, result3_2) = match data_modification_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds2,
                                    nanoseconds: nanoseconds2,
                                } = e;
                                (
                                    2i32,
                                    wit_bindgen::rt::as_i64(seconds2),
                                    wit_bindgen::rt::as_i32(nanoseconds2),
                                )
                            }
                        };
                        let ptr4 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                        ) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            result1_0,
                            result1_1,
                            result1_2,
                            result3_0,
                            result3_1,
                            result3_2,
                            ptr4,
                        );
                        let l5 = i32::from(*((ptr4 + 0) as *const u8));
                        match l5 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*((ptr4 + 1) as *const u8));
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Read from a descriptor, without using and updating the descriptor's offset.
                ///
                /// This function returns a list of bytes containing the data that was
                /// read, along with a bool which, when true, indicates that the end of the
                /// file was reached. The returned list will contain up to `length` bytes; it
                /// may return fewer than requested, if the end of the file is reached or
                /// if the I/O operation is interrupted.
                ///
                /// In the future, this may change to return a `stream<u8, error-code>`.
                ///
                /// Note: This is similar to `pread` in POSIX.
                pub fn read(
                    &self,
                    length: Filesize,
                    offset: Filesize,
                ) -> Result<(wit_bindgen::rt::vec::Vec<u8>, bool), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 16]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(length),
                            wit_bindgen::rt::as_i64(offset),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 4) as *const i32);
                                    let l3 = *((ptr0 + 8) as *const i32);
                                    let len4 = l3 as usize;
                                    let l5 = i32::from(*((ptr0 + 12) as *const u8));
                                    (
                                        Vec::from_raw_parts(l2 as *mut _, len4, len4),
                                        wit_bindgen::rt::bool_lift(l5 as u8),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*((ptr0 + 4) as *const u8));
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Write to a descriptor, without using and updating the descriptor's offset.
                ///
                /// It is valid to write past the end of a file; the file is extended to the
                /// extent of the write, with bytes between the previous end and the start of
                /// the write set to zero.
                ///
                /// In the future, this may change to take a `stream<u8, error-code>`.
                ///
                /// Note: This is similar to `pwrite` in POSIX.
                pub fn write(
                    &self,
                    buffer: &[u8],
                    offset: Filesize,
                ) -> Result<Filesize, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 16]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = buffer;
                        let ptr0 = vec0.as_ptr() as i32;
                        let len0 = vec0.len() as i32;
                        let ptr1 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0,
                            len0,
                            wit_bindgen::rt::as_i64(offset),
                            ptr1,
                        );
                        let l2 = i32::from(*((ptr1 + 0) as *const u8));
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *((ptr1 + 8) as *const i64);
                                    l3 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*((ptr1 + 8) as *const u8));
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Read directory entries from a directory.
                ///
                /// On filesystems where directories contain entries referring to themselves
                /// and their parents, often named `.` and `..` respectively, these entries
                /// are omitted.
                ///
                /// This always returns a new stream which starts at the beginning of the
                /// directory. Multiple streams may be active on the same directory, and they
                /// do not interfere with each other.
                pub fn read_directory(&self) -> Result<DirectoryEntryStream, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 8]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 4) as *const i32);
                                    DirectoryEntryStream::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 4) as *const u8));
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Synchronize the data and metadata of a file to disk.
                ///
                /// This function succeeds with no effect if the file descriptor is not
                /// opened for writing.
                ///
                /// Note: This is similar to `fsync` in POSIX.
                pub fn sync(&self) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 1) as *const u8));
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a directory.
                ///
                /// Note: This is similar to `mkdirat` in POSIX.
                pub fn create_directory_at(&self, path: &str) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr() as i32;
                        let len0 = vec0.len() as i32;
                        let ptr1 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0, len0, ptr1);
                        let l2 = i32::from(*((ptr1 + 0) as *const u8));
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr1 + 1) as *const u8));
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return the attributes of an open file or directory.
                ///
                /// Note: This is similar to `fstat` in POSIX, except that it does not return
                /// device and inode information. For testing whether two descriptors refer to
                /// the same underlying filesystem object, use `is-same-object`. To obtain
                /// additional data that can be used do determine whether a file has been
                /// modified, use `metadata-hash`.
                ///
                /// Note: This was called `fd_filestat_get` in earlier versions of WASI.
                pub fn stat(&self) -> Result<DescriptorStat, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 104]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 8) as *const u8));
                                    let l3 = *((ptr0 + 16) as *const i64);
                                    let l4 = *((ptr0 + 24) as *const i64);
                                    let l5 = i32::from(*((ptr0 + 32) as *const u8));
                                    let l8 = i32::from(*((ptr0 + 56) as *const u8));
                                    let l11 = i32::from(*((ptr0 + 80) as *const u8));
                                    DescriptorStat {
                                        type_: DescriptorType::_lift(l2 as u8),
                                        link_count: l3 as u64,
                                        size: l4 as u64,
                                        data_access_timestamp: match l5 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l6 = *((ptr0 + 40) as *const i64);
                                                    let l7 = *((ptr0 + 48) as *const i32);
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l6 as u64,
                                                        nanoseconds: l7 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                                        },
                                        data_modification_timestamp: match l8 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l9 = *((ptr0 + 64) as *const i64);
                                                    let l10 = *((ptr0 + 72) as *const i32);
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l9 as u64,
                                                        nanoseconds: l10 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                                        },
                                        status_change_timestamp: match l11 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l12 = *((ptr0 + 88) as *const i64);
                                                    let l13 = *((ptr0 + 96) as *const i32);
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l12 as u64,
                                                        nanoseconds: l13 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                                        },
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l14 = i32::from(*((ptr0 + 8) as *const u8));
                                    ErrorCode::_lift(l14 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return the attributes of a file or directory.
                ///
                /// Note: This is similar to `fstatat` in POSIX, except that it does not
                /// return device and inode information. See the `stat` description for a
                /// discussion of alternatives.
                ///
                /// Note: This was called `path_filestat_get` in earlier versions of WASI.
                pub fn stat_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                ) -> Result<DescriptorStat, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 104]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;
                        let ptr2 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1,
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*((ptr2 + 0) as *const u8));
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = i32::from(*((ptr2 + 8) as *const u8));
                                    let l5 = *((ptr2 + 16) as *const i64);
                                    let l6 = *((ptr2 + 24) as *const i64);
                                    let l7 = i32::from(*((ptr2 + 32) as *const u8));
                                    let l10 = i32::from(*((ptr2 + 56) as *const u8));
                                    let l13 = i32::from(*((ptr2 + 80) as *const u8));
                                    DescriptorStat {
                                        type_: DescriptorType::_lift(l4 as u8),
                                        link_count: l5 as u64,
                                        size: l6 as u64,
                                        data_access_timestamp: match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *((ptr2 + 40) as *const i64);
                                                    let l9 = *((ptr2 + 48) as *const i32);
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l8 as u64,
                                                        nanoseconds: l9 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                                        },
                                        data_modification_timestamp: match l10 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l11 = *((ptr2 + 64) as *const i64);
                                                    let l12 = *((ptr2 + 72) as *const i32);
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l11 as u64,
                                                        nanoseconds: l12 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                                        },
                                        status_change_timestamp: match l13 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l14 = *((ptr2 + 88) as *const i64);
                                                    let l15 = *((ptr2 + 96) as *const i32);
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l14 as u64,
                                                        nanoseconds: l15 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                                        },
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l16 = i32::from(*((ptr2 + 8) as *const u8));
                                    ErrorCode::_lift(l16 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Adjust the timestamps of a file or directory.
                ///
                /// Note: This is similar to `utimensat` in POSIX.
                ///
                /// Note: This was called `path_filestat_set_times` in earlier versions of
                /// WASI.
                pub fn set_times_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                    data_access_timestamp: NewTimestamp,
                    data_modification_timestamp: NewTimestamp,
                ) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;
                        let (result3_0, result3_1, result3_2) = match data_access_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds2,
                                    nanoseconds: nanoseconds2,
                                } = e;
                                (
                                    2i32,
                                    wit_bindgen::rt::as_i64(seconds2),
                                    wit_bindgen::rt::as_i32(nanoseconds2),
                                )
                            }
                        };
                        let (result5_0, result5_1, result5_2) = match data_modification_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds4,
                                    nanoseconds: nanoseconds4,
                                } = e;
                                (
                                    2i32,
                                    wit_bindgen::rt::as_i64(seconds4),
                                    wit_bindgen::rt::as_i32(nanoseconds4),
                                )
                            }
                        };
                        let ptr6 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                        ) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1,
                            len1,
                            result3_0,
                            result3_1,
                            result3_2,
                            result5_0,
                            result5_1,
                            result5_2,
                            ptr6,
                        );
                        let l7 = i32::from(*((ptr6 + 0) as *const u8));
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*((ptr6 + 1) as *const u8));
                                    ErrorCode::_lift(l8 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a hard link.
                ///
                /// Note: This is similar to `linkat` in POSIX.
                pub fn link_at(
                    &self,
                    old_path_flags: PathFlags,
                    old_path: &str,
                    new_descriptor: &Descriptor,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let flags0 = old_path_flags;
                        let vec1 = old_path;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;
                        let vec2 = new_path;
                        let ptr2 = vec2.as_ptr() as i32;
                        let len2 = vec2.len() as i32;
                        let ptr3 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                        ) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1,
                            len1,
                            (new_descriptor).handle() as i32,
                            ptr2,
                            len2,
                            ptr3,
                        );
                        let l4 = i32::from(*((ptr3 + 0) as *const u8));
                        match l4 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*((ptr3 + 1) as *const u8));
                                    ErrorCode::_lift(l5 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Open a file or directory.
                ///
                /// The returned descriptor is not guaranteed to be the lowest-numbered
                /// descriptor not currently open/ it is randomized to prevent applications
                /// from depending on making assumptions about indexes, since this is
                /// error-prone in multi-threaded contexts. The returned descriptor is
                /// guaranteed to be less than 2**31.
                ///
                /// If `flags` contains `descriptor-flags::mutate-directory`, and the base
                /// descriptor doesn't have `descriptor-flags::mutate-directory` set,
                /// `open-at` fails with `error-code::read-only`.
                ///
                /// If `flags` contains `write` or `mutate-directory`, or `open-flags`
                /// contains `truncate` or `create`, and the base descriptor doesn't have
                /// `descriptor-flags::mutate-directory` set, `open-at` fails with
                /// `error-code::read-only`.
                ///
                /// Note: This is similar to `openat` in POSIX.
                pub fn open_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                    open_flags: OpenFlags,
                    flags: DescriptorFlags,
                ) -> Result<Descriptor, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 8]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;
                        let flags2 = open_flags;
                        let flags3 = flags;
                        let ptr4 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                        ) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1,
                            len1,
                            (flags2.bits() >> 0) as i32,
                            (flags3.bits() >> 0) as i32,
                            ptr4,
                        );
                        let l5 = i32::from(*((ptr4 + 0) as *const u8));
                        match l5 {
                            0 => {
                                let e = {
                                    let l6 = *((ptr4 + 4) as *const i32);
                                    Descriptor::from_handle(l6 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*((ptr4 + 4) as *const u8));
                                    ErrorCode::_lift(l7 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Read the contents of a symbolic link.
                ///
                /// If the contents contain an absolute or rooted path in the underlying
                /// filesystem, this function fails with `error-code::not-permitted`.
                ///
                /// Note: This is similar to `readlinkat` in POSIX.
                pub fn readlink_at(
                    &self,
                    path: &str,
                ) -> Result<wit_bindgen::rt::string::String, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 12]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr() as i32;
                        let len0 = vec0.len() as i32;
                        let ptr1 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0, len0, ptr1);
                        let l2 = i32::from(*((ptr1 + 0) as *const u8));
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *((ptr1 + 4) as *const i32);
                                    let l4 = *((ptr1 + 8) as *const i32);
                                    let len5 = l4 as usize;
                                    let bytes5 = Vec::from_raw_parts(l3 as *mut _, len5, len5);
                                    wit_bindgen::rt::string_lift(bytes5)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*((ptr1 + 4) as *const u8));
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Remove a directory.
                ///
                /// Return `error-code::not-empty` if the directory is not empty.
                ///
                /// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
                pub fn remove_directory_at(&self, path: &str) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr() as i32;
                        let len0 = vec0.len() as i32;
                        let ptr1 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0, len0, ptr1);
                        let l2 = i32::from(*((ptr1 + 0) as *const u8));
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr1 + 1) as *const u8));
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Rename a filesystem object.
                ///
                /// Note: This is similar to `renameat` in POSIX.
                pub fn rename_at(
                    &self,
                    old_path: &str,
                    new_descriptor: &Descriptor,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = old_path;
                        let ptr0 = vec0.as_ptr() as i32;
                        let len0 = vec0.len() as i32;
                        let vec1 = new_path;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;
                        let ptr2 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                        ) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0,
                            len0,
                            (new_descriptor).handle() as i32,
                            ptr1,
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*((ptr2 + 0) as *const u8));
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*((ptr2 + 1) as *const u8));
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a symbolic link (also known as a "symlink").
                ///
                /// If `old-path` starts with `/`, the function fails with
                /// `error-code::not-permitted`.
                ///
                /// Note: This is similar to `symlinkat` in POSIX.
                pub fn symlink_at(
                    &self,
                    old_path: &str,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = old_path;
                        let ptr0 = vec0.as_ptr() as i32;
                        let len0 = vec0.len() as i32;
                        let vec1 = new_path;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;
                        let ptr2 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0, len0, ptr1, len1, ptr2);
                        let l3 = i32::from(*((ptr2 + 0) as *const u8));
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*((ptr2 + 1) as *const u8));
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Unlink a filesystem object that is not a directory.
                ///
                /// Return `error-code::is-directory` if the path refers to a directory.
                /// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
                pub fn unlink_file_at(&self, path: &str) -> Result<(), ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([u8; 2]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr() as i32;
                        let len0 = vec0.len() as i32;
                        let ptr1 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0, len0, ptr1);
                        let l2 = i32::from(*((ptr1 + 0) as *const u8));
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr1 + 1) as *const u8));
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Test whether two descriptors refer to the same filesystem object.
                ///
                /// In POSIX, this corresponds to testing whether the two descriptors have the
                /// same device (`st_dev`) and inode (`st_ino` or `d_ino`) numbers.
                /// wasi-filesystem does not expose device and inode numbers, so this function
                /// may be used instead.
                pub fn is_same_object(&self, other: &Descriptor) -> bool {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            (other).handle() as i32,
                        );
                        wit_bindgen::rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a hash of the metadata associated with a filesystem object referred
                /// to by a descriptor.
                ///
                /// This returns a hash of the last-modification timestamp and file size, and
                /// may also include the inode number, device number, birth timestamp, and
                /// other metadata fields that may change when the file is modified or
                /// replaced. It may also include a secret value chosen by the
                /// implementation and not otherwise exposed.
                ///
                /// Implementations are encourated to provide the following properties:
                ///
                /// - If the file is not modified or replaced, the computed hash value should
                /// usually not change.
                /// - If the object is modified or replaced, the computed hash value should
                /// usually change.
                /// - The inputs to the hash should not be easily computable from the
                /// computed hash.
                ///
                /// However, none of these is required.
                pub fn metadata_hash(&self) -> Result<MetadataHashValue, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 24]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 8) as *const i64);
                                    let l3 = *((ptr0 + 16) as *const i64);
                                    MetadataHashValue {
                                        lower: l2 as u64,
                                        upper: l3 as u64,
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*((ptr0 + 8) as *const u8));
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a hash of the metadata associated with a filesystem object referred
                /// to by a directory descriptor and a relative path.
                ///
                /// This performs the same hash computation as `metadata-hash`.
                pub fn metadata_hash_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                ) -> Result<MetadataHashValue, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 24]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;
                        let ptr2 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1,
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*((ptr2 + 0) as *const u8));
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = *((ptr2 + 8) as *const i64);
                                    let l5 = *((ptr2 + 16) as *const i64);
                                    MetadataHashValue {
                                        lower: l4 as u64,
                                        upper: l5 as u64,
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*((ptr2 + 8) as *const u8));
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl DirectoryEntryStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read a single directory entry from a `directory-entry-stream`.
                pub fn read_directory_entry(
                    &self,
                ) -> Result<Option<DirectoryEntry>, ErrorCode> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 20]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                    match l2 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                                let l4 = *((ptr0 + 12) as *const i32);
                                                let l5 = *((ptr0 + 16) as *const i32);
                                                let len6 = l5 as usize;
                                                let bytes6 = Vec::from_raw_parts(l4 as *mut _, len6, len6);
                                                DirectoryEntry {
                                                    type_: DescriptorType::_lift(l3 as u8),
                                                    name: wit_bindgen::rt::string_lift(bytes6),
                                                }
                                            };
                                            Some(e)
                                        }
                                        _ => wit_bindgen::rt::invalid_enum_discriminant(),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*((ptr0 + 4) as *const u8));
                                    ErrorCode::_lift(l7 as u8)
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Attempts to extract a filesystem-related `error-code` from the stream
            /// `error` provided.
            ///
            /// Stream operations which return `stream-error::last-operation-failed`
            /// have a payload with more information about the operation that failed.
            /// This payload can be passed through to this function to see if there's
            /// filesystem-related information about the error to return.
            ///
            /// Note that this function is fallible because not all stream-related
            /// errors are filesystem-related errors.
            pub fn filesystem_error_code(err: &Error) -> Option<ErrorCode> {
                #[allow(unused_imports)]
                use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                unsafe {
                    #[repr(align(1))]
                    struct RetArea([u8; 2]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    let ptr0 = ret_area.as_mut_ptr() as i32;
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                    wit_import((err).handle() as i32, ptr0);
                    let l1 = i32::from(*((ptr0 + 0) as *const u8));
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = i32::from(*((ptr0 + 1) as *const u8));
                                ErrorCode::_lift(l2 as u8)
                            };
                            Some(e)
                        }
                        _ => wit_bindgen::rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(clippy::all)]
        pub mod preopens {
            pub type Descriptor = super::super::super::wasi::filesystem::types::Descriptor;
            #[allow(unused_unsafe, clippy::all)]
            /// Return the set of preopened directories, and their path.
            pub fn get_directories() -> wit_bindgen::rt::vec::Vec<
                (Descriptor, wit_bindgen::rt::string::String),
            > {
                #[allow(unused_imports)]
                use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([u8; 8]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    let ptr0 = ret_area.as_mut_ptr() as i32;
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                    wit_import(ptr0);
                    let l1 = *((ptr0 + 0) as *const i32);
                    let l2 = *((ptr0 + 4) as *const i32);
                    let base7 = l1;
                    let len7 = l2;
                    let mut result7 = Vec::with_capacity(len7 as usize);
                    for i in 0..len7 {
                        let base = base7 + i * 12;
                        let e7 = {
                            let l3 = *((base + 0) as *const i32);
                            let l4 = *((base + 4) as *const i32);
                            let l5 = *((base + 8) as *const i32);
                            let len6 = l5 as usize;
                            let bytes6 = Vec::from_raw_parts(l4 as *mut _, len6, len6);
                            (
                                super::super::super::wasi::filesystem::types::Descriptor::from_handle(
                                    l3 as u32,
                                ),
                                wit_bindgen::rt::string_lift(bytes6),
                            )
                        };
                        result7.push(e7);
                    }
                    wit_bindgen::rt::dealloc(base7, (len7 as usize) * 12, 4);
                    result7
                }
            }
        }
    }
    pub mod io {
        #[allow(clippy::all)]
        pub mod error {
            /// A resource which represents some error information.
            ///
            /// The only method provided by this resource is `to-debug-string`,
            /// which provides some human-readable information about the error.
            ///
            /// In the `wasi:io` package, this resource is returned through the
            /// `wasi:io/streams/stream-error` type.
            ///
            /// To provide more specific error information, other interfaces may
            /// provide functions to further "downcast" this error into more specific
            /// error information. For example, `error`s returned in streams derived
            /// from filesystem types to be described using the filesystem's own
            /// error-code type, using the function
            /// `wasi:filesystem/types/filesystem-error-code`, which takes a parameter
            /// `borrow<error>` and returns
            /// `option<wasi:filesystem/types/error-code>`.
            ///
            /// The set of functions which can "downcast" an `error` into a more
            /// concrete type is open.
            #[repr(transparent)]
            pub struct Error {
                handle: wit_bindgen::rt::Resource<Error>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Error {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Error",
                        "handle",
                        &&self.handle,
                    )
                }
            }
            impl Error {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: wit_bindgen::rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn into_handle(self) -> u32 {
                    wit_bindgen::rt::Resource::into_handle(self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    wit_bindgen::rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl wit_bindgen::rt::WasmResource for Error {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    ::core::panicking::panic("internal error: entered unreachable code");
                }
            }
            impl Error {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns a string that is suitable to assist humans in debugging
                /// this error.
                ///
                /// WARNING: The returned string should not be consumed mechanically!
                /// It may change across platforms, hosts, or other implementation
                /// details. Parsing this string is a major platform-compatibility
                /// hazard.
                pub fn to_debug_string(&self) -> wit_bindgen::rt::string::String {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 8]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *((ptr0 + 0) as *const i32);
                        let l2 = *((ptr0 + 4) as *const i32);
                        let len3 = l2 as usize;
                        let bytes3 = Vec::from_raw_parts(l1 as *mut _, len3, len3);
                        wit_bindgen::rt::string_lift(bytes3)
                    }
                }
            }
        }
        #[allow(clippy::all)]
        pub mod poll {
            /// `pollable` represents a single I/O event which may be ready, or not.
            #[repr(transparent)]
            pub struct Pollable {
                handle: wit_bindgen::rt::Resource<Pollable>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Pollable {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Pollable",
                        "handle",
                        &&self.handle,
                    )
                }
            }
            impl Pollable {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: wit_bindgen::rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn into_handle(self) -> u32 {
                    wit_bindgen::rt::Resource::into_handle(self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    wit_bindgen::rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl wit_bindgen::rt::WasmResource for Pollable {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    ::core::panicking::panic("internal error: entered unreachable code");
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                /// Return the readiness of a pollable. This function never blocks.
                ///
                /// Returns `true` when the pollable is ready, and `false` otherwise.
                pub fn ready(&self) -> bool {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        let ret = wit_import((self).handle() as i32);
                        wit_bindgen::rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                /// `block` returns immediately if the pollable is ready, and otherwise
                /// blocks until ready.
                ///
                /// This function is equivalent to calling `poll.poll` on a list
                /// containing only this pollable.
                pub fn block(&self) {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Poll for completion on a set of pollables.
            ///
            /// This function takes a list of pollables, which identify I/O sources of
            /// interest, and waits until one or more of the events is ready for I/O.
            ///
            /// The result `list<u32>` contains one or more indices of handles in the
            /// argument list that is ready for I/O.
            ///
            /// If the list contains more elements than can be indexed with a `u32`
            /// value, this function traps.
            ///
            /// A timeout can be implemented by adding a pollable from the
            /// wasi-clocks API to the list.
            ///
            /// This function does not return a `result`; polling in itself does not
            /// do any I/O so it doesn't fail. If any of the I/O sources identified by
            /// the pollables has an error, it is indicated by marking the source as
            /// being reaedy for I/O.
            pub fn poll(in_: &[&Pollable]) -> wit_bindgen::rt::vec::Vec<u32> {
                #[allow(unused_imports)]
                use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([u8; 8]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    let vec0 = in_;
                    let len0 = vec0.len() as i32;
                    let layout0 = alloc::Layout::from_size_align_unchecked(
                        vec0.len() * 4,
                        4,
                    );
                    let result0 = if layout0.size() != 0 {
                        let ptr = alloc::alloc(layout0);
                        if ptr.is_null() {
                            alloc::handle_alloc_error(layout0);
                        }
                        ptr
                    } else {
                        { ::core::ptr::null_mut() }
                    };
                    for (i, e) in vec0.into_iter().enumerate() {
                        let base = result0 as i32 + (i as i32) * 4;
                        {
                            *((base + 0) as *mut i32) = (e).handle() as i32;
                        }
                    }
                    let ptr1 = ret_area.as_mut_ptr() as i32;
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32, _: i32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                    wit_import(result0 as i32, len0, ptr1);
                    let l2 = *((ptr1 + 0) as *const i32);
                    let l3 = *((ptr1 + 4) as *const i32);
                    let len4 = l3 as usize;
                    if layout0.size() != 0 {
                        alloc::dealloc(result0, layout0);
                    }
                    Vec::from_raw_parts(l2 as *mut _, len4, len4)
                }
            }
        }
        #[allow(clippy::all)]
        pub mod streams {
            pub type Error = super::super::super::wasi::io::error::Error;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            /// An error for input-stream and output-stream operations.
            pub enum StreamError {
                /// The last operation (a write or flush) failed before completion.
                ///
                /// More information is available in the `error` payload.
                LastOperationFailed(Error),
                /// The stream is closed: no more input will be accepted by the
                /// stream. A closed output-stream will return this error on all
                /// future operations.
                Closed,
            }
            impl ::core::fmt::Debug for StreamError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        StreamError::LastOperationFailed(e) => {
                            f.debug_tuple("StreamError::LastOperationFailed")
                                .field(e)
                                .finish()
                        }
                        StreamError::Closed => {
                            f.debug_tuple("StreamError::Closed").finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for StreamError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.write_fmt(format_args!("{0:?}", self))
                }
            }
            impl std::error::Error for StreamError {}
            /// An input bytestream.
            ///
            /// `input-stream`s are *non-blocking* to the extent practical on underlying
            /// platforms. I/O operations always return promptly; if fewer bytes are
            /// promptly available than requested, they return the number of bytes promptly
            /// available, which could even be zero. To wait for data to be available,
            /// use the `subscribe` function to obtain a `pollable` which can be polled
            /// for using `wasi:io/poll`.
            #[repr(transparent)]
            pub struct InputStream {
                handle: wit_bindgen::rt::Resource<InputStream>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for InputStream {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InputStream",
                        "handle",
                        &&self.handle,
                    )
                }
            }
            impl InputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: wit_bindgen::rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn into_handle(self) -> u32 {
                    wit_bindgen::rt::Resource::into_handle(self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    wit_bindgen::rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl wit_bindgen::rt::WasmResource for InputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    ::core::panicking::panic("internal error: entered unreachable code");
                }
            }
            /// An output bytestream.
            ///
            /// `output-stream`s are *non-blocking* to the extent practical on
            /// underlying platforms. Except where specified otherwise, I/O operations also
            /// always return promptly, after the number of bytes that can be written
            /// promptly, which could even be zero. To wait for the stream to be ready to
            /// accept data, the `subscribe` function to obtain a `pollable` which can be
            /// polled for using `wasi:io/poll`.
            #[repr(transparent)]
            pub struct OutputStream {
                handle: wit_bindgen::rt::Resource<OutputStream>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for OutputStream {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "OutputStream",
                        "handle",
                        &&self.handle,
                    )
                }
            }
            impl OutputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: wit_bindgen::rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn into_handle(self) -> u32 {
                    wit_bindgen::rt::Resource::into_handle(self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    wit_bindgen::rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl wit_bindgen::rt::WasmResource for OutputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    ::core::panicking::panic("internal error: entered unreachable code");
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a non-blocking read from the stream.
                ///
                /// When the source of a `read` is binary data, the bytes from the source
                /// are returned verbatim. When the source of a `read` is known to the
                /// implementation to be text, bytes containing the UTF-8 encoding of the
                /// text are returned.
                ///
                /// This function returns a list of bytes containing the read data,
                /// when successful. The returned list will contain up to `len` bytes;
                /// it may return fewer than requested, but not more. The list is
                /// empty when no bytes are available for reading at this time. The
                /// pollable given by `subscribe` will be ready when more bytes are
                /// available.
                ///
                /// This function fails with a `stream-error` when the operation
                /// encounters an error, giving `last-operation-failed`, or when the
                /// stream is closed, giving `closed`.
                ///
                /// When the caller gives a `len` of 0, it represents a request to
                /// read 0 bytes. If the stream is still open, this call should
                /// succeed and return an empty list, or otherwise fail with `closed`.
                ///
                /// The `len` parameter is a `u64`, which could represent a list of u8 which
                /// is not possible to allocate in wasm32, or not desirable to allocate as
                /// as a return value by the callee. The callee may return a list of bytes
                /// less than `len` in size while more bytes are available for reading.
                pub fn read(
                    &self,
                    len: u64,
                ) -> Result<wit_bindgen::rt::vec::Vec<u8>, StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 12]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(len),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 4) as *const i32);
                                    let l3 = *((ptr0 + 8) as *const i32);
                                    let len4 = l3 as usize;
                                    Vec::from_raw_parts(l2 as *mut _, len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*((ptr0 + 4) as *const u8));
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *((ptr0 + 8) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read bytes from a stream, after blocking until at least one byte can
                /// be read. Except for blocking, behavior is identical to `read`.
                pub fn blocking_read(
                    &self,
                    len: u64,
                ) -> Result<wit_bindgen::rt::vec::Vec<u8>, StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 12]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(len),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 4) as *const i32);
                                    let l3 = *((ptr0 + 8) as *const i32);
                                    let len4 = l3 as usize;
                                    Vec::from_raw_parts(l2 as *mut _, len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*((ptr0 + 4) as *const u8));
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *((ptr0 + 8) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Skip bytes from a stream. Returns number of bytes skipped.
                ///
                /// Behaves identical to `read`, except instead of returning a list
                /// of bytes, returns the number of bytes consumed from the stream.
                pub fn skip(&self, len: u64) -> Result<u64, StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 16]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(len),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 8) as *const i64);
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *((ptr0 + 12) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Skip bytes from a stream, after blocking until at least one byte
                /// can be skipped. Except for blocking behavior, identical to `skip`.
                pub fn blocking_skip(&self, len: u64) -> Result<u64, StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 16]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(len),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 8) as *const i64);
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *((ptr0 + 12) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once either the specified stream
                /// has bytes available to read or the other end of the stream has been
                /// closed.
                /// The created `pollable` is a child resource of the `input-stream`.
                /// Implementations may trap if the `input-stream` is dropped before
                /// all derived `pollable`s created with this function are dropped.
                pub fn subscribe(&self) -> Pollable {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Check readiness for writing. This function never blocks.
                ///
                /// Returns the number of bytes permitted for the next call to `write`,
                /// or an error. Calling `write` with more bytes than this function has
                /// permitted will trap.
                ///
                /// When this function returns 0 bytes, the `subscribe` pollable will
                /// become ready when this function will report at least 1 byte, or an
                /// error.
                pub fn check_write(&self) -> Result<u64, StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 16]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 8) as *const i64);
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *((ptr0 + 12) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a write. This function never blocks.
                ///
                /// When the destination of a `write` is binary data, the bytes from
                /// `contents` are written verbatim. When the destination of a `write` is
                /// known to the implementation to be text, the bytes of `contents` are
                /// transcoded from UTF-8 into the encoding of the destination and then
                /// written.
                ///
                /// Precondition: check-write gave permit of Ok(n) and contents has a
                /// length of less than or equal to n. Otherwise, this function will trap.
                ///
                /// returns Err(closed) without writing if the stream has closed since
                /// the last call to check-write provided a permit.
                pub fn write(&self, contents: &[u8]) -> Result<(), StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 12]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr() as i32;
                        let len0 = vec0.len() as i32;
                        let ptr1 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0, len0, ptr1);
                        let l2 = i32::from(*((ptr1 + 0) as *const u8));
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr1 + 4) as *const u8));
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *((ptr1 + 8) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a write of up to 4096 bytes, and then flush the stream. Block
                /// until all of these operations are complete, or an error occurs.
                ///
                /// This is a convenience wrapper around the use of `check-write`,
                /// `subscribe`, `write`, and `flush`, and is implemented with the
                /// following pseudo-code:
                ///
                /// ```text
                /// let pollable = this.subscribe();
                /// while !contents.is_empty() {
                /// // Wait for the stream to become writable
                /// pollable.block();
                /// let Ok(n) = this.check-write(); // eliding error handling
                /// let len = min(n, contents.len());
                /// let (chunk, rest) = contents.split_at(len);
                /// this.write(chunk  );            // eliding error handling
                /// contents = rest;
                /// }
                /// this.flush();
                /// // Wait for completion of `flush`
                /// pollable.block();
                /// // Check for any errors that arose during `flush`
                /// let _ = this.check-write();         // eliding error handling
                /// ```
                pub fn blocking_write_and_flush(
                    &self,
                    contents: &[u8],
                ) -> Result<(), StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 12]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr() as i32;
                        let len0 = vec0.len() as i32;
                        let ptr1 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0, len0, ptr1);
                        let l2 = i32::from(*((ptr1 + 0) as *const u8));
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr1 + 4) as *const u8));
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *((ptr1 + 8) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Request to flush buffered output. This function never blocks.
                ///
                /// This tells the output-stream that the caller intends any buffered
                /// output to be flushed. the output which is expected to be flushed
                /// is all that has been passed to `write` prior to this call.
                ///
                /// Upon calling this function, the `output-stream` will not accept any
                /// writes (`check-write` will return `ok(0)`) until the flush has
                /// completed. The `subscribe` pollable will become ready when the
                /// flush has completed and the stream can accept more writes.
                pub fn flush(&self) -> Result<(), StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 12]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *((ptr0 + 8) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Request to flush buffered output, and block until flush completes
                /// and stream is ready for writing again.
                pub fn blocking_flush(&self) -> Result<(), StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 12]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *((ptr0 + 8) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once the output-stream
                /// is ready for more writing, or an error has occured. When this
                /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an
                /// error.
                ///
                /// If the stream is closed, this pollable is always ready immediately.
                ///
                /// The created `pollable` is a child resource of the `output-stream`.
                /// Implementations may trap if the `output-stream` is dropped before
                /// all derived `pollable`s created with this function are dropped.
                pub fn subscribe(&self) -> Pollable {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Write zeroes to a stream.
                ///
                /// This should be used precisely like `write` with the exact same
                /// preconditions (must use check-write first), but instead of
                /// passing a list of bytes, you simply pass the number of zero-bytes
                /// that should be written.
                pub fn write_zeroes(&self, len: u64) -> Result<(), StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 12]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(len),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *((ptr0 + 8) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a write of up to 4096 zeroes, and then flush the stream.
                /// Block until all of these operations are complete, or an error
                /// occurs.
                ///
                /// This is a convenience wrapper around the use of `check-write`,
                /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with
                /// the following pseudo-code:
                ///
                /// ```text
                /// let pollable = this.subscribe();
                /// while num_zeroes != 0 {
                /// // Wait for the stream to become writable
                /// pollable.block();
                /// let Ok(n) = this.check-write(); // eliding error handling
                /// let len = min(n, num_zeroes);
                /// this.write-zeroes(len);         // eliding error handling
                /// num_zeroes -= len;
                /// }
                /// this.flush();
                /// // Wait for completion of `flush`
                /// pollable.block();
                /// // Check for any errors that arose during `flush`
                /// let _ = this.check-write();         // eliding error handling
                /// ```
                pub fn blocking_write_zeroes_and_flush(
                    &self,
                    len: u64,
                ) -> Result<(), StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 12]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            wit_bindgen::rt::as_i64(len),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *((ptr0 + 8) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read from one stream and write to another.
                ///
                /// The behavior of splice is equivelant to:
                /// 1. calling `check-write` on the `output-stream`
                /// 2. calling `read` on the `input-stream` with the smaller of the
                /// `check-write` permitted length and the `len` provided to `splice`
                /// 3. calling `write` on the `output-stream` with that read data.
                ///
                /// Any error reported by the call to `check-write`, `read`, or
                /// `write` ends the splice and reports that error.
                ///
                /// This function returns the number of bytes transferred; it may be less
                /// than `len`.
                pub fn splice(
                    &self,
                    src: &InputStream,
                    len: u64,
                ) -> Result<u64, StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 16]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            wit_bindgen::rt::as_i64(len),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 8) as *const i64);
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *((ptr0 + 12) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read from one stream and write to another, with blocking.
                ///
                /// This is similar to `splice`, except that it blocks until the
                /// `output-stream` is ready for writing, and the `input-stream`
                /// is ready for reading, before performing the `splice`.
                pub fn blocking_splice(
                    &self,
                    src: &InputStream,
                    len: u64,
                ) -> Result<u64, StreamError> {
                    #[allow(unused_imports)]
                    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([u8; 16]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let ptr0 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            wit_bindgen::rt::as_i64(len),
                            ptr0,
                        );
                        let l1 = i32::from(*((ptr0 + 0) as *const u8));
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *((ptr0 + 8) as *const i64);
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *((ptr0 + 12) as *const i32);
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => wit_bindgen::rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
}
const _: &str = "package wasi:io@0.2.0;\n\n/// A poll API intended to let users wait for I/O events on multiple handles\n/// at once.\ninterface poll {\n    /// `pollable` represents a single I/O event which may be ready, or not.\n    resource pollable {\n\n      /// Return the readiness of a pollable. This function never blocks.\n      ///\n      /// Returns `true` when the pollable is ready, and `false` otherwise.\n      ready: func() -> bool;\n\n      /// `block` returns immediately if the pollable is ready, and otherwise\n      /// blocks until ready.\n      ///\n      /// This function is equivalent to calling `poll.poll` on a list\n      /// containing only this pollable.\n      block: func();\n    }\n\n    /// Poll for completion on a set of pollables.\n    ///\n    /// This function takes a list of pollables, which identify I/O sources of\n    /// interest, and waits until one or more of the events is ready for I/O.\n    ///\n    /// The result `list<u32>` contains one or more indices of handles in the\n    /// argument list that is ready for I/O.\n    ///\n    /// If the list contains more elements than can be indexed with a `u32`\n    /// value, this function traps.\n    ///\n    /// A timeout can be implemented by adding a pollable from the\n    /// wasi-clocks API to the list.\n    ///\n    /// This function does not return a `result`; polling in itself does not\n    /// do any I/O so it doesn\'t fail. If any of the I/O sources identified by\n    /// the pollables has an error, it is indicated by marking the source as\n    /// being reaedy for I/O.\n    poll: func(in: list<borrow<pollable>>) -> list<u32>;\n}\n";
const _: &str = "package wasi:io@0.2.0;\n\n/// WASI I/O is an I/O abstraction API which is currently focused on providing\n/// stream types.\n///\n/// In the future, the component model is expected to add built-in stream types;\n/// when it does, they are expected to subsume this API.\ninterface streams {\n    use error.{error};\n    use poll.{pollable};\n\n    /// An error for input-stream and output-stream operations.\n    variant stream-error {\n        /// The last operation (a write or flush) failed before completion.\n        ///\n        /// More information is available in the `error` payload.\n        last-operation-failed(error),\n        /// The stream is closed: no more input will be accepted by the\n        /// stream. A closed output-stream will return this error on all\n        /// future operations.\n        closed\n    }\n\n    /// An input bytestream.\n    ///\n    /// `input-stream`s are *non-blocking* to the extent practical on underlying\n    /// platforms. I/O operations always return promptly; if fewer bytes are\n    /// promptly available than requested, they return the number of bytes promptly\n    /// available, which could even be zero. To wait for data to be available,\n    /// use the `subscribe` function to obtain a `pollable` which can be polled\n    /// for using `wasi:io/poll`.\n    resource input-stream {\n        /// Perform a non-blocking read from the stream.\n        ///\n        /// When the source of a `read` is binary data, the bytes from the source\n        /// are returned verbatim. When the source of a `read` is known to the\n        /// implementation to be text, bytes containing the UTF-8 encoding of the\n        /// text are returned.\n        ///\n        /// This function returns a list of bytes containing the read data,\n        /// when successful. The returned list will contain up to `len` bytes;\n        /// it may return fewer than requested, but not more. The list is\n        /// empty when no bytes are available for reading at this time. The\n        /// pollable given by `subscribe` will be ready when more bytes are\n        /// available.\n        ///\n        /// This function fails with a `stream-error` when the operation\n        /// encounters an error, giving `last-operation-failed`, or when the\n        /// stream is closed, giving `closed`.\n        ///\n        /// When the caller gives a `len` of 0, it represents a request to\n        /// read 0 bytes. If the stream is still open, this call should\n        /// succeed and return an empty list, or otherwise fail with `closed`.\n        ///\n        /// The `len` parameter is a `u64`, which could represent a list of u8 which\n        /// is not possible to allocate in wasm32, or not desirable to allocate as\n        /// as a return value by the callee. The callee may return a list of bytes\n        /// less than `len` in size while more bytes are available for reading.\n        read: func(\n            /// The maximum number of bytes to read\n            len: u64\n        ) -> result<list<u8>, stream-error>;\n\n        /// Read bytes from a stream, after blocking until at least one byte can\n        /// be read. Except for blocking, behavior is identical to `read`.\n        blocking-read: func(\n            /// The maximum number of bytes to read\n            len: u64\n        ) -> result<list<u8>, stream-error>;\n\n        /// Skip bytes from a stream. Returns number of bytes skipped.\n        ///\n        /// Behaves identical to `read`, except instead of returning a list\n        /// of bytes, returns the number of bytes consumed from the stream.\n        skip: func(\n            /// The maximum number of bytes to skip.\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Skip bytes from a stream, after blocking until at least one byte\n        /// can be skipped. Except for blocking behavior, identical to `skip`.\n        blocking-skip: func(\n            /// The maximum number of bytes to skip.\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Create a `pollable` which will resolve once either the specified stream\n        /// has bytes available to read or the other end of the stream has been\n        /// closed.\n        /// The created `pollable` is a child resource of the `input-stream`.\n        /// Implementations may trap if the `input-stream` is dropped before\n        /// all derived `pollable`s created with this function are dropped.\n        subscribe: func() -> pollable;\n    }\n\n\n    /// An output bytestream.\n    ///\n    /// `output-stream`s are *non-blocking* to the extent practical on\n    /// underlying platforms. Except where specified otherwise, I/O operations also\n    /// always return promptly, after the number of bytes that can be written\n    /// promptly, which could even be zero. To wait for the stream to be ready to\n    /// accept data, the `subscribe` function to obtain a `pollable` which can be\n    /// polled for using `wasi:io/poll`.\n    resource output-stream {\n        /// Check readiness for writing. This function never blocks.\n        ///\n        /// Returns the number of bytes permitted for the next call to `write`,\n        /// or an error. Calling `write` with more bytes than this function has\n        /// permitted will trap.\n        ///\n        /// When this function returns 0 bytes, the `subscribe` pollable will\n        /// become ready when this function will report at least 1 byte, or an\n        /// error.\n        check-write: func() -> result<u64, stream-error>;\n\n        /// Perform a write. This function never blocks.\n        ///\n        /// When the destination of a `write` is binary data, the bytes from\n        /// `contents` are written verbatim. When the destination of a `write` is\n        /// known to the implementation to be text, the bytes of `contents` are\n        /// transcoded from UTF-8 into the encoding of the destination and then\n        /// written.\n        ///\n        /// Precondition: check-write gave permit of Ok(n) and contents has a\n        /// length of less than or equal to n. Otherwise, this function will trap.\n        ///\n        /// returns Err(closed) without writing if the stream has closed since\n        /// the last call to check-write provided a permit.\n        write: func(\n            contents: list<u8>\n        ) -> result<_, stream-error>;\n\n        /// Perform a write of up to 4096 bytes, and then flush the stream. Block\n        /// until all of these operations are complete, or an error occurs.\n        ///\n        /// This is a convenience wrapper around the use of `check-write`,\n        /// `subscribe`, `write`, and `flush`, and is implemented with the\n        /// following pseudo-code:\n        ///\n        /// ```text\n        /// let pollable = this.subscribe();\n        /// while !contents.is_empty() {\n        ///     // Wait for the stream to become writable\n        ///     pollable.block();\n        ///     let Ok(n) = this.check-write(); // eliding error handling\n        ///     let len = min(n, contents.len());\n        ///     let (chunk, rest) = contents.split_at(len);\n        ///     this.write(chunk  );            // eliding error handling\n        ///     contents = rest;\n        /// }\n        /// this.flush();\n        /// // Wait for completion of `flush`\n        /// pollable.block();\n        /// // Check for any errors that arose during `flush`\n        /// let _ = this.check-write();         // eliding error handling\n        /// ```\n        blocking-write-and-flush: func(\n            contents: list<u8>\n        ) -> result<_, stream-error>;\n\n        /// Request to flush buffered output. This function never blocks.\n        ///\n        /// This tells the output-stream that the caller intends any buffered\n        /// output to be flushed. the output which is expected to be flushed\n        /// is all that has been passed to `write` prior to this call.\n        ///\n        /// Upon calling this function, the `output-stream` will not accept any\n        /// writes (`check-write` will return `ok(0)`) until the flush has\n        /// completed. The `subscribe` pollable will become ready when the\n        /// flush has completed and the stream can accept more writes.\n        flush: func() -> result<_, stream-error>;\n\n        /// Request to flush buffered output, and block until flush completes\n        /// and stream is ready for writing again.\n        blocking-flush: func() -> result<_, stream-error>;\n\n        /// Create a `pollable` which will resolve once the output-stream\n        /// is ready for more writing, or an error has occured. When this\n        /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an\n        /// error.\n        ///\n        /// If the stream is closed, this pollable is always ready immediately.\n        ///\n        /// The created `pollable` is a child resource of the `output-stream`.\n        /// Implementations may trap if the `output-stream` is dropped before\n        /// all derived `pollable`s created with this function are dropped.\n        subscribe: func() -> pollable;\n\n        /// Write zeroes to a stream.\n        ///\n        /// This should be used precisely like `write` with the exact same\n        /// preconditions (must use check-write first), but instead of\n        /// passing a list of bytes, you simply pass the number of zero-bytes\n        /// that should be written.\n        write-zeroes: func(\n            /// The number of zero-bytes to write\n            len: u64\n        ) -> result<_, stream-error>;\n\n        /// Perform a write of up to 4096 zeroes, and then flush the stream.\n        /// Block until all of these operations are complete, or an error\n        /// occurs.\n        ///\n        /// This is a convenience wrapper around the use of `check-write`,\n        /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with\n        /// the following pseudo-code:\n        ///\n        /// ```text\n        /// let pollable = this.subscribe();\n        /// while num_zeroes != 0 {\n        ///     // Wait for the stream to become writable\n        ///     pollable.block();\n        ///     let Ok(n) = this.check-write(); // eliding error handling\n        ///     let len = min(n, num_zeroes);\n        ///     this.write-zeroes(len);         // eliding error handling\n        ///     num_zeroes -= len;\n        /// }\n        /// this.flush();\n        /// // Wait for completion of `flush`\n        /// pollable.block();\n        /// // Check for any errors that arose during `flush`\n        /// let _ = this.check-write();         // eliding error handling\n        /// ```\n        blocking-write-zeroes-and-flush: func(\n            /// The number of zero-bytes to write\n            len: u64\n        ) -> result<_, stream-error>;\n\n        /// Read from one stream and write to another.\n        ///\n        /// The behavior of splice is equivelant to:\n        /// 1. calling `check-write` on the `output-stream`\n        /// 2. calling `read` on the `input-stream` with the smaller of the\n        /// `check-write` permitted length and the `len` provided to `splice`\n        /// 3. calling `write` on the `output-stream` with that read data.\n        ///\n        /// Any error reported by the call to `check-write`, `read`, or\n        /// `write` ends the splice and reports that error.\n        ///\n        /// This function returns the number of bytes transferred; it may be less\n        /// than `len`.\n        splice: func(\n            /// The stream to read from\n            src: borrow<input-stream>,\n            /// The number of bytes to splice\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Read from one stream and write to another, with blocking.\n        ///\n        /// This is similar to `splice`, except that it blocks until the\n        /// `output-stream` is ready for writing, and the `input-stream`\n        /// is ready for reading, before performing the `splice`.\n        blocking-splice: func(\n            /// The stream to read from\n            src: borrow<input-stream>,\n            /// The number of bytes to splice\n            len: u64,\n        ) -> result<u64, stream-error>;\n    }\n}\n";
const _: &str = "package wasi:io@0.2.0;\n\nworld imports {\n    import streams;\n    import poll;\n}\n";
const _: &str = "package wasi:io@0.2.0;\n\n\ninterface error {\n    /// A resource which represents some error information.\n    ///\n    /// The only method provided by this resource is `to-debug-string`,\n    /// which provides some human-readable information about the error.\n    ///\n    /// In the `wasi:io` package, this resource is returned through the\n    /// `wasi:io/streams/stream-error` type.\n    ///\n    /// To provide more specific error information, other interfaces may\n    /// provide functions to further \"downcast\" this error into more specific\n    /// error information. For example, `error`s returned in streams derived\n    /// from filesystem types to be described using the filesystem\'s own\n    /// error-code type, using the function\n    /// `wasi:filesystem/types/filesystem-error-code`, which takes a parameter\n    /// `borrow<error>` and returns\n    /// `option<wasi:filesystem/types/error-code>`.\n    ///\n    /// The set of functions which can \"downcast\" an `error` into a more\n    /// concrete type is open.\n    resource error {\n        /// Returns a string that is suitable to assist humans in debugging\n        /// this error.\n        ///\n        /// WARNING: The returned string should not be consumed mechanically!\n        /// It may change across platforms, hosts, or other implementation\n        /// details. Parsing this string is a major platform-compatibility\n        /// hazard.\n        to-debug-string: func() -> string;\n    }\n}\n";
const _: &str = "package wasi:clocks@0.2.0;\n/// WASI Wall Clock is a clock API intended to let users query the current\n/// time. The name \"wall\" makes an analogy to a \"clock on the wall\", which\n/// is not necessarily monotonic as it may be reset.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\n///\n/// A wall clock is a clock which measures the date and time according to\n/// some external reference.\n///\n/// External references may be reset, so this clock is not necessarily\n/// monotonic, making it unsuitable for measuring elapsed time.\n///\n/// It is intended for reporting the current date and time for humans.\ninterface wall-clock {\n    /// A time and date in seconds plus nanoseconds.\n    record datetime {\n        seconds: u64,\n        nanoseconds: u32,\n    }\n\n    /// Read the current value of the clock.\n    ///\n    /// This clock is not monotonic, therefore calling this function repeatedly\n    /// will not necessarily produce a sequence of non-decreasing values.\n    ///\n    /// The returned timestamps represent the number of seconds since\n    /// 1970-01-01T00:00:00Z, also known as [POSIX\'s Seconds Since the Epoch],\n    /// also known as [Unix Time].\n    ///\n    /// The nanoseconds field of the output is always less than 1000000000.\n    ///\n    /// [POSIX\'s Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16\n    /// [Unix Time]: https://en.wikipedia.org/wiki/Unix_time\n    now: func() -> datetime;\n\n    /// Query the resolution of the clock.\n    ///\n    /// The nanoseconds field of the output is always less than 1000000000.\n    resolution: func() -> datetime;\n}\n";
const _: &str = "package wasi:clocks@0.2.0;\n\nworld imports {\n    import monotonic-clock;\n    import wall-clock;\n}\n";
const _: &str = "package wasi:clocks@0.2.0;\n/// WASI Monotonic Clock is a clock API intended to let users measure elapsed\n/// time.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\n///\n/// A monotonic clock is a clock which has an unspecified initial value, and\n/// successive reads of the clock will produce non-decreasing values.\n///\n/// It is intended for measuring elapsed time.\ninterface monotonic-clock {\n    use wasi:io/poll@0.2.0.{pollable};\n\n    /// An instant in time, in nanoseconds. An instant is relative to an\n    /// unspecified initial value, and can only be compared to instances from\n    /// the same monotonic-clock.\n    type instant = u64;\n\n    /// A duration of time, in nanoseconds.\n    type duration = u64;\n\n    /// Read the current value of the clock.\n    ///\n    /// The clock is monotonic, therefore calling this function repeatedly will\n    /// produce a sequence of non-decreasing values.\n    now: func() -> instant;\n\n    /// Query the resolution of the clock. Returns the duration of time\n    /// corresponding to a clock tick.\n    resolution: func() -> duration;\n\n    /// Create a `pollable` which will resolve once the specified instant\n    /// occured.\n    subscribe-instant: func(\n        when: instant,\n    ) -> pollable;\n\n    /// Create a `pollable` which will resolve once the given duration has\n    /// elapsed, starting at the time at which this function was called.\n    /// occured.\n    subscribe-duration: func(\n        when: duration,\n    ) -> pollable;\n}\n";
const _: &str = "package wasi:filesystem@0.2.0;\n/// WASI filesystem is a filesystem API primarily intended to let users run WASI\n/// programs that access their files on their existing filesystems, without\n/// significant overhead.\n///\n/// It is intended to be roughly portable between Unix-family platforms and\n/// Windows, though it does not hide many of the major differences.\n///\n/// Paths are passed as interface-type `string`s, meaning they must consist of\n/// a sequence of Unicode Scalar Values (USVs). Some filesystems may contain\n/// paths which are not accessible by this API.\n///\n/// The directory separator in WASI is always the forward-slash (`/`).\n///\n/// All paths in WASI are relative paths, and are interpreted relative to a\n/// `descriptor` referring to a base directory. If a `path` argument to any WASI\n/// function starts with `/`, or if any step of resolving a `path`, including\n/// `..` and symbolic link steps, reaches a directory outside of the base\n/// directory, or reaches a symlink to an absolute or rooted path in the\n/// underlying filesystem, the function fails with `error-code::not-permitted`.\n///\n/// For more information about WASI path resolution and sandboxing, see\n/// [WASI filesystem path resolution].\n///\n/// [WASI filesystem path resolution]: https://github.com/WebAssembly/wasi-filesystem/blob/main/path-resolution.md\ninterface types {\n    use wasi:io/streams@0.2.0.{input-stream, output-stream, error};\n    use wasi:clocks/wall-clock@0.2.0.{datetime};\n\n    /// File size or length of a region within a file.\n    type filesize = u64;\n\n    /// The type of a filesystem object referenced by a descriptor.\n    ///\n    /// Note: This was called `filetype` in earlier versions of WASI.\n    enum descriptor-type {\n        /// The type of the descriptor or file is unknown or is different from\n        /// any of the other types specified.\n        unknown,\n        /// The descriptor refers to a block device inode.\n        block-device,\n        /// The descriptor refers to a character device inode.\n        character-device,\n        /// The descriptor refers to a directory inode.\n        directory,\n        /// The descriptor refers to a named pipe.\n        fifo,\n        /// The file refers to a symbolic link inode.\n        symbolic-link,\n        /// The descriptor refers to a regular file inode.\n        regular-file,\n        /// The descriptor refers to a socket.\n        socket,\n    }\n\n    /// Descriptor flags.\n    ///\n    /// Note: This was called `fdflags` in earlier versions of WASI.\n    flags descriptor-flags {\n        /// Read mode: Data can be read.\n        read,\n        /// Write mode: Data can be written to.\n        write,\n        /// Request that writes be performed according to synchronized I/O file\n        /// integrity completion. The data stored in the file and the file\'s\n        /// metadata are synchronized. This is similar to `O_SYNC` in POSIX.\n        ///\n        /// The precise semantics of this operation have not yet been defined for\n        /// WASI. At this time, it should be interpreted as a request, and not a\n        /// requirement.\n        file-integrity-sync,\n        /// Request that writes be performed according to synchronized I/O data\n        /// integrity completion. Only the data stored in the file is\n        /// synchronized. This is similar to `O_DSYNC` in POSIX.\n        ///\n        /// The precise semantics of this operation have not yet been defined for\n        /// WASI. At this time, it should be interpreted as a request, and not a\n        /// requirement.\n        data-integrity-sync,\n        /// Requests that reads be performed at the same level of integrety\n        /// requested for writes. This is similar to `O_RSYNC` in POSIX.\n        ///\n        /// The precise semantics of this operation have not yet been defined for\n        /// WASI. At this time, it should be interpreted as a request, and not a\n        /// requirement.\n        requested-write-sync,\n        /// Mutating directories mode: Directory contents may be mutated.\n        ///\n        /// When this flag is unset on a descriptor, operations using the\n        /// descriptor which would create, rename, delete, modify the data or\n        /// metadata of filesystem objects, or obtain another handle which\n        /// would permit any of those, shall fail with `error-code::read-only` if\n        /// they would otherwise succeed.\n        ///\n        /// This may only be set on directories.\n        mutate-directory,\n    }\n\n    /// File attributes.\n    ///\n    /// Note: This was called `filestat` in earlier versions of WASI.\n    record descriptor-stat {\n        /// File type.\n        %type: descriptor-type,\n        /// Number of hard links to the file.\n        link-count: link-count,\n        /// For regular files, the file size in bytes. For symbolic links, the\n        /// length in bytes of the pathname contained in the symbolic link.\n        size: filesize,\n        /// Last data access timestamp.\n        ///\n        /// If the `option` is none, the platform doesn\'t maintain an access\n        /// timestamp for this file.\n        data-access-timestamp: option<datetime>,\n        /// Last data modification timestamp.\n        ///\n        /// If the `option` is none, the platform doesn\'t maintain a\n        /// modification timestamp for this file.\n        data-modification-timestamp: option<datetime>,\n        /// Last file status-change timestamp.\n        ///\n        /// If the `option` is none, the platform doesn\'t maintain a\n        /// status-change timestamp for this file.\n        status-change-timestamp: option<datetime>,\n    }\n\n    /// Flags determining the method of how paths are resolved.\n    flags path-flags {\n        /// As long as the resolved path corresponds to a symbolic link, it is\n        /// expanded.\n        symlink-follow,\n    }\n\n    /// Open flags used by `open-at`.\n    flags open-flags {\n        /// Create file if it does not exist, similar to `O_CREAT` in POSIX.\n        create,\n        /// Fail if not a directory, similar to `O_DIRECTORY` in POSIX.\n        directory,\n        /// Fail if file already exists, similar to `O_EXCL` in POSIX.\n        exclusive,\n        /// Truncate file to size 0, similar to `O_TRUNC` in POSIX.\n        truncate,\n    }\n\n    /// Number of hard links to an inode.\n    type link-count = u64;\n\n    /// When setting a timestamp, this gives the value to set it to.\n    variant new-timestamp {\n        /// Leave the timestamp set to its previous value.\n        no-change,\n        /// Set the timestamp to the current time of the system clock associated\n        /// with the filesystem.\n        now,\n        /// Set the timestamp to the given value.\n        timestamp(datetime),\n    }\n\n    /// A directory entry.\n    record directory-entry {\n        /// The type of the file referred to by this directory entry.\n        %type: descriptor-type,\n\n        /// The name of the object.\n        name: string,\n    }\n\n    /// Error codes returned by functions, similar to `errno` in POSIX.\n    /// Not all of these error codes are returned by the functions provided by this\n    /// API; some are used in higher-level library layers, and others are provided\n    /// merely for alignment with POSIX.\n    enum error-code {\n        /// Permission denied, similar to `EACCES` in POSIX.\n        access,\n        /// Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX.\n        would-block,\n        /// Connection already in progress, similar to `EALREADY` in POSIX.\n        already,\n        /// Bad descriptor, similar to `EBADF` in POSIX.\n        bad-descriptor,\n        /// Device or resource busy, similar to `EBUSY` in POSIX.\n        busy,\n        /// Resource deadlock would occur, similar to `EDEADLK` in POSIX.\n        deadlock,\n        /// Storage quota exceeded, similar to `EDQUOT` in POSIX.\n        quota,\n        /// File exists, similar to `EEXIST` in POSIX.\n        exist,\n        /// File too large, similar to `EFBIG` in POSIX.\n        file-too-large,\n        /// Illegal byte sequence, similar to `EILSEQ` in POSIX.\n        illegal-byte-sequence,\n        /// Operation in progress, similar to `EINPROGRESS` in POSIX.\n        in-progress,\n        /// Interrupted function, similar to `EINTR` in POSIX.\n        interrupted,\n        /// Invalid argument, similar to `EINVAL` in POSIX.\n        invalid,\n        /// I/O error, similar to `EIO` in POSIX.\n        io,\n        /// Is a directory, similar to `EISDIR` in POSIX.\n        is-directory,\n        /// Too many levels of symbolic links, similar to `ELOOP` in POSIX.\n        loop,\n        /// Too many links, similar to `EMLINK` in POSIX.\n        too-many-links,\n        /// Message too large, similar to `EMSGSIZE` in POSIX.\n        message-size,\n        /// Filename too long, similar to `ENAMETOOLONG` in POSIX.\n        name-too-long,\n        /// No such device, similar to `ENODEV` in POSIX.\n        no-device,\n        /// No such file or directory, similar to `ENOENT` in POSIX.\n        no-entry,\n        /// No locks available, similar to `ENOLCK` in POSIX.\n        no-lock,\n        /// Not enough space, similar to `ENOMEM` in POSIX.\n        insufficient-memory,\n        /// No space left on device, similar to `ENOSPC` in POSIX.\n        insufficient-space,\n        /// Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX.\n        not-directory,\n        /// Directory not empty, similar to `ENOTEMPTY` in POSIX.\n        not-empty,\n        /// State not recoverable, similar to `ENOTRECOVERABLE` in POSIX.\n        not-recoverable,\n        /// Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX.\n        unsupported,\n        /// Inappropriate I/O control operation, similar to `ENOTTY` in POSIX.\n        no-tty,\n        /// No such device or address, similar to `ENXIO` in POSIX.\n        no-such-device,\n        /// Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX.\n        overflow,\n        /// Operation not permitted, similar to `EPERM` in POSIX.\n        not-permitted,\n        /// Broken pipe, similar to `EPIPE` in POSIX.\n        pipe,\n        /// Read-only file system, similar to `EROFS` in POSIX.\n        read-only,\n        /// Invalid seek, similar to `ESPIPE` in POSIX.\n        invalid-seek,\n        /// Text file busy, similar to `ETXTBSY` in POSIX.\n        text-file-busy,\n        /// Cross-device link, similar to `EXDEV` in POSIX.\n        cross-device,\n    }\n\n    /// File or memory access pattern advisory information.\n    enum advice {\n        /// The application has no advice to give on its behavior with respect\n        /// to the specified data.\n        normal,\n        /// The application expects to access the specified data sequentially\n        /// from lower offsets to higher offsets.\n        sequential,\n        /// The application expects to access the specified data in a random\n        /// order.\n        random,\n        /// The application expects to access the specified data in the near\n        /// future.\n        will-need,\n        /// The application expects that it will not access the specified data\n        /// in the near future.\n        dont-need,\n        /// The application expects to access the specified data once and then\n        /// not reuse it thereafter.\n        no-reuse,\n    }\n\n    /// A 128-bit hash value, split into parts because wasm doesn\'t have a\n    /// 128-bit integer type.\n    record metadata-hash-value {\n       /// 64 bits of a 128-bit hash value.\n       lower: u64,\n       /// Another 64 bits of a 128-bit hash value.\n       upper: u64,\n    }\n\n    /// A descriptor is a reference to a filesystem object, which may be a file,\n    /// directory, named pipe, special file, or other object on which filesystem\n    /// calls may be made.\n    resource descriptor {\n        /// Return a stream for reading from a file, if available.\n        ///\n        /// May fail with an error-code describing why the file cannot be read.\n        ///\n        /// Multiple read, write, and append streams may be active on the same open\n        /// file and they do not interfere with each other.\n        ///\n        /// Note: This allows using `read-stream`, which is similar to `read` in POSIX.\n        read-via-stream: func(\n            /// The offset within the file at which to start reading.\n            offset: filesize,\n        ) -> result<input-stream, error-code>;\n\n        /// Return a stream for writing to a file, if available.\n        ///\n        /// May fail with an error-code describing why the file cannot be written.\n        ///\n        /// Note: This allows using `write-stream`, which is similar to `write` in\n        /// POSIX.\n        write-via-stream: func(\n            /// The offset within the file at which to start writing.\n            offset: filesize,\n        ) -> result<output-stream, error-code>;\n\n        /// Return a stream for appending to a file, if available.\n        ///\n        /// May fail with an error-code describing why the file cannot be appended.\n        ///\n        /// Note: This allows using `write-stream`, which is similar to `write` with\n        /// `O_APPEND` in in POSIX.\n        append-via-stream: func() -> result<output-stream, error-code>;\n\n        /// Provide file advisory information on a descriptor.\n        ///\n        /// This is similar to `posix_fadvise` in POSIX.\n        advise: func(\n            /// The offset within the file to which the advisory applies.\n            offset: filesize,\n            /// The length of the region to which the advisory applies.\n            length: filesize,\n            /// The advice.\n            advice: advice\n        ) -> result<_, error-code>;\n\n        /// Synchronize the data of a file to disk.\n        ///\n        /// This function succeeds with no effect if the file descriptor is not\n        /// opened for writing.\n        ///\n        /// Note: This is similar to `fdatasync` in POSIX.\n        sync-data: func() -> result<_, error-code>;\n\n        /// Get flags associated with a descriptor.\n        ///\n        /// Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.\n        ///\n        /// Note: This returns the value that was the `fs_flags` value returned\n        /// from `fdstat_get` in earlier versions of WASI.\n        get-flags: func() -> result<descriptor-flags, error-code>;\n\n        /// Get the dynamic type of a descriptor.\n        ///\n        /// Note: This returns the same value as the `type` field of the `fd-stat`\n        /// returned by `stat`, `stat-at` and similar.\n        ///\n        /// Note: This returns similar flags to the `st_mode & S_IFMT` value provided\n        /// by `fstat` in POSIX.\n        ///\n        /// Note: This returns the value that was the `fs_filetype` value returned\n        /// from `fdstat_get` in earlier versions of WASI.\n        get-type: func() -> result<descriptor-type, error-code>;\n\n        /// Adjust the size of an open file. If this increases the file\'s size, the\n        /// extra bytes are filled with zeros.\n        ///\n        /// Note: This was called `fd_filestat_set_size` in earlier versions of WASI.\n        set-size: func(size: filesize) -> result<_, error-code>;\n\n        /// Adjust the timestamps of an open file or directory.\n        ///\n        /// Note: This is similar to `futimens` in POSIX.\n        ///\n        /// Note: This was called `fd_filestat_set_times` in earlier versions of WASI.\n        set-times: func(\n            /// The desired values of the data access timestamp.\n            data-access-timestamp: new-timestamp,\n            /// The desired values of the data modification timestamp.\n            data-modification-timestamp: new-timestamp,\n        ) -> result<_, error-code>;\n\n        /// Read from a descriptor, without using and updating the descriptor\'s offset.\n        ///\n        /// This function returns a list of bytes containing the data that was\n        /// read, along with a bool which, when true, indicates that the end of the\n        /// file was reached. The returned list will contain up to `length` bytes; it\n        /// may return fewer than requested, if the end of the file is reached or\n        /// if the I/O operation is interrupted.\n        ///\n        /// In the future, this may change to return a `stream<u8, error-code>`.\n        ///\n        /// Note: This is similar to `pread` in POSIX.\n        read: func(\n            /// The maximum number of bytes to read.\n            length: filesize,\n            /// The offset within the file at which to read.\n            offset: filesize,\n        ) -> result<tuple<list<u8>, bool>, error-code>;\n\n        /// Write to a descriptor, without using and updating the descriptor\'s offset.\n        ///\n        /// It is valid to write past the end of a file; the file is extended to the\n        /// extent of the write, with bytes between the previous end and the start of\n        /// the write set to zero.\n        ///\n        /// In the future, this may change to take a `stream<u8, error-code>`.\n        ///\n        /// Note: This is similar to `pwrite` in POSIX.\n        write: func(\n            /// Data to write\n            buffer: list<u8>,\n            /// The offset within the file at which to write.\n            offset: filesize,\n        ) -> result<filesize, error-code>;\n\n        /// Read directory entries from a directory.\n        ///\n        /// On filesystems where directories contain entries referring to themselves\n        /// and their parents, often named `.` and `..` respectively, these entries\n        /// are omitted.\n        ///\n        /// This always returns a new stream which starts at the beginning of the\n        /// directory. Multiple streams may be active on the same directory, and they\n        /// do not interfere with each other.\n        read-directory: func() -> result<directory-entry-stream, error-code>;\n\n        /// Synchronize the data and metadata of a file to disk.\n        ///\n        /// This function succeeds with no effect if the file descriptor is not\n        /// opened for writing.\n        ///\n        /// Note: This is similar to `fsync` in POSIX.\n        sync: func() -> result<_, error-code>;\n\n        /// Create a directory.\n        ///\n        /// Note: This is similar to `mkdirat` in POSIX.\n        create-directory-at: func(\n            /// The relative path at which to create the directory.\n            path: string,\n        ) -> result<_, error-code>;\n\n        /// Return the attributes of an open file or directory.\n        ///\n        /// Note: This is similar to `fstat` in POSIX, except that it does not return\n        /// device and inode information. For testing whether two descriptors refer to\n        /// the same underlying filesystem object, use `is-same-object`. To obtain\n        /// additional data that can be used do determine whether a file has been\n        /// modified, use `metadata-hash`.\n        ///\n        /// Note: This was called `fd_filestat_get` in earlier versions of WASI.\n        stat: func() -> result<descriptor-stat, error-code>;\n\n        /// Return the attributes of a file or directory.\n        ///\n        /// Note: This is similar to `fstatat` in POSIX, except that it does not\n        /// return device and inode information. See the `stat` description for a\n        /// discussion of alternatives.\n        ///\n        /// Note: This was called `path_filestat_get` in earlier versions of WASI.\n        stat-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the file or directory to inspect.\n            path: string,\n        ) -> result<descriptor-stat, error-code>;\n\n        /// Adjust the timestamps of a file or directory.\n        ///\n        /// Note: This is similar to `utimensat` in POSIX.\n        ///\n        /// Note: This was called `path_filestat_set_times` in earlier versions of\n        /// WASI.\n        set-times-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the file or directory to operate on.\n            path: string,\n            /// The desired values of the data access timestamp.\n            data-access-timestamp: new-timestamp,\n            /// The desired values of the data modification timestamp.\n            data-modification-timestamp: new-timestamp,\n        ) -> result<_, error-code>;\n\n        /// Create a hard link.\n        ///\n        /// Note: This is similar to `linkat` in POSIX.\n        link-at: func(\n            /// Flags determining the method of how the path is resolved.\n            old-path-flags: path-flags,\n            /// The relative source path from which to link.\n            old-path: string,\n            /// The base directory for `new-path`.\n            new-descriptor: borrow<descriptor>,\n            /// The relative destination path at which to create the hard link.\n            new-path: string,\n        ) -> result<_, error-code>;\n\n        /// Open a file or directory.\n        ///\n        /// The returned descriptor is not guaranteed to be the lowest-numbered\n        /// descriptor not currently open/ it is randomized to prevent applications\n        /// from depending on making assumptions about indexes, since this is\n        /// error-prone in multi-threaded contexts. The returned descriptor is\n        /// guaranteed to be less than 2**31.\n        ///\n        /// If `flags` contains `descriptor-flags::mutate-directory`, and the base\n        /// descriptor doesn\'t have `descriptor-flags::mutate-directory` set,\n        /// `open-at` fails with `error-code::read-only`.\n        ///\n        /// If `flags` contains `write` or `mutate-directory`, or `open-flags`\n        /// contains `truncate` or `create`, and the base descriptor doesn\'t have\n        /// `descriptor-flags::mutate-directory` set, `open-at` fails with\n        /// `error-code::read-only`.\n        ///\n        /// Note: This is similar to `openat` in POSIX.\n        open-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the object to open.\n            path: string,\n            /// The method by which to open the file.\n            open-flags: open-flags,\n            /// Flags to use for the resulting descriptor.\n            %flags: descriptor-flags,\n        ) -> result<descriptor, error-code>;\n\n        /// Read the contents of a symbolic link.\n        ///\n        /// If the contents contain an absolute or rooted path in the underlying\n        /// filesystem, this function fails with `error-code::not-permitted`.\n        ///\n        /// Note: This is similar to `readlinkat` in POSIX.\n        readlink-at: func(\n            /// The relative path of the symbolic link from which to read.\n            path: string,\n        ) -> result<string, error-code>;\n\n        /// Remove a directory.\n        ///\n        /// Return `error-code::not-empty` if the directory is not empty.\n        ///\n        /// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.\n        remove-directory-at: func(\n            /// The relative path to a directory to remove.\n            path: string,\n        ) -> result<_, error-code>;\n\n        /// Rename a filesystem object.\n        ///\n        /// Note: This is similar to `renameat` in POSIX.\n        rename-at: func(\n            /// The relative source path of the file or directory to rename.\n            old-path: string,\n            /// The base directory for `new-path`.\n            new-descriptor: borrow<descriptor>,\n            /// The relative destination path to which to rename the file or directory.\n            new-path: string,\n        ) -> result<_, error-code>;\n\n        /// Create a symbolic link (also known as a \"symlink\").\n        ///\n        /// If `old-path` starts with `/`, the function fails with\n        /// `error-code::not-permitted`.\n        ///\n        /// Note: This is similar to `symlinkat` in POSIX.\n        symlink-at: func(\n            /// The contents of the symbolic link.\n            old-path: string,\n            /// The relative destination path at which to create the symbolic link.\n            new-path: string,\n        ) -> result<_, error-code>;\n\n        /// Unlink a filesystem object that is not a directory.\n        ///\n        /// Return `error-code::is-directory` if the path refers to a directory.\n        /// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.\n        unlink-file-at: func(\n            /// The relative path to a file to unlink.\n            path: string,\n        ) -> result<_, error-code>;\n\n        /// Test whether two descriptors refer to the same filesystem object.\n        ///\n        /// In POSIX, this corresponds to testing whether the two descriptors have the\n        /// same device (`st_dev`) and inode (`st_ino` or `d_ino`) numbers.\n        /// wasi-filesystem does not expose device and inode numbers, so this function\n        /// may be used instead.\n        is-same-object: func(other: borrow<descriptor>) -> bool;\n\n        /// Return a hash of the metadata associated with a filesystem object referred\n        /// to by a descriptor.\n        ///\n        /// This returns a hash of the last-modification timestamp and file size, and\n        /// may also include the inode number, device number, birth timestamp, and\n        /// other metadata fields that may change when the file is modified or\n        /// replaced. It may also include a secret value chosen by the\n        /// implementation and not otherwise exposed.\n        ///\n        /// Implementations are encourated to provide the following properties:\n        ///\n        ///  - If the file is not modified or replaced, the computed hash value should\n        ///    usually not change.\n        ///  - If the object is modified or replaced, the computed hash value should\n        ///    usually change.\n        ///  - The inputs to the hash should not be easily computable from the\n        ///    computed hash.\n        ///\n        /// However, none of these is required.\n        metadata-hash: func() -> result<metadata-hash-value, error-code>;\n\n        /// Return a hash of the metadata associated with a filesystem object referred\n        /// to by a directory descriptor and a relative path.\n        ///\n        /// This performs the same hash computation as `metadata-hash`.\n        metadata-hash-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the file or directory to inspect.\n            path: string,\n        ) -> result<metadata-hash-value, error-code>;\n    }\n\n    /// A stream of directory entries.\n    resource directory-entry-stream {\n        /// Read a single directory entry from a `directory-entry-stream`.\n        read-directory-entry: func() -> result<option<directory-entry>, error-code>;\n    }\n\n    /// Attempts to extract a filesystem-related `error-code` from the stream\n    /// `error` provided.\n    ///\n    /// Stream operations which return `stream-error::last-operation-failed`\n    /// have a payload with more information about the operation that failed.\n    /// This payload can be passed through to this function to see if there\'s\n    /// filesystem-related information about the error to return.\n    ///\n    /// Note that this function is fallible because not all stream-related\n    /// errors are filesystem-related errors.\n    filesystem-error-code: func(err: borrow<error>) -> option<error-code>;\n}\n";
const _: &str = "package wasi:filesystem@0.2.0;\n\ninterface preopens {\n    use types.{descriptor};\n\n    /// Return the set of preopened directories, and their path.\n    get-directories: func() -> list<tuple<descriptor, string>>;\n}\n";
const _: &str = "package wasi:filesystem@0.2.0;\n\nworld imports {\n    import types;\n    import preopens;\n}\n";
const _: &str = "package research:hello;\n\nworld hello {\n    include wasi:filesystem/imports@0.2.0;\n    // Ask some data from the host\n    import get-name: func() -> string;\n    // Ask the host to print a message \n    import print: func(msg: string);\n    // Export the run method\n    export hello: func();\n}\n\n\n";
struct Hello;
impl Guest for Hello {
    fn hello() {
        let name = get_name();
        let msg = {
            let res = ::alloc::fmt::format(format_args!("Hello, {0}!", name));
            res
        };
        print(msg.as_str());
    }
}
