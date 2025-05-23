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

//! Generated file from `CmdMapRotationType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMapRotationType)
pub enum CmdMapRotationType {
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdMapRotationTypeNone)
    CmdMapRotationTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdLeaveMapRotationRegionCsReq)
    CmdLeaveMapRotationRegionCsReq = 6828,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdUpdateEnergyScNotify)
    CmdUpdateEnergyScNotify = 6880,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdInteractChargerScRsp)
    CmdInteractChargerScRsp = 6824,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRemoveRotaterScRsp)
    CmdRemoveRotaterScRsp = 6823,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdGetMapRotationDataScRsp)
    CmdGetMapRotationDataScRsp = 6814,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRotateMapScRsp)
    CmdRotateMapScRsp = 6876,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRotateMapCsReq)
    CmdRotateMapCsReq = 6897,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdInteractChargerCsReq)
    CmdInteractChargerCsReq = 6858,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdResetMapRotationRegionCsReq)
    CmdResetMapRotationRegionCsReq = 6849,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdDeployRotaterCsReq)
    CmdDeployRotaterCsReq = 6830,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdUpdateMapRotationDataScNotify)
    CmdUpdateMapRotationDataScNotify = 6839,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdLeaveMapRotationRegionScNotify)
    CmdLeaveMapRotationRegionScNotify = 6847,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdLeaveMapRotationRegionScRsp)
    CmdLeaveMapRotationRegionScRsp = 6811,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdEnterMapRotationRegionScRsp)
    CmdEnterMapRotationRegionScRsp = 6868,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdEnterMapRotationRegionCsReq)
    CmdEnterMapRotationRegionCsReq = 6801,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdGetMapRotationDataCsReq)
    CmdGetMapRotationDataCsReq = 6805,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdResetMapRotationRegionScRsp)
    CmdResetMapRotationRegionScRsp = 6822,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdUpdateRotaterScNotify)
    CmdUpdateRotaterScNotify = 6812,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRemoveRotaterCsReq)
    CmdRemoveRotaterCsReq = 6896,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdDeployRotaterScRsp)
    CmdDeployRotaterScRsp = 6856,
}

