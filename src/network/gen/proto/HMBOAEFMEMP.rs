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

//! Generated file from `HMBOAEFMEMP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HMBOAEFMEMP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HMBOAEFMEMP {
    // message fields
    // @@protoc_insertion_point(field:HMBOAEFMEMP.OFAGGKBMPJN)
    pub OFAGGKBMPJN: u32,
    // @@protoc_insertion_point(field:HMBOAEFMEMP.FOEHLABGICC)
    pub FOEHLABGICC: u32,
    // @@protoc_insertion_point(field:HMBOAEFMEMP.BIODAPMJNGM)
    pub BIODAPMJNGM: u32,
    // @@protoc_insertion_point(field:HMBOAEFMEMP.GBKIEEKEJKD)
    pub GBKIEEKEJKD: u32,
    // @@protoc_insertion_point(field:HMBOAEFMEMP.IDGGIBBPPLK)
    pub IDGGIBBPPLK: u32,
    // @@protoc_insertion_point(field:HMBOAEFMEMP.KBNIFCHMAOB)
    pub KBNIFCHMAOB: u32,
    // @@protoc_insertion_point(field:HMBOAEFMEMP.LHFPBNAIABI)
    pub LHFPBNAIABI: bool,
    // @@protoc_insertion_point(field:HMBOAEFMEMP.APKEFKGPHIE)
    pub APKEFKGPHIE: ::protobuf::EnumOrUnknown<super::EOFOHACMKEP::EOFOHACMKEP>,
    // special fields
    // @@protoc_insertion_point(special_field:HMBOAEFMEMP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HMBOAEFMEMP {
    fn default() -> &'a HMBOAEFMEMP {
        <HMBOAEFMEMP as ::protobuf::Message>::default_instance()
    }
}

