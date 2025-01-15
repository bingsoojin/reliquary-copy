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

//! Generated file from `CHMBNFJJIJN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CHMBNFJJIJN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CHMBNFJJIJN {
    // message fields
    // @@protoc_insertion_point(field:CHMBNFJJIJN.NMJMMDADKLF)
    pub NMJMMDADKLF: ::protobuf::MessageField<super::NDHNICCLBDN::NDHNICCLBDN>,
    // @@protoc_insertion_point(field:CHMBNFJJIJN.IIBAHAEAEEB)
    pub IIBAHAEAEEB: ::protobuf::MessageField<super::PMBHFPCDGBM::PMBHFPCDGBM>,
    // @@protoc_insertion_point(field:CHMBNFJJIJN.CFNJJEJIGOK)
    pub CFNJJEJIGOK: u32,
    // @@protoc_insertion_point(field:CHMBNFJJIJN.NBLJPGFHDFI)
    pub NBLJPGFHDFI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:CHMBNFJJIJN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CHMBNFJJIJN {
    fn default() -> &'a CHMBNFJJIJN {
        <CHMBNFJJIJN as ::protobuf::Message>::default_instance()
    }
}

impl CHMBNFJJIJN {
    pub fn new() -> CHMBNFJJIJN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NDHNICCLBDN::NDHNICCLBDN>(
            "NMJMMDADKLF",
            |m: &CHMBNFJJIJN| { &m.NMJMMDADKLF },
            |m: &mut CHMBNFJJIJN| { &mut m.NMJMMDADKLF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PMBHFPCDGBM::PMBHFPCDGBM>(
            "IIBAHAEAEEB",
            |m: &CHMBNFJJIJN| { &m.IIBAHAEAEEB },
            |m: &mut CHMBNFJJIJN| { &mut m.IIBAHAEAEEB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFNJJEJIGOK",
            |m: &CHMBNFJJIJN| { &m.CFNJJEJIGOK },
            |m: &mut CHMBNFJJIJN| { &mut m.CFNJJEJIGOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NBLJPGFHDFI",
            |m: &CHMBNFJJIJN| { &m.NBLJPGFHDFI },
            |m: &mut CHMBNFJJIJN| { &mut m.NBLJPGFHDFI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CHMBNFJJIJN>(
            "CHMBNFJJIJN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CHMBNFJJIJN {
    const NAME: &'static str = "CHMBNFJJIJN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NMJMMDADKLF)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IIBAHAEAEEB)?;
                },
                72 => {
                    self.CFNJJEJIGOK = is.read_uint32()?;
                },
                120 => {
                    self.NBLJPGFHDFI = is.read_uint32()?;
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
        if let Some(v) = self.NMJMMDADKLF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.IIBAHAEAEEB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.CFNJJEJIGOK != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.CFNJJEJIGOK);
        }
        if self.NBLJPGFHDFI != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.NBLJPGFHDFI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.NMJMMDADKLF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.IIBAHAEAEEB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if self.CFNJJEJIGOK != 0 {
            os.write_uint32(9, self.CFNJJEJIGOK)?;
        }
        if self.NBLJPGFHDFI != 0 {
            os.write_uint32(15, self.NBLJPGFHDFI)?;
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

    fn new() -> CHMBNFJJIJN {
        CHMBNFJJIJN::new()
    }

    fn clear(&mut self) {
        self.NMJMMDADKLF.clear();
        self.IIBAHAEAEEB.clear();
        self.CFNJJEJIGOK = 0;
        self.NBLJPGFHDFI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CHMBNFJJIJN {
        static instance: CHMBNFJJIJN = CHMBNFJJIJN {
            NMJMMDADKLF: ::protobuf::MessageField::none(),
            IIBAHAEAEEB: ::protobuf::MessageField::none(),
            CFNJJEJIGOK: 0,
            NBLJPGFHDFI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CHMBNFJJIJN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CHMBNFJJIJN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CHMBNFJJIJN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CHMBNFJJIJN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CHMBNFJJIJN.proto\x1a\x11NDHNICCLBDN.proto\x1a\x11PMBHFPCDGBM.prot\
    o\"\xb1\x01\n\x0bCHMBNFJJIJN\x12.\n\x0bNMJMMDADKLF\x18\x01\x20\x01(\x0b2\
    \x0c.NDHNICCLBDNR\x0bNMJMMDADKLF\x12.\n\x0bIIBAHAEAEEB\x18\x06\x20\x01(\
    \x0b2\x0c.PMBHFPCDGBMR\x0bIIBAHAEAEEB\x12\x20\n\x0bCFNJJEJIGOK\x18\t\x20\
    \x01(\rR\x0bCFNJJEJIGOK\x12\x20\n\x0bNBLJPGFHDFI\x18\x0f\x20\x01(\rR\x0b\
    NBLJPGFHDFIb\x06proto3\
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
            deps.push(super::NDHNICCLBDN::file_descriptor().clone());
            deps.push(super::PMBHFPCDGBM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CHMBNFJJIJN::generated_message_descriptor_data());
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
