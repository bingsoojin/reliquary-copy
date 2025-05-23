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

//! Generated file from `RogueMagicScepterDressInUnitCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RogueMagicScepterDressInUnitCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueMagicScepterDressInUnitCsReq {
    // message fields
    // @@protoc_insertion_point(field:RogueMagicScepterDressInUnitCsReq.ELPINNNALBD)
    pub ELPINNNALBD: u32,
    // @@protoc_insertion_point(field:RogueMagicScepterDressInUnitCsReq.GJKJFGEEHCB)
    pub GJKJFGEEHCB: u32,
    // @@protoc_insertion_point(field:RogueMagicScepterDressInUnitCsReq.FPNODEEPFKI)
    pub FPNODEEPFKI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueMagicScepterDressInUnitCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueMagicScepterDressInUnitCsReq {
    fn default() -> &'a RogueMagicScepterDressInUnitCsReq {
        <RogueMagicScepterDressInUnitCsReq as ::protobuf::Message>::default_instance()
    }
}

impl RogueMagicScepterDressInUnitCsReq {
    pub fn new() -> RogueMagicScepterDressInUnitCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELPINNNALBD",
            |m: &RogueMagicScepterDressInUnitCsReq| { &m.ELPINNNALBD },
            |m: &mut RogueMagicScepterDressInUnitCsReq| { &mut m.ELPINNNALBD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GJKJFGEEHCB",
            |m: &RogueMagicScepterDressInUnitCsReq| { &m.GJKJFGEEHCB },
            |m: &mut RogueMagicScepterDressInUnitCsReq| { &mut m.GJKJFGEEHCB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FPNODEEPFKI",
            |m: &RogueMagicScepterDressInUnitCsReq| { &m.FPNODEEPFKI },
            |m: &mut RogueMagicScepterDressInUnitCsReq| { &mut m.FPNODEEPFKI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueMagicScepterDressInUnitCsReq>(
            "RogueMagicScepterDressInUnitCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueMagicScepterDressInUnitCsReq {
    const NAME: &'static str = "RogueMagicScepterDressInUnitCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.ELPINNNALBD = is.read_uint32()?;
                },
                120 => {
                    self.GJKJFGEEHCB = is.read_uint32()?;
                },
                48 => {
                    self.FPNODEEPFKI = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.ELPINNNALBD != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.ELPINNNALBD);
        }
        if self.GJKJFGEEHCB != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.GJKJFGEEHCB);
        }
        if self.FPNODEEPFKI != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FPNODEEPFKI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ELPINNNALBD != 0 {
            os.write_uint32(12, self.ELPINNNALBD)?;
        }
        if self.GJKJFGEEHCB != 0 {
            os.write_uint32(15, self.GJKJFGEEHCB)?;
        }
        if self.FPNODEEPFKI != 0 {
            os.write_uint32(6, self.FPNODEEPFKI)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RogueMagicScepterDressInUnitCsReq {
        RogueMagicScepterDressInUnitCsReq::new()
    }

    fn clear(&mut self) {
        self.ELPINNNALBD = 0;
        self.GJKJFGEEHCB = 0;
        self.FPNODEEPFKI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueMagicScepterDressInUnitCsReq {
        static instance: RogueMagicScepterDressInUnitCsReq = RogueMagicScepterDressInUnitCsReq {
            ELPINNNALBD: 0,
            GJKJFGEEHCB: 0,
            FPNODEEPFKI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueMagicScepterDressInUnitCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueMagicScepterDressInUnitCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueMagicScepterDressInUnitCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueMagicScepterDressInUnitCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'RogueMagicScepterDressInUnitCsReq.proto\"\x89\x01\n!RogueMagicScepter\
    DressInUnitCsReq\x12\x20\n\x0bELPINNNALBD\x18\x0c\x20\x01(\rR\x0bELPINNN\
    ALBD\x12\x20\n\x0bGJKJFGEEHCB\x18\x0f\x20\x01(\rR\x0bGJKJFGEEHCB\x12\x20\
    \n\x0bFPNODEEPFKI\x18\x06\x20\x01(\rR\x0bFPNODEEPFKIb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueMagicScepterDressInUnitCsReq::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
