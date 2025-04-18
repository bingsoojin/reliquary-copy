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

//! Generated file from `CmdClockParkType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdClockParkType)
pub enum CmdClockParkType {
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkTypeNone)
    CmdClockParkTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkUseBuffScRsp)
    CmdClockParkUseBuffScRsp = 7209,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkUnlockTalentCsReq)
    CmdClockParkUnlockTalentCsReq = 7206,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkGetOngoingScriptInfoScRsp)
    CmdClockParkGetOngoingScriptInfoScRsp = 7215,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkGetOngoingScriptInfoCsReq)
    CmdClockParkGetOngoingScriptInfoCsReq = 7225,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkHandleWaitOperationCsReq)
    CmdClockParkHandleWaitOperationCsReq = 7230,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkStartScriptScRsp)
    CmdClockParkStartScriptScRsp = 7229,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkFinishScriptScNotify)
    CmdClockParkFinishScriptScNotify = 7224,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkBattleEndScNotify)
    CmdClockParkBattleEndScNotify = 7240,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkStartScriptCsReq)
    CmdClockParkStartScriptCsReq = 7207,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkGetInfoScRsp)
    CmdClockParkGetInfoScRsp = 7237,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkGetInfoCsReq)
    CmdClockParkGetInfoCsReq = 7226,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkUnlockTalentScRsp)
    CmdClockParkUnlockTalentScRsp = 7233,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkHandleWaitOperationScRsp)
    CmdClockParkHandleWaitOperationScRsp = 7202,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkQuitScriptScRsp)
    CmdClockParkQuitScriptScRsp = 7204,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkUseBuffCsReq)
    CmdClockParkUseBuffCsReq = 7235,
    // @@protoc_insertion_point(enum_value:CmdClockParkType.CmdClockParkQuitScriptCsReq)
    CmdClockParkQuitScriptCsReq = 7248,
}

