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

//! Generated file from `CmdRogueType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdRogueType)
pub enum CmdRogueType {
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdRogueTypeNone)
    CmdRogueTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdExchangeRogueRewardKeyScRsp)
    CmdExchangeRogueRewardKeyScRsp = 1831,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueBuffEnhanceInfoCsReq)
    CmdGetRogueBuffEnhanceInfoCsReq = 1851,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdQuitRogueScRsp)
    CmdQuitRogueScRsp = 1836,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdEnhanceRogueBuffCsReq)
    CmdEnhanceRogueBuffCsReq = 1883,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdOpenRogueChestScRsp)
    CmdOpenRogueChestScRsp = 1832,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueGetItemScNotify)
    CmdSyncRogueGetItemScNotify = 1818,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdLeaveRogueCsReq)
    CmdLeaveRogueCsReq = 1834,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdEnableRogueTalentScRsp)
    CmdEnableRogueTalentScRsp = 1895,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueFinishScNotify)
    CmdSyncRogueFinishScNotify = 1848,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueSeasonFinishScNotify)
    CmdSyncRogueSeasonFinishScNotify = 1813,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdExchangeRogueRewardKeyCsReq)
    CmdExchangeRogueRewardKeyCsReq = 1896,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdTakeRogueScoreRewardScRsp)
    CmdTakeRogueScoreRewardScRsp = 1833,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueRewardInfoScNotify)
    CmdSyncRogueRewardInfoScNotify = 1889,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueAeonInfoScRsp)
    CmdGetRogueAeonInfoScRsp = 1858,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueMapRoomScNotify)
    CmdSyncRogueMapRoomScNotify = 1814,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueExploreWinScNotify)
    CmdSyncRogueExploreWinScNotify = 1826,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdOpenRogueChestCsReq)
    CmdOpenRogueChestCsReq = 1860,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueTalentInfoScRsp)
    CmdGetRogueTalentInfoScRsp = 1808,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdPickRogueAvatarCsReq)
    CmdPickRogueAvatarCsReq = 1890,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdFinishAeonDialogueGroupCsReq)
    CmdFinishAeonDialogueGroupCsReq = 1887,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdEnterRogueMapRoomCsReq)
    CmdEnterRogueMapRoomCsReq = 1804,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueScoreRewardInfoScRsp)
    CmdGetRogueScoreRewardInfoScRsp = 1840,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdLeaveRogueScRsp)
    CmdLeaveRogueScRsp = 1837,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdEnterRogueMapRoomScRsp)
    CmdEnterRogueMapRoomScRsp = 1888,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdPickRogueAvatarScRsp)
    CmdPickRogueAvatarScRsp = 1879,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueAeonInfoCsReq)
    CmdGetRogueAeonInfoCsReq = 1841,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdStartRogueScRsp)
    CmdStartRogueScRsp = 1846,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueAeonScNotify)
    CmdSyncRogueAeonScNotify = 1835,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdReviveRogueAvatarCsReq)
    CmdReviveRogueAvatarCsReq = 1861,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueInfoCsReq)
    CmdGetRogueInfoCsReq = 1859,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueVirtualItemInfoScNotify)
    CmdSyncRogueVirtualItemInfoScNotify = 1886,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueInfoScRsp)
    CmdGetRogueInfoScRsp = 1820,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdQuitRogueCsReq)
    CmdQuitRogueCsReq = 1870,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueStatusScNotify)
    CmdSyncRogueStatusScNotify = 1821,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueInitialScoreCsReq)
    CmdGetRogueInitialScoreCsReq = 1843,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdEnableRogueTalentCsReq)
    CmdEnableRogueTalentCsReq = 1900,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueBuffEnhanceInfoScRsp)
    CmdGetRogueBuffEnhanceInfoScRsp = 1873,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdFinishAeonDialogueGroupScRsp)
    CmdFinishAeonDialogueGroupScRsp = 1844,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdEnhanceRogueBuffScRsp)
    CmdEnhanceRogueBuffScRsp = 1899,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueScoreRewardInfoCsReq)
    CmdGetRogueScoreRewardInfoCsReq = 1842,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueAreaUnlockScNotify)
    CmdSyncRogueAreaUnlockScNotify = 1807,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdReviveRogueAvatarScRsp)
    CmdReviveRogueAvatarScRsp = 1825,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdStartRogueCsReq)
    CmdStartRogueCsReq = 1803,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdTakeRogueScoreRewardCsReq)
    CmdTakeRogueScoreRewardCsReq = 1829,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueTalentInfoCsReq)
    CmdGetRogueTalentInfoCsReq = 1811,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRoguePickAvatarInfoScNotify)
    CmdSyncRoguePickAvatarInfoScNotify = 1828,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdTakeRogueAeonLevelRewardScRsp)
    CmdTakeRogueAeonLevelRewardScRsp = 1850,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueReviveInfoScNotify)
    CmdSyncRogueReviveInfoScNotify = 1810,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdTakeRogueAeonLevelRewardCsReq)
    CmdTakeRogueAeonLevelRewardCsReq = 1881,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdSyncRogueAeonLevelUpRewardScNotify)
    CmdSyncRogueAeonLevelUpRewardScNotify = 1898,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdEnterRogueScRsp)
    CmdEnterRogueScRsp = 1853,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdGetRogueInitialScoreScRsp)
    CmdGetRogueInitialScoreScRsp = 1863,
    // @@protoc_insertion_point(enum_value:CmdRogueType.CmdEnterRogueCsReq)
    CmdEnterRogueCsReq = 1839,
}

