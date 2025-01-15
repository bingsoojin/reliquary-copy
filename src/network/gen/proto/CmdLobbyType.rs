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

//! Generated file from `CmdLobbyType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdLobbyType)
pub enum CmdLobbyType {
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyTypeNone)
    CmdLobbyTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyBeginCsReq)
    CmdLobbyBeginCsReq = 7365,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyInviteScRsp)
    CmdLobbyInviteScRsp = 7360,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbySyncInfoScNotify)
    CmdLobbySyncInfoScNotify = 7395,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyInviteCsReq)
    CmdLobbyInviteCsReq = 7378,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyKickOutScRsp)
    CmdLobbyKickOutScRsp = 7396,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyQuitCsReq)
    CmdLobbyQuitCsReq = 7398,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyQuitScRsp)
    CmdLobbyQuitScRsp = 7363,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyCreateScRsp)
    CmdLobbyCreateScRsp = 7400,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyKickOutCsReq)
    CmdLobbyKickOutCsReq = 7397,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyModifyPlayerInfoScRsp)
    CmdLobbyModifyPlayerInfoScRsp = 7393,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyGetInfoCsReq)
    CmdLobbyGetInfoCsReq = 7354,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyGetInfoScRsp)
    CmdLobbyGetInfoScRsp = 7390,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyModifyPlayerInfoCsReq)
    CmdLobbyModifyPlayerInfoCsReq = 7359,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyBeginScRsp)
    CmdLobbyBeginScRsp = 7399,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyInviteScNotify)
    CmdLobbyInviteScNotify = 7367,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyJoinCsReq)
    CmdLobbyJoinCsReq = 7380,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyJoinScRsp)
    CmdLobbyJoinScRsp = 7373,
    // @@protoc_insertion_point(enum_value:CmdLobbyType.CmdLobbyCreateCsReq)
    CmdLobbyCreateCsReq = 7362,
}

