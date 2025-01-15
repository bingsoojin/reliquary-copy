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

//! Generated file from `CmdTreasureDungeonType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTreasureDungeonType)
pub enum CmdTreasureDungeonType {
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdTreasureDungeonTypeNone)
    CmdTreasureDungeonTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdGetTreasureDungeonActivityDataCsReq)
    CmdGetTreasureDungeonActivityDataCsReq = 4434,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdUseTreasureDungeonItemScRsp)
    CmdUseTreasureDungeonItemScRsp = 4490,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdEnterTreasureDungeonScRsp)
    CmdEnterTreasureDungeonScRsp = 4425,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdFightTreasureDungeonMonsterCsReq)
    CmdFightTreasureDungeonMonsterCsReq = 4475,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdUseTreasureDungeonItemCsReq)
    CmdUseTreasureDungeonItemCsReq = 4453,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdQuitTreasureDungeonScRsp)
    CmdQuitTreasureDungeonScRsp = 4412,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdQuitTreasureDungeonCsReq)
    CmdQuitTreasureDungeonCsReq = 4459,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdOpenTreasureDungeonGridCsReq)
    CmdOpenTreasureDungeonGridCsReq = 4496,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdGetTreasureDungeonActivityDataScRsp)
    CmdGetTreasureDungeonActivityDataScRsp = 4443,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdInteractTreasureDungeonGridScRsp)
    CmdInteractTreasureDungeonGridScRsp = 4407,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdFightTreasureDungeonMonsterScRsp)
    CmdFightTreasureDungeonMonsterScRsp = 4419,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdTreasureDungeonFinishScNotify)
    CmdTreasureDungeonFinishScNotify = 4495,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdOpenTreasureDungeonGridScRsp)
    CmdOpenTreasureDungeonGridScRsp = 4405,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdInteractTreasureDungeonGridCsReq)
    CmdInteractTreasureDungeonGridCsReq = 4426,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdTreasureDungeonDataScNotify)
    CmdTreasureDungeonDataScNotify = 4436,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdEnterTreasureDungeonCsReq)
    CmdEnterTreasureDungeonCsReq = 4446,
}