impl ::protobuf::Enum for CmdRogueType {
    const NAME: &'static str = "CmdRogueType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdRogueType> {
        match value {
            0 => ::std::option::Option::Some(CmdRogueType::CmdRogueTypeNone),
            1831 => ::std::option::Option::Some(CmdRogueType::CmdExchangeRogueRewardKeyScRsp),
            1851 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueBuffEnhanceInfoCsReq),
            1836 => ::std::option::Option::Some(CmdRogueType::CmdQuitRogueScRsp),
            1883 => ::std::option::Option::Some(CmdRogueType::CmdEnhanceRogueBuffCsReq),
            1832 => ::std::option::Option::Some(CmdRogueType::CmdOpenRogueChestScRsp),
            1818 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueGetItemScNotify),
            1834 => ::std::option::Option::Some(CmdRogueType::CmdLeaveRogueCsReq),
            1895 => ::std::option::Option::Some(CmdRogueType::CmdEnableRogueTalentScRsp),
            1848 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueFinishScNotify),
            1813 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueSeasonFinishScNotify),
            1896 => ::std::option::Option::Some(CmdRogueType::CmdExchangeRogueRewardKeyCsReq),
            1833 => ::std::option::Option::Some(CmdRogueType::CmdTakeRogueScoreRewardScRsp),
            1889 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueRewardInfoScNotify),
            1858 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueAeonInfoScRsp),
            1814 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueMapRoomScNotify),
            1826 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueExploreWinScNotify),
            1860 => ::std::option::Option::Some(CmdRogueType::CmdOpenRogueChestCsReq),
            1808 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueTalentInfoScRsp),
            1890 => ::std::option::Option::Some(CmdRogueType::CmdPickRogueAvatarCsReq),
            1887 => ::std::option::Option::Some(CmdRogueType::CmdFinishAeonDialogueGroupCsReq),
            1804 => ::std::option::Option::Some(CmdRogueType::CmdEnterRogueMapRoomCsReq),
            1840 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueScoreRewardInfoScRsp),
            1837 => ::std::option::Option::Some(CmdRogueType::CmdLeaveRogueScRsp),
            1888 => ::std::option::Option::Some(CmdRogueType::CmdEnterRogueMapRoomScRsp),
            1879 => ::std::option::Option::Some(CmdRogueType::CmdPickRogueAvatarScRsp),
            1841 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueAeonInfoCsReq),
            1846 => ::std::option::Option::Some(CmdRogueType::CmdStartRogueScRsp),
            1835 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueAeonScNotify),
            1861 => ::std::option::Option::Some(CmdRogueType::CmdReviveRogueAvatarCsReq),
            1859 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueInfoCsReq),
            1886 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueVirtualItemInfoScNotify),
            1820 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueInfoScRsp),
            1870 => ::std::option::Option::Some(CmdRogueType::CmdQuitRogueCsReq),
            1821 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueStatusScNotify),
            1843 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueInitialScoreCsReq),
            1900 => ::std::option::Option::Some(CmdRogueType::CmdEnableRogueTalentCsReq),
            1873 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueBuffEnhanceInfoScRsp),
            1844 => ::std::option::Option::Some(CmdRogueType::CmdFinishAeonDialogueGroupScRsp),
            1899 => ::std::option::Option::Some(CmdRogueType::CmdEnhanceRogueBuffScRsp),
            1842 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueScoreRewardInfoCsReq),
            1807 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueAreaUnlockScNotify),
            1825 => ::std::option::Option::Some(CmdRogueType::CmdReviveRogueAvatarScRsp),
            1803 => ::std::option::Option::Some(CmdRogueType::CmdStartRogueCsReq),
            1829 => ::std::option::Option::Some(CmdRogueType::CmdTakeRogueScoreRewardCsReq),
            1811 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueTalentInfoCsReq),
            1828 => ::std::option::Option::Some(CmdRogueType::CmdSyncRoguePickAvatarInfoScNotify),
            1850 => ::std::option::Option::Some(CmdRogueType::CmdTakeRogueAeonLevelRewardScRsp),
            1810 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueReviveInfoScNotify),
            1881 => ::std::option::Option::Some(CmdRogueType::CmdTakeRogueAeonLevelRewardCsReq),
            1898 => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueAeonLevelUpRewardScNotify),
            1853 => ::std::option::Option::Some(CmdRogueType::CmdEnterRogueScRsp),
            1863 => ::std::option::Option::Some(CmdRogueType::CmdGetRogueInitialScoreScRsp),
            1839 => ::std::option::Option::Some(CmdRogueType::CmdEnterRogueCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdRogueType> {
        match str {
            "CmdRogueTypeNone" => ::std::option::Option::Some(CmdRogueType::CmdRogueTypeNone),
            "CmdExchangeRogueRewardKeyScRsp" => ::std::option::Option::Some(CmdRogueType::CmdExchangeRogueRewardKeyScRsp),
            "CmdGetRogueBuffEnhanceInfoCsReq" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueBuffEnhanceInfoCsReq),
            "CmdQuitRogueScRsp" => ::std::option::Option::Some(CmdRogueType::CmdQuitRogueScRsp),
            "CmdEnhanceRogueBuffCsReq" => ::std::option::Option::Some(CmdRogueType::CmdEnhanceRogueBuffCsReq),
            "CmdOpenRogueChestScRsp" => ::std::option::Option::Some(CmdRogueType::CmdOpenRogueChestScRsp),
            "CmdSyncRogueGetItemScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueGetItemScNotify),
            "CmdLeaveRogueCsReq" => ::std::option::Option::Some(CmdRogueType::CmdLeaveRogueCsReq),
            "CmdEnableRogueTalentScRsp" => ::std::option::Option::Some(CmdRogueType::CmdEnableRogueTalentScRsp),
            "CmdSyncRogueFinishScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueFinishScNotify),
            "CmdSyncRogueSeasonFinishScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueSeasonFinishScNotify),
            "CmdExchangeRogueRewardKeyCsReq" => ::std::option::Option::Some(CmdRogueType::CmdExchangeRogueRewardKeyCsReq),
            "CmdTakeRogueScoreRewardScRsp" => ::std::option::Option::Some(CmdRogueType::CmdTakeRogueScoreRewardScRsp),
            "CmdSyncRogueRewardInfoScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueRewardInfoScNotify),
            "CmdGetRogueAeonInfoScRsp" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueAeonInfoScRsp),
            "CmdSyncRogueMapRoomScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueMapRoomScNotify),
            "CmdSyncRogueExploreWinScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueExploreWinScNotify),
            "CmdOpenRogueChestCsReq" => ::std::option::Option::Some(CmdRogueType::CmdOpenRogueChestCsReq),
            "CmdGetRogueTalentInfoScRsp" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueTalentInfoScRsp),
            "CmdPickRogueAvatarCsReq" => ::std::option::Option::Some(CmdRogueType::CmdPickRogueAvatarCsReq),
            "CmdFinishAeonDialogueGroupCsReq" => ::std::option::Option::Some(CmdRogueType::CmdFinishAeonDialogueGroupCsReq),
            "CmdEnterRogueMapRoomCsReq" => ::std::option::Option::Some(CmdRogueType::CmdEnterRogueMapRoomCsReq),
            "CmdGetRogueScoreRewardInfoScRsp" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueScoreRewardInfoScRsp),
            "CmdLeaveRogueScRsp" => ::std::option::Option::Some(CmdRogueType::CmdLeaveRogueScRsp),
            "CmdEnterRogueMapRoomScRsp" => ::std::option::Option::Some(CmdRogueType::CmdEnterRogueMapRoomScRsp),
            "CmdPickRogueAvatarScRsp" => ::std::option::Option::Some(CmdRogueType::CmdPickRogueAvatarScRsp),
            "CmdGetRogueAeonInfoCsReq" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueAeonInfoCsReq),
            "CmdStartRogueScRsp" => ::std::option::Option::Some(CmdRogueType::CmdStartRogueScRsp),
            "CmdSyncRogueAeonScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueAeonScNotify),
            "CmdReviveRogueAvatarCsReq" => ::std::option::Option::Some(CmdRogueType::CmdReviveRogueAvatarCsReq),
            "CmdGetRogueInfoCsReq" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueInfoCsReq),
            "CmdSyncRogueVirtualItemInfoScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueVirtualItemInfoScNotify),
            "CmdGetRogueInfoScRsp" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueInfoScRsp),
            "CmdQuitRogueCsReq" => ::std::option::Option::Some(CmdRogueType::CmdQuitRogueCsReq),
            "CmdSyncRogueStatusScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueStatusScNotify),
            "CmdGetRogueInitialScoreCsReq" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueInitialScoreCsReq),
            "CmdEnableRogueTalentCsReq" => ::std::option::Option::Some(CmdRogueType::CmdEnableRogueTalentCsReq),
            "CmdGetRogueBuffEnhanceInfoScRsp" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueBuffEnhanceInfoScRsp),
            "CmdFinishAeonDialogueGroupScRsp" => ::std::option::Option::Some(CmdRogueType::CmdFinishAeonDialogueGroupScRsp),
            "CmdEnhanceRogueBuffScRsp" => ::std::option::Option::Some(CmdRogueType::CmdEnhanceRogueBuffScRsp),
            "CmdGetRogueScoreRewardInfoCsReq" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueScoreRewardInfoCsReq),
            "CmdSyncRogueAreaUnlockScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueAreaUnlockScNotify),
            "CmdReviveRogueAvatarScRsp" => ::std::option::Option::Some(CmdRogueType::CmdReviveRogueAvatarScRsp),
            "CmdStartRogueCsReq" => ::std::option::Option::Some(CmdRogueType::CmdStartRogueCsReq),
            "CmdTakeRogueScoreRewardCsReq" => ::std::option::Option::Some(CmdRogueType::CmdTakeRogueScoreRewardCsReq),
            "CmdGetRogueTalentInfoCsReq" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueTalentInfoCsReq),
            "CmdSyncRoguePickAvatarInfoScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRoguePickAvatarInfoScNotify),
            "CmdTakeRogueAeonLevelRewardScRsp" => ::std::option::Option::Some(CmdRogueType::CmdTakeRogueAeonLevelRewardScRsp),
            "CmdSyncRogueReviveInfoScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueReviveInfoScNotify),
            "CmdTakeRogueAeonLevelRewardCsReq" => ::std::option::Option::Some(CmdRogueType::CmdTakeRogueAeonLevelRewardCsReq),
            "CmdSyncRogueAeonLevelUpRewardScNotify" => ::std::option::Option::Some(CmdRogueType::CmdSyncRogueAeonLevelUpRewardScNotify),
            "CmdEnterRogueScRsp" => ::std::option::Option::Some(CmdRogueType::CmdEnterRogueScRsp),
            "CmdGetRogueInitialScoreScRsp" => ::std::option::Option::Some(CmdRogueType::CmdGetRogueInitialScoreScRsp),
            "CmdEnterRogueCsReq" => ::std::option::Option::Some(CmdRogueType::CmdEnterRogueCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdRogueType] = &[
        CmdRogueType::CmdRogueTypeNone,
        CmdRogueType::CmdExchangeRogueRewardKeyScRsp,
        CmdRogueType::CmdGetRogueBuffEnhanceInfoCsReq,
        CmdRogueType::CmdQuitRogueScRsp,
        CmdRogueType::CmdEnhanceRogueBuffCsReq,
        CmdRogueType::CmdOpenRogueChestScRsp,
        CmdRogueType::CmdSyncRogueGetItemScNotify,
        CmdRogueType::CmdLeaveRogueCsReq,
        CmdRogueType::CmdEnableRogueTalentScRsp,
        CmdRogueType::CmdSyncRogueFinishScNotify,
        CmdRogueType::CmdSyncRogueSeasonFinishScNotify,
        CmdRogueType::CmdExchangeRogueRewardKeyCsReq,
        CmdRogueType::CmdTakeRogueScoreRewardScRsp,
        CmdRogueType::CmdSyncRogueRewardInfoScNotify,
        CmdRogueType::CmdGetRogueAeonInfoScRsp,
        CmdRogueType::CmdSyncRogueMapRoomScNotify,
        CmdRogueType::CmdSyncRogueExploreWinScNotify,
        CmdRogueType::CmdOpenRogueChestCsReq,
        CmdRogueType::CmdGetRogueTalentInfoScRsp,
        CmdRogueType::CmdPickRogueAvatarCsReq,
        CmdRogueType::CmdFinishAeonDialogueGroupCsReq,
        CmdRogueType::CmdEnterRogueMapRoomCsReq,
        CmdRogueType::CmdGetRogueScoreRewardInfoScRsp,
        CmdRogueType::CmdLeaveRogueScRsp,
        CmdRogueType::CmdEnterRogueMapRoomScRsp,
        CmdRogueType::CmdPickRogueAvatarScRsp,
        CmdRogueType::CmdGetRogueAeonInfoCsReq,
        CmdRogueType::CmdStartRogueScRsp,
        CmdRogueType::CmdSyncRogueAeonScNotify,
        CmdRogueType::CmdReviveRogueAvatarCsReq,
        CmdRogueType::CmdGetRogueInfoCsReq,
        CmdRogueType::CmdSyncRogueVirtualItemInfoScNotify,
        CmdRogueType::CmdGetRogueInfoScRsp,
        CmdRogueType::CmdQuitRogueCsReq,
        CmdRogueType::CmdSyncRogueStatusScNotify,
        CmdRogueType::CmdGetRogueInitialScoreCsReq,
        CmdRogueType::CmdEnableRogueTalentCsReq,
        CmdRogueType::CmdGetRogueBuffEnhanceInfoScRsp,
        CmdRogueType::CmdFinishAeonDialogueGroupScRsp,
        CmdRogueType::CmdEnhanceRogueBuffScRsp,
        CmdRogueType::CmdGetRogueScoreRewardInfoCsReq,
        CmdRogueType::CmdSyncRogueAreaUnlockScNotify,
        CmdRogueType::CmdReviveRogueAvatarScRsp,
        CmdRogueType::CmdStartRogueCsReq,
        CmdRogueType::CmdTakeRogueScoreRewardCsReq,
        CmdRogueType::CmdGetRogueTalentInfoCsReq,
        CmdRogueType::CmdSyncRoguePickAvatarInfoScNotify,
        CmdRogueType::CmdTakeRogueAeonLevelRewardScRsp,
        CmdRogueType::CmdSyncRogueReviveInfoScNotify,
        CmdRogueType::CmdTakeRogueAeonLevelRewardCsReq,
        CmdRogueType::CmdSyncRogueAeonLevelUpRewardScNotify,
        CmdRogueType::CmdEnterRogueScRsp,
        CmdRogueType::CmdGetRogueInitialScoreScRsp,
        CmdRogueType::CmdEnterRogueCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdRogueType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdRogueType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdRogueType::CmdRogueTypeNone => 0,
            CmdRogueType::CmdExchangeRogueRewardKeyScRsp => 1,
            CmdRogueType::CmdGetRogueBuffEnhanceInfoCsReq => 2,
            CmdRogueType::CmdQuitRogueScRsp => 3,
            CmdRogueType::CmdEnhanceRogueBuffCsReq => 4,
            CmdRogueType::CmdOpenRogueChestScRsp => 5,
            CmdRogueType::CmdSyncRogueGetItemScNotify => 6,
            CmdRogueType::CmdLeaveRogueCsReq => 7,
            CmdRogueType::CmdEnableRogueTalentScRsp => 8,
            CmdRogueType::CmdSyncRogueFinishScNotify => 9,
            CmdRogueType::CmdSyncRogueSeasonFinishScNotify => 10,
            CmdRogueType::CmdExchangeRogueRewardKeyCsReq => 11,
            CmdRogueType::CmdTakeRogueScoreRewardScRsp => 12,
            CmdRogueType::CmdSyncRogueRewardInfoScNotify => 13,
            CmdRogueType::CmdGetRogueAeonInfoScRsp => 14,
            CmdRogueType::CmdSyncRogueMapRoomScNotify => 15,
            CmdRogueType::CmdSyncRogueExploreWinScNotify => 16,
            CmdRogueType::CmdOpenRogueChestCsReq => 17,
            CmdRogueType::CmdGetRogueTalentInfoScRsp => 18,
            CmdRogueType::CmdPickRogueAvatarCsReq => 19,
            CmdRogueType::CmdFinishAeonDialogueGroupCsReq => 20,
            CmdRogueType::CmdEnterRogueMapRoomCsReq => 21,
            CmdRogueType::CmdGetRogueScoreRewardInfoScRsp => 22,
            CmdRogueType::CmdLeaveRogueScRsp => 23,
            CmdRogueType::CmdEnterRogueMapRoomScRsp => 24,
            CmdRogueType::CmdPickRogueAvatarScRsp => 25,
            CmdRogueType::CmdGetRogueAeonInfoCsReq => 26,
            CmdRogueType::CmdStartRogueScRsp => 27,
            CmdRogueType::CmdSyncRogueAeonScNotify => 28,
            CmdRogueType::CmdReviveRogueAvatarCsReq => 29,
            CmdRogueType::CmdGetRogueInfoCsReq => 30,
            CmdRogueType::CmdSyncRogueVirtualItemInfoScNotify => 31,
            CmdRogueType::CmdGetRogueInfoScRsp => 32,
            CmdRogueType::CmdQuitRogueCsReq => 33,
            CmdRogueType::CmdSyncRogueStatusScNotify => 34,
            CmdRogueType::CmdGetRogueInitialScoreCsReq => 35,
            CmdRogueType::CmdEnableRogueTalentCsReq => 36,
            CmdRogueType::CmdGetRogueBuffEnhanceInfoScRsp => 37,
            CmdRogueType::CmdFinishAeonDialogueGroupScRsp => 38,
            CmdRogueType::CmdEnhanceRogueBuffScRsp => 39,
            CmdRogueType::CmdGetRogueScoreRewardInfoCsReq => 40,
            CmdRogueType::CmdSyncRogueAreaUnlockScNotify => 41,
            CmdRogueType::CmdReviveRogueAvatarScRsp => 42,
            CmdRogueType::CmdStartRogueCsReq => 43,
            CmdRogueType::CmdTakeRogueScoreRewardCsReq => 44,
            CmdRogueType::CmdGetRogueTalentInfoCsReq => 45,
            CmdRogueType::CmdSyncRoguePickAvatarInfoScNotify => 46,
            CmdRogueType::CmdTakeRogueAeonLevelRewardScRsp => 47,
            CmdRogueType::CmdSyncRogueReviveInfoScNotify => 48,
            CmdRogueType::CmdTakeRogueAeonLevelRewardCsReq => 49,
            CmdRogueType::CmdSyncRogueAeonLevelUpRewardScNotify => 50,
            CmdRogueType::CmdEnterRogueScRsp => 51,
            CmdRogueType::CmdGetRogueInitialScoreScRsp => 52,
            CmdRogueType::CmdEnterRogueCsReq => 53,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdRogueType {
    fn default() -> Self {
        CmdRogueType::CmdRogueTypeNone
    }
}

impl CmdRogueType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdRogueType>("CmdRogueType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12CmdRogueType.proto*\xfb\r\n\x0cCmdRogueType\x12\x14\n\x10CmdRogueT\
    ypeNone\x10\0\x12#\n\x1eCmdExchangeRogueRewardKeyScRsp\x10\xa7\x0e\x12$\
    \n\x1fCmdGetRogueBuffEnhanceInfoCsReq\x10\xbb\x0e\x12\x16\n\x11CmdQuitRo\
    gueScRsp\x10\xac\x0e\x12\x1d\n\x18CmdEnhanceRogueBuffCsReq\x10\xdb\x0e\
    \x12\x1b\n\x16CmdOpenRogueChestScRsp\x10\xa8\x0e\x12\x20\n\x1bCmdSyncRog\
    ueGetItemScNotify\x10\x9a\x0e\x12\x17\n\x12CmdLeaveRogueCsReq\x10\xaa\
    \x0e\x12\x1e\n\x19CmdEnableRogueTalentScRsp\x10\xe7\x0e\x12\x1f\n\x1aCmd\
    SyncRogueFinishScNotify\x10\xb8\x0e\x12%\n\x20CmdSyncRogueSeasonFinishSc\
    Notify\x10\x95\x0e\x12#\n\x1eCmdExchangeRogueRewardKeyCsReq\x10\xe8\x0e\
    \x12!\n\x1cCmdTakeRogueScoreRewardScRsp\x10\xa9\x0e\x12#\n\x1eCmdSyncRog\
    ueRewardInfoScNotify\x10\xe1\x0e\x12\x1d\n\x18CmdGetRogueAeonInfoScRsp\
    \x10\xc2\x0e\x12\x20\n\x1bCmdSyncRogueMapRoomScNotify\x10\x96\x0e\x12#\n\
    \x1eCmdSyncRogueExploreWinScNotify\x10\xa2\x0e\x12\x1b\n\x16CmdOpenRogue\
    ChestCsReq\x10\xc4\x0e\x12\x1f\n\x1aCmdGetRogueTalentInfoScRsp\x10\x90\
    \x0e\x12\x1c\n\x17CmdPickRogueAvatarCsReq\x10\xe2\x0e\x12$\n\x1fCmdFinis\
    hAeonDialogueGroupCsReq\x10\xdf\x0e\x12\x1e\n\x19CmdEnterRogueMapRoomCsR\
    eq\x10\x8c\x0e\x12$\n\x1fCmdGetRogueScoreRewardInfoScRsp\x10\xb0\x0e\x12\
    \x17\n\x12CmdLeaveRogueScRsp\x10\xad\x0e\x12\x1e\n\x19CmdEnterRogueMapRo\
    omScRsp\x10\xe0\x0e\x12\x1c\n\x17CmdPickRogueAvatarScRsp\x10\xd7\x0e\x12\
    \x1d\n\x18CmdGetRogueAeonInfoCsReq\x10\xb1\x0e\x12\x17\n\x12CmdStartRogu\
    eScRsp\x10\xb6\x0e\x12\x1d\n\x18CmdSyncRogueAeonScNotify\x10\xab\x0e\x12\
    \x1e\n\x19CmdReviveRogueAvatarCsReq\x10\xc5\x0e\x12\x19\n\x14CmdGetRogue\
    InfoCsReq\x10\xc3\x0e\x12(\n#CmdSyncRogueVirtualItemInfoScNotify\x10\xde\
    \x0e\x12\x19\n\x14CmdGetRogueInfoScRsp\x10\x9c\x0e\x12\x16\n\x11CmdQuitR\
    ogueCsReq\x10\xce\x0e\x12\x1f\n\x1aCmdSyncRogueStatusScNotify\x10\x9d\
    \x0e\x12!\n\x1cCmdGetRogueInitialScoreCsReq\x10\xb3\x0e\x12\x1e\n\x19Cmd\
    EnableRogueTalentCsReq\x10\xec\x0e\x12$\n\x1fCmdGetRogueBuffEnhanceInfoS\
    cRsp\x10\xd1\x0e\x12$\n\x1fCmdFinishAeonDialogueGroupScRsp\x10\xb4\x0e\
    \x12\x1d\n\x18CmdEnhanceRogueBuffScRsp\x10\xeb\x0e\x12$\n\x1fCmdGetRogue\
    ScoreRewardInfoCsReq\x10\xb2\x0e\x12#\n\x1eCmdSyncRogueAreaUnlockScNotif\
    y\x10\x8f\x0e\x12\x1e\n\x19CmdReviveRogueAvatarScRsp\x10\xa1\x0e\x12\x17\
    \n\x12CmdStartRogueCsReq\x10\x8b\x0e\x12!\n\x1cCmdTakeRogueScoreRewardCs\
    Req\x10\xa5\x0e\x12\x1f\n\x1aCmdGetRogueTalentInfoCsReq\x10\x93\x0e\x12'\
    \n\"CmdSyncRoguePickAvatarInfoScNotify\x10\xa4\x0e\x12%\n\x20CmdTakeRogu\
    eAeonLevelRewardScRsp\x10\xba\x0e\x12#\n\x1eCmdSyncRogueReviveInfoScNoti\
    fy\x10\x92\x0e\x12%\n\x20CmdTakeRogueAeonLevelRewardCsReq\x10\xd9\x0e\
    \x12*\n%CmdSyncRogueAeonLevelUpRewardScNotify\x10\xea\x0e\x12\x17\n\x12C\
    mdEnterRogueScRsp\x10\xbd\x0e\x12!\n\x1cCmdGetRogueInitialScoreScRsp\x10\
    \xc7\x0e\x12\x17\n\x12CmdEnterRogueCsReq\x10\xaf\x0eb\x06proto3\
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
            enums.push(CmdRogueType::generated_enum_descriptor_data());
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
