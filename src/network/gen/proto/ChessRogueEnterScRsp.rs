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

//! Generated file from `ChessRogueEnterScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueEnterScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueEnterScRsp {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueEnterScRsp.POCDNJBLNAN)
    pub POCDNJBLNAN: ::protobuf::MessageField<super::DIEDGIEDKHM::DIEDGIEDKHM>,
    // @@protoc_insertion_point(field:ChessRogueEnterScRsp.ROGUE_DEBUG_MESSAGE_TYPE_INFO)
    pub ROGUE_DEBUG_MESSAGE_TYPE_INFO: ::protobuf::MessageField<super::NOMDNDHHELA::NOMDNDHHELA>,
    // @@protoc_insertion_point(field:ChessRogueEnterScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ChessRogueEnterScRsp.CPJGLAJDNAC)
    pub CPJGLAJDNAC: ::protobuf::MessageField<super::IKHALMEKJNA::IKHALMEKJNA>,
    // @@protoc_insertion_point(field:ChessRogueEnterScRsp.FFKNMAONGIB)
    pub FFKNMAONGIB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueEnterScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueEnterScRsp {
    fn default() -> &'a ChessRogueEnterScRsp {
        <ChessRogueEnterScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueEnterScRsp {
    pub fn new() -> ChessRogueEnterScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DIEDGIEDKHM::DIEDGIEDKHM>(
            "POCDNJBLNAN",
            |m: &ChessRogueEnterScRsp| { &m.POCDNJBLNAN },
            |m: &mut ChessRogueEnterScRsp| { &mut m.POCDNJBLNAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NOMDNDHHELA::NOMDNDHHELA>(
            "ROGUE_DEBUG_MESSAGE_TYPE_INFO",
            |m: &ChessRogueEnterScRsp| { &m.ROGUE_DEBUG_MESSAGE_TYPE_INFO },
            |m: &mut ChessRogueEnterScRsp| { &mut m.ROGUE_DEBUG_MESSAGE_TYPE_INFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChessRogueEnterScRsp| { &m.retcode },
            |m: &mut ChessRogueEnterScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IKHALMEKJNA::IKHALMEKJNA>(
            "CPJGLAJDNAC",
            |m: &ChessRogueEnterScRsp| { &m.CPJGLAJDNAC },
            |m: &mut ChessRogueEnterScRsp| { &mut m.CPJGLAJDNAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFKNMAONGIB",
            |m: &ChessRogueEnterScRsp| { &m.FFKNMAONGIB },
            |m: &mut ChessRogueEnterScRsp| { &mut m.FFKNMAONGIB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueEnterScRsp>(
            "ChessRogueEnterScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueEnterScRsp {
    const NAME: &'static str = "ChessRogueEnterScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.POCDNJBLNAN)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ROGUE_DEBUG_MESSAGE_TYPE_INFO)?;
                },
                16 => {
                    self.retcode = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CPJGLAJDNAC)?;
                },
                64 => {
                    self.FFKNMAONGIB = is.read_uint32()?;
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
        if let Some(v) = self.POCDNJBLNAN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.ROGUE_DEBUG_MESSAGE_TYPE_INFO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        if let Some(v) = self.CPJGLAJDNAC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.FFKNMAONGIB != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.FFKNMAONGIB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.POCDNJBLNAN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.ROGUE_DEBUG_MESSAGE_TYPE_INFO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
        }
        if let Some(v) = self.CPJGLAJDNAC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.FFKNMAONGIB != 0 {
            os.write_uint32(8, self.FFKNMAONGIB)?;
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

    fn new() -> ChessRogueEnterScRsp {
        ChessRogueEnterScRsp::new()
    }

    fn clear(&mut self) {
        self.POCDNJBLNAN.clear();
        self.ROGUE_DEBUG_MESSAGE_TYPE_INFO.clear();
        self.retcode = 0;
        self.CPJGLAJDNAC.clear();
        self.FFKNMAONGIB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueEnterScRsp {
        static instance: ChessRogueEnterScRsp = ChessRogueEnterScRsp {
            POCDNJBLNAN: ::protobuf::MessageField::none(),
            ROGUE_DEBUG_MESSAGE_TYPE_INFO: ::protobuf::MessageField::none(),
            retcode: 0,
            CPJGLAJDNAC: ::protobuf::MessageField::none(),
            FFKNMAONGIB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueEnterScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueEnterScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueEnterScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueEnterScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aChessRogueEnterScRsp.proto\x1a\x11DIEDGIEDKHM.proto\x1a\x11IKHALME\
    KJNA.proto\x1a\x11NOMDNDHHELA.proto\"\x82\x02\n\x14ChessRogueEnterScRsp\
    \x12.\n\x0bPOCDNJBLNAN\x18\x01\x20\x01(\x0b2\x0c.DIEDGIEDKHMR\x0bPOCDNJB\
    LNAN\x12N\n\x1dROGUE_DEBUG_MESSAGE_TYPE_INFO\x18\x04\x20\x01(\x0b2\x0c.N\
    OMDNDHHELAR\x19ROGUEDEBUGMESSAGETYPEINFO\x12\x18\n\x07retcode\x18\x02\
    \x20\x01(\rR\x07retcode\x12.\n\x0bCPJGLAJDNAC\x18\x0f\x20\x01(\x0b2\x0c.\
    IKHALMEKJNAR\x0bCPJGLAJDNAC\x12\x20\n\x0bFFKNMAONGIB\x18\x08\x20\x01(\rR\
    \x0bFFKNMAONGIBb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::DIEDGIEDKHM::file_descriptor().clone());
            deps.push(super::IKHALMEKJNA::file_descriptor().clone());
            deps.push(super::NOMDNDHHELA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueEnterScRsp::generated_message_descriptor_data());
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
