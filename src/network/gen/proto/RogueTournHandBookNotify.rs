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

//! Generated file from `RogueTournHandBookNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueTournHandBookNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueTournHandBookNotify {
    // message fields
    // @@protoc_insertion_point(field:RogueTournHandBookNotify.HHDCKGBMHAI)
    pub HHDCKGBMHAI: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:RogueTournHandBookNotify.PKDJDPGCMFN)
    pub PKDJDPGCMFN: ::protobuf::EnumOrUnknown<super::JDHNHBBCGPK::JDHNHBBCGPK>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueTournHandBookNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueTournHandBookNotify {
    fn default() -> &'a RogueTournHandBookNotify {
        <RogueTournHandBookNotify as ::protobuf::Message>::default_instance()
    }
}

impl RogueTournHandBookNotify {
    pub fn new() -> RogueTournHandBookNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HHDCKGBMHAI",
            |m: &RogueTournHandBookNotify| { &m.HHDCKGBMHAI },
            |m: &mut RogueTournHandBookNotify| { &mut m.HHDCKGBMHAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PKDJDPGCMFN",
            |m: &RogueTournHandBookNotify| { &m.PKDJDPGCMFN },
            |m: &mut RogueTournHandBookNotify| { &mut m.PKDJDPGCMFN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueTournHandBookNotify>(
            "RogueTournHandBookNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueTournHandBookNotify {
    const NAME: &'static str = "RogueTournHandBookNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.HHDCKGBMHAI)?;
                },
                40 => {
                    self.HHDCKGBMHAI.push(is.read_uint32()?);
                },
                64 => {
                    self.PKDJDPGCMFN = is.read_enum_or_unknown()?;
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
        for value in &self.HHDCKGBMHAI {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if self.PKDJDPGCMFN != ::protobuf::EnumOrUnknown::new(super::JDHNHBBCGPK::JDHNHBBCGPK::ROGUE_TOURN_HANDBOOK_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.PKDJDPGCMFN.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.HHDCKGBMHAI {
            os.write_uint32(5, *v)?;
        };
        if self.PKDJDPGCMFN != ::protobuf::EnumOrUnknown::new(super::JDHNHBBCGPK::JDHNHBBCGPK::ROGUE_TOURN_HANDBOOK_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.PKDJDPGCMFN))?;
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

    fn new() -> RogueTournHandBookNotify {
        RogueTournHandBookNotify::new()
    }

    fn clear(&mut self) {
        self.HHDCKGBMHAI.clear();
        self.PKDJDPGCMFN = ::protobuf::EnumOrUnknown::new(super::JDHNHBBCGPK::JDHNHBBCGPK::ROGUE_TOURN_HANDBOOK_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueTournHandBookNotify {
        static instance: RogueTournHandBookNotify = RogueTournHandBookNotify {
            HHDCKGBMHAI: ::std::vec::Vec::new(),
            PKDJDPGCMFN: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueTournHandBookNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueTournHandBookNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueTournHandBookNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueTournHandBookNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eRogueTournHandBookNotify.proto\x1a\x11JDHNHBBCGPK.proto\"l\n\x18Ro\
    gueTournHandBookNotify\x12\x20\n\x0bHHDCKGBMHAI\x18\x05\x20\x03(\rR\x0bH\
    HDCKGBMHAI\x12.\n\x0bPKDJDPGCMFN\x18\x08\x20\x01(\x0e2\x0c.JDHNHBBCGPKR\
    \x0bPKDJDPGCMFNb\x06proto3\
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
            deps.push(super::JDHNHBBCGPK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueTournHandBookNotify::generated_message_descriptor_data());
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
