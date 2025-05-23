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

//! Generated file from `LPJOLCINFDN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:LPJOLCINFDN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LPJOLCINFDN {
    // message fields
    // @@protoc_insertion_point(field:LPJOLCINFDN.PGCDMMNNCJC)
    pub PGCDMMNNCJC: u32,
    // @@protoc_insertion_point(field:LPJOLCINFDN.PJCJNKBEIMK)
    pub PJCJNKBEIMK: u32,
    // @@protoc_insertion_point(field:LPJOLCINFDN.LJPEKEDICML)
    pub LJPEKEDICML: u32,
    // @@protoc_insertion_point(field:LPJOLCINFDN.BDBMIKDJLKO)
    pub BDBMIKDJLKO: u32,
    // @@protoc_insertion_point(field:LPJOLCINFDN.NLJIFEKDPHN)
    pub NLJIFEKDPHN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LPJOLCINFDN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LPJOLCINFDN {
    fn default() -> &'a LPJOLCINFDN {
        <LPJOLCINFDN as ::protobuf::Message>::default_instance()
    }
}

impl LPJOLCINFDN {
    pub fn new() -> LPJOLCINFDN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PGCDMMNNCJC",
            |m: &LPJOLCINFDN| { &m.PGCDMMNNCJC },
            |m: &mut LPJOLCINFDN| { &mut m.PGCDMMNNCJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJCJNKBEIMK",
            |m: &LPJOLCINFDN| { &m.PJCJNKBEIMK },
            |m: &mut LPJOLCINFDN| { &mut m.PJCJNKBEIMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LJPEKEDICML",
            |m: &LPJOLCINFDN| { &m.LJPEKEDICML },
            |m: &mut LPJOLCINFDN| { &mut m.LJPEKEDICML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BDBMIKDJLKO",
            |m: &LPJOLCINFDN| { &m.BDBMIKDJLKO },
            |m: &mut LPJOLCINFDN| { &mut m.BDBMIKDJLKO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLJIFEKDPHN",
            |m: &LPJOLCINFDN| { &m.NLJIFEKDPHN },
            |m: &mut LPJOLCINFDN| { &mut m.NLJIFEKDPHN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LPJOLCINFDN>(
            "LPJOLCINFDN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LPJOLCINFDN {
    const NAME: &'static str = "LPJOLCINFDN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.PGCDMMNNCJC = is.read_uint32()?;
                },
                64 => {
                    self.PJCJNKBEIMK = is.read_uint32()?;
                },
                8 => {
                    self.LJPEKEDICML = is.read_uint32()?;
                },
                120 => {
                    self.BDBMIKDJLKO = is.read_uint32()?;
                },
                88 => {
                    self.NLJIFEKDPHN = is.read_uint32()?;
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
        if self.PGCDMMNNCJC != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.PGCDMMNNCJC);
        }
        if self.PJCJNKBEIMK != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PJCJNKBEIMK);
        }
        if self.LJPEKEDICML != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.LJPEKEDICML);
        }
        if self.BDBMIKDJLKO != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.BDBMIKDJLKO);
        }
        if self.NLJIFEKDPHN != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.NLJIFEKDPHN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PGCDMMNNCJC != 0 {
            os.write_uint32(3, self.PGCDMMNNCJC)?;
        }
        if self.PJCJNKBEIMK != 0 {
            os.write_uint32(8, self.PJCJNKBEIMK)?;
        }
        if self.LJPEKEDICML != 0 {
            os.write_uint32(1, self.LJPEKEDICML)?;
        }
        if self.BDBMIKDJLKO != 0 {
            os.write_uint32(15, self.BDBMIKDJLKO)?;
        }
        if self.NLJIFEKDPHN != 0 {
            os.write_uint32(11, self.NLJIFEKDPHN)?;
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

    fn new() -> LPJOLCINFDN {
        LPJOLCINFDN::new()
    }

    fn clear(&mut self) {
        self.PGCDMMNNCJC = 0;
        self.PJCJNKBEIMK = 0;
        self.LJPEKEDICML = 0;
        self.BDBMIKDJLKO = 0;
        self.NLJIFEKDPHN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LPJOLCINFDN {
        static instance: LPJOLCINFDN = LPJOLCINFDN {
            PGCDMMNNCJC: 0,
            PJCJNKBEIMK: 0,
            LJPEKEDICML: 0,
            BDBMIKDJLKO: 0,
            NLJIFEKDPHN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LPJOLCINFDN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LPJOLCINFDN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LPJOLCINFDN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LPJOLCINFDN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LPJOLCINFDN.proto\"\xb7\x01\n\x0bLPJOLCINFDN\x12\x20\n\x0bPGCDMMNN\
    CJC\x18\x03\x20\x01(\rR\x0bPGCDMMNNCJC\x12\x20\n\x0bPJCJNKBEIMK\x18\x08\
    \x20\x01(\rR\x0bPJCJNKBEIMK\x12\x20\n\x0bLJPEKEDICML\x18\x01\x20\x01(\rR\
    \x0bLJPEKEDICML\x12\x20\n\x0bBDBMIKDJLKO\x18\x0f\x20\x01(\rR\x0bBDBMIKDJ\
    LKO\x12\x20\n\x0bNLJIFEKDPHN\x18\x0b\x20\x01(\rR\x0bNLJIFEKDPHNb\x06prot\
    o3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LPJOLCINFDN::generated_message_descriptor_data());
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
