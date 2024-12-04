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

//! Generated file from `PCHLANPIMHG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:PCHLANPIMHG)
pub enum PCHLANPIMHG {
    // @@protoc_insertion_point(enum_value:PCHLANPIMHG.SCENE_CAST_SKILL_NONE)
    SCENE_CAST_SKILL_NONE = 0,
    // @@protoc_insertion_point(enum_value:PCHLANPIMHG.SCENE_CAST_SKILL_PROJECTILE_HIT)
    SCENE_CAST_SKILL_PROJECTILE_HIT = 1,
    // @@protoc_insertion_point(enum_value:PCHLANPIMHG.SCENE_CAST_SKILL_PROJECTILE_LIFETIME_FINISH)
    SCENE_CAST_SKILL_PROJECTILE_LIFETIME_FINISH = 2,
}

impl ::protobuf::Enum for PCHLANPIMHG {
    const NAME: &'static str = "PCHLANPIMHG";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PCHLANPIMHG> {
        match value {
            0 => ::std::option::Option::Some(PCHLANPIMHG::SCENE_CAST_SKILL_NONE),
            1 => ::std::option::Option::Some(PCHLANPIMHG::SCENE_CAST_SKILL_PROJECTILE_HIT),
            2 => ::std::option::Option::Some(PCHLANPIMHG::SCENE_CAST_SKILL_PROJECTILE_LIFETIME_FINISH),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<PCHLANPIMHG> {
        match str {
            "SCENE_CAST_SKILL_NONE" => ::std::option::Option::Some(PCHLANPIMHG::SCENE_CAST_SKILL_NONE),
            "SCENE_CAST_SKILL_PROJECTILE_HIT" => ::std::option::Option::Some(PCHLANPIMHG::SCENE_CAST_SKILL_PROJECTILE_HIT),
            "SCENE_CAST_SKILL_PROJECTILE_LIFETIME_FINISH" => ::std::option::Option::Some(PCHLANPIMHG::SCENE_CAST_SKILL_PROJECTILE_LIFETIME_FINISH),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [PCHLANPIMHG] = &[
        PCHLANPIMHG::SCENE_CAST_SKILL_NONE,
        PCHLANPIMHG::SCENE_CAST_SKILL_PROJECTILE_HIT,
        PCHLANPIMHG::SCENE_CAST_SKILL_PROJECTILE_LIFETIME_FINISH,
    ];
}

impl ::protobuf::EnumFull for PCHLANPIMHG {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("PCHLANPIMHG").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for PCHLANPIMHG {
    fn default() -> Self {
        PCHLANPIMHG::SCENE_CAST_SKILL_NONE
    }
}

impl PCHLANPIMHG {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PCHLANPIMHG>("PCHLANPIMHG")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PCHLANPIMHG.proto*~\n\x0bPCHLANPIMHG\x12\x19\n\x15SCENE_CAST_SKILL\
    _NONE\x10\0\x12#\n\x1fSCENE_CAST_SKILL_PROJECTILE_HIT\x10\x01\x12/\n+SCE\
    NE_CAST_SKILL_PROJECTILE_LIFETIME_FINISH\x10\x02b\x06proto3\
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
            enums.push(PCHLANPIMHG::generated_enum_descriptor_data());
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
