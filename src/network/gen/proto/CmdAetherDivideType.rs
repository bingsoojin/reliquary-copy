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

//! Generated file from `CmdAetherDivideType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdAetherDivideType)
pub enum CmdAetherDivideType {
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideTypeNone)
    CmdAetherDivideTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdEnterAetherDivideSceneScRsp)
    CmdEnterAetherDivideSceneScRsp = 4837,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideLineupScNotify)
    CmdAetherDivideLineupScNotify = 4831,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdEquipAetherDividePassiveSkillScRsp)
    CmdEquipAetherDividePassiveSkillScRsp = 4814,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideRefreshEndlessScRsp)
    CmdAetherDivideRefreshEndlessScRsp = 4816,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdEquipAetherDividePassiveSkillCsReq)
    CmdEquipAetherDividePassiveSkillCsReq = 4832,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdClearAetherDividePassiveSkillScRsp)
    CmdClearAetherDividePassiveSkillScRsp = 4810,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdSwitchAetherDivideLineUpSlotCsReq)
    CmdSwitchAetherDivideLineUpSlotCsReq = 4835,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideTainerInfoScNotify)
    CmdAetherDivideTainerInfoScNotify = 4812,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdEnterAetherDivideSceneCsReq)
    CmdEnterAetherDivideSceneCsReq = 4826,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdSetAetherDivideLineUpCsReq)
    CmdSetAetherDivideLineUpCsReq = 4848,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdGetAetherDivideInfoScRsp)
    CmdGetAetherDivideInfoScRsp = 4802,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdClearAetherDividePassiveSkillCsReq)
    CmdClearAetherDividePassiveSkillCsReq = 4840,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdSwitchAetherDivideLineUpSlotScRsp)
    CmdSwitchAetherDivideLineUpSlotScRsp = 4809,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdGetAetherDivideChallengeInfoScRsp)
    CmdGetAetherDivideChallengeInfoScRsp = 4803,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdGetAetherDivideChallengeInfoCsReq)
    CmdGetAetherDivideChallengeInfoCsReq = 4845,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideTakeChallengeRewardScRsp)
    CmdAetherDivideTakeChallengeRewardScRsp = 4827,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdLeaveAetherDivideSceneCsReq)
    CmdLeaveAetherDivideSceneCsReq = 4841,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideStageBattleScRsp)
    CmdStartAetherDivideStageBattleScRsp = 4801,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideTakeChallengeRewardCsReq)
    CmdAetherDivideTakeChallengeRewardCsReq = 4849,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideSpiritExpUpScRsp)
    CmdAetherDivideSpiritExpUpScRsp = 4820,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideChallengeBattleScRsp)
    CmdStartAetherDivideChallengeBattleScRsp = 4829,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideSpiritInfoScNotify)
    CmdAetherDivideSpiritInfoScNotify = 4839,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideStageBattleCsReq)
    CmdStartAetherDivideStageBattleCsReq = 4824,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideRefreshEndlessCsReq)
    CmdAetherDivideRefreshEndlessCsReq = 4834,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideChallengeBattleCsReq)
    CmdStartAetherDivideChallengeBattleCsReq = 4807,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdSetAetherDivideLineUpScRsp)
    CmdSetAetherDivideLineUpScRsp = 4804,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdGetAetherDivideInfoCsReq)
    CmdGetAetherDivideInfoCsReq = 4830,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideSceneBattleCsReq)
    CmdStartAetherDivideSceneBattleCsReq = 4806,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideSkillItemScNotify)
    CmdAetherDivideSkillItemScNotify = 4817,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdLeaveAetherDivideSceneScRsp)
    CmdLeaveAetherDivideSceneScRsp = 4843,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideRefreshEndlessScNotify)
    CmdAetherDivideRefreshEndlessScNotify = 4846,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideFinishChallengeScNotify)
    CmdAetherDivideFinishChallengeScNotify = 4828,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideSpiritExpUpCsReq)
    CmdAetherDivideSpiritExpUpCsReq = 4821,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideSceneBattleScRsp)
    CmdStartAetherDivideSceneBattleScRsp = 4833,
}