impl ::protobuf::Enum for CmdClockParkType {
    const NAME: &'static str = "CmdClockParkType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdClockParkType> {
        match value {
            0 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkTypeNone),
            7209 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkUseBuffScRsp),
            7206 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkUnlockTalentCsReq),
            7215 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkGetOngoingScriptInfoScRsp),
            7225 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkGetOngoingScriptInfoCsReq),
            7230 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkHandleWaitOperationCsReq),
            7229 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkStartScriptScRsp),
            7224 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkFinishScriptScNotify),
            7240 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkBattleEndScNotify),
            7207 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkStartScriptCsReq),
            7237 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkGetInfoScRsp),
            7226 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkGetInfoCsReq),
            7233 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkUnlockTalentScRsp),
            7202 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkHandleWaitOperationScRsp),
            7204 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkQuitScriptScRsp),
            7235 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkUseBuffCsReq),
            7248 => ::std::option::Option::Some(CmdClockParkType::CmdClockParkQuitScriptCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdClockParkType> {
        match str {
            "CmdClockParkTypeNone" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkTypeNone),
            "CmdClockParkUseBuffScRsp" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkUseBuffScRsp),
            "CmdClockParkUnlockTalentCsReq" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkUnlockTalentCsReq),
            "CmdClockParkGetOngoingScriptInfoScRsp" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkGetOngoingScriptInfoScRsp),
            "CmdClockParkGetOngoingScriptInfoCsReq" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkGetOngoingScriptInfoCsReq),
            "CmdClockParkHandleWaitOperationCsReq" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkHandleWaitOperationCsReq),
            "CmdClockParkStartScriptScRsp" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkStartScriptScRsp),
            "CmdClockParkFinishScriptScNotify" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkFinishScriptScNotify),
            "CmdClockParkBattleEndScNotify" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkBattleEndScNotify),
            "CmdClockParkStartScriptCsReq" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkStartScriptCsReq),
            "CmdClockParkGetInfoScRsp" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkGetInfoScRsp),
            "CmdClockParkGetInfoCsReq" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkGetInfoCsReq),
            "CmdClockParkUnlockTalentScRsp" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkUnlockTalentScRsp),
            "CmdClockParkHandleWaitOperationScRsp" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkHandleWaitOperationScRsp),
            "CmdClockParkQuitScriptScRsp" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkQuitScriptScRsp),
            "CmdClockParkUseBuffCsReq" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkUseBuffCsReq),
            "CmdClockParkQuitScriptCsReq" => ::std::option::Option::Some(CmdClockParkType::CmdClockParkQuitScriptCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdClockParkType] = &[
        CmdClockParkType::CmdClockParkTypeNone,
        CmdClockParkType::CmdClockParkUseBuffScRsp,
        CmdClockParkType::CmdClockParkUnlockTalentCsReq,
        CmdClockParkType::CmdClockParkGetOngoingScriptInfoScRsp,
        CmdClockParkType::CmdClockParkGetOngoingScriptInfoCsReq,
        CmdClockParkType::CmdClockParkHandleWaitOperationCsReq,
        CmdClockParkType::CmdClockParkStartScriptScRsp,
        CmdClockParkType::CmdClockParkFinishScriptScNotify,
        CmdClockParkType::CmdClockParkBattleEndScNotify,
        CmdClockParkType::CmdClockParkStartScriptCsReq,
        CmdClockParkType::CmdClockParkGetInfoScRsp,
        CmdClockParkType::CmdClockParkGetInfoCsReq,
        CmdClockParkType::CmdClockParkUnlockTalentScRsp,
        CmdClockParkType::CmdClockParkHandleWaitOperationScRsp,
        CmdClockParkType::CmdClockParkQuitScriptScRsp,
        CmdClockParkType::CmdClockParkUseBuffCsReq,
        CmdClockParkType::CmdClockParkQuitScriptCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdClockParkType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdClockParkType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdClockParkType::CmdClockParkTypeNone => 0,
            CmdClockParkType::CmdClockParkUseBuffScRsp => 1,
            CmdClockParkType::CmdClockParkUnlockTalentCsReq => 2,
            CmdClockParkType::CmdClockParkGetOngoingScriptInfoScRsp => 3,
            CmdClockParkType::CmdClockParkGetOngoingScriptInfoCsReq => 4,
            CmdClockParkType::CmdClockParkHandleWaitOperationCsReq => 5,
            CmdClockParkType::CmdClockParkStartScriptScRsp => 6,
            CmdClockParkType::CmdClockParkFinishScriptScNotify => 7,
            CmdClockParkType::CmdClockParkBattleEndScNotify => 8,
            CmdClockParkType::CmdClockParkStartScriptCsReq => 9,
            CmdClockParkType::CmdClockParkGetInfoScRsp => 10,
            CmdClockParkType::CmdClockParkGetInfoCsReq => 11,
            CmdClockParkType::CmdClockParkUnlockTalentScRsp => 12,
            CmdClockParkType::CmdClockParkHandleWaitOperationScRsp => 13,
            CmdClockParkType::CmdClockParkQuitScriptScRsp => 14,
            CmdClockParkType::CmdClockParkUseBuffCsReq => 15,
            CmdClockParkType::CmdClockParkQuitScriptCsReq => 16,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdClockParkType {
    fn default() -> Self {
        CmdClockParkType::CmdClockParkTypeNone
    }
}

impl CmdClockParkType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdClockParkType>("CmdClockParkType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16CmdClockParkType.proto*\xf3\x04\n\x10CmdClockParkType\x12\x18\n\
    \x14CmdClockParkTypeNone\x10\0\x12\x1d\n\x18CmdClockParkUseBuffScRsp\x10\
    \xa98\x12\"\n\x1dCmdClockParkUnlockTalentCsReq\x10\xa68\x12*\n%CmdClockP\
    arkGetOngoingScriptInfoScRsp\x10\xaf8\x12*\n%CmdClockParkGetOngoingScrip\
    tInfoCsReq\x10\xb98\x12)\n$CmdClockParkHandleWaitOperationCsReq\x10\xbe8\
    \x12!\n\x1cCmdClockParkStartScriptScRsp\x10\xbd8\x12%\n\x20CmdClockParkF\
    inishScriptScNotify\x10\xb88\x12\"\n\x1dCmdClockParkBattleEndScNotify\
    \x10\xc88\x12!\n\x1cCmdClockParkStartScriptCsReq\x10\xa78\x12\x1d\n\x18C\
    mdClockParkGetInfoScRsp\x10\xc58\x12\x1d\n\x18CmdClockParkGetInfoCsReq\
    \x10\xba8\x12\"\n\x1dCmdClockParkUnlockTalentScRsp\x10\xc18\x12)\n$CmdCl\
    ockParkHandleWaitOperationScRsp\x10\xa28\x12\x20\n\x1bCmdClockParkQuitSc\
    riptScRsp\x10\xa48\x12\x1d\n\x18CmdClockParkUseBuffCsReq\x10\xc38\x12\
    \x20\n\x1bCmdClockParkQuitScriptCsReq\x10\xd08b\x06proto3\
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
            enums.push(CmdClockParkType::generated_enum_descriptor_data());
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
