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

//! Generated file from `COCAIPNLEPD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:COCAIPNLEPD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct COCAIPNLEPD {
    // message fields
    // @@protoc_insertion_point(field:COCAIPNLEPD.DMNKFBNAKPM)
    pub DMNKFBNAKPM: ::std::vec::Vec<super::AAJPODHBOKA::AAJPODHBOKA>,
    // @@protoc_insertion_point(field:COCAIPNLEPD.BOJDINBGOOJ)
    pub BOJDINBGOOJ: ::protobuf::MessageField<super::KBJFBECCGJN::KBJFBECCGJN>,
    // @@protoc_insertion_point(field:COCAIPNLEPD.MOHADDCECEP)
    pub MOHADDCECEP: u32,
    // @@protoc_insertion_point(field:COCAIPNLEPD.DIAJDGPBDFL)
    pub DIAJDGPBDFL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:COCAIPNLEPD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a COCAIPNLEPD {
    fn default() -> &'a COCAIPNLEPD {
        <COCAIPNLEPD as ::protobuf::Message>::default_instance()
    }
}

impl COCAIPNLEPD {
    pub fn new() -> COCAIPNLEPD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DMNKFBNAKPM",
            |m: &COCAIPNLEPD| { &m.DMNKFBNAKPM },
            |m: &mut COCAIPNLEPD| { &mut m.DMNKFBNAKPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KBJFBECCGJN::KBJFBECCGJN>(
            "BOJDINBGOOJ",
            |m: &COCAIPNLEPD| { &m.BOJDINBGOOJ },
            |m: &mut COCAIPNLEPD| { &mut m.BOJDINBGOOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MOHADDCECEP",
            |m: &COCAIPNLEPD| { &m.MOHADDCECEP },
            |m: &mut COCAIPNLEPD| { &mut m.MOHADDCECEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DIAJDGPBDFL",
            |m: &COCAIPNLEPD| { &m.DIAJDGPBDFL },
            |m: &mut COCAIPNLEPD| { &mut m.DIAJDGPBDFL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<COCAIPNLEPD>(
            "COCAIPNLEPD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for COCAIPNLEPD {
    const NAME: &'static str = "COCAIPNLEPD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.DMNKFBNAKPM.push(is.read_message()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BOJDINBGOOJ)?;
                },
                24 => {
                    self.MOHADDCECEP = is.read_uint32()?;
                },
                32 => {
                    self.DIAJDGPBDFL = is.read_uint32()?;
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
        for value in &self.DMNKFBNAKPM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.BOJDINBGOOJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.MOHADDCECEP != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.MOHADDCECEP);
        }
        if self.DIAJDGPBDFL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DIAJDGPBDFL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.DMNKFBNAKPM {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if let Some(v) = self.BOJDINBGOOJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.MOHADDCECEP != 0 {
            os.write_uint32(3, self.MOHADDCECEP)?;
        }
        if self.DIAJDGPBDFL != 0 {
            os.write_uint32(4, self.DIAJDGPBDFL)?;
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

    fn new() -> COCAIPNLEPD {
        COCAIPNLEPD::new()
    }

    fn clear(&mut self) {
        self.DMNKFBNAKPM.clear();
        self.BOJDINBGOOJ.clear();
        self.MOHADDCECEP = 0;
        self.DIAJDGPBDFL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static COCAIPNLEPD {
        static instance: COCAIPNLEPD = COCAIPNLEPD {
            DMNKFBNAKPM: ::std::vec::Vec::new(),
            BOJDINBGOOJ: ::protobuf::MessageField::none(),
            MOHADDCECEP: 0,
            DIAJDGPBDFL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for COCAIPNLEPD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("COCAIPNLEPD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for COCAIPNLEPD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for COCAIPNLEPD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11COCAIPNLEPD.proto\x1a\x11AAJPODHBOKA.proto\x1a\x11KBJFBECCGJN.prot\
    o\"\xb1\x01\n\x0bCOCAIPNLEPD\x12.\n\x0bDMNKFBNAKPM\x18\x01\x20\x03(\x0b2\
    \x0c.AAJPODHBOKAR\x0bDMNKFBNAKPM\x12.\n\x0bBOJDINBGOOJ\x18\x02\x20\x01(\
    \x0b2\x0c.KBJFBECCGJNR\x0bBOJDINBGOOJ\x12\x20\n\x0bMOHADDCECEP\x18\x03\
    \x20\x01(\rR\x0bMOHADDCECEP\x12\x20\n\x0bDIAJDGPBDFL\x18\x04\x20\x01(\rR\
    \x0bDIAJDGPBDFLb\x06proto3\
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
            deps.push(super::AAJPODHBOKA::file_descriptor().clone());
            deps.push(super::KBJFBECCGJN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(COCAIPNLEPD::generated_message_descriptor_data());
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
