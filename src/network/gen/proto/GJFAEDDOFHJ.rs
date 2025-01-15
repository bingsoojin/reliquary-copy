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

//! Generated file from `GJFAEDDOFHJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GJFAEDDOFHJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GJFAEDDOFHJ {
    // message fields
    // @@protoc_insertion_point(field:GJFAEDDOFHJ.AAMKJLINEOB)
    pub AAMKJLINEOB: u32,
    // @@protoc_insertion_point(field:GJFAEDDOFHJ.DEPEKPIEGJO)
    pub DEPEKPIEGJO: u32,
    // @@protoc_insertion_point(field:GJFAEDDOFHJ.JHFEDGIMCDG)
    pub JHFEDGIMCDG: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // @@protoc_insertion_point(field:GJFAEDDOFHJ.base_avatar_id)
    pub base_avatar_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GJFAEDDOFHJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GJFAEDDOFHJ {
    fn default() -> &'a GJFAEDDOFHJ {
        <GJFAEDDOFHJ as ::protobuf::Message>::default_instance()
    }
}

impl GJFAEDDOFHJ {
    pub fn new() -> GJFAEDDOFHJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAMKJLINEOB",
            |m: &GJFAEDDOFHJ| { &m.AAMKJLINEOB },
            |m: &mut GJFAEDDOFHJ| { &mut m.AAMKJLINEOB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DEPEKPIEGJO",
            |m: &GJFAEDDOFHJ| { &m.DEPEKPIEGJO },
            |m: &mut GJFAEDDOFHJ| { &mut m.DEPEKPIEGJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JHFEDGIMCDG",
            |m: &GJFAEDDOFHJ| { &m.JHFEDGIMCDG },
            |m: &mut GJFAEDDOFHJ| { &mut m.JHFEDGIMCDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &GJFAEDDOFHJ| { &m.base_avatar_id },
            |m: &mut GJFAEDDOFHJ| { &mut m.base_avatar_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GJFAEDDOFHJ>(
            "GJFAEDDOFHJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GJFAEDDOFHJ {
    const NAME: &'static str = "GJFAEDDOFHJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.AAMKJLINEOB = is.read_uint32()?;
                },
                112 => {
                    self.DEPEKPIEGJO = is.read_uint32()?;
                },
                40 => {
                    self.JHFEDGIMCDG = is.read_enum_or_unknown()?;
                },
                64 => {
                    self.base_avatar_id = is.read_uint32()?;
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
        if self.AAMKJLINEOB != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.AAMKJLINEOB);
        }
        if self.DEPEKPIEGJO != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.DEPEKPIEGJO);
        }
        if self.JHFEDGIMCDG != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(5, self.JHFEDGIMCDG.value());
        }
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.base_avatar_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.AAMKJLINEOB != 0 {
            os.write_uint32(2, self.AAMKJLINEOB)?;
        }
        if self.DEPEKPIEGJO != 0 {
            os.write_uint32(14, self.DEPEKPIEGJO)?;
        }
        if self.JHFEDGIMCDG != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.JHFEDGIMCDG))?;
        }
        if self.base_avatar_id != 0 {
            os.write_uint32(8, self.base_avatar_id)?;
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

    fn new() -> GJFAEDDOFHJ {
        GJFAEDDOFHJ::new()
    }

    fn clear(&mut self) {
        self.AAMKJLINEOB = 0;
        self.DEPEKPIEGJO = 0;
        self.JHFEDGIMCDG = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.base_avatar_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GJFAEDDOFHJ {
        static instance: GJFAEDDOFHJ = GJFAEDDOFHJ {
            AAMKJLINEOB: 0,
            DEPEKPIEGJO: 0,
            JHFEDGIMCDG: ::protobuf::EnumOrUnknown::from_i32(0),
            base_avatar_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GJFAEDDOFHJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GJFAEDDOFHJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GJFAEDDOFHJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GJFAEDDOFHJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GJFAEDDOFHJ.proto\x1a\x10AvatarType.proto\"\xa6\x01\n\x0bGJFAEDDOF\
    HJ\x12\x20\n\x0bAAMKJLINEOB\x18\x02\x20\x01(\rR\x0bAAMKJLINEOB\x12\x20\n\
    \x0bDEPEKPIEGJO\x18\x0e\x20\x01(\rR\x0bDEPEKPIEGJO\x12-\n\x0bJHFEDGIMCDG\
    \x18\x05\x20\x01(\x0e2\x0b.AvatarTypeR\x0bJHFEDGIMCDG\x12$\n\x0ebase_ava\
    tar_id\x18\x08\x20\x01(\rR\x0cbaseAvatarIdb\x06proto3\
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
            deps.push(super::AvatarType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GJFAEDDOFHJ::generated_message_descriptor_data());
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
