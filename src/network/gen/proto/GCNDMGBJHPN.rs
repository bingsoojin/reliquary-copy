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

//! Generated file from `GCNDMGBJHPN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GCNDMGBJHPN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GCNDMGBJHPN {
    // message fields
    // @@protoc_insertion_point(field:GCNDMGBJHPN.HINOBHHIBAA)
    pub HINOBHHIBAA: bool,
    // @@protoc_insertion_point(field:GCNDMGBJHPN.HAEIHIGJJMH)
    pub HAEIHIGJJMH: u32,
    // @@protoc_insertion_point(field:GCNDMGBJHPN.OHDLMBEGCIB)
    pub OHDLMBEGCIB: bool,
    // @@protoc_insertion_point(field:GCNDMGBJHPN.LMHDMEDOOCP)
    pub LMHDMEDOOCP: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:GCNDMGBJHPN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GCNDMGBJHPN {
    fn default() -> &'a GCNDMGBJHPN {
        <GCNDMGBJHPN as ::protobuf::Message>::default_instance()
    }
}

impl GCNDMGBJHPN {
    pub fn new() -> GCNDMGBJHPN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HINOBHHIBAA",
            |m: &GCNDMGBJHPN| { &m.HINOBHHIBAA },
            |m: &mut GCNDMGBJHPN| { &mut m.HINOBHHIBAA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HAEIHIGJJMH",
            |m: &GCNDMGBJHPN| { &m.HAEIHIGJJMH },
            |m: &mut GCNDMGBJHPN| { &mut m.HAEIHIGJJMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OHDLMBEGCIB",
            |m: &GCNDMGBJHPN| { &m.OHDLMBEGCIB },
            |m: &mut GCNDMGBJHPN| { &mut m.OHDLMBEGCIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LMHDMEDOOCP",
            |m: &GCNDMGBJHPN| { &m.LMHDMEDOOCP },
            |m: &mut GCNDMGBJHPN| { &mut m.LMHDMEDOOCP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GCNDMGBJHPN>(
            "GCNDMGBJHPN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GCNDMGBJHPN {
    const NAME: &'static str = "GCNDMGBJHPN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.HINOBHHIBAA = is.read_bool()?;
                },
                24 => {
                    self.HAEIHIGJJMH = is.read_uint32()?;
                },
                64 => {
                    self.OHDLMBEGCIB = is.read_bool()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.LMHDMEDOOCP)?;
                },
                104 => {
                    self.LMHDMEDOOCP.push(is.read_uint32()?);
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
        if self.HINOBHHIBAA != false {
            my_size += 1 + 1;
        }
        if self.HAEIHIGJJMH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.HAEIHIGJJMH);
        }
        if self.OHDLMBEGCIB != false {
            my_size += 1 + 1;
        }
        for value in &self.LMHDMEDOOCP {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HINOBHHIBAA != false {
            os.write_bool(15, self.HINOBHHIBAA)?;
        }
        if self.HAEIHIGJJMH != 0 {
            os.write_uint32(3, self.HAEIHIGJJMH)?;
        }
        if self.OHDLMBEGCIB != false {
            os.write_bool(8, self.OHDLMBEGCIB)?;
        }
        for v in &self.LMHDMEDOOCP {
            os.write_uint32(13, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GCNDMGBJHPN {
        GCNDMGBJHPN::new()
    }

    fn clear(&mut self) {
        self.HINOBHHIBAA = false;
        self.HAEIHIGJJMH = 0;
        self.OHDLMBEGCIB = false;
        self.LMHDMEDOOCP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GCNDMGBJHPN {
        static instance: GCNDMGBJHPN = GCNDMGBJHPN {
            HINOBHHIBAA: false,
            HAEIHIGJJMH: 0,
            OHDLMBEGCIB: false,
            LMHDMEDOOCP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GCNDMGBJHPN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GCNDMGBJHPN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GCNDMGBJHPN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GCNDMGBJHPN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GCNDMGBJHPN.proto\"\x95\x01\n\x0bGCNDMGBJHPN\x12\x20\n\x0bHINOBHHI\
    BAA\x18\x0f\x20\x01(\x08R\x0bHINOBHHIBAA\x12\x20\n\x0bHAEIHIGJJMH\x18\
    \x03\x20\x01(\rR\x0bHAEIHIGJJMH\x12\x20\n\x0bOHDLMBEGCIB\x18\x08\x20\x01\
    (\x08R\x0bOHDLMBEGCIB\x12\x20\n\x0bLMHDMEDOOCP\x18\r\x20\x03(\rR\x0bLMHD\
    MEDOOCPb\x06proto3\
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
            messages.push(GCNDMGBJHPN::generated_message_descriptor_data());
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
