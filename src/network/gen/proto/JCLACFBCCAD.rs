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

//! Generated file from `JCLACFBCCAD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JCLACFBCCAD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JCLACFBCCAD {
    // message fields
    // @@protoc_insertion_point(field:JCLACFBCCAD.IIIODOFEDHC)
    pub IIIODOFEDHC: i32,
    // @@protoc_insertion_point(field:JCLACFBCCAD.ANHKKMBJGFE)
    pub ANHKKMBJGFE: i32,
    // @@protoc_insertion_point(field:JCLACFBCCAD.DDPBECJEPEI)
    pub DDPBECJEPEI: i32,
    // special fields
    // @@protoc_insertion_point(special_field:JCLACFBCCAD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JCLACFBCCAD {
    fn default() -> &'a JCLACFBCCAD {
        <JCLACFBCCAD as ::protobuf::Message>::default_instance()
    }
}

impl JCLACFBCCAD {
    pub fn new() -> JCLACFBCCAD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIIODOFEDHC",
            |m: &JCLACFBCCAD| { &m.IIIODOFEDHC },
            |m: &mut JCLACFBCCAD| { &mut m.IIIODOFEDHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ANHKKMBJGFE",
            |m: &JCLACFBCCAD| { &m.ANHKKMBJGFE },
            |m: &mut JCLACFBCCAD| { &mut m.ANHKKMBJGFE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDPBECJEPEI",
            |m: &JCLACFBCCAD| { &m.DDPBECJEPEI },
            |m: &mut JCLACFBCCAD| { &mut m.DDPBECJEPEI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JCLACFBCCAD>(
            "JCLACFBCCAD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JCLACFBCCAD {
    const NAME: &'static str = "JCLACFBCCAD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.IIIODOFEDHC = is.read_int32()?;
                },
                72 => {
                    self.ANHKKMBJGFE = is.read_int32()?;
                },
                32 => {
                    self.DDPBECJEPEI = is.read_int32()?;
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
        if self.IIIODOFEDHC != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.IIIODOFEDHC);
        }
        if self.ANHKKMBJGFE != 0 {
            my_size += ::protobuf::rt::int32_size(9, self.ANHKKMBJGFE);
        }
        if self.DDPBECJEPEI != 0 {
            my_size += ::protobuf::rt::int32_size(4, self.DDPBECJEPEI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IIIODOFEDHC != 0 {
            os.write_int32(1, self.IIIODOFEDHC)?;
        }
        if self.ANHKKMBJGFE != 0 {
            os.write_int32(9, self.ANHKKMBJGFE)?;
        }
        if self.DDPBECJEPEI != 0 {
            os.write_int32(4, self.DDPBECJEPEI)?;
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

    fn new() -> JCLACFBCCAD {
        JCLACFBCCAD::new()
    }

    fn clear(&mut self) {
        self.IIIODOFEDHC = 0;
        self.ANHKKMBJGFE = 0;
        self.DDPBECJEPEI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JCLACFBCCAD {
        static instance: JCLACFBCCAD = JCLACFBCCAD {
            IIIODOFEDHC: 0,
            ANHKKMBJGFE: 0,
            DDPBECJEPEI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JCLACFBCCAD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JCLACFBCCAD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JCLACFBCCAD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JCLACFBCCAD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JCLACFBCCAD.proto\"s\n\x0bJCLACFBCCAD\x12\x20\n\x0bIIIODOFEDHC\x18\
    \x01\x20\x01(\x05R\x0bIIIODOFEDHC\x12\x20\n\x0bANHKKMBJGFE\x18\t\x20\x01\
    (\x05R\x0bANHKKMBJGFE\x12\x20\n\x0bDDPBECJEPEI\x18\x04\x20\x01(\x05R\x0b\
    DDPBECJEPEIb\x06proto3\
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
            messages.push(JCLACFBCCAD::generated_message_descriptor_data());
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
