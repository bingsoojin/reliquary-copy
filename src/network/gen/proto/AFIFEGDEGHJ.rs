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

//! Generated file from `AFIFEGDEGHJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AFIFEGDEGHJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AFIFEGDEGHJ {
    // message fields
    // @@protoc_insertion_point(field:AFIFEGDEGHJ.PKFEOCAGECF)
    pub PKFEOCAGECF: ::std::vec::Vec<super::GPGGAINLLIN::GPGGAINLLIN>,
    // @@protoc_insertion_point(field:AFIFEGDEGHJ.HIODKMAPOAE)
    pub HIODKMAPOAE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AFIFEGDEGHJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AFIFEGDEGHJ {
    fn default() -> &'a AFIFEGDEGHJ {
        <AFIFEGDEGHJ as ::protobuf::Message>::default_instance()
    }
}

impl AFIFEGDEGHJ {
    pub fn new() -> AFIFEGDEGHJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PKFEOCAGECF",
            |m: &AFIFEGDEGHJ| { &m.PKFEOCAGECF },
            |m: &mut AFIFEGDEGHJ| { &mut m.PKFEOCAGECF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HIODKMAPOAE",
            |m: &AFIFEGDEGHJ| { &m.HIODKMAPOAE },
            |m: &mut AFIFEGDEGHJ| { &mut m.HIODKMAPOAE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AFIFEGDEGHJ>(
            "AFIFEGDEGHJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AFIFEGDEGHJ {
    const NAME: &'static str = "AFIFEGDEGHJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    self.PKFEOCAGECF.push(is.read_message()?);
                },
                40 => {
                    self.HIODKMAPOAE = is.read_uint32()?;
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
        for value in &self.PKFEOCAGECF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.HIODKMAPOAE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.HIODKMAPOAE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PKFEOCAGECF {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.HIODKMAPOAE != 0 {
            os.write_uint32(5, self.HIODKMAPOAE)?;
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

    fn new() -> AFIFEGDEGHJ {
        AFIFEGDEGHJ::new()
    }

    fn clear(&mut self) {
        self.PKFEOCAGECF.clear();
        self.HIODKMAPOAE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AFIFEGDEGHJ {
        static instance: AFIFEGDEGHJ = AFIFEGDEGHJ {
            PKFEOCAGECF: ::std::vec::Vec::new(),
            HIODKMAPOAE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AFIFEGDEGHJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AFIFEGDEGHJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AFIFEGDEGHJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AFIFEGDEGHJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AFIFEGDEGHJ.proto\x1a\x11GPGGAINLLIN.proto\"_\n\x0bAFIFEGDEGHJ\x12\
    .\n\x0bPKFEOCAGECF\x18\x06\x20\x03(\x0b2\x0c.GPGGAINLLINR\x0bPKFEOCAGECF\
    \x12\x20\n\x0bHIODKMAPOAE\x18\x05\x20\x01(\rR\x0bHIODKMAPOAEb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::GPGGAINLLIN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AFIFEGDEGHJ::generated_message_descriptor_data());
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
