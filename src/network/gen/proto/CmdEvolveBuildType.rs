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

//! Generated file from `CmdEvolveBuildType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdEvolveBuildType)
pub enum CmdEvolveBuildType {
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildNone)
    CmdEvolveBuildNone = 0,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityDownCsReq)
    CmdEvolveBuildShopAbilityDownCsReq = 7110,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildReRandomStageScRsp)
    CmdEvolveBuildReRandomStageScRsp = 7148,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityUpCsReq)
    CmdEvolveBuildShopAbilityUpCsReq = 7114,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityDownScRsp)
    CmdEvolveBuildShopAbilityDownScRsp = 7135,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildCoinNotify)
    CmdEvolveBuildCoinNotify = 7120,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildReRandomStageCsReq)
    CmdEvolveBuildReRandomStageCsReq = 7102,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildLeaveScRsp)
    CmdEvolveBuildLeaveScRsp = 7115,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildFinishScNotify)
    CmdEvolveBuildFinishScNotify = 7130,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildQueryInfoScRsp)
    CmdEvolveBuildQueryInfoScRsp = 7137,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildUnlockInfoNotify)
    CmdEvolveBuildUnlockInfoNotify = 7139,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildTakeExpRewardCsReq)
    CmdEvolveBuildTakeExpRewardCsReq = 7109,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartLevelCsReq)
    CmdEvolveBuildStartLevelCsReq = 7141,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartLevelScRsp)
    CmdEvolveBuildStartLevelScRsp = 7143,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartStageScRsp)
    CmdEvolveBuildStartStageScRsp = 7133,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityResetCsReq)
    CmdEvolveBuildShopAbilityResetCsReq = 7101,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityUpScRsp)
    CmdEvolveBuildShopAbilityUpScRsp = 7140,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildGiveupCsReq)
    CmdEvolveBuildGiveupCsReq = 7107,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartStageCsReq)
    CmdEvolveBuildStartStageCsReq = 7106,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildGiveupScRsp)
    CmdEvolveBuildGiveupScRsp = 7129,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildQueryInfoCsReq)
    CmdEvolveBuildQueryInfoCsReq = 7126,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildTakeExpRewardScRsp)
    CmdEvolveBuildTakeExpRewardScRsp = 7124,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityResetScRsp)
    CmdEvolveBuildShopAbilityResetScRsp = 7121,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildLeaveCsReq)
    CmdEvolveBuildLeaveCsReq = 7125,
}

impl ::protobuf::Enum for CmdEvolveBuildType {
    const NAME: &'static str = "CmdEvolveBuildType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdEvolveBuildType> {
        match value {
            0 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildNone),
            7110 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq),
            7148 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp),
            7114 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq),
            7135 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp),
            7120 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildCoinNotify),
            7102 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq),
            7115 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp),
            7130 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildFinishScNotify),
            7137 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp),
            7139 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify),
            7109 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq),
            7141 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq),
            7143 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp),
            7133 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp),
            7101 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq),
            7140 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp),
            7107 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq),
            7106 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq),
            7129 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp),
            7126 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq),
            7124 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp),
            7121 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp),
            7125 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdEvolveBuildType> {
        match str {
            "CmdEvolveBuildNone" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildNone),
            "CmdEvolveBuildShopAbilityDownCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq),
            "CmdEvolveBuildReRandomStageScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp),
            "CmdEvolveBuildShopAbilityUpCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq),
            "CmdEvolveBuildShopAbilityDownScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp),
            "CmdEvolveBuildCoinNotify" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildCoinNotify),
            "CmdEvolveBuildReRandomStageCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq),
            "CmdEvolveBuildLeaveScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp),
            "CmdEvolveBuildFinishScNotify" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildFinishScNotify),
            "CmdEvolveBuildQueryInfoScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp),
            "CmdEvolveBuildUnlockInfoNotify" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify),
            "CmdEvolveBuildTakeExpRewardCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq),
            "CmdEvolveBuildStartLevelCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq),
            "CmdEvolveBuildStartLevelScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp),
            "CmdEvolveBuildStartStageScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp),
            "CmdEvolveBuildShopAbilityResetCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq),
            "CmdEvolveBuildShopAbilityUpScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp),
            "CmdEvolveBuildGiveupCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq),
            "CmdEvolveBuildStartStageCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq),
            "CmdEvolveBuildGiveupScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp),
            "CmdEvolveBuildQueryInfoCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq),
            "CmdEvolveBuildTakeExpRewardScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp),
            "CmdEvolveBuildShopAbilityResetScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp),
            "CmdEvolveBuildLeaveCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdEvolveBuildType] = &[
        CmdEvolveBuildType::CmdEvolveBuildNone,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq,
        CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp,
        CmdEvolveBuildType::CmdEvolveBuildCoinNotify,
        CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq,
        CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp,
        CmdEvolveBuildType::CmdEvolveBuildFinishScNotify,
        CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp,
        CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify,
        CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq,
        CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq,
        CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp,
        CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp,
        CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq,
        CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq,
        CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp,
        CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq,
        CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp,
        CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdEvolveBuildType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdEvolveBuildType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdEvolveBuildType::CmdEvolveBuildNone => 0,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq => 1,
            CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp => 2,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq => 3,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp => 4,
            CmdEvolveBuildType::CmdEvolveBuildCoinNotify => 5,
            CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq => 6,
            CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp => 7,
            CmdEvolveBuildType::CmdEvolveBuildFinishScNotify => 8,
            CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp => 9,
            CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify => 10,
            CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq => 11,
            CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq => 12,
            CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp => 13,
            CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp => 14,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq => 15,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp => 16,
            CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq => 17,
            CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq => 18,
            CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp => 19,
            CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq => 20,
            CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp => 21,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp => 22,
            CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq => 23,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdEvolveBuildType {
    fn default() -> Self {
        CmdEvolveBuildType::CmdEvolveBuildNone
    }
}

