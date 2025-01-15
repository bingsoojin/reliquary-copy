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

//! Generated file from `JHIPGPGHHMG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:JHIPGPGHHMG)
pub enum JHIPGPGHHMG {
    // @@protoc_insertion_point(enum_value:JHIPGPGHHMG.BIG_DATA_RECOMMEND_TYPE_NONE)
    BIG_DATA_RECOMMEND_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:JHIPGPGHHMG.BIG_DATA_RECOMMEND_TYPE_EQUIPMENT)
    BIG_DATA_RECOMMEND_TYPE_EQUIPMENT = 1,
    // @@protoc_insertion_point(enum_value:JHIPGPGHHMG.BIG_DATA_RECOMMEND_TYPE_RELIC_SUIT)
    BIG_DATA_RECOMMEND_TYPE_RELIC_SUIT = 2,
}

impl ::protobuf::Enum for JHIPGPGHHMG {
    const NAME: &'static str = "JHIPGPGHHMG";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<JHIPGPGHHMG> {
        match value {
            0 => ::std::option::Option::Some(JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_NONE),
            1 => ::std::option::Option::Some(JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_EQUIPMENT),
            2 => ::std::option::Option::Some(JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_RELIC_SUIT),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<JHIPGPGHHMG> {
        match str {
            "BIG_DATA_RECOMMEND_TYPE_NONE" => ::std::option::Option::Some(JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_NONE),
            "BIG_DATA_RECOMMEND_TYPE_EQUIPMENT" => ::std::option::Option::Some(JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_EQUIPMENT),
            "BIG_DATA_RECOMMEND_TYPE_RELIC_SUIT" => ::std::option::Option::Some(JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_RELIC_SUIT),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [JHIPGPGHHMG] = &[
        JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_NONE,
        JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_EQUIPMENT,
        JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_RELIC_SUIT,
    ];
}

impl ::protobuf::EnumFull for JHIPGPGHHMG {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("JHIPGPGHHMG").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for JHIPGPGHHMG {
    fn default() -> Self {
        JHIPGPGHHMG::BIG_DATA_RECOMMEND_TYPE_NONE
    }
}

impl JHIPGPGHHMG {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<JHIPGPGHHMG>("JHIPGPGHHMG")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JHIPGPGHHMG.proto*~\n\x0bJHIPGPGHHMG\x12\x20\n\x1cBIG_DATA_RECOMME\
    ND_TYPE_NONE\x10\0\x12%\n!BIG_DATA_RECOMMEND_TYPE_EQUIPMENT\x10\x01\x12&\
    \n\"BIG_DATA_RECOMMEND_TYPE_RELIC_SUIT\x10\x02b\x06proto3\
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
            enums.push(JHIPGPGHHMG::generated_enum_descriptor_data());
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
