// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `DEPJMMBPHBJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:DEPJMMBPHBJ)
pub enum DEPJMMBPHBJ {
    // @@protoc_insertion_point(enum_value:DEPJMMBPHBJ.AETHERDIVIDE_SPIRIT_LINEUP_NONE)
    AETHERDIVIDE_SPIRIT_LINEUP_NONE = 0,
    // @@protoc_insertion_point(enum_value:DEPJMMBPHBJ.AETHERDIVIDE_SPIRIT_LINEUP_NORMAL)
    AETHERDIVIDE_SPIRIT_LINEUP_NORMAL = 1,
    // @@protoc_insertion_point(enum_value:DEPJMMBPHBJ.AETHERDIVIDE_SPIRIT_LINEUP_TRIAL)
    AETHERDIVIDE_SPIRIT_LINEUP_TRIAL = 2,
}

impl ::protobuf::Enum for DEPJMMBPHBJ {
    const NAME: &'static str = "DEPJMMBPHBJ";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DEPJMMBPHBJ> {
        match value {
            0 => ::std::option::Option::Some(DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_NONE),
            1 => ::std::option::Option::Some(DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_NORMAL),
            2 => ::std::option::Option::Some(DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_TRIAL),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<DEPJMMBPHBJ> {
        match str {
            "AETHERDIVIDE_SPIRIT_LINEUP_NONE" => ::std::option::Option::Some(DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_NONE),
            "AETHERDIVIDE_SPIRIT_LINEUP_NORMAL" => ::std::option::Option::Some(DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_NORMAL),
            "AETHERDIVIDE_SPIRIT_LINEUP_TRIAL" => ::std::option::Option::Some(DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_TRIAL),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DEPJMMBPHBJ] = &[
        DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_NONE,
        DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_NORMAL,
        DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_TRIAL,
    ];
}

impl ::protobuf::EnumFull for DEPJMMBPHBJ {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DEPJMMBPHBJ").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DEPJMMBPHBJ {
    fn default() -> Self {
        DEPJMMBPHBJ::AETHERDIVIDE_SPIRIT_LINEUP_NONE
    }
}

impl DEPJMMBPHBJ {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DEPJMMBPHBJ>("DEPJMMBPHBJ")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DEPJMMBPHBJ.proto*\x7f\n\x0bDEPJMMBPHBJ\x12#\n\x1fAETHERDIVIDE_SPI\
    RIT_LINEUP_NONE\x10\0\x12%\n!AETHERDIVIDE_SPIRIT_LINEUP_NORMAL\x10\x01\
    \x12$\n\x20AETHERDIVIDE_SPIRIT_LINEUP_TRIAL\x10\x02b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(DEPJMMBPHBJ::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