impl ::protobuf::Enum for CmdTreasureDungeonType {
    const NAME: &'static str = "CmdTreasureDungeonType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTreasureDungeonType> {
        match value {
            0 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonTypeNone),
            4434 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataCsReq),
            4490 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdUseTreasureDungeonItemScRsp),
            4425 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdEnterTreasureDungeonScRsp),
            4475 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterCsReq),
            4453 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdUseTreasureDungeonItemCsReq),
            4412 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdQuitTreasureDungeonScRsp),
            4459 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdQuitTreasureDungeonCsReq),
            4496 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdOpenTreasureDungeonGridCsReq),
            4443 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataScRsp),
            4407 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdInteractTreasureDungeonGridScRsp),
            4419 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterScRsp),
            4495 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonFinishScNotify),
            4405 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdOpenTreasureDungeonGridScRsp),
            4426 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdInteractTreasureDungeonGridCsReq),
            4436 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonDataScNotify),
            4446 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdEnterTreasureDungeonCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTreasureDungeonType> {
        match str {
            "CmdTreasureDungeonTypeNone" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonTypeNone),
            "CmdGetTreasureDungeonActivityDataCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataCsReq),
            "CmdUseTreasureDungeonItemScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdUseTreasureDungeonItemScRsp),
            "CmdEnterTreasureDungeonScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdEnterTreasureDungeonScRsp),
            "CmdFightTreasureDungeonMonsterCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterCsReq),
            "CmdUseTreasureDungeonItemCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdUseTreasureDungeonItemCsReq),
            "CmdQuitTreasureDungeonScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdQuitTreasureDungeonScRsp),
            "CmdQuitTreasureDungeonCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdQuitTreasureDungeonCsReq),
            "CmdOpenTreasureDungeonGridCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdOpenTreasureDungeonGridCsReq),
            "CmdGetTreasureDungeonActivityDataScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataScRsp),
            "CmdInteractTreasureDungeonGridScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdInteractTreasureDungeonGridScRsp),
            "CmdFightTreasureDungeonMonsterScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterScRsp),
            "CmdTreasureDungeonFinishScNotify" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonFinishScNotify),
            "CmdOpenTreasureDungeonGridScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdOpenTreasureDungeonGridScRsp),
            "CmdInteractTreasureDungeonGridCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdInteractTreasureDungeonGridCsReq),
            "CmdTreasureDungeonDataScNotify" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonDataScNotify),
            "CmdEnterTreasureDungeonCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdEnterTreasureDungeonCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTreasureDungeonType] = &[
        CmdTreasureDungeonType::CmdTreasureDungeonTypeNone,
        CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataCsReq,
        CmdTreasureDungeonType::CmdUseTreasureDungeonItemScRsp,
        CmdTreasureDungeonType::CmdEnterTreasureDungeonScRsp,
        CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterCsReq,
        CmdTreasureDungeonType::CmdUseTreasureDungeonItemCsReq,
        CmdTreasureDungeonType::CmdQuitTreasureDungeonScRsp,
        CmdTreasureDungeonType::CmdQuitTreasureDungeonCsReq,
        CmdTreasureDungeonType::CmdOpenTreasureDungeonGridCsReq,
        CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataScRsp,
        CmdTreasureDungeonType::CmdInteractTreasureDungeonGridScRsp,
        CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterScRsp,
        CmdTreasureDungeonType::CmdTreasureDungeonFinishScNotify,
        CmdTreasureDungeonType::CmdOpenTreasureDungeonGridScRsp,
        CmdTreasureDungeonType::CmdInteractTreasureDungeonGridCsReq,
        CmdTreasureDungeonType::CmdTreasureDungeonDataScNotify,
        CmdTreasureDungeonType::CmdEnterTreasureDungeonCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdTreasureDungeonType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTreasureDungeonType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTreasureDungeonType::CmdTreasureDungeonTypeNone => 0,
            CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataCsReq => 1,
            CmdTreasureDungeonType::CmdUseTreasureDungeonItemScRsp => 2,
            CmdTreasureDungeonType::CmdEnterTreasureDungeonScRsp => 3,
            CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterCsReq => 4,
            CmdTreasureDungeonType::CmdUseTreasureDungeonItemCsReq => 5,
            CmdTreasureDungeonType::CmdQuitTreasureDungeonScRsp => 6,
            CmdTreasureDungeonType::CmdQuitTreasureDungeonCsReq => 7,
            CmdTreasureDungeonType::CmdOpenTreasureDungeonGridCsReq => 8,
            CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataScRsp => 9,
            CmdTreasureDungeonType::CmdInteractTreasureDungeonGridScRsp => 10,
            CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterScRsp => 11,
            CmdTreasureDungeonType::CmdTreasureDungeonFinishScNotify => 12,
            CmdTreasureDungeonType::CmdOpenTreasureDungeonGridScRsp => 13,
            CmdTreasureDungeonType::CmdInteractTreasureDungeonGridCsReq => 14,
            CmdTreasureDungeonType::CmdTreasureDungeonDataScNotify => 15,
            CmdTreasureDungeonType::CmdEnterTreasureDungeonCsReq => 16,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTreasureDungeonType {
    fn default() -> Self {
        CmdTreasureDungeonType::CmdTreasureDungeonTypeNone
    }
}

impl CmdTreasureDungeonType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTreasureDungeonType>("CmdTreasureDungeonType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cCmdTreasureDungeonType.proto*\xa6\x05\n\x16CmdTreasureDungeonType\
    \x12\x1e\n\x1aCmdTreasureDungeonTypeNone\x10\0\x12+\n&CmdGetTreasureDung\
    eonActivityDataCsReq\x10\xd2\"\x12#\n\x1eCmdUseTreasureDungeonItemScRsp\
    \x10\x8a#\x12!\n\x1cCmdEnterTreasureDungeonScRsp\x10\xc9\"\x12(\n#CmdFig\
    htTreasureDungeonMonsterCsReq\x10\xfb\"\x12#\n\x1eCmdUseTreasureDungeonI\
    temCsReq\x10\xe5\"\x12\x20\n\x1bCmdQuitTreasureDungeonScRsp\x10\xbc\"\
    \x12\x20\n\x1bCmdQuitTreasureDungeonCsReq\x10\xeb\"\x12$\n\x1fCmdOpenTre\
    asureDungeonGridCsReq\x10\x90#\x12+\n&CmdGetTreasureDungeonActivityDataS\
    cRsp\x10\xdb\"\x12(\n#CmdInteractTreasureDungeonGridScRsp\x10\xb7\"\x12(\
    \n#CmdFightTreasureDungeonMonsterScRsp\x10\xc3\"\x12%\n\x20CmdTreasureDu\
    ngeonFinishScNotify\x10\x8f#\x12$\n\x1fCmdOpenTreasureDungeonGridScRsp\
    \x10\xb5\"\x12(\n#CmdInteractTreasureDungeonGridCsReq\x10\xca\"\x12#\n\
    \x1eCmdTreasureDungeonDataScNotify\x10\xd4\"\x12!\n\x1cCmdEnterTreasureD\
    ungeonCsReq\x10\xde\"b\x06proto3\
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
            enums.push(CmdTreasureDungeonType::generated_enum_descriptor_data());
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
