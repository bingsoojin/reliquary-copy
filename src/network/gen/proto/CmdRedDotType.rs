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

//! Generated file from `CmdRedDotType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdRedDotType)
pub enum CmdRedDotType {
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdRedDotTypeNone)
    CmdRedDotTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdUpdateRedDotDataScRsp)
    CmdUpdateRedDotDataScRsp = 5946,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdGetAllRedDotDataScRsp)
    CmdGetAllRedDotDataScRsp = 5920,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdGetSingleRedDotParamGroupCsReq)
    CmdGetSingleRedDotParamGroupCsReq = 5939,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdGetSingleRedDotParamGroupScRsp)
    CmdGetSingleRedDotParamGroupScRsp = 5953,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdGetAllRedDotDataCsReq)
    CmdGetAllRedDotDataCsReq = 5959,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdUpdateRedDotDataCsReq)
    CmdUpdateRedDotDataCsReq = 5903,
}

impl ::protobuf::Enum for CmdRedDotType {
    const NAME: &'static str = "CmdRedDotType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdRedDotType> {
        match value {
            0 => ::std::option::Option::Some(CmdRedDotType::CmdRedDotTypeNone),
            5946 => ::std::option::Option::Some(CmdRedDotType::CmdUpdateRedDotDataScRsp),
            5920 => ::std::option::Option::Some(CmdRedDotType::CmdGetAllRedDotDataScRsp),
            5939 => ::std::option::Option::Some(CmdRedDotType::CmdGetSingleRedDotParamGroupCsReq),
            5953 => ::std::option::Option::Some(CmdRedDotType::CmdGetSingleRedDotParamGroupScRsp),
            5959 => ::std::option::Option::Some(CmdRedDotType::CmdGetAllRedDotDataCsReq),
            5903 => ::std::option::Option::Some(CmdRedDotType::CmdUpdateRedDotDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdRedDotType> {
        match str {
            "CmdRedDotTypeNone" => ::std::option::Option::Some(CmdRedDotType::CmdRedDotTypeNone),
            "CmdUpdateRedDotDataScRsp" => ::std::option::Option::Some(CmdRedDotType::CmdUpdateRedDotDataScRsp),
            "CmdGetAllRedDotDataScRsp" => ::std::option::Option::Some(CmdRedDotType::CmdGetAllRedDotDataScRsp),
            "CmdGetSingleRedDotParamGroupCsReq" => ::std::option::Option::Some(CmdRedDotType::CmdGetSingleRedDotParamGroupCsReq),
            "CmdGetSingleRedDotParamGroupScRsp" => ::std::option::Option::Some(CmdRedDotType::CmdGetSingleRedDotParamGroupScRsp),
            "CmdGetAllRedDotDataCsReq" => ::std::option::Option::Some(CmdRedDotType::CmdGetAllRedDotDataCsReq),
            "CmdUpdateRedDotDataCsReq" => ::std::option::Option::Some(CmdRedDotType::CmdUpdateRedDotDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdRedDotType] = &[
        CmdRedDotType::CmdRedDotTypeNone,
        CmdRedDotType::CmdUpdateRedDotDataScRsp,
        CmdRedDotType::CmdGetAllRedDotDataScRsp,
        CmdRedDotType::CmdGetSingleRedDotParamGroupCsReq,
        CmdRedDotType::CmdGetSingleRedDotParamGroupScRsp,
        CmdRedDotType::CmdGetAllRedDotDataCsReq,
        CmdRedDotType::CmdUpdateRedDotDataCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdRedDotType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdRedDotType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdRedDotType::CmdRedDotTypeNone => 0,
            CmdRedDotType::CmdUpdateRedDotDataScRsp => 1,
            CmdRedDotType::CmdGetAllRedDotDataScRsp => 2,
            CmdRedDotType::CmdGetSingleRedDotParamGroupCsReq => 3,
            CmdRedDotType::CmdGetSingleRedDotParamGroupScRsp => 4,
            CmdRedDotType::CmdGetAllRedDotDataCsReq => 5,
            CmdRedDotType::CmdUpdateRedDotDataCsReq => 6,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdRedDotType {
    fn default() -> Self {
        CmdRedDotType::CmdRedDotTypeNone
    }
}

impl CmdRedDotType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdRedDotType>("CmdRedDotType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdRedDotType.proto*\xf2\x01\n\rCmdRedDotType\x12\x15\n\x11CmdRedD\
    otTypeNone\x10\0\x12\x1d\n\x18CmdUpdateRedDotDataScRsp\x10\xba.\x12\x1d\
    \n\x18CmdGetAllRedDotDataScRsp\x10\xa0.\x12&\n!CmdGetSingleRedDotParamGr\
    oupCsReq\x10\xb3.\x12&\n!CmdGetSingleRedDotParamGroupScRsp\x10\xc1.\x12\
    \x1d\n\x18CmdGetAllRedDotDataCsReq\x10\xc7.\x12\x1d\n\x18CmdUpdateRedDot\
    DataCsReq\x10\x8f.b\x06proto3\
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
            enums.push(CmdRedDotType::generated_enum_descriptor_data());
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
