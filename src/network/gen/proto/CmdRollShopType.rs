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

//! Generated file from `CmdRollShopType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdRollShopType)
pub enum CmdRollShopType {
    // @@protoc_insertion_point(enum_value:CmdRollShopType.CmdRollShopTypeNone)
    CmdRollShopTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdRollShopType.CmdGetRollShopInfoScRsp)
    CmdGetRollShopInfoScRsp = 6908,
    // @@protoc_insertion_point(enum_value:CmdRollShopType.CmdTakeRollShopRewardScRsp)
    CmdTakeRollShopRewardScRsp = 6901,
    // @@protoc_insertion_point(enum_value:CmdRollShopType.CmdTakeRollShopRewardCsReq)
    CmdTakeRollShopRewardCsReq = 6904,
    // @@protoc_insertion_point(enum_value:CmdRollShopType.CmdDoGachaInRollShopCsReq)
    CmdDoGachaInRollShopCsReq = 6912,
    // @@protoc_insertion_point(enum_value:CmdRollShopType.CmdDoGachaInRollShopScRsp)
    CmdDoGachaInRollShopScRsp = 6902,
    // @@protoc_insertion_point(enum_value:CmdRollShopType.CmdGetRollShopInfoCsReq)
    CmdGetRollShopInfoCsReq = 6917,
}

impl ::protobuf::Enum for CmdRollShopType {
    const NAME: &'static str = "CmdRollShopType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdRollShopType> {
        match value {
            0 => ::std::option::Option::Some(CmdRollShopType::CmdRollShopTypeNone),
            6908 => ::std::option::Option::Some(CmdRollShopType::CmdGetRollShopInfoScRsp),
            6901 => ::std::option::Option::Some(CmdRollShopType::CmdTakeRollShopRewardScRsp),
            6904 => ::std::option::Option::Some(CmdRollShopType::CmdTakeRollShopRewardCsReq),
            6912 => ::std::option::Option::Some(CmdRollShopType::CmdDoGachaInRollShopCsReq),
            6902 => ::std::option::Option::Some(CmdRollShopType::CmdDoGachaInRollShopScRsp),
            6917 => ::std::option::Option::Some(CmdRollShopType::CmdGetRollShopInfoCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdRollShopType> {
        match str {
            "CmdRollShopTypeNone" => ::std::option::Option::Some(CmdRollShopType::CmdRollShopTypeNone),
            "CmdGetRollShopInfoScRsp" => ::std::option::Option::Some(CmdRollShopType::CmdGetRollShopInfoScRsp),
            "CmdTakeRollShopRewardScRsp" => ::std::option::Option::Some(CmdRollShopType::CmdTakeRollShopRewardScRsp),
            "CmdTakeRollShopRewardCsReq" => ::std::option::Option::Some(CmdRollShopType::CmdTakeRollShopRewardCsReq),
            "CmdDoGachaInRollShopCsReq" => ::std::option::Option::Some(CmdRollShopType::CmdDoGachaInRollShopCsReq),
            "CmdDoGachaInRollShopScRsp" => ::std::option::Option::Some(CmdRollShopType::CmdDoGachaInRollShopScRsp),
            "CmdGetRollShopInfoCsReq" => ::std::option::Option::Some(CmdRollShopType::CmdGetRollShopInfoCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdRollShopType] = &[
        CmdRollShopType::CmdRollShopTypeNone,
        CmdRollShopType::CmdGetRollShopInfoScRsp,
        CmdRollShopType::CmdTakeRollShopRewardScRsp,
        CmdRollShopType::CmdTakeRollShopRewardCsReq,
        CmdRollShopType::CmdDoGachaInRollShopCsReq,
        CmdRollShopType::CmdDoGachaInRollShopScRsp,
        CmdRollShopType::CmdGetRollShopInfoCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdRollShopType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdRollShopType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdRollShopType::CmdRollShopTypeNone => 0,
            CmdRollShopType::CmdGetRollShopInfoScRsp => 1,
            CmdRollShopType::CmdTakeRollShopRewardScRsp => 2,
            CmdRollShopType::CmdTakeRollShopRewardCsReq => 3,
            CmdRollShopType::CmdDoGachaInRollShopCsReq => 4,
            CmdRollShopType::CmdDoGachaInRollShopScRsp => 5,
            CmdRollShopType::CmdGetRollShopInfoCsReq => 6,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdRollShopType {
    fn default() -> Self {
        CmdRollShopType::CmdRollShopTypeNone
    }
}

impl CmdRollShopType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdRollShopType>("CmdRollShopType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15CmdRollShopType.proto*\xe8\x01\n\x0fCmdRollShopType\x12\x17\n\x13C\
    mdRollShopTypeNone\x10\0\x12\x1c\n\x17CmdGetRollShopInfoScRsp\x10\xfc5\
    \x12\x1f\n\x1aCmdTakeRollShopRewardScRsp\x10\xf55\x12\x1f\n\x1aCmdTakeRo\
    llShopRewardCsReq\x10\xf85\x12\x1e\n\x19CmdDoGachaInRollShopCsReq\x10\
    \x806\x12\x1e\n\x19CmdDoGachaInRollShopScRsp\x10\xf65\x12\x1c\n\x17CmdGe\
    tRollShopInfoCsReq\x10\x856b\x06proto3\
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
            enums.push(CmdRollShopType::generated_enum_descriptor_data());
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