impl ::protobuf::Enum for CmdAetherDivideType {
    const NAME: &'static str = "CmdAetherDivideType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdAetherDivideType> {
        match value {
            0 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTypeNone),
            4837 => ::std::option::Option::Some(CmdAetherDivideType::CmdEnterAetherDivideSceneScRsp),
            4831 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideLineupScNotify),
            4814 => ::std::option::Option::Some(CmdAetherDivideType::CmdEquipAetherDividePassiveSkillScRsp),
            4816 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessScRsp),
            4832 => ::std::option::Option::Some(CmdAetherDivideType::CmdEquipAetherDividePassiveSkillCsReq),
            4810 => ::std::option::Option::Some(CmdAetherDivideType::CmdClearAetherDividePassiveSkillScRsp),
            4835 => ::std::option::Option::Some(CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotCsReq),
            4812 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTainerInfoScNotify),
            4826 => ::std::option::Option::Some(CmdAetherDivideType::CmdEnterAetherDivideSceneCsReq),
            4848 => ::std::option::Option::Some(CmdAetherDivideType::CmdSetAetherDivideLineUpCsReq),
            4802 => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideInfoScRsp),
            4840 => ::std::option::Option::Some(CmdAetherDivideType::CmdClearAetherDividePassiveSkillCsReq),
            4809 => ::std::option::Option::Some(CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotScRsp),
            4803 => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideChallengeInfoScRsp),
            4845 => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideChallengeInfoCsReq),
            4827 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardScRsp),
            4841 => ::std::option::Option::Some(CmdAetherDivideType::CmdLeaveAetherDivideSceneCsReq),
            4801 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideStageBattleScRsp),
            4849 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardCsReq),
            4820 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritExpUpScRsp),
            4829 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideChallengeBattleScRsp),
            4839 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritInfoScNotify),
            4824 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideStageBattleCsReq),
            4834 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessCsReq),
            4807 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideChallengeBattleCsReq),
            4804 => ::std::option::Option::Some(CmdAetherDivideType::CmdSetAetherDivideLineUpScRsp),
            4830 => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideInfoCsReq),
            4806 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideSceneBattleCsReq),
            4817 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSkillItemScNotify),
            4843 => ::std::option::Option::Some(CmdAetherDivideType::CmdLeaveAetherDivideSceneScRsp),
            4846 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessScNotify),
            4828 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideFinishChallengeScNotify),
            4821 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritExpUpCsReq),
            4833 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideSceneBattleScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdAetherDivideType> {
        match str {
            "CmdAetherDivideTypeNone" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTypeNone),
            "CmdEnterAetherDivideSceneScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdEnterAetherDivideSceneScRsp),
            "CmdAetherDivideLineupScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideLineupScNotify),
            "CmdEquipAetherDividePassiveSkillScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdEquipAetherDividePassiveSkillScRsp),
            "CmdAetherDivideRefreshEndlessScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessScRsp),
            "CmdEquipAetherDividePassiveSkillCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdEquipAetherDividePassiveSkillCsReq),
            "CmdClearAetherDividePassiveSkillScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdClearAetherDividePassiveSkillScRsp),
            "CmdSwitchAetherDivideLineUpSlotCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotCsReq),
            "CmdAetherDivideTainerInfoScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTainerInfoScNotify),
            "CmdEnterAetherDivideSceneCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdEnterAetherDivideSceneCsReq),
            "CmdSetAetherDivideLineUpCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdSetAetherDivideLineUpCsReq),
            "CmdGetAetherDivideInfoScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideInfoScRsp),
            "CmdClearAetherDividePassiveSkillCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdClearAetherDividePassiveSkillCsReq),
            "CmdSwitchAetherDivideLineUpSlotScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotScRsp),
            "CmdGetAetherDivideChallengeInfoScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideChallengeInfoScRsp),
            "CmdGetAetherDivideChallengeInfoCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideChallengeInfoCsReq),
            "CmdAetherDivideTakeChallengeRewardScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardScRsp),
            "CmdLeaveAetherDivideSceneCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdLeaveAetherDivideSceneCsReq),
            "CmdStartAetherDivideStageBattleScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideStageBattleScRsp),
            "CmdAetherDivideTakeChallengeRewardCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardCsReq),
            "CmdAetherDivideSpiritExpUpScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritExpUpScRsp),
            "CmdStartAetherDivideChallengeBattleScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideChallengeBattleScRsp),
            "CmdAetherDivideSpiritInfoScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritInfoScNotify),
            "CmdStartAetherDivideStageBattleCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideStageBattleCsReq),
            "CmdAetherDivideRefreshEndlessCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessCsReq),
            "CmdStartAetherDivideChallengeBattleCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideChallengeBattleCsReq),
            "CmdSetAetherDivideLineUpScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdSetAetherDivideLineUpScRsp),
            "CmdGetAetherDivideInfoCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideInfoCsReq),
            "CmdStartAetherDivideSceneBattleCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideSceneBattleCsReq),
            "CmdAetherDivideSkillItemScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSkillItemScNotify),
            "CmdLeaveAetherDivideSceneScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdLeaveAetherDivideSceneScRsp),
            "CmdAetherDivideRefreshEndlessScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessScNotify),
            "CmdAetherDivideFinishChallengeScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideFinishChallengeScNotify),
            "CmdAetherDivideSpiritExpUpCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritExpUpCsReq),
            "CmdStartAetherDivideSceneBattleScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideSceneBattleScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdAetherDivideType] = &[
        CmdAetherDivideType::CmdAetherDivideTypeNone,
        CmdAetherDivideType::CmdEnterAetherDivideSceneScRsp,
        CmdAetherDivideType::CmdAetherDivideLineupScNotify,
        CmdAetherDivideType::CmdEquipAetherDividePassiveSkillScRsp,
        CmdAetherDivideType::CmdAetherDivideRefreshEndlessScRsp,
        CmdAetherDivideType::CmdEquipAetherDividePassiveSkillCsReq,
        CmdAetherDivideType::CmdClearAetherDividePassiveSkillScRsp,
        CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotCsReq,
        CmdAetherDivideType::CmdAetherDivideTainerInfoScNotify,
        CmdAetherDivideType::CmdEnterAetherDivideSceneCsReq,
        CmdAetherDivideType::CmdSetAetherDivideLineUpCsReq,
        CmdAetherDivideType::CmdGetAetherDivideInfoScRsp,
        CmdAetherDivideType::CmdClearAetherDividePassiveSkillCsReq,
        CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotScRsp,
        CmdAetherDivideType::CmdGetAetherDivideChallengeInfoScRsp,
        CmdAetherDivideType::CmdGetAetherDivideChallengeInfoCsReq,
        CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardScRsp,
        CmdAetherDivideType::CmdLeaveAetherDivideSceneCsReq,
        CmdAetherDivideType::CmdStartAetherDivideStageBattleScRsp,
        CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardCsReq,
        CmdAetherDivideType::CmdAetherDivideSpiritExpUpScRsp,
        CmdAetherDivideType::CmdStartAetherDivideChallengeBattleScRsp,
        CmdAetherDivideType::CmdAetherDivideSpiritInfoScNotify,
        CmdAetherDivideType::CmdStartAetherDivideStageBattleCsReq,
        CmdAetherDivideType::CmdAetherDivideRefreshEndlessCsReq,
        CmdAetherDivideType::CmdStartAetherDivideChallengeBattleCsReq,
        CmdAetherDivideType::CmdSetAetherDivideLineUpScRsp,
        CmdAetherDivideType::CmdGetAetherDivideInfoCsReq,
        CmdAetherDivideType::CmdStartAetherDivideSceneBattleCsReq,
        CmdAetherDivideType::CmdAetherDivideSkillItemScNotify,
        CmdAetherDivideType::CmdLeaveAetherDivideSceneScRsp,
        CmdAetherDivideType::CmdAetherDivideRefreshEndlessScNotify,
        CmdAetherDivideType::CmdAetherDivideFinishChallengeScNotify,
        CmdAetherDivideType::CmdAetherDivideSpiritExpUpCsReq,
        CmdAetherDivideType::CmdStartAetherDivideSceneBattleScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdAetherDivideType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdAetherDivideType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdAetherDivideType::CmdAetherDivideTypeNone => 0,
            CmdAetherDivideType::CmdEnterAetherDivideSceneScRsp => 1,
            CmdAetherDivideType::CmdAetherDivideLineupScNotify => 2,
            CmdAetherDivideType::CmdEquipAetherDividePassiveSkillScRsp => 3,
            CmdAetherDivideType::CmdAetherDivideRefreshEndlessScRsp => 4,
            CmdAetherDivideType::CmdEquipAetherDividePassiveSkillCsReq => 5,
            CmdAetherDivideType::CmdClearAetherDividePassiveSkillScRsp => 6,
            CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotCsReq => 7,
            CmdAetherDivideType::CmdAetherDivideTainerInfoScNotify => 8,
            CmdAetherDivideType::CmdEnterAetherDivideSceneCsReq => 9,
            CmdAetherDivideType::CmdSetAetherDivideLineUpCsReq => 10,
            CmdAetherDivideType::CmdGetAetherDivideInfoScRsp => 11,
            CmdAetherDivideType::CmdClearAetherDividePassiveSkillCsReq => 12,
            CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotScRsp => 13,
            CmdAetherDivideType::CmdGetAetherDivideChallengeInfoScRsp => 14,
            CmdAetherDivideType::CmdGetAetherDivideChallengeInfoCsReq => 15,
            CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardScRsp => 16,
            CmdAetherDivideType::CmdLeaveAetherDivideSceneCsReq => 17,
            CmdAetherDivideType::CmdStartAetherDivideStageBattleScRsp => 18,
            CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardCsReq => 19,
            CmdAetherDivideType::CmdAetherDivideSpiritExpUpScRsp => 20,
            CmdAetherDivideType::CmdStartAetherDivideChallengeBattleScRsp => 21,
            CmdAetherDivideType::CmdAetherDivideSpiritInfoScNotify => 22,
            CmdAetherDivideType::CmdStartAetherDivideStageBattleCsReq => 23,
            CmdAetherDivideType::CmdAetherDivideRefreshEndlessCsReq => 24,
            CmdAetherDivideType::CmdStartAetherDivideChallengeBattleCsReq => 25,
            CmdAetherDivideType::CmdSetAetherDivideLineUpScRsp => 26,
            CmdAetherDivideType::CmdGetAetherDivideInfoCsReq => 27,
            CmdAetherDivideType::CmdStartAetherDivideSceneBattleCsReq => 28,
            CmdAetherDivideType::CmdAetherDivideSkillItemScNotify => 29,
            CmdAetherDivideType::CmdLeaveAetherDivideSceneScRsp => 30,
            CmdAetherDivideType::CmdAetherDivideRefreshEndlessScNotify => 31,
            CmdAetherDivideType::CmdAetherDivideFinishChallengeScNotify => 32,
            CmdAetherDivideType::CmdAetherDivideSpiritExpUpCsReq => 33,
            CmdAetherDivideType::CmdStartAetherDivideSceneBattleScRsp => 34,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdAetherDivideType {
    fn default() -> Self {
        CmdAetherDivideType::CmdAetherDivideTypeNone
    }
}

impl CmdAetherDivideType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdAetherDivideType>("CmdAetherDivideType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19CmdAetherDivideType.proto*\xa6\x0b\n\x13CmdAetherDivideType\x12\
    \x1b\n\x17CmdAetherDivideTypeNone\x10\0\x12#\n\x1eCmdEnterAetherDivideSc\
    eneScRsp\x10\xe5%\x12\"\n\x1dCmdAetherDivideLineupScNotify\x10\xdf%\x12*\
    \n%CmdEquipAetherDividePassiveSkillScRsp\x10\xce%\x12'\n\"CmdAetherDivid\
    eRefreshEndlessScRsp\x10\xd0%\x12*\n%CmdEquipAetherDividePassiveSkillCsR\
    eq\x10\xe0%\x12*\n%CmdClearAetherDividePassiveSkillScRsp\x10\xca%\x12)\n\
    $CmdSwitchAetherDivideLineUpSlotCsReq\x10\xe3%\x12&\n!CmdAetherDivideTai\
    nerInfoScNotify\x10\xcc%\x12#\n\x1eCmdEnterAetherDivideSceneCsReq\x10\
    \xda%\x12\"\n\x1dCmdSetAetherDivideLineUpCsReq\x10\xf0%\x12\x20\n\x1bCmd\
    GetAetherDivideInfoScRsp\x10\xc2%\x12*\n%CmdClearAetherDividePassiveSkil\
    lCsReq\x10\xe8%\x12)\n$CmdSwitchAetherDivideLineUpSlotScRsp\x10\xc9%\x12\
    )\n$CmdGetAetherDivideChallengeInfoScRsp\x10\xc3%\x12)\n$CmdGetAetherDiv\
    ideChallengeInfoCsReq\x10\xed%\x12,\n'CmdAetherDivideTakeChallengeReward\
    ScRsp\x10\xdb%\x12#\n\x1eCmdLeaveAetherDivideSceneCsReq\x10\xe9%\x12)\n$\
    CmdStartAetherDivideStageBattleScRsp\x10\xc1%\x12,\n'CmdAetherDivideTake\
    ChallengeRewardCsReq\x10\xf1%\x12$\n\x1fCmdAetherDivideSpiritExpUpScRsp\
    \x10\xd4%\x12-\n(CmdStartAetherDivideChallengeBattleScRsp\x10\xdd%\x12&\
    \n!CmdAetherDivideSpiritInfoScNotify\x10\xe7%\x12)\n$CmdStartAetherDivid\
    eStageBattleCsReq\x10\xd8%\x12'\n\"CmdAetherDivideRefreshEndlessCsReq\
    \x10\xe2%\x12-\n(CmdStartAetherDivideChallengeBattleCsReq\x10\xc7%\x12\"\
    \n\x1dCmdSetAetherDivideLineUpScRsp\x10\xc4%\x12\x20\n\x1bCmdGetAetherDi\
    videInfoCsReq\x10\xde%\x12)\n$CmdStartAetherDivideSceneBattleCsReq\x10\
    \xc6%\x12%\n\x20CmdAetherDivideSkillItemScNotify\x10\xd1%\x12#\n\x1eCmdL\
    eaveAetherDivideSceneScRsp\x10\xeb%\x12*\n%CmdAetherDivideRefreshEndless\
    ScNotify\x10\xee%\x12+\n&CmdAetherDivideFinishChallengeScNotify\x10\xdc%\
    \x12$\n\x1fCmdAetherDivideSpiritExpUpCsReq\x10\xd5%\x12)\n$CmdStartAethe\
    rDivideSceneBattleScRsp\x10\xe1%b\x06proto3\
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
            enums.push(CmdAetherDivideType::generated_enum_descriptor_data());
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
