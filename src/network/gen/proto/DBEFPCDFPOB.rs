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

//! Generated file from `DBEFPCDFPOB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:DBEFPCDFPOB)
pub enum DBEFPCDFPOB {
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_NONE)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SELECT)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SELECT = 1,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE = 2,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_BONUS)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_BONUS = 3,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_USE)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_USE = 4,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_RESET)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_RESET = 5,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_REPLACE)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_REPLACE = 6,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_TRADE)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_TRADE = 7,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_GET)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_GET = 8,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SHOP)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SHOP = 9,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL = 10,
    // @@protoc_insertion_point(enum_value:DBEFPCDFPOB.CHESS_ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM)
    CHESS_ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM = 11,
}

impl ::protobuf::Enum for DBEFPCDFPOB {
    const NAME: &'static str = "DBEFPCDFPOB";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DBEFPCDFPOB> {
        match value {
            0 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_NONE),
            1 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SELECT),
            2 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE),
            3 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_BONUS),
            4 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_USE),
            5 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_RESET),
            6 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_REPLACE),
            7 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_TRADE),
            8 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_GET),
            9 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SHOP),
            10 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL),
            11 => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<DBEFPCDFPOB> {
        match str {
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_NONE" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_NONE),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SELECT" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SELECT),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_BONUS" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_BONUS),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_USE" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_USE),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_RESET" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_RESET),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_REPLACE" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_REPLACE),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_TRADE" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_TRADE),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_GET" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_GET),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SHOP" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SHOP),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL),
            "CHESS_ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM" => ::std::option::Option::Some(DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DBEFPCDFPOB] = &[
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_NONE,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SELECT,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_BONUS,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_USE,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_RESET,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_REPLACE,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_TRADE,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_GET,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SHOP,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL,
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM,
    ];
}

impl ::protobuf::EnumFull for DBEFPCDFPOB {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DBEFPCDFPOB").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DBEFPCDFPOB {
    fn default() -> Self {
        DBEFPCDFPOB::CHESS_ROGUE_MIRACLE_SOURCE_TYPE_NONE
    }
}

impl DBEFPCDFPOB {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DBEFPCDFPOB>("DBEFPCDFPOB")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DBEFPCDFPOB.proto*\xa0\x04\n\x0bDBEFPCDFPOB\x12(\n$CHESS_ROGUE_MIR\
    ACLE_SOURCE_TYPE_NONE\x10\0\x12*\n&CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SELEC\
    T\x10\x01\x12,\n(CHESS_ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE\x10\x02\x12)\n\
    %CHESS_ROGUE_MIRACLE_SOURCE_TYPE_BONUS\x10\x03\x12'\n#CHESS_ROGUE_MIRACL\
    E_SOURCE_TYPE_USE\x10\x04\x12)\n%CHESS_ROGUE_MIRACLE_SOURCE_TYPE_RESET\
    \x10\x05\x12+\n'CHESS_ROGUE_MIRACLE_SOURCE_TYPE_REPLACE\x10\x06\x12)\n%C\
    HESS_ROGUE_MIRACLE_SOURCE_TYPE_TRADE\x10\x07\x12'\n#CHESS_ROGUE_MIRACLE_\
    SOURCE_TYPE_GET\x10\x08\x12(\n$CHESS_ROGUE_MIRACLE_SOURCE_TYPE_SHOP\x10\
    \t\x12.\n*CHESS_ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL\x10\n\x123\n/CHESS_\
    ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM\x10\x0bb\x06proto3\
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
            enums.push(DBEFPCDFPOB::generated_enum_descriptor_data());
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
