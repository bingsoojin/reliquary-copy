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

//! Generated file from `HGFILPKFGNG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:HGFILPKFGNG)
pub enum HGFILPKFGNG {
    // @@protoc_insertion_point(enum_value:HGFILPKFGNG.BATTLE_STATICTIC_EVENT_NONE)
    BATTLE_STATICTIC_EVENT_NONE = 0,
    // @@protoc_insertion_point(enum_value:HGFILPKFGNG.BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_ADD_EXPLORE)
    BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_ADD_EXPLORE = 1,
    // @@protoc_insertion_point(enum_value:HGFILPKFGNG.BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_OPEN_GRID)
    BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_OPEN_GRID = 2,
    // @@protoc_insertion_point(enum_value:HGFILPKFGNG.BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_PICKUP_ITEM)
    BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_PICKUP_ITEM = 3,
    // @@protoc_insertion_point(enum_value:HGFILPKFGNG.BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_USE_BUFF)
    BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_USE_BUFF = 4,
    // @@protoc_insertion_point(enum_value:HGFILPKFGNG.BATTLE_STATICTIC_EVENT_TELEVISION_ACTIVITY_UPDATE_MAZE_BUFF_LAYER)
    BATTLE_STATICTIC_EVENT_TELEVISION_ACTIVITY_UPDATE_MAZE_BUFF_LAYER = 5,
}

impl ::protobuf::Enum for HGFILPKFGNG {
    const NAME: &'static str = "HGFILPKFGNG";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HGFILPKFGNG> {
        match value {
            0 => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_NONE),
            1 => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_ADD_EXPLORE),
            2 => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_OPEN_GRID),
            3 => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_PICKUP_ITEM),
            4 => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_USE_BUFF),
            5 => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TELEVISION_ACTIVITY_UPDATE_MAZE_BUFF_LAYER),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<HGFILPKFGNG> {
        match str {
            "BATTLE_STATICTIC_EVENT_NONE" => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_NONE),
            "BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_ADD_EXPLORE" => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_ADD_EXPLORE),
            "BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_OPEN_GRID" => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_OPEN_GRID),
            "BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_PICKUP_ITEM" => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_PICKUP_ITEM),
            "BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_USE_BUFF" => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_USE_BUFF),
            "BATTLE_STATICTIC_EVENT_TELEVISION_ACTIVITY_UPDATE_MAZE_BUFF_LAYER" => ::std::option::Option::Some(HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TELEVISION_ACTIVITY_UPDATE_MAZE_BUFF_LAYER),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [HGFILPKFGNG] = &[
        HGFILPKFGNG::BATTLE_STATICTIC_EVENT_NONE,
        HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_ADD_EXPLORE,
        HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_OPEN_GRID,
        HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_PICKUP_ITEM,
        HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_USE_BUFF,
        HGFILPKFGNG::BATTLE_STATICTIC_EVENT_TELEVISION_ACTIVITY_UPDATE_MAZE_BUFF_LAYER,
    ];
}

impl ::protobuf::EnumFull for HGFILPKFGNG {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("HGFILPKFGNG").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for HGFILPKFGNG {
    fn default() -> Self {
        HGFILPKFGNG::BATTLE_STATICTIC_EVENT_NONE
    }
}

impl HGFILPKFGNG {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<HGFILPKFGNG>("HGFILPKFGNG")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HGFILPKFGNG.proto*\xd4\x02\n\x0bHGFILPKFGNG\x12\x1f\n\x1bBATTLE_ST\
    ATICTIC_EVENT_NONE\x10\0\x127\n3BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_\
    ADD_EXPLORE\x10\x01\x125\n1BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_OPEN_\
    GRID\x10\x02\x127\n3BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_PICKUP_ITEM\
    \x10\x03\x124\n0BATTLE_STATICTIC_EVENT_TREASURE_DUNGEON_USE_BUFF\x10\x04\
    \x12E\nABATTLE_STATICTIC_EVENT_TELEVISION_ACTIVITY_UPDATE_MAZE_BUFF_LAYE\
    R\x10\x05b\x06proto3\
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
            enums.push(HGFILPKFGNG::generated_enum_descriptor_data());
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
