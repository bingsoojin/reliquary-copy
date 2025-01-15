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

//! Generated file from `CmdGachaType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdGachaType)
pub enum CmdGachaType {
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGachaTypeNone)
    CmdGachaTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGetGachaCeilingCsReq)
    CmdGetGachaCeilingCsReq = 1967,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdExchangeGachaCeilingCsReq)
    CmdExchangeGachaCeilingCsReq = 1952,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGetGachaInfoScRsp)
    CmdGetGachaInfoScRsp = 1995,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGetGachaCeilingScRsp)
    CmdGetGachaCeilingScRsp = 1928,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdDoGachaScRsp)
    CmdDoGachaScRsp = 1927,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdExchangeGachaCeilingScRsp)
    CmdExchangeGachaCeilingScRsp = 1974,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdDoGachaCsReq)
    CmdDoGachaCsReq = 1984,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGetGachaInfoCsReq)
    CmdGetGachaInfoCsReq = 1936,
}

impl ::protobuf::Enum for CmdGachaType {
    const NAME: &'static str = "CmdGachaType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdGachaType> {
        match value {
            0 => ::std::option::Option::Some(CmdGachaType::CmdGachaTypeNone),
            1967 => ::std::option::Option::Some(CmdGachaType::CmdGetGachaCeilingCsReq),
            1952 => ::std::option::Option::Some(CmdGachaType::CmdExchangeGachaCeilingCsReq),
            1995 => ::std::option::Option::Some(CmdGachaType::CmdGetGachaInfoScRsp),
            1928 => ::std::option::Option::Some(CmdGachaType::CmdGetGachaCeilingScRsp),
            1927 => ::std::option::Option::Some(CmdGachaType::CmdDoGachaScRsp),
            1974 => ::std::option::Option::Some(CmdGachaType::CmdExchangeGachaCeilingScRsp),
            1984 => ::std::option::Option::Some(CmdGachaType::CmdDoGachaCsReq),
            1936 => ::std::option::Option::Some(CmdGachaType::CmdGetGachaInfoCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdGachaType> {
        match str {
            "CmdGachaTypeNone" => ::std::option::Option::Some(CmdGachaType::CmdGachaTypeNone),
            "CmdGetGachaCeilingCsReq" => ::std::option::Option::Some(CmdGachaType::CmdGetGachaCeilingCsReq),
            "CmdExchangeGachaCeilingCsReq" => ::std::option::Option::Some(CmdGachaType::CmdExchangeGachaCeilingCsReq),
            "CmdGetGachaInfoScRsp" => ::std::option::Option::Some(CmdGachaType::CmdGetGachaInfoScRsp),
            "CmdGetGachaCeilingScRsp" => ::std::option::Option::Some(CmdGachaType::CmdGetGachaCeilingScRsp),
            "CmdDoGachaScRsp" => ::std::option::Option::Some(CmdGachaType::CmdDoGachaScRsp),
            "CmdExchangeGachaCeilingScRsp" => ::std::option::Option::Some(CmdGachaType::CmdExchangeGachaCeilingScRsp),
            "CmdDoGachaCsReq" => ::std::option::Option::Some(CmdGachaType::CmdDoGachaCsReq),
            "CmdGetGachaInfoCsReq" => ::std::option::Option::Some(CmdGachaType::CmdGetGachaInfoCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdGachaType] = &[
        CmdGachaType::CmdGachaTypeNone,
        CmdGachaType::CmdGetGachaCeilingCsReq,
        CmdGachaType::CmdExchangeGachaCeilingCsReq,
        CmdGachaType::CmdGetGachaInfoScRsp,
        CmdGachaType::CmdGetGachaCeilingScRsp,
        CmdGachaType::CmdDoGachaScRsp,
        CmdGachaType::CmdExchangeGachaCeilingScRsp,
        CmdGachaType::CmdDoGachaCsReq,
        CmdGachaType::CmdGetGachaInfoCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdGachaType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdGachaType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdGachaType::CmdGachaTypeNone => 0,
            CmdGachaType::CmdGetGachaCeilingCsReq => 1,
            CmdGachaType::CmdExchangeGachaCeilingCsReq => 2,
            CmdGachaType::CmdGetGachaInfoScRsp => 3,
            CmdGachaType::CmdGetGachaCeilingScRsp => 4,
            CmdGachaType::CmdDoGachaScRsp => 5,
            CmdGachaType::CmdExchangeGachaCeilingScRsp => 6,
            CmdGachaType::CmdDoGachaCsReq => 7,
            CmdGachaType::CmdGetGachaInfoCsReq => 8,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdGachaType {
    fn default() -> Self {
        CmdGachaType::CmdGachaTypeNone
    }
}

impl CmdGachaType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdGachaType>("CmdGachaType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12CmdGachaType.proto*\x88\x02\n\x0cCmdGachaType\x12\x14\n\x10CmdGach\
    aTypeNone\x10\0\x12\x1c\n\x17CmdGetGachaCeilingCsReq\x10\xaf\x0f\x12!\n\
    \x1cCmdExchangeGachaCeilingCsReq\x10\xa0\x0f\x12\x19\n\x14CmdGetGachaInf\
    oScRsp\x10\xcb\x0f\x12\x1c\n\x17CmdGetGachaCeilingScRsp\x10\x88\x0f\x12\
    \x14\n\x0fCmdDoGachaScRsp\x10\x87\x0f\x12!\n\x1cCmdExchangeGachaCeilingS\
    cRsp\x10\xb6\x0f\x12\x14\n\x0fCmdDoGachaCsReq\x10\xc0\x0f\x12\x19\n\x14C\
    mdGetGachaInfoCsReq\x10\x90\x0fb\x06proto3\
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
            enums.push(CmdGachaType::generated_enum_descriptor_data());
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
