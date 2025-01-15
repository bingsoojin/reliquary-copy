// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `KOAPFJLFMDO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:KOAPFJLFMDO)
pub enum KOAPFJLFMDO {
    // @@protoc_insertion_point(enum_value:KOAPFJLFMDO.PRODUCT_NONE)
    PRODUCT_NONE = 0,
    // @@protoc_insertion_point(enum_value:KOAPFJLFMDO.PRODUCT_NORMAL)
    PRODUCT_NORMAL = 1,
    // @@protoc_insertion_point(enum_value:KOAPFJLFMDO.PRODUCT_LIMIT)
    PRODUCT_LIMIT = 2,
    // @@protoc_insertion_point(enum_value:KOAPFJLFMDO.PRODUCT_LIMIT_NO_PAY)
    PRODUCT_LIMIT_NO_PAY = 3,
    // @@protoc_insertion_point(enum_value:KOAPFJLFMDO.PRODUCT_NO_PROCESS_ORDER)
    PRODUCT_NO_PROCESS_ORDER = 4,
}

impl ::protobuf::Enum for KOAPFJLFMDO {
    const NAME: &'static str = "KOAPFJLFMDO";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KOAPFJLFMDO> {
        match value {
            0 => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_NONE),
            1 => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_NORMAL),
            2 => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_LIMIT),
            3 => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_LIMIT_NO_PAY),
            4 => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_NO_PROCESS_ORDER),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<KOAPFJLFMDO> {
        match str {
            "PRODUCT_NONE" => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_NONE),
            "PRODUCT_NORMAL" => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_NORMAL),
            "PRODUCT_LIMIT" => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_LIMIT),
            "PRODUCT_LIMIT_NO_PAY" => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_LIMIT_NO_PAY),
            "PRODUCT_NO_PROCESS_ORDER" => ::std::option::Option::Some(KOAPFJLFMDO::PRODUCT_NO_PROCESS_ORDER),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [KOAPFJLFMDO] = &[
        KOAPFJLFMDO::PRODUCT_NONE,
        KOAPFJLFMDO::PRODUCT_NORMAL,
        KOAPFJLFMDO::PRODUCT_LIMIT,
        KOAPFJLFMDO::PRODUCT_LIMIT_NO_PAY,
        KOAPFJLFMDO::PRODUCT_NO_PROCESS_ORDER,
    ];
}

impl ::protobuf::EnumFull for KOAPFJLFMDO {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("KOAPFJLFMDO").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for KOAPFJLFMDO {
    fn default() -> Self {
        KOAPFJLFMDO::PRODUCT_NONE
    }
}

impl KOAPFJLFMDO {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<KOAPFJLFMDO>("KOAPFJLFMDO")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KOAPFJLFMDO.proto*~\n\x0bKOAPFJLFMDO\x12\x10\n\x0cPRODUCT_NONE\x10\
    \0\x12\x12\n\x0ePRODUCT_NORMAL\x10\x01\x12\x11\n\rPRODUCT_LIMIT\x10\x02\
    \x12\x18\n\x14PRODUCT_LIMIT_NO_PAY\x10\x03\x12\x1c\n\x18PRODUCT_NO_PROCE\
    SS_ORDER\x10\x04b\x06proto3\
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
            enums.push(KOAPFJLFMDO::generated_enum_descriptor_data());
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
