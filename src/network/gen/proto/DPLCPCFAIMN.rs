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

//! Generated file from `DPLCPCFAIMN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:DPLCPCFAIMN)
pub enum DPLCPCFAIMN {
    // @@protoc_insertion_point(enum_value:DPLCPCFAIMN.OFFERING_STATE_NONE)
    OFFERING_STATE_NONE = 0,
    // @@protoc_insertion_point(enum_value:DPLCPCFAIMN.OFFERING_STATE_LOCK)
    OFFERING_STATE_LOCK = 1,
    // @@protoc_insertion_point(enum_value:DPLCPCFAIMN.OFFERING_STATE_OPEN)
    OFFERING_STATE_OPEN = 2,
}

impl ::protobuf::Enum for DPLCPCFAIMN {
    const NAME: &'static str = "DPLCPCFAIMN";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DPLCPCFAIMN> {
        match value {
            0 => ::std::option::Option::Some(DPLCPCFAIMN::OFFERING_STATE_NONE),
            1 => ::std::option::Option::Some(DPLCPCFAIMN::OFFERING_STATE_LOCK),
            2 => ::std::option::Option::Some(DPLCPCFAIMN::OFFERING_STATE_OPEN),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<DPLCPCFAIMN> {
        match str {
            "OFFERING_STATE_NONE" => ::std::option::Option::Some(DPLCPCFAIMN::OFFERING_STATE_NONE),
            "OFFERING_STATE_LOCK" => ::std::option::Option::Some(DPLCPCFAIMN::OFFERING_STATE_LOCK),
            "OFFERING_STATE_OPEN" => ::std::option::Option::Some(DPLCPCFAIMN::OFFERING_STATE_OPEN),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DPLCPCFAIMN] = &[
        DPLCPCFAIMN::OFFERING_STATE_NONE,
        DPLCPCFAIMN::OFFERING_STATE_LOCK,
        DPLCPCFAIMN::OFFERING_STATE_OPEN,
    ];
}

impl ::protobuf::EnumFull for DPLCPCFAIMN {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DPLCPCFAIMN").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DPLCPCFAIMN {
    fn default() -> Self {
        DPLCPCFAIMN::OFFERING_STATE_NONE
    }
}

impl DPLCPCFAIMN {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DPLCPCFAIMN>("DPLCPCFAIMN")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DPLCPCFAIMN.proto*X\n\x0bDPLCPCFAIMN\x12\x17\n\x13OFFERING_STATE_N\
    ONE\x10\0\x12\x17\n\x13OFFERING_STATE_LOCK\x10\x01\x12\x17\n\x13OFFERING\
    _STATE_OPEN\x10\x02b\x06proto3\
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
            enums.push(DPLCPCFAIMN::generated_enum_descriptor_data());
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
