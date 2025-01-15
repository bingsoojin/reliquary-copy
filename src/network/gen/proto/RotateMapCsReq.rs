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

//! Generated file from `RotateMapCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RotateMapCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RotateMapCsReq {
    // message fields
    // @@protoc_insertion_point(field:RotateMapCsReq.FPGMDIPJKDJ)
    pub FPGMDIPJKDJ: ::protobuf::MessageField<super::PJNIEDFALHN::PJNIEDFALHN>,
    // @@protoc_insertion_point(field:RotateMapCsReq.IOPPGEGDHGL)
    pub IOPPGEGDHGL: u32,
    // @@protoc_insertion_point(field:RotateMapCsReq.FEOFOLNNLDG)
    pub FEOFOLNNLDG: u32,
    // @@protoc_insertion_point(field:RotateMapCsReq.LNKKMEHBDPG)
    pub LNKKMEHBDPG: ::protobuf::MessageField<super::LDFPBJIHOPD::LDFPBJIHOPD>,
    // special fields
    // @@protoc_insertion_point(special_field:RotateMapCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RotateMapCsReq {
    fn default() -> &'a RotateMapCsReq {
        <RotateMapCsReq as ::protobuf::Message>::default_instance()
    }
}

impl RotateMapCsReq {
    pub fn new() -> RotateMapCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PJNIEDFALHN::PJNIEDFALHN>(
            "FPGMDIPJKDJ",
            |m: &RotateMapCsReq| { &m.FPGMDIPJKDJ },
            |m: &mut RotateMapCsReq| { &mut m.FPGMDIPJKDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOPPGEGDHGL",
            |m: &RotateMapCsReq| { &m.IOPPGEGDHGL },
            |m: &mut RotateMapCsReq| { &mut m.IOPPGEGDHGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FEOFOLNNLDG",
            |m: &RotateMapCsReq| { &m.FEOFOLNNLDG },
            |m: &mut RotateMapCsReq| { &mut m.FEOFOLNNLDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LDFPBJIHOPD::LDFPBJIHOPD>(
            "LNKKMEHBDPG",
            |m: &RotateMapCsReq| { &m.LNKKMEHBDPG },
            |m: &mut RotateMapCsReq| { &mut m.LNKKMEHBDPG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RotateMapCsReq>(
            "RotateMapCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RotateMapCsReq {
    const NAME: &'static str = "RotateMapCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FPGMDIPJKDJ)?;
                },
                64 => {
                    self.IOPPGEGDHGL = is.read_uint32()?;
                },
                32 => {
                    self.FEOFOLNNLDG = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LNKKMEHBDPG)?;
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
        if let Some(v) = self.FPGMDIPJKDJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.IOPPGEGDHGL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.IOPPGEGDHGL);
        }
        if self.FEOFOLNNLDG != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FEOFOLNNLDG);
        }
        if let Some(v) = self.LNKKMEHBDPG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.FPGMDIPJKDJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.IOPPGEGDHGL != 0 {
            os.write_uint32(8, self.IOPPGEGDHGL)?;
        }
        if self.FEOFOLNNLDG != 0 {
            os.write_uint32(4, self.FEOFOLNNLDG)?;
        }
        if let Some(v) = self.LNKKMEHBDPG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> RotateMapCsReq {
        RotateMapCsReq::new()
    }

    fn clear(&mut self) {
        self.FPGMDIPJKDJ.clear();
        self.IOPPGEGDHGL = 0;
        self.FEOFOLNNLDG = 0;
        self.LNKKMEHBDPG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RotateMapCsReq {
        static instance: RotateMapCsReq = RotateMapCsReq {
            FPGMDIPJKDJ: ::protobuf::MessageField::none(),
            IOPPGEGDHGL: 0,
            FEOFOLNNLDG: 0,
            LNKKMEHBDPG: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RotateMapCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RotateMapCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RotateMapCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RotateMapCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14RotateMapCsReq.proto\x1a\x11LDFPBJIHOPD.proto\x1a\x11PJNIEDFALHN.p\
    roto\"\xb4\x01\n\x0eRotateMapCsReq\x12.\n\x0bFPGMDIPJKDJ\x18\r\x20\x01(\
    \x0b2\x0c.PJNIEDFALHNR\x0bFPGMDIPJKDJ\x12\x20\n\x0bIOPPGEGDHGL\x18\x08\
    \x20\x01(\rR\x0bIOPPGEGDHGL\x12\x20\n\x0bFEOFOLNNLDG\x18\x04\x20\x01(\rR\
    \x0bFEOFOLNNLDG\x12.\n\x0bLNKKMEHBDPG\x18\x0f\x20\x01(\x0b2\x0c.LDFPBJIH\
    OPDR\x0bLNKKMEHBDPGb\x06proto3\
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
            deps.push(super::LDFPBJIHOPD::file_descriptor().clone());
            deps.push(super::PJNIEDFALHN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RotateMapCsReq::generated_message_descriptor_data());
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
