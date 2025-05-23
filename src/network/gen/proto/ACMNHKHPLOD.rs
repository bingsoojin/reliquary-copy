// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `ACMNHKHPLOD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ACMNHKHPLOD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ACMNHKHPLOD {
    // message fields
    // @@protoc_insertion_point(field:ACMNHKHPLOD.ELGANMDPMID)
    pub ELGANMDPMID: u32,
    // @@protoc_insertion_point(field:ACMNHKHPLOD.IHLEAMDIKKN)
    pub IHLEAMDIKKN: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // @@protoc_insertion_point(field:ACMNHKHPLOD.FBNHDEFNECI)
    pub FBNHDEFNECI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ACMNHKHPLOD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ACMNHKHPLOD {
    fn default() -> &'a ACMNHKHPLOD {
        <ACMNHKHPLOD as ::protobuf::Message>::default_instance()
    }
}

impl ACMNHKHPLOD {
    pub fn new() -> ACMNHKHPLOD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELGANMDPMID",
            |m: &ACMNHKHPLOD| { &m.ELGANMDPMID },
            |m: &mut ACMNHKHPLOD| { &mut m.ELGANMDPMID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHLEAMDIKKN",
            |m: &ACMNHKHPLOD| { &m.IHLEAMDIKKN },
            |m: &mut ACMNHKHPLOD| { &mut m.IHLEAMDIKKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FBNHDEFNECI",
            |m: &ACMNHKHPLOD| { &m.FBNHDEFNECI },
            |m: &mut ACMNHKHPLOD| { &mut m.FBNHDEFNECI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ACMNHKHPLOD>(
            "ACMNHKHPLOD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ACMNHKHPLOD {
    const NAME: &'static str = "ACMNHKHPLOD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.ELGANMDPMID = is.read_uint32()?;
                },
                104 => {
                    self.IHLEAMDIKKN = is.read_enum_or_unknown()?;
                },
                96 => {
                    self.FBNHDEFNECI = is.read_uint32()?;
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
        if self.ELGANMDPMID != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.ELGANMDPMID);
        }
        if self.IHLEAMDIKKN != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.IHLEAMDIKKN.value());
        }
        if self.FBNHDEFNECI != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.FBNHDEFNECI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ELGANMDPMID != 0 {
            os.write_uint32(3, self.ELGANMDPMID)?;
        }
        if self.IHLEAMDIKKN != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.IHLEAMDIKKN))?;
        }
        if self.FBNHDEFNECI != 0 {
            os.write_uint32(12, self.FBNHDEFNECI)?;
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

    fn new() -> ACMNHKHPLOD {
        ACMNHKHPLOD::new()
    }

    fn clear(&mut self) {
        self.ELGANMDPMID = 0;
        self.IHLEAMDIKKN = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.FBNHDEFNECI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ACMNHKHPLOD {
        static instance: ACMNHKHPLOD = ACMNHKHPLOD {
            ELGANMDPMID: 0,
            IHLEAMDIKKN: ::protobuf::EnumOrUnknown::from_i32(0),
            FBNHDEFNECI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ACMNHKHPLOD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ACMNHKHPLOD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ACMNHKHPLOD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ACMNHKHPLOD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ACMNHKHPLOD.proto\x1a\x10AvatarType.proto\"\x80\x01\n\x0bACMNHKHPL\
    OD\x12\x20\n\x0bELGANMDPMID\x18\x03\x20\x01(\rR\x0bELGANMDPMID\x12-\n\
    \x0bIHLEAMDIKKN\x18\r\x20\x01(\x0e2\x0b.AvatarTypeR\x0bIHLEAMDIKKN\x12\
    \x20\n\x0bFBNHDEFNECI\x18\x0c\x20\x01(\rR\x0bFBNHDEFNECIb\x06proto3\
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
            messages.push(ACMNHKHPLOD::generated_message_descriptor_data());
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
