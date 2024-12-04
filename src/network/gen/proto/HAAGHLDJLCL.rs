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

//! Generated file from `HAAGHLDJLCL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HAAGHLDJLCL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HAAGHLDJLCL {
    // message fields
    // @@protoc_insertion_point(field:HAAGHLDJLCL.GPKMFMJLMJP)
    pub GPKMFMJLMJP: ::std::vec::Vec<super::IEDMIAHONDF::IEDMIAHONDF>,
    // @@protoc_insertion_point(field:HAAGHLDJLCL.COOINBPIEOC)
    pub COOINBPIEOC: u32,
    // @@protoc_insertion_point(field:HAAGHLDJLCL.KGGHLADEKGP)
    pub KGGHLADEKGP: ::protobuf::EnumOrUnknown<super::ADHODBDKOEJ::ADHODBDKOEJ>,
    // @@protoc_insertion_point(field:HAAGHLDJLCL.OJBAILGKLBM)
    pub OJBAILGKLBM: ::protobuf::EnumOrUnknown<super::RogueTournLevelStatus::RogueTournLevelStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:HAAGHLDJLCL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HAAGHLDJLCL {
    fn default() -> &'a HAAGHLDJLCL {
        <HAAGHLDJLCL as ::protobuf::Message>::default_instance()
    }
}

impl HAAGHLDJLCL {
    pub fn new() -> HAAGHLDJLCL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GPKMFMJLMJP",
            |m: &HAAGHLDJLCL| { &m.GPKMFMJLMJP },
            |m: &mut HAAGHLDJLCL| { &mut m.GPKMFMJLMJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COOINBPIEOC",
            |m: &HAAGHLDJLCL| { &m.COOINBPIEOC },
            |m: &mut HAAGHLDJLCL| { &mut m.COOINBPIEOC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGGHLADEKGP",
            |m: &HAAGHLDJLCL| { &m.KGGHLADEKGP },
            |m: &mut HAAGHLDJLCL| { &mut m.KGGHLADEKGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJBAILGKLBM",
            |m: &HAAGHLDJLCL| { &m.OJBAILGKLBM },
            |m: &mut HAAGHLDJLCL| { &mut m.OJBAILGKLBM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HAAGHLDJLCL>(
            "HAAGHLDJLCL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HAAGHLDJLCL {
    const NAME: &'static str = "HAAGHLDJLCL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    self.GPKMFMJLMJP.push(is.read_message()?);
                },
                24 => {
                    self.COOINBPIEOC = is.read_uint32()?;
                },
                104 => {
                    self.KGGHLADEKGP = is.read_enum_or_unknown()?;
                },
                120 => {
                    self.OJBAILGKLBM = is.read_enum_or_unknown()?;
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
        for value in &self.GPKMFMJLMJP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.COOINBPIEOC != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.COOINBPIEOC);
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::ADHODBDKOEJ::ADHODBDKOEJ::ROGUE_TOURN_SETTLE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.KGGHLADEKGP.value());
        }
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.OJBAILGKLBM.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.GPKMFMJLMJP {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.COOINBPIEOC != 0 {
            os.write_uint32(3, self.COOINBPIEOC)?;
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::ADHODBDKOEJ::ADHODBDKOEJ::ROGUE_TOURN_SETTLE_REASON_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.KGGHLADEKGP))?;
        }
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.OJBAILGKLBM))?;
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

    fn new() -> HAAGHLDJLCL {
        HAAGHLDJLCL::new()
    }

    fn clear(&mut self) {
        self.GPKMFMJLMJP.clear();
        self.COOINBPIEOC = 0;
        self.KGGHLADEKGP = ::protobuf::EnumOrUnknown::new(super::ADHODBDKOEJ::ADHODBDKOEJ::ROGUE_TOURN_SETTLE_REASON_NONE);
        self.OJBAILGKLBM = ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HAAGHLDJLCL {
        static instance: HAAGHLDJLCL = HAAGHLDJLCL {
            GPKMFMJLMJP: ::std::vec::Vec::new(),
            COOINBPIEOC: 0,
            KGGHLADEKGP: ::protobuf::EnumOrUnknown::from_i32(0),
            OJBAILGKLBM: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HAAGHLDJLCL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HAAGHLDJLCL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HAAGHLDJLCL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HAAGHLDJLCL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HAAGHLDJLCL.proto\x1a\x11ADHODBDKOEJ.proto\x1a\x11IEDMIAHONDF.prot\
    o\x1a\x1bRogueTournLevelStatus.proto\"\xc9\x01\n\x0bHAAGHLDJLCL\x12.\n\
    \x0bGPKMFMJLMJP\x18\x04\x20\x03(\x0b2\x0c.IEDMIAHONDFR\x0bGPKMFMJLMJP\
    \x12\x20\n\x0bCOOINBPIEOC\x18\x03\x20\x01(\rR\x0bCOOINBPIEOC\x12.\n\x0bK\
    GGHLADEKGP\x18\r\x20\x01(\x0e2\x0c.ADHODBDKOEJR\x0bKGGHLADEKGP\x128\n\
    \x0bOJBAILGKLBM\x18\x0f\x20\x01(\x0e2\x16.RogueTournLevelStatusR\x0bOJBA\
    ILGKLBMb\x06proto3\
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
            deps.push(super::ADHODBDKOEJ::file_descriptor().clone());
            deps.push(super::IEDMIAHONDF::file_descriptor().clone());
            deps.push(super::RogueTournLevelStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HAAGHLDJLCL::generated_message_descriptor_data());
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
