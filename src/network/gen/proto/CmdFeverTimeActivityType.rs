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

//! Generated file from `CmdFeverTimeActivityType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdFeverTimeActivityType)
pub enum CmdFeverTimeActivityType {
    // @@protoc_insertion_point(enum_value:CmdFeverTimeActivityType.CmdFeverTimeActivityTypeNone)
    CmdFeverTimeActivityTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdFeverTimeActivityType.CmdEnterFeverTimeActivityStageCsReq)
    CmdEnterFeverTimeActivityStageCsReq = 7159,
    // @@protoc_insertion_point(enum_value:CmdFeverTimeActivityType.CmdFeverTimeActivityBattleEndScNotify)
    CmdFeverTimeActivityBattleEndScNotify = 7151,
    // @@protoc_insertion_point(enum_value:CmdFeverTimeActivityType.CmdEnterFeverTimeActivityStageScRsp)
    CmdEnterFeverTimeActivityStageScRsp = 7158,
    // @@protoc_insertion_point(enum_value:CmdFeverTimeActivityType.CmdGetFeverTimeActivityDataCsReq)
    CmdGetFeverTimeActivityDataCsReq = 7156,
    // @@protoc_insertion_point(enum_value:CmdFeverTimeActivityType.CmdGetFeverTimeActivityDataScRsp)
    CmdGetFeverTimeActivityDataScRsp = 7155,
}

impl ::protobuf::Enum for CmdFeverTimeActivityType {
    const NAME: &'static str = "CmdFeverTimeActivityType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdFeverTimeActivityType> {
        match value {
            0 => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdFeverTimeActivityTypeNone),
            7159 => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdEnterFeverTimeActivityStageCsReq),
            7151 => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdFeverTimeActivityBattleEndScNotify),
            7158 => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdEnterFeverTimeActivityStageScRsp),
            7156 => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdGetFeverTimeActivityDataCsReq),
            7155 => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdGetFeverTimeActivityDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdFeverTimeActivityType> {
        match str {
            "CmdFeverTimeActivityTypeNone" => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdFeverTimeActivityTypeNone),
            "CmdEnterFeverTimeActivityStageCsReq" => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdEnterFeverTimeActivityStageCsReq),
            "CmdFeverTimeActivityBattleEndScNotify" => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdFeverTimeActivityBattleEndScNotify),
            "CmdEnterFeverTimeActivityStageScRsp" => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdEnterFeverTimeActivityStageScRsp),
            "CmdGetFeverTimeActivityDataCsReq" => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdGetFeverTimeActivityDataCsReq),
            "CmdGetFeverTimeActivityDataScRsp" => ::std::option::Option::Some(CmdFeverTimeActivityType::CmdGetFeverTimeActivityDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdFeverTimeActivityType] = &[
        CmdFeverTimeActivityType::CmdFeverTimeActivityTypeNone,
        CmdFeverTimeActivityType::CmdEnterFeverTimeActivityStageCsReq,
        CmdFeverTimeActivityType::CmdFeverTimeActivityBattleEndScNotify,
        CmdFeverTimeActivityType::CmdEnterFeverTimeActivityStageScRsp,
        CmdFeverTimeActivityType::CmdGetFeverTimeActivityDataCsReq,
        CmdFeverTimeActivityType::CmdGetFeverTimeActivityDataScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdFeverTimeActivityType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdFeverTimeActivityType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdFeverTimeActivityType::CmdFeverTimeActivityTypeNone => 0,
            CmdFeverTimeActivityType::CmdEnterFeverTimeActivityStageCsReq => 1,
            CmdFeverTimeActivityType::CmdFeverTimeActivityBattleEndScNotify => 2,
            CmdFeverTimeActivityType::CmdEnterFeverTimeActivityStageScRsp => 3,
            CmdFeverTimeActivityType::CmdGetFeverTimeActivityDataCsReq => 4,
            CmdFeverTimeActivityType::CmdGetFeverTimeActivityDataScRsp => 5,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdFeverTimeActivityType {
    fn default() -> Self {
        CmdFeverTimeActivityType::CmdFeverTimeActivityTypeNone
    }
}

impl CmdFeverTimeActivityType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdFeverTimeActivityType>("CmdFeverTimeActivityType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eCmdFeverTimeActivityType.proto*\x8a\x02\n\x18CmdFeverTimeActivityT\
    ype\x12\x20\n\x1cCmdFeverTimeActivityTypeNone\x10\0\x12(\n#CmdEnterFever\
    TimeActivityStageCsReq\x10\xf77\x12*\n%CmdFeverTimeActivityBattleEndScNo\
    tify\x10\xef7\x12(\n#CmdEnterFeverTimeActivityStageScRsp\x10\xf67\x12%\n\
    \x20CmdGetFeverTimeActivityDataCsReq\x10\xf47\x12%\n\x20CmdGetFeverTimeA\
    ctivityDataScRsp\x10\xf37b\x06proto3\
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
            enums.push(CmdFeverTimeActivityType::generated_enum_descriptor_data());
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
