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

//! Generated file from `DAPBKIAMOAE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:DAPBKIAMOAE)
pub enum DAPBKIAMOAE {
    // @@protoc_insertion_point(enum_value:DAPBKIAMOAE.MONOPOLY_CELL_STATE_IDLE)
    MONOPOLY_CELL_STATE_IDLE = 0,
    // @@protoc_insertion_point(enum_value:DAPBKIAMOAE.MONOPOLY_CELL_STATE_BARRIER)
    MONOPOLY_CELL_STATE_BARRIER = 1,
    // @@protoc_insertion_point(enum_value:DAPBKIAMOAE.MONOPOLY_CELL_STATE_GROUND)
    MONOPOLY_CELL_STATE_GROUND = 2,
    // @@protoc_insertion_point(enum_value:DAPBKIAMOAE.MONOPOLY_CELL_STATE_FINISH)
    MONOPOLY_CELL_STATE_FINISH = 3,
}

impl ::protobuf::Enum for DAPBKIAMOAE {
    const NAME: &'static str = "DAPBKIAMOAE";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DAPBKIAMOAE> {
        match value {
            0 => ::std::option::Option::Some(DAPBKIAMOAE::MONOPOLY_CELL_STATE_IDLE),
            1 => ::std::option::Option::Some(DAPBKIAMOAE::MONOPOLY_CELL_STATE_BARRIER),
            2 => ::std::option::Option::Some(DAPBKIAMOAE::MONOPOLY_CELL_STATE_GROUND),
            3 => ::std::option::Option::Some(DAPBKIAMOAE::MONOPOLY_CELL_STATE_FINISH),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<DAPBKIAMOAE> {
        match str {
            "MONOPOLY_CELL_STATE_IDLE" => ::std::option::Option::Some(DAPBKIAMOAE::MONOPOLY_CELL_STATE_IDLE),
            "MONOPOLY_CELL_STATE_BARRIER" => ::std::option::Option::Some(DAPBKIAMOAE::MONOPOLY_CELL_STATE_BARRIER),
            "MONOPOLY_CELL_STATE_GROUND" => ::std::option::Option::Some(DAPBKIAMOAE::MONOPOLY_CELL_STATE_GROUND),
            "MONOPOLY_CELL_STATE_FINISH" => ::std::option::Option::Some(DAPBKIAMOAE::MONOPOLY_CELL_STATE_FINISH),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DAPBKIAMOAE] = &[
        DAPBKIAMOAE::MONOPOLY_CELL_STATE_IDLE,
        DAPBKIAMOAE::MONOPOLY_CELL_STATE_BARRIER,
        DAPBKIAMOAE::MONOPOLY_CELL_STATE_GROUND,
        DAPBKIAMOAE::MONOPOLY_CELL_STATE_FINISH,
    ];
}

impl ::protobuf::EnumFull for DAPBKIAMOAE {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DAPBKIAMOAE").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DAPBKIAMOAE {
    fn default() -> Self {
        DAPBKIAMOAE::MONOPOLY_CELL_STATE_IDLE
    }
}

impl DAPBKIAMOAE {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DAPBKIAMOAE>("DAPBKIAMOAE")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DAPBKIAMOAE.proto*\x8c\x01\n\x0bDAPBKIAMOAE\x12\x1c\n\x18MONOPOLY_\
    CELL_STATE_IDLE\x10\0\x12\x1f\n\x1bMONOPOLY_CELL_STATE_BARRIER\x10\x01\
    \x12\x1e\n\x1aMONOPOLY_CELL_STATE_GROUND\x10\x02\x12\x1e\n\x1aMONOPOLY_C\
    ELL_STATE_FINISH\x10\x03b\x06proto3\
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
            enums.push(DAPBKIAMOAE::generated_enum_descriptor_data());
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
