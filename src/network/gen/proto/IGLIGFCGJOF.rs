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

//! Generated file from `IGLIGFCGJOF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IGLIGFCGJOF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IGLIGFCGJOF {
    // message fields
    // @@protoc_insertion_point(field:IGLIGFCGJOF.FNDLOLLFEPP)
    pub FNDLOLLFEPP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IGLIGFCGJOF.BKKJJEAAABN)
    pub BKKJJEAAABN: ::std::vec::Vec<super::AJENGOMMJPN::AJENGOMMJPN>,
    // @@protoc_insertion_point(field:IGLIGFCGJOF.OAHGEGIELON)
    pub OAHGEGIELON: ::protobuf::EnumOrUnknown<super::ActivitySwordTraining::SwordTrainingDailyPhaseType>,
    // special fields
    // @@protoc_insertion_point(special_field:IGLIGFCGJOF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IGLIGFCGJOF {
    fn default() -> &'a IGLIGFCGJOF {
        <IGLIGFCGJOF as ::protobuf::Message>::default_instance()
    }
}

impl IGLIGFCGJOF {
    pub fn new() -> IGLIGFCGJOF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FNDLOLLFEPP",
            |m: &IGLIGFCGJOF| { &m.FNDLOLLFEPP },
            |m: &mut IGLIGFCGJOF| { &mut m.FNDLOLLFEPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BKKJJEAAABN",
            |m: &IGLIGFCGJOF| { &m.BKKJJEAAABN },
            |m: &mut IGLIGFCGJOF| { &mut m.BKKJJEAAABN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OAHGEGIELON",
            |m: &IGLIGFCGJOF| { &m.OAHGEGIELON },
            |m: &mut IGLIGFCGJOF| { &mut m.OAHGEGIELON },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IGLIGFCGJOF>(
            "IGLIGFCGJOF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IGLIGFCGJOF {
    const NAME: &'static str = "IGLIGFCGJOF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.FNDLOLLFEPP)?;
                },
                64 => {
                    self.FNDLOLLFEPP.push(is.read_uint32()?);
                },
                26 => {
                    self.BKKJJEAAABN.push(is.read_message()?);
                },
                56 => {
                    self.OAHGEGIELON = is.read_enum_or_unknown()?;
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
        for value in &self.FNDLOLLFEPP {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        for value in &self.BKKJJEAAABN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.OAHGEGIELON != ::protobuf::EnumOrUnknown::new(super::ActivitySwordTraining::SwordTrainingDailyPhaseType::SWORD_TRAINING_DAILY_PHASE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.OAHGEGIELON.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.FNDLOLLFEPP {
            os.write_uint32(8, *v)?;
        };
        for v in &self.BKKJJEAAABN {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.OAHGEGIELON != ::protobuf::EnumOrUnknown::new(super::ActivitySwordTraining::SwordTrainingDailyPhaseType::SWORD_TRAINING_DAILY_PHASE_TYPE_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.OAHGEGIELON))?;
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

    fn new() -> IGLIGFCGJOF {
        IGLIGFCGJOF::new()
    }

    fn clear(&mut self) {
        self.FNDLOLLFEPP.clear();
        self.BKKJJEAAABN.clear();
        self.OAHGEGIELON = ::protobuf::EnumOrUnknown::new(super::ActivitySwordTraining::SwordTrainingDailyPhaseType::SWORD_TRAINING_DAILY_PHASE_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IGLIGFCGJOF {
        static instance: IGLIGFCGJOF = IGLIGFCGJOF {
            FNDLOLLFEPP: ::std::vec::Vec::new(),
            BKKJJEAAABN: ::std::vec::Vec::new(),
            OAHGEGIELON: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IGLIGFCGJOF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IGLIGFCGJOF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IGLIGFCGJOF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IGLIGFCGJOF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IGLIGFCGJOF.proto\x1a\x11AJENGOMMJPN.proto\x1a!SwordTrainingDailyP\
    haseType.proto\"\x9f\x01\n\x0bIGLIGFCGJOF\x12\x20\n\x0bFNDLOLLFEPP\x18\
    \x08\x20\x03(\rR\x0bFNDLOLLFEPP\x12.\n\x0bBKKJJEAAABN\x18\x03\x20\x03(\
    \x0b2\x0c.AJENGOMMJPNR\x0bBKKJJEAAABN\x12>\n\x0bOAHGEGIELON\x18\x07\x20\
    \x01(\x0e2\x1c.SwordTrainingDailyPhaseTypeR\x0bOAHGEGIELONb\x06proto3\
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
            deps.push(super::AJENGOMMJPN::file_descriptor().clone());
            deps.push(super::SwordTrainingDailyPhaseType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IGLIGFCGJOF::generated_message_descriptor_data());
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
