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

//! Generated file from `EnhanceChessRogueBuffScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnhanceChessRogueBuffScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnhanceChessRogueBuffScRsp {
    // message fields
    // @@protoc_insertion_point(field:EnhanceChessRogueBuffScRsp.PHLELGAJABE)
    pub PHLELGAJABE: bool,
    // @@protoc_insertion_point(field:EnhanceChessRogueBuffScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:EnhanceChessRogueBuffScRsp.DIMIOKPLFMC)
    pub DIMIOKPLFMC: ::protobuf::MessageField<super::AAEMICFOLKA::AAEMICFOLKA>,
    // @@protoc_insertion_point(field:EnhanceChessRogueBuffScRsp.KPAAOBODFPF)
    pub KPAAOBODFPF: ::protobuf::MessageField<super::PAPPEKLEDIK::PAPPEKLEDIK>,
    // special fields
    // @@protoc_insertion_point(special_field:EnhanceChessRogueBuffScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnhanceChessRogueBuffScRsp {
    fn default() -> &'a EnhanceChessRogueBuffScRsp {
        <EnhanceChessRogueBuffScRsp as ::protobuf::Message>::default_instance()
    }
}

impl EnhanceChessRogueBuffScRsp {
    pub fn new() -> EnhanceChessRogueBuffScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PHLELGAJABE",
            |m: &EnhanceChessRogueBuffScRsp| { &m.PHLELGAJABE },
            |m: &mut EnhanceChessRogueBuffScRsp| { &mut m.PHLELGAJABE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &EnhanceChessRogueBuffScRsp| { &m.ADADHIHDHJC },
            |m: &mut EnhanceChessRogueBuffScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AAEMICFOLKA::AAEMICFOLKA>(
            "DIMIOKPLFMC",
            |m: &EnhanceChessRogueBuffScRsp| { &m.DIMIOKPLFMC },
            |m: &mut EnhanceChessRogueBuffScRsp| { &mut m.DIMIOKPLFMC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PAPPEKLEDIK::PAPPEKLEDIK>(
            "KPAAOBODFPF",
            |m: &EnhanceChessRogueBuffScRsp| { &m.KPAAOBODFPF },
            |m: &mut EnhanceChessRogueBuffScRsp| { &mut m.KPAAOBODFPF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnhanceChessRogueBuffScRsp>(
            "EnhanceChessRogueBuffScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnhanceChessRogueBuffScRsp {
    const NAME: &'static str = "EnhanceChessRogueBuffScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.PHLELGAJABE = is.read_bool()?;
                },
                104 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DIMIOKPLFMC)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KPAAOBODFPF)?;
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
        if self.PHLELGAJABE != false {
            my_size += 1 + 1;
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.ADADHIHDHJC);
        }
        if let Some(v) = self.DIMIOKPLFMC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KPAAOBODFPF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PHLELGAJABE != false {
            os.write_bool(3, self.PHLELGAJABE)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(13, self.ADADHIHDHJC)?;
        }
        if let Some(v) = self.DIMIOKPLFMC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.KPAAOBODFPF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> EnhanceChessRogueBuffScRsp {
        EnhanceChessRogueBuffScRsp::new()
    }

    fn clear(&mut self) {
        self.PHLELGAJABE = false;
        self.ADADHIHDHJC = 0;
        self.DIMIOKPLFMC.clear();
        self.KPAAOBODFPF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnhanceChessRogueBuffScRsp {
        static instance: EnhanceChessRogueBuffScRsp = EnhanceChessRogueBuffScRsp {
            PHLELGAJABE: false,
            ADADHIHDHJC: 0,
            DIMIOKPLFMC: ::protobuf::MessageField::none(),
            KPAAOBODFPF: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnhanceChessRogueBuffScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnhanceChessRogueBuffScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnhanceChessRogueBuffScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnhanceChessRogueBuffScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20EnhanceChessRogueBuffScRsp.proto\x1a\x11AAEMICFOLKA.proto\x1a\x11P\
    APPEKLEDIK.proto\"\xc0\x01\n\x1aEnhanceChessRogueBuffScRsp\x12\x20\n\x0b\
    PHLELGAJABE\x18\x03\x20\x01(\x08R\x0bPHLELGAJABE\x12\x20\n\x0bADADHIHDHJ\
    C\x18\r\x20\x01(\rR\x0bADADHIHDHJC\x12.\n\x0bDIMIOKPLFMC\x18\t\x20\x01(\
    \x0b2\x0c.AAEMICFOLKAR\x0bDIMIOKPLFMC\x12.\n\x0bKPAAOBODFPF\x18\x04\x20\
    \x01(\x0b2\x0c.PAPPEKLEDIKR\x0bKPAAOBODFPFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::AAEMICFOLKA::file_descriptor().clone());
            deps.push(super::PAPPEKLEDIK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnhanceChessRogueBuffScRsp::generated_message_descriptor_data());
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
