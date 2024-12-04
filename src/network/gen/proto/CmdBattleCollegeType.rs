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

//! Generated file from `CmdBattleCollegeType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdBattleCollegeType)
pub enum CmdBattleCollegeType {
    // @@protoc_insertion_point(enum_value:CmdBattleCollegeType.CmdBattleCollegeTypeNone)
    CmdBattleCollegeTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdBattleCollegeType.CmdBattleCollegeDataChangeScNotify)
    CmdBattleCollegeDataChangeScNotify = 5703,
    // @@protoc_insertion_point(enum_value:CmdBattleCollegeType.CmdGetBattleCollegeDataCsReq)
    CmdGetBattleCollegeDataCsReq = 5759,
    // @@protoc_insertion_point(enum_value:CmdBattleCollegeType.CmdStartBattleCollegeCsReq)
    CmdStartBattleCollegeCsReq = 5746,
    // @@protoc_insertion_point(enum_value:CmdBattleCollegeType.CmdGetBattleCollegeDataScRsp)
    CmdGetBattleCollegeDataScRsp = 5720,
    // @@protoc_insertion_point(enum_value:CmdBattleCollegeType.CmdStartBattleCollegeScRsp)
    CmdStartBattleCollegeScRsp = 5739,
}

impl ::protobuf::Enum for CmdBattleCollegeType {
    const NAME: &'static str = "CmdBattleCollegeType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdBattleCollegeType> {
        match value {
            0 => ::std::option::Option::Some(CmdBattleCollegeType::CmdBattleCollegeTypeNone),
            5703 => ::std::option::Option::Some(CmdBattleCollegeType::CmdBattleCollegeDataChangeScNotify),
            5759 => ::std::option::Option::Some(CmdBattleCollegeType::CmdGetBattleCollegeDataCsReq),
            5746 => ::std::option::Option::Some(CmdBattleCollegeType::CmdStartBattleCollegeCsReq),
            5720 => ::std::option::Option::Some(CmdBattleCollegeType::CmdGetBattleCollegeDataScRsp),
            5739 => ::std::option::Option::Some(CmdBattleCollegeType::CmdStartBattleCollegeScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdBattleCollegeType> {
        match str {
            "CmdBattleCollegeTypeNone" => ::std::option::Option::Some(CmdBattleCollegeType::CmdBattleCollegeTypeNone),
            "CmdBattleCollegeDataChangeScNotify" => ::std::option::Option::Some(CmdBattleCollegeType::CmdBattleCollegeDataChangeScNotify),
            "CmdGetBattleCollegeDataCsReq" => ::std::option::Option::Some(CmdBattleCollegeType::CmdGetBattleCollegeDataCsReq),
            "CmdStartBattleCollegeCsReq" => ::std::option::Option::Some(CmdBattleCollegeType::CmdStartBattleCollegeCsReq),
            "CmdGetBattleCollegeDataScRsp" => ::std::option::Option::Some(CmdBattleCollegeType::CmdGetBattleCollegeDataScRsp),
            "CmdStartBattleCollegeScRsp" => ::std::option::Option::Some(CmdBattleCollegeType::CmdStartBattleCollegeScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdBattleCollegeType] = &[
        CmdBattleCollegeType::CmdBattleCollegeTypeNone,
        CmdBattleCollegeType::CmdBattleCollegeDataChangeScNotify,
        CmdBattleCollegeType::CmdGetBattleCollegeDataCsReq,
        CmdBattleCollegeType::CmdStartBattleCollegeCsReq,
        CmdBattleCollegeType::CmdGetBattleCollegeDataScRsp,
        CmdBattleCollegeType::CmdStartBattleCollegeScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdBattleCollegeType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdBattleCollegeType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdBattleCollegeType::CmdBattleCollegeTypeNone => 0,
            CmdBattleCollegeType::CmdBattleCollegeDataChangeScNotify => 1,
            CmdBattleCollegeType::CmdGetBattleCollegeDataCsReq => 2,
            CmdBattleCollegeType::CmdStartBattleCollegeCsReq => 3,
            CmdBattleCollegeType::CmdGetBattleCollegeDataScRsp => 4,
            CmdBattleCollegeType::CmdStartBattleCollegeScRsp => 5,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdBattleCollegeType {
    fn default() -> Self {
        CmdBattleCollegeType::CmdBattleCollegeTypeNone
    }
}

impl CmdBattleCollegeType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdBattleCollegeType>("CmdBattleCollegeType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aCmdBattleCollegeType.proto*\xe5\x01\n\x14CmdBattleCollegeType\x12\
    \x1c\n\x18CmdBattleCollegeTypeNone\x10\0\x12'\n\"CmdBattleCollegeDataCha\
    ngeScNotify\x10\xc7,\x12!\n\x1cCmdGetBattleCollegeDataCsReq\x10\xff,\x12\
    \x1f\n\x1aCmdStartBattleCollegeCsReq\x10\xf2,\x12!\n\x1cCmdGetBattleColl\
    egeDataScRsp\x10\xd8,\x12\x1f\n\x1aCmdStartBattleCollegeScRsp\x10\xeb,b\
    \x06proto3\
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
            enums.push(CmdBattleCollegeType::generated_enum_descriptor_data());
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