impl CmdEvolveBuildType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdEvolveBuildType>("CmdEvolveBuildType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdEvolveBuildType.proto*\xf7\x06\n\x12CmdEvolveBuildType\x12\x16\
    \n\x12CmdEvolveBuildNone\x10\0\x12'\n\"CmdEvolveBuildShopAbilityDownCsRe\
    q\x10\xc67\x12%\n\x20CmdEvolveBuildReRandomStageScRsp\x10\xec7\x12%\n\
    \x20CmdEvolveBuildShopAbilityUpCsReq\x10\xca7\x12'\n\"CmdEvolveBuildShop\
    AbilityDownScRsp\x10\xdf7\x12\x1d\n\x18CmdEvolveBuildCoinNotify\x10\xd07\
    \x12%\n\x20CmdEvolveBuildReRandomStageCsReq\x10\xbe7\x12\x1d\n\x18CmdEvo\
    lveBuildLeaveScRsp\x10\xcb7\x12!\n\x1cCmdEvolveBuildFinishScNotify\x10\
    \xda7\x12!\n\x1cCmdEvolveBuildQueryInfoScRsp\x10\xe17\x12#\n\x1eCmdEvolv\
    eBuildUnlockInfoNotify\x10\xe37\x12%\n\x20CmdEvolveBuildTakeExpRewardCsR\
    eq\x10\xc57\x12\"\n\x1dCmdEvolveBuildStartLevelCsReq\x10\xe57\x12\"\n\
    \x1dCmdEvolveBuildStartLevelScRsp\x10\xe77\x12\"\n\x1dCmdEvolveBuildStar\
    tStageScRsp\x10\xdd7\x12(\n#CmdEvolveBuildShopAbilityResetCsReq\x10\xbd7\
    \x12%\n\x20CmdEvolveBuildShopAbilityUpScRsp\x10\xe47\x12\x1e\n\x19CmdEvo\
    lveBuildGiveupCsReq\x10\xc37\x12\"\n\x1dCmdEvolveBuildStartStageCsReq\
    \x10\xc27\x12\x1e\n\x19CmdEvolveBuildGiveupScRsp\x10\xd97\x12!\n\x1cCmdE\
    volveBuildQueryInfoCsReq\x10\xd67\x12%\n\x20CmdEvolveBuildTakeExpRewardS\
    cRsp\x10\xd47\x12(\n#CmdEvolveBuildShopAbilityResetScRsp\x10\xd17\x12\
    \x1d\n\x18CmdEvolveBuildLeaveCsReq\x10\xd57b\x06proto3\
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
            enums.push(CmdEvolveBuildType::generated_enum_descriptor_data());
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
