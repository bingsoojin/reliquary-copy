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

//! Generated file from `GKEJFKAKENM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:GKEJFKAKENM)
pub enum GKEJFKAKENM {
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_NONE)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_EFFECT)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_EFFECT = 1,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_BONUS)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_BONUS = 2,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_TAX)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_TAX = 3,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_UPGRADE)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_UPGRADE = 4,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_GAME_SETTLE)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_GAME_SETTLE = 5,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_BUY_GOODS)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_BUY_GOODS = 6,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_CLICK)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_CLICK = 7,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_SOCIAL_EVENT)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_SOCIAL_EVENT = 8,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_LIKE)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_LIKE = 9,
    // @@protoc_insertion_point(enum_value:GKEJFKAKENM.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_QUIZ_GAME_SETTLE)
    MONOPOLY_ACTION_RESULT_SOURCE_TYPE_QUIZ_GAME_SETTLE = 10,
}

impl ::protobuf::Enum for GKEJFKAKENM {
    const NAME: &'static str = "GKEJFKAKENM";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GKEJFKAKENM> {
        match value {
            0 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_NONE),
            1 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_EFFECT),
            2 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_BONUS),
            3 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_TAX),
            4 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_UPGRADE),
            5 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_GAME_SETTLE),
            6 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_BUY_GOODS),
            7 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_CLICK),
            8 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_SOCIAL_EVENT),
            9 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_LIKE),
            10 => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_QUIZ_GAME_SETTLE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<GKEJFKAKENM> {
        match str {
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_NONE" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_NONE),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_EFFECT" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_EFFECT),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_BONUS" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_BONUS),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_TAX" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_TAX),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_UPGRADE" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_UPGRADE),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_GAME_SETTLE" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_GAME_SETTLE),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_BUY_GOODS" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_BUY_GOODS),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_CLICK" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_CLICK),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_SOCIAL_EVENT" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_SOCIAL_EVENT),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_LIKE" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_LIKE),
            "MONOPOLY_ACTION_RESULT_SOURCE_TYPE_QUIZ_GAME_SETTLE" => ::std::option::Option::Some(GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_QUIZ_GAME_SETTLE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [GKEJFKAKENM] = &[
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_NONE,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_EFFECT,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_BONUS,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_TAX,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_UPGRADE,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_GAME_SETTLE,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_BUY_GOODS,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_CLICK,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_SOCIAL_EVENT,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_LIKE,
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_QUIZ_GAME_SETTLE,
    ];
}

impl ::protobuf::EnumFull for GKEJFKAKENM {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("GKEJFKAKENM").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for GKEJFKAKENM {
    fn default() -> Self {
        GKEJFKAKENM::MONOPOLY_ACTION_RESULT_SOURCE_TYPE_NONE
    }
}

impl GKEJFKAKENM {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<GKEJFKAKENM>("GKEJFKAKENM")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GKEJFKAKENM.proto*\xb4\x04\n\x0bGKEJFKAKENM\x12+\n'MONOPOLY_ACTION\
    _RESULT_SOURCE_TYPE_NONE\x10\0\x12-\n)MONOPOLY_ACTION_RESULT_SOURCE_TYPE\
    _EFFECT\x10\x01\x122\n.MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_BONUS\
    \x10\x02\x120\n,MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_TAX\x10\x03\x12\
    4\n0MONOPOLY_ACTION_RESULT_SOURCE_TYPE_ASSET_UPGRADE\x10\x04\x122\n.MONO\
    POLY_ACTION_RESULT_SOURCE_TYPE_GAME_SETTLE\x10\x05\x120\n,MONOPOLY_ACTIO\
    N_RESULT_SOURCE_TYPE_BUY_GOODS\x10\x06\x12,\n(MONOPOLY_ACTION_RESULT_SOU\
    RCE_TYPE_CLICK\x10\x07\x123\n/MONOPOLY_ACTION_RESULT_SOURCE_TYPE_SOCIAL_\
    EVENT\x10\x08\x12+\n'MONOPOLY_ACTION_RESULT_SOURCE_TYPE_LIKE\x10\t\x127\
    \n3MONOPOLY_ACTION_RESULT_SOURCE_TYPE_QUIZ_GAME_SETTLE\x10\nb\x06proto3\
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
            enums.push(GKEJFKAKENM::generated_enum_descriptor_data());
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
