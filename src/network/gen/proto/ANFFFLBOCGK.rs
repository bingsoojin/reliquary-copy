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

//! Generated file from `ANFFFLBOCGK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ANFFFLBOCGK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ANFFFLBOCGK {
    // message fields
    // @@protoc_insertion_point(field:ANFFFLBOCGK.IGACJHCKEHC)
    pub IGACJHCKEHC: u32,
    // @@protoc_insertion_point(field:ANFFFLBOCGK.DOCBJCJFDAL)
    pub DOCBJCJFDAL: bool,
    // @@protoc_insertion_point(field:ANFFFLBOCGK.KOKJNMHLNIB)
    pub KOKJNMHLNIB: ::std::vec::Vec<super::DDNLKCBDLFH::DDNLKCBDLFH>,
    // special fields
    // @@protoc_insertion_point(special_field:ANFFFLBOCGK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ANFFFLBOCGK {
    fn default() -> &'a ANFFFLBOCGK {
        <ANFFFLBOCGK as ::protobuf::Message>::default_instance()
    }
}

impl ANFFFLBOCGK {
    pub fn new() -> ANFFFLBOCGK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IGACJHCKEHC",
            |m: &ANFFFLBOCGK| { &m.IGACJHCKEHC },
            |m: &mut ANFFFLBOCGK| { &mut m.IGACJHCKEHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DOCBJCJFDAL",
            |m: &ANFFFLBOCGK| { &m.DOCBJCJFDAL },
            |m: &mut ANFFFLBOCGK| { &mut m.DOCBJCJFDAL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KOKJNMHLNIB",
            |m: &ANFFFLBOCGK| { &m.KOKJNMHLNIB },
            |m: &mut ANFFFLBOCGK| { &mut m.KOKJNMHLNIB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ANFFFLBOCGK>(
            "ANFFFLBOCGK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ANFFFLBOCGK {
    const NAME: &'static str = "ANFFFLBOCGK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.IGACJHCKEHC = is.read_uint32()?;
                },
                80 => {
                    self.DOCBJCJFDAL = is.read_bool()?;
                },
                58 => {
                    self.KOKJNMHLNIB.push(is.read_message()?);
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
        if self.IGACJHCKEHC != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.IGACJHCKEHC);
        }
        if self.DOCBJCJFDAL != false {
            my_size += 1 + 1;
        }
        for value in &self.KOKJNMHLNIB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IGACJHCKEHC != 0 {
            os.write_uint32(15, self.IGACJHCKEHC)?;
        }
        if self.DOCBJCJFDAL != false {
            os.write_bool(10, self.DOCBJCJFDAL)?;
        }
        for v in &self.KOKJNMHLNIB {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> ANFFFLBOCGK {
        ANFFFLBOCGK::new()
    }

    fn clear(&mut self) {
        self.IGACJHCKEHC = 0;
        self.DOCBJCJFDAL = false;
        self.KOKJNMHLNIB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ANFFFLBOCGK {
        static instance: ANFFFLBOCGK = ANFFFLBOCGK {
            IGACJHCKEHC: 0,
            DOCBJCJFDAL: false,
            KOKJNMHLNIB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ANFFFLBOCGK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ANFFFLBOCGK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ANFFFLBOCGK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ANFFFLBOCGK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ANFFFLBOCGK.proto\x1a\x11DDNLKCBDLFH.proto\"\x81\x01\n\x0bANFFFLBO\
    CGK\x12\x20\n\x0bIGACJHCKEHC\x18\x0f\x20\x01(\rR\x0bIGACJHCKEHC\x12\x20\
    \n\x0bDOCBJCJFDAL\x18\n\x20\x01(\x08R\x0bDOCBJCJFDAL\x12.\n\x0bKOKJNMHLN\
    IB\x18\x07\x20\x03(\x0b2\x0c.DDNLKCBDLFHR\x0bKOKJNMHLNIBb\x06proto3\
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
            deps.push(super::DDNLKCBDLFH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ANFFFLBOCGK::generated_message_descriptor_data());
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