impl ::protobuf::Enum for CmdMapRotationType {
    const NAME: &'static str = "CmdMapRotationType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMapRotationType> {
        match value {
            0 => ::std::option::Option::Some(CmdMapRotationType::CmdMapRotationTypeNone),
            6828 => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionCsReq),
            6880 => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateEnergyScNotify),
            6824 => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerScRsp),
            6823 => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterScRsp),
            6814 => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataScRsp),
            6876 => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapScRsp),
            6897 => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapCsReq),
            6858 => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerCsReq),
            6849 => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionCsReq),
            6830 => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterCsReq),
            6839 => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateMapRotationDataScNotify),
            6847 => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScNotify),
            6811 => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScRsp),
            6868 => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionScRsp),
            6801 => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionCsReq),
            6805 => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataCsReq),
            6822 => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionScRsp),
            6812 => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateRotaterScNotify),
            6896 => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterCsReq),
            6856 => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMapRotationType> {
        match str {
            "CmdMapRotationTypeNone" => ::std::option::Option::Some(CmdMapRotationType::CmdMapRotationTypeNone),
            "CmdLeaveMapRotationRegionCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionCsReq),
            "CmdUpdateEnergyScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateEnergyScNotify),
            "CmdInteractChargerScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerScRsp),
            "CmdRemoveRotaterScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterScRsp),
            "CmdGetMapRotationDataScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataScRsp),
            "CmdRotateMapScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapScRsp),
            "CmdRotateMapCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapCsReq),
            "CmdInteractChargerCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerCsReq),
            "CmdResetMapRotationRegionCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionCsReq),
            "CmdDeployRotaterCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterCsReq),
            "CmdUpdateMapRotationDataScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateMapRotationDataScNotify),
            "CmdLeaveMapRotationRegionScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScNotify),
            "CmdLeaveMapRotationRegionScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScRsp),
            "CmdEnterMapRotationRegionScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionScRsp),
            "CmdEnterMapRotationRegionCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionCsReq),
            "CmdGetMapRotationDataCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataCsReq),
            "CmdResetMapRotationRegionScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionScRsp),
            "CmdUpdateRotaterScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateRotaterScNotify),
            "CmdRemoveRotaterCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterCsReq),
            "CmdDeployRotaterScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMapRotationType] = &[
        CmdMapRotationType::CmdMapRotationTypeNone,
        CmdMapRotationType::CmdLeaveMapRotationRegionCsReq,
        CmdMapRotationType::CmdUpdateEnergyScNotify,
        CmdMapRotationType::CmdInteractChargerScRsp,
        CmdMapRotationType::CmdRemoveRotaterScRsp,
        CmdMapRotationType::CmdGetMapRotationDataScRsp,
        CmdMapRotationType::CmdRotateMapScRsp,
        CmdMapRotationType::CmdRotateMapCsReq,
        CmdMapRotationType::CmdInteractChargerCsReq,
        CmdMapRotationType::CmdResetMapRotationRegionCsReq,
        CmdMapRotationType::CmdDeployRotaterCsReq,
        CmdMapRotationType::CmdUpdateMapRotationDataScNotify,
        CmdMapRotationType::CmdLeaveMapRotationRegionScNotify,
        CmdMapRotationType::CmdLeaveMapRotationRegionScRsp,
        CmdMapRotationType::CmdEnterMapRotationRegionScRsp,
        CmdMapRotationType::CmdEnterMapRotationRegionCsReq,
        CmdMapRotationType::CmdGetMapRotationDataCsReq,
        CmdMapRotationType::CmdResetMapRotationRegionScRsp,
        CmdMapRotationType::CmdUpdateRotaterScNotify,
        CmdMapRotationType::CmdRemoveRotaterCsReq,
        CmdMapRotationType::CmdDeployRotaterScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdMapRotationType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMapRotationType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMapRotationType::CmdMapRotationTypeNone => 0,
            CmdMapRotationType::CmdLeaveMapRotationRegionCsReq => 1,
            CmdMapRotationType::CmdUpdateEnergyScNotify => 2,
            CmdMapRotationType::CmdInteractChargerScRsp => 3,
            CmdMapRotationType::CmdRemoveRotaterScRsp => 4,
            CmdMapRotationType::CmdGetMapRotationDataScRsp => 5,
            CmdMapRotationType::CmdRotateMapScRsp => 6,
            CmdMapRotationType::CmdRotateMapCsReq => 7,
            CmdMapRotationType::CmdInteractChargerCsReq => 8,
            CmdMapRotationType::CmdResetMapRotationRegionCsReq => 9,
            CmdMapRotationType::CmdDeployRotaterCsReq => 10,
            CmdMapRotationType::CmdUpdateMapRotationDataScNotify => 11,
            CmdMapRotationType::CmdLeaveMapRotationRegionScNotify => 12,
            CmdMapRotationType::CmdLeaveMapRotationRegionScRsp => 13,
            CmdMapRotationType::CmdEnterMapRotationRegionScRsp => 14,
            CmdMapRotationType::CmdEnterMapRotationRegionCsReq => 15,
            CmdMapRotationType::CmdGetMapRotationDataCsReq => 16,
            CmdMapRotationType::CmdResetMapRotationRegionScRsp => 17,
            CmdMapRotationType::CmdUpdateRotaterScNotify => 18,
            CmdMapRotationType::CmdRemoveRotaterCsReq => 19,
            CmdMapRotationType::CmdDeployRotaterScRsp => 20,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMapRotationType {
    fn default() -> Self {
        CmdMapRotationType::CmdMapRotationTypeNone
    }
}

impl CmdMapRotationType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMapRotationType>("CmdMapRotationType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdMapRotationType.proto*\xb8\x05\n\x12CmdMapRotationType\x12\x1a\
    \n\x16CmdMapRotationTypeNone\x10\0\x12#\n\x1eCmdLeaveMapRotationRegionCs\
    Req\x10\xac5\x12\x1c\n\x17CmdUpdateEnergyScNotify\x10\xe05\x12\x1c\n\x17\
    CmdInteractChargerScRsp\x10\xa85\x12\x1a\n\x15CmdRemoveRotaterScRsp\x10\
    \xa75\x12\x1f\n\x1aCmdGetMapRotationDataScRsp\x10\x9e5\x12\x16\n\x11CmdR\
    otateMapScRsp\x10\xdc5\x12\x16\n\x11CmdRotateMapCsReq\x10\xf15\x12\x1c\n\
    \x17CmdInteractChargerCsReq\x10\xca5\x12#\n\x1eCmdResetMapRotationRegion\
    CsReq\x10\xc15\x12\x1a\n\x15CmdDeployRotaterCsReq\x10\xae5\x12%\n\x20Cmd\
    UpdateMapRotationDataScNotify\x10\xb75\x12&\n!CmdLeaveMapRotationRegionS\
    cNotify\x10\xbf5\x12#\n\x1eCmdLeaveMapRotationRegionScRsp\x10\x9b5\x12#\
    \n\x1eCmdEnterMapRotationRegionScRsp\x10\xd45\x12#\n\x1eCmdEnterMapRotat\
    ionRegionCsReq\x10\x915\x12\x1f\n\x1aCmdGetMapRotationDataCsReq\x10\x955\
    \x12#\n\x1eCmdResetMapRotationRegionScRsp\x10\xa65\x12\x1d\n\x18CmdUpdat\
    eRotaterScNotify\x10\x9c5\x12\x1a\n\x15CmdRemoveRotaterCsReq\x10\xf05\
    \x12\x1a\n\x15CmdDeployRotaterScRsp\x10\xc85b\x06proto3\
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
            enums.push(CmdMapRotationType::generated_enum_descriptor_data());
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