impl ::protobuf::Enum for CmdLobbyType {
    const NAME: &'static str = "CmdLobbyType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdLobbyType> {
        match value {
            0 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyTypeNone),
            7365 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyBeginCsReq),
            7360 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyInviteScRsp),
            7395 => ::std::option::Option::Some(CmdLobbyType::CmdLobbySyncInfoScNotify),
            7378 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyInviteCsReq),
            7396 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyKickOutScRsp),
            7398 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyQuitCsReq),
            7363 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyQuitScRsp),
            7400 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyCreateScRsp),
            7397 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyKickOutCsReq),
            7393 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyModifyPlayerInfoScRsp),
            7354 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyGetInfoCsReq),
            7390 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyGetInfoScRsp),
            7359 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyModifyPlayerInfoCsReq),
            7399 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyBeginScRsp),
            7367 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyInviteScNotify),
            7380 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyJoinCsReq),
            7373 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyJoinScRsp),
            7362 => ::std::option::Option::Some(CmdLobbyType::CmdLobbyCreateCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdLobbyType> {
        match str {
            "CmdLobbyTypeNone" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyTypeNone),
            "CmdLobbyBeginCsReq" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyBeginCsReq),
            "CmdLobbyInviteScRsp" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyInviteScRsp),
            "CmdLobbySyncInfoScNotify" => ::std::option::Option::Some(CmdLobbyType::CmdLobbySyncInfoScNotify),
            "CmdLobbyInviteCsReq" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyInviteCsReq),
            "CmdLobbyKickOutScRsp" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyKickOutScRsp),
            "CmdLobbyQuitCsReq" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyQuitCsReq),
            "CmdLobbyQuitScRsp" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyQuitScRsp),
            "CmdLobbyCreateScRsp" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyCreateScRsp),
            "CmdLobbyKickOutCsReq" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyKickOutCsReq),
            "CmdLobbyModifyPlayerInfoScRsp" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyModifyPlayerInfoScRsp),
            "CmdLobbyGetInfoCsReq" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyGetInfoCsReq),
            "CmdLobbyGetInfoScRsp" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyGetInfoScRsp),
            "CmdLobbyModifyPlayerInfoCsReq" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyModifyPlayerInfoCsReq),
            "CmdLobbyBeginScRsp" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyBeginScRsp),
            "CmdLobbyInviteScNotify" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyInviteScNotify),
            "CmdLobbyJoinCsReq" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyJoinCsReq),
            "CmdLobbyJoinScRsp" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyJoinScRsp),
            "CmdLobbyCreateCsReq" => ::std::option::Option::Some(CmdLobbyType::CmdLobbyCreateCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdLobbyType] = &[
        CmdLobbyType::CmdLobbyTypeNone,
        CmdLobbyType::CmdLobbyBeginCsReq,
        CmdLobbyType::CmdLobbyInviteScRsp,
        CmdLobbyType::CmdLobbySyncInfoScNotify,
        CmdLobbyType::CmdLobbyInviteCsReq,
        CmdLobbyType::CmdLobbyKickOutScRsp,
        CmdLobbyType::CmdLobbyQuitCsReq,
        CmdLobbyType::CmdLobbyQuitScRsp,
        CmdLobbyType::CmdLobbyCreateScRsp,
        CmdLobbyType::CmdLobbyKickOutCsReq,
        CmdLobbyType::CmdLobbyModifyPlayerInfoScRsp,
        CmdLobbyType::CmdLobbyGetInfoCsReq,
        CmdLobbyType::CmdLobbyGetInfoScRsp,
        CmdLobbyType::CmdLobbyModifyPlayerInfoCsReq,
        CmdLobbyType::CmdLobbyBeginScRsp,
        CmdLobbyType::CmdLobbyInviteScNotify,
        CmdLobbyType::CmdLobbyJoinCsReq,
        CmdLobbyType::CmdLobbyJoinScRsp,
        CmdLobbyType::CmdLobbyCreateCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdLobbyType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdLobbyType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdLobbyType::CmdLobbyTypeNone => 0,
            CmdLobbyType::CmdLobbyBeginCsReq => 1,
            CmdLobbyType::CmdLobbyInviteScRsp => 2,
            CmdLobbyType::CmdLobbySyncInfoScNotify => 3,
            CmdLobbyType::CmdLobbyInviteCsReq => 4,
            CmdLobbyType::CmdLobbyKickOutScRsp => 5,
            CmdLobbyType::CmdLobbyQuitCsReq => 6,
            CmdLobbyType::CmdLobbyQuitScRsp => 7,
            CmdLobbyType::CmdLobbyCreateScRsp => 8,
            CmdLobbyType::CmdLobbyKickOutCsReq => 9,
            CmdLobbyType::CmdLobbyModifyPlayerInfoScRsp => 10,
            CmdLobbyType::CmdLobbyGetInfoCsReq => 11,
            CmdLobbyType::CmdLobbyGetInfoScRsp => 12,
            CmdLobbyType::CmdLobbyModifyPlayerInfoCsReq => 13,
            CmdLobbyType::CmdLobbyBeginScRsp => 14,
            CmdLobbyType::CmdLobbyInviteScNotify => 15,
            CmdLobbyType::CmdLobbyJoinCsReq => 16,
            CmdLobbyType::CmdLobbyJoinScRsp => 17,
            CmdLobbyType::CmdLobbyCreateCsReq => 18,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdLobbyType {
    fn default() -> Self {
        CmdLobbyType::CmdLobbyTypeNone
    }
}

impl CmdLobbyType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdLobbyType>("CmdLobbyType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12CmdLobbyType.proto*\x8e\x04\n\x0cCmdLobbyType\x12\x14\n\x10CmdLobb\
    yTypeNone\x10\0\x12\x17\n\x12CmdLobbyBeginCsReq\x10\xc59\x12\x18\n\x13Cm\
    dLobbyInviteScRsp\x10\xc09\x12\x1d\n\x18CmdLobbySyncInfoScNotify\x10\xe3\
    9\x12\x18\n\x13CmdLobbyInviteCsReq\x10\xd29\x12\x19\n\x14CmdLobbyKickOut\
    ScRsp\x10\xe49\x12\x16\n\x11CmdLobbyQuitCsReq\x10\xe69\x12\x16\n\x11CmdL\
    obbyQuitScRsp\x10\xc39\x12\x18\n\x13CmdLobbyCreateScRsp\x10\xe89\x12\x19\
    \n\x14CmdLobbyKickOutCsReq\x10\xe59\x12\"\n\x1dCmdLobbyModifyPlayerInfoS\
    cRsp\x10\xe19\x12\x19\n\x14CmdLobbyGetInfoCsReq\x10\xba9\x12\x19\n\x14Cm\
    dLobbyGetInfoScRsp\x10\xde9\x12\"\n\x1dCmdLobbyModifyPlayerInfoCsReq\x10\
    \xbf9\x12\x17\n\x12CmdLobbyBeginScRsp\x10\xe79\x12\x1b\n\x16CmdLobbyInvi\
    teScNotify\x10\xc79\x12\x16\n\x11CmdLobbyJoinCsReq\x10\xd49\x12\x16\n\
    \x11CmdLobbyJoinScRsp\x10\xcd9\x12\x18\n\x13CmdLobbyCreateCsReq\x10\xc29\
    b\x06proto3\
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
            enums.push(CmdLobbyType::generated_enum_descriptor_data());
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
