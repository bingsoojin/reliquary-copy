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

//! Generated file from `TelevisionActivityBattleEndScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TelevisionActivityBattleEndScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TelevisionActivityBattleEndScNotify {
    // message fields
    // @@protoc_insertion_point(field:TelevisionActivityBattleEndScNotify.BFJAIFDEDBE)
    pub BFJAIFDEDBE: u32,
    // @@protoc_insertion_point(field:TelevisionActivityBattleEndScNotify.FLHCOOPGDKK)
    pub FLHCOOPGDKK: u32,
    // @@protoc_insertion_point(field:TelevisionActivityBattleEndScNotify.JCCIILLCAAP)
    pub JCCIILLCAAP: ::protobuf::MessageField<super::DHFCCBNANKI::DHFCCBNANKI>,
    // @@protoc_insertion_point(field:TelevisionActivityBattleEndScNotify.AAIJABGNADA)
    pub AAIJABGNADA: u32,
    // @@protoc_insertion_point(field:TelevisionActivityBattleEndScNotify.OKCOLKLICCA)
    pub OKCOLKLICCA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TelevisionActivityBattleEndScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TelevisionActivityBattleEndScNotify {
    fn default() -> &'a TelevisionActivityBattleEndScNotify {
        <TelevisionActivityBattleEndScNotify as ::protobuf::Message>::default_instance()
    }
}

impl TelevisionActivityBattleEndScNotify {
    pub fn new() -> TelevisionActivityBattleEndScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BFJAIFDEDBE",
            |m: &TelevisionActivityBattleEndScNotify| { &m.BFJAIFDEDBE },
            |m: &mut TelevisionActivityBattleEndScNotify| { &mut m.BFJAIFDEDBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FLHCOOPGDKK",
            |m: &TelevisionActivityBattleEndScNotify| { &m.FLHCOOPGDKK },
            |m: &mut TelevisionActivityBattleEndScNotify| { &mut m.FLHCOOPGDKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DHFCCBNANKI::DHFCCBNANKI>(
            "JCCIILLCAAP",
            |m: &TelevisionActivityBattleEndScNotify| { &m.JCCIILLCAAP },
            |m: &mut TelevisionActivityBattleEndScNotify| { &mut m.JCCIILLCAAP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAIJABGNADA",
            |m: &TelevisionActivityBattleEndScNotify| { &m.AAIJABGNADA },
            |m: &mut TelevisionActivityBattleEndScNotify| { &mut m.AAIJABGNADA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OKCOLKLICCA",
            |m: &TelevisionActivityBattleEndScNotify| { &m.OKCOLKLICCA },
            |m: &mut TelevisionActivityBattleEndScNotify| { &mut m.OKCOLKLICCA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TelevisionActivityBattleEndScNotify>(
            "TelevisionActivityBattleEndScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TelevisionActivityBattleEndScNotify {
    const NAME: &'static str = "TelevisionActivityBattleEndScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.BFJAIFDEDBE = is.read_uint32()?;
                },
                8 => {
                    self.FLHCOOPGDKK = is.read_uint32()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JCCIILLCAAP)?;
                },
                104 => {
                    self.AAIJABGNADA = is.read_uint32()?;
                },
                80 => {
                    self.OKCOLKLICCA = is.read_uint32()?;
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
        if self.BFJAIFDEDBE != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.BFJAIFDEDBE);
        }
        if self.FLHCOOPGDKK != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FLHCOOPGDKK);
        }
        if let Some(v) = self.JCCIILLCAAP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.AAIJABGNADA != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.AAIJABGNADA);
        }
        if self.OKCOLKLICCA != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.OKCOLKLICCA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BFJAIFDEDBE != 0 {
            os.write_uint32(11, self.BFJAIFDEDBE)?;
        }
        if self.FLHCOOPGDKK != 0 {
            os.write_uint32(1, self.FLHCOOPGDKK)?;
        }
        if let Some(v) = self.JCCIILLCAAP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if self.AAIJABGNADA != 0 {
            os.write_uint32(13, self.AAIJABGNADA)?;
        }
        if self.OKCOLKLICCA != 0 {
            os.write_uint32(10, self.OKCOLKLICCA)?;
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

    fn new() -> TelevisionActivityBattleEndScNotify {
        TelevisionActivityBattleEndScNotify::new()
    }

    fn clear(&mut self) {
        self.BFJAIFDEDBE = 0;
        self.FLHCOOPGDKK = 0;
        self.JCCIILLCAAP.clear();
        self.AAIJABGNADA = 0;
        self.OKCOLKLICCA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TelevisionActivityBattleEndScNotify {
        static instance: TelevisionActivityBattleEndScNotify = TelevisionActivityBattleEndScNotify {
            BFJAIFDEDBE: 0,
            FLHCOOPGDKK: 0,
            JCCIILLCAAP: ::protobuf::MessageField::none(),
            AAIJABGNADA: 0,
            OKCOLKLICCA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TelevisionActivityBattleEndScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TelevisionActivityBattleEndScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TelevisionActivityBattleEndScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TelevisionActivityBattleEndScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)TelevisionActivityBattleEndScNotify.proto\x1a\x11DHFCCBNANKI.proto\"\
    \xdd\x01\n#TelevisionActivityBattleEndScNotify\x12\x20\n\x0bBFJAIFDEDBE\
    \x18\x0b\x20\x01(\rR\x0bBFJAIFDEDBE\x12\x20\n\x0bFLHCOOPGDKK\x18\x01\x20\
    \x01(\rR\x0bFLHCOOPGDKK\x12.\n\x0bJCCIILLCAAP\x18\x06\x20\x01(\x0b2\x0c.\
    DHFCCBNANKIR\x0bJCCIILLCAAP\x12\x20\n\x0bAAIJABGNADA\x18\r\x20\x01(\rR\
    \x0bAAIJABGNADA\x12\x20\n\x0bOKCOLKLICCA\x18\n\x20\x01(\rR\x0bOKCOLKLICC\
    Ab\x06proto3\
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
            deps.push(super::DHFCCBNANKI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TelevisionActivityBattleEndScNotify::generated_message_descriptor_data());
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
