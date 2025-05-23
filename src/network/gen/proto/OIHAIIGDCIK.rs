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

//! Generated file from `OIHAIIGDCIK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:OIHAIIGDCIK)
pub enum OIHAIIGDCIK {
    // @@protoc_insertion_point(enum_value:OIHAIIGDCIK.ENTER_SCENE_REASON_NONE)
    ENTER_SCENE_REASON_NONE = 0,
    // @@protoc_insertion_point(enum_value:OIHAIIGDCIK.ENTER_SCENE_REASON_CHALLENGE_TIMEOUT)
    ENTER_SCENE_REASON_CHALLENGE_TIMEOUT = 1,
    // @@protoc_insertion_point(enum_value:OIHAIIGDCIK.ENTER_SCENE_REASON_ROGUE_TIMEOUT)
    ENTER_SCENE_REASON_ROGUE_TIMEOUT = 2,
    // @@protoc_insertion_point(enum_value:OIHAIIGDCIK.ENTER_SCENE_REASON_CHANGE_STORYLINE)
    ENTER_SCENE_REASON_CHANGE_STORYLINE = 3,
    // @@protoc_insertion_point(enum_value:OIHAIIGDCIK.ENTER_SCENE_REASON_DIMENSION_MERGE)
    ENTER_SCENE_REASON_DIMENSION_MERGE = 4,
}

impl ::protobuf::Enum for OIHAIIGDCIK {
    const NAME: &'static str = "OIHAIIGDCIK";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OIHAIIGDCIK> {
        match value {
            0 => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_NONE),
            1 => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_CHALLENGE_TIMEOUT),
            2 => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_ROGUE_TIMEOUT),
            3 => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_CHANGE_STORYLINE),
            4 => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_DIMENSION_MERGE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<OIHAIIGDCIK> {
        match str {
            "ENTER_SCENE_REASON_NONE" => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_NONE),
            "ENTER_SCENE_REASON_CHALLENGE_TIMEOUT" => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_CHALLENGE_TIMEOUT),
            "ENTER_SCENE_REASON_ROGUE_TIMEOUT" => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_ROGUE_TIMEOUT),
            "ENTER_SCENE_REASON_CHANGE_STORYLINE" => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_CHANGE_STORYLINE),
            "ENTER_SCENE_REASON_DIMENSION_MERGE" => ::std::option::Option::Some(OIHAIIGDCIK::ENTER_SCENE_REASON_DIMENSION_MERGE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [OIHAIIGDCIK] = &[
        OIHAIIGDCIK::ENTER_SCENE_REASON_NONE,
        OIHAIIGDCIK::ENTER_SCENE_REASON_CHALLENGE_TIMEOUT,
        OIHAIIGDCIK::ENTER_SCENE_REASON_ROGUE_TIMEOUT,
        OIHAIIGDCIK::ENTER_SCENE_REASON_CHANGE_STORYLINE,
        OIHAIIGDCIK::ENTER_SCENE_REASON_DIMENSION_MERGE,
    ];
}

impl ::protobuf::EnumFull for OIHAIIGDCIK {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("OIHAIIGDCIK").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for OIHAIIGDCIK {
    fn default() -> Self {
        OIHAIIGDCIK::ENTER_SCENE_REASON_NONE
    }
}

impl OIHAIIGDCIK {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<OIHAIIGDCIK>("OIHAIIGDCIK")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OIHAIIGDCIK.proto*\xcb\x01\n\x0bOIHAIIGDCIK\x12\x1b\n\x17ENTER_SCE\
    NE_REASON_NONE\x10\0\x12(\n$ENTER_SCENE_REASON_CHALLENGE_TIMEOUT\x10\x01\
    \x12$\n\x20ENTER_SCENE_REASON_ROGUE_TIMEOUT\x10\x02\x12'\n#ENTER_SCENE_R\
    EASON_CHANGE_STORYLINE\x10\x03\x12&\n\"ENTER_SCENE_REASON_DIMENSION_MERG\
    E\x10\x04b\x06proto3\
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
            enums.push(OIHAIIGDCIK::generated_enum_descriptor_data());
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
