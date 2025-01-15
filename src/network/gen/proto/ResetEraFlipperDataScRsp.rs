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

//! Generated file from `ResetEraFlipperDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ResetEraFlipperDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ResetEraFlipperDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:ResetEraFlipperDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ResetEraFlipperDataScRsp.CEFPLAOBMNB)
    pub CEFPLAOBMNB: ::protobuf::MessageField<super::BOOEDABNMMB::BOOEDABNMMB>,
    // @@protoc_insertion_point(field:ResetEraFlipperDataScRsp.OPILECFLAPF)
    pub OPILECFLAPF: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ResetEraFlipperDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ResetEraFlipperDataScRsp {
    fn default() -> &'a ResetEraFlipperDataScRsp {
        <ResetEraFlipperDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ResetEraFlipperDataScRsp {
    pub fn new() -> ResetEraFlipperDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ResetEraFlipperDataScRsp| { &m.retcode },
            |m: &mut ResetEraFlipperDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BOOEDABNMMB::BOOEDABNMMB>(
            "CEFPLAOBMNB",
            |m: &ResetEraFlipperDataScRsp| { &m.CEFPLAOBMNB },
            |m: &mut ResetEraFlipperDataScRsp| { &mut m.CEFPLAOBMNB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OPILECFLAPF",
            |m: &ResetEraFlipperDataScRsp| { &m.OPILECFLAPF },
            |m: &mut ResetEraFlipperDataScRsp| { &mut m.OPILECFLAPF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ResetEraFlipperDataScRsp>(
            "ResetEraFlipperDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ResetEraFlipperDataScRsp {
    const NAME: &'static str = "ResetEraFlipperDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.retcode = is.read_uint32()?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CEFPLAOBMNB)?;
                },
                64 => {
                    self.OPILECFLAPF = is.read_bool()?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.retcode);
        }
        if let Some(v) = self.CEFPLAOBMNB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.OPILECFLAPF != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(5, self.retcode)?;
        }
        if let Some(v) = self.CEFPLAOBMNB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.OPILECFLAPF != false {
            os.write_bool(8, self.OPILECFLAPF)?;
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

    fn new() -> ResetEraFlipperDataScRsp {
        ResetEraFlipperDataScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.CEFPLAOBMNB.clear();
        self.OPILECFLAPF = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ResetEraFlipperDataScRsp {
        static instance: ResetEraFlipperDataScRsp = ResetEraFlipperDataScRsp {
            retcode: 0,
            CEFPLAOBMNB: ::protobuf::MessageField::none(),
            OPILECFLAPF: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ResetEraFlipperDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ResetEraFlipperDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ResetEraFlipperDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResetEraFlipperDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eResetEraFlipperDataScRsp.proto\x1a\x11BOOEDABNMMB.proto\"\x86\x01\
    \n\x18ResetEraFlipperDataScRsp\x12\x18\n\x07retcode\x18\x05\x20\x01(\rR\
    \x07retcode\x12.\n\x0bCEFPLAOBMNB\x18\x0c\x20\x01(\x0b2\x0c.BOOEDABNMMBR\
    \x0bCEFPLAOBMNB\x12\x20\n\x0bOPILECFLAPF\x18\x08\x20\x01(\x08R\x0bOPILEC\
    FLAPFb\x06proto3\
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
            deps.push(super::BOOEDABNMMB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ResetEraFlipperDataScRsp::generated_message_descriptor_data());
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