impl HMBOAEFMEMP {
    pub fn new() -> HMBOAEFMEMP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFAGGKBMPJN",
            |m: &HMBOAEFMEMP| { &m.OFAGGKBMPJN },
            |m: &mut HMBOAEFMEMP| { &mut m.OFAGGKBMPJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FOEHLABGICC",
            |m: &HMBOAEFMEMP| { &m.FOEHLABGICC },
            |m: &mut HMBOAEFMEMP| { &mut m.FOEHLABGICC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BIODAPMJNGM",
            |m: &HMBOAEFMEMP| { &m.BIODAPMJNGM },
            |m: &mut HMBOAEFMEMP| { &mut m.BIODAPMJNGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GBKIEEKEJKD",
            |m: &HMBOAEFMEMP| { &m.GBKIEEKEJKD },
            |m: &mut HMBOAEFMEMP| { &mut m.GBKIEEKEJKD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IDGGIBBPPLK",
            |m: &HMBOAEFMEMP| { &m.IDGGIBBPPLK },
            |m: &mut HMBOAEFMEMP| { &mut m.IDGGIBBPPLK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KBNIFCHMAOB",
            |m: &HMBOAEFMEMP| { &m.KBNIFCHMAOB },
            |m: &mut HMBOAEFMEMP| { &mut m.KBNIFCHMAOB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LHFPBNAIABI",
            |m: &HMBOAEFMEMP| { &m.LHFPBNAIABI },
            |m: &mut HMBOAEFMEMP| { &mut m.LHFPBNAIABI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "APKEFKGPHIE",
            |m: &HMBOAEFMEMP| { &m.APKEFKGPHIE },
            |m: &mut HMBOAEFMEMP| { &mut m.APKEFKGPHIE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HMBOAEFMEMP>(
            "HMBOAEFMEMP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HMBOAEFMEMP {
    const NAME: &'static str = "HMBOAEFMEMP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.OFAGGKBMPJN = is.read_uint32()?;
                },
                32 => {
                    self.FOEHLABGICC = is.read_uint32()?;
                },
                120 => {
                    self.BIODAPMJNGM = is.read_uint32()?;
                },
                8 => {
                    self.GBKIEEKEJKD = is.read_uint32()?;
                },
                48 => {
                    self.IDGGIBBPPLK = is.read_uint32()?;
                },
                96 => {
                    self.KBNIFCHMAOB = is.read_uint32()?;
                },
                112 => {
                    self.LHFPBNAIABI = is.read_bool()?;
                },
                80 => {
                    self.APKEFKGPHIE = is.read_enum_or_unknown()?;
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
        if self.OFAGGKBMPJN != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.OFAGGKBMPJN);
        }
        if self.FOEHLABGICC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FOEHLABGICC);
        }
        if self.BIODAPMJNGM != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.BIODAPMJNGM);
        }
        if self.GBKIEEKEJKD != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.GBKIEEKEJKD);
        }
        if self.IDGGIBBPPLK != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.IDGGIBBPPLK);
        }
        if self.KBNIFCHMAOB != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.KBNIFCHMAOB);
        }
        if self.LHFPBNAIABI != false {
            my_size += 1 + 1;
        }
        if self.APKEFKGPHIE != ::protobuf::EnumOrUnknown::new(super::EOFOHACMKEP::EOFOHACMKEP::MATCH3_PLAYER_STATE_ALIVE) {
            my_size += ::protobuf::rt::int32_size(10, self.APKEFKGPHIE.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OFAGGKBMPJN != 0 {
            os.write_uint32(13, self.OFAGGKBMPJN)?;
        }
        if self.FOEHLABGICC != 0 {
            os.write_uint32(4, self.FOEHLABGICC)?;
        }
        if self.BIODAPMJNGM != 0 {
            os.write_uint32(15, self.BIODAPMJNGM)?;
        }
        if self.GBKIEEKEJKD != 0 {
            os.write_uint32(1, self.GBKIEEKEJKD)?;
        }
        if self.IDGGIBBPPLK != 0 {
            os.write_uint32(6, self.IDGGIBBPPLK)?;
        }
        if self.KBNIFCHMAOB != 0 {
            os.write_uint32(12, self.KBNIFCHMAOB)?;
        }
        if self.LHFPBNAIABI != false {
            os.write_bool(14, self.LHFPBNAIABI)?;
        }
        if self.APKEFKGPHIE != ::protobuf::EnumOrUnknown::new(super::EOFOHACMKEP::EOFOHACMKEP::MATCH3_PLAYER_STATE_ALIVE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.APKEFKGPHIE))?;
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

    fn new() -> HMBOAEFMEMP {
        HMBOAEFMEMP::new()
    }

    fn clear(&mut self) {
        self.OFAGGKBMPJN = 0;
        self.FOEHLABGICC = 0;
        self.BIODAPMJNGM = 0;
        self.GBKIEEKEJKD = 0;
        self.IDGGIBBPPLK = 0;
        self.KBNIFCHMAOB = 0;
        self.LHFPBNAIABI = false;
        self.APKEFKGPHIE = ::protobuf::EnumOrUnknown::new(super::EOFOHACMKEP::EOFOHACMKEP::MATCH3_PLAYER_STATE_ALIVE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HMBOAEFMEMP {
        static instance: HMBOAEFMEMP = HMBOAEFMEMP {
            OFAGGKBMPJN: 0,
            FOEHLABGICC: 0,
            BIODAPMJNGM: 0,
            GBKIEEKEJKD: 0,
            IDGGIBBPPLK: 0,
            KBNIFCHMAOB: 0,
            LHFPBNAIABI: false,
            APKEFKGPHIE: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HMBOAEFMEMP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HMBOAEFMEMP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HMBOAEFMEMP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HMBOAEFMEMP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HMBOAEFMEMP.proto\x1a\x11EOFOHACMKEP.proto\"\xab\x02\n\x0bHMBOAEFM\
    EMP\x12\x20\n\x0bOFAGGKBMPJN\x18\r\x20\x01(\rR\x0bOFAGGKBMPJN\x12\x20\n\
    \x0bFOEHLABGICC\x18\x04\x20\x01(\rR\x0bFOEHLABGICC\x12\x20\n\x0bBIODAPMJ\
    NGM\x18\x0f\x20\x01(\rR\x0bBIODAPMJNGM\x12\x20\n\x0bGBKIEEKEJKD\x18\x01\
    \x20\x01(\rR\x0bGBKIEEKEJKD\x12\x20\n\x0bIDGGIBBPPLK\x18\x06\x20\x01(\rR\
    \x0bIDGGIBBPPLK\x12\x20\n\x0bKBNIFCHMAOB\x18\x0c\x20\x01(\rR\x0bKBNIFCHM\
    AOB\x12\x20\n\x0bLHFPBNAIABI\x18\x0e\x20\x01(\x08R\x0bLHFPBNAIABI\x12.\n\
    \x0bAPKEFKGPHIE\x18\n\x20\x01(\x0e2\x0c.EOFOHACMKEPR\x0bAPKEFKGPHIEb\x06\
    proto3\
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
            deps.push(super::EOFOHACMKEP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HMBOAEFMEMP::generated_message_descriptor_data());
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
