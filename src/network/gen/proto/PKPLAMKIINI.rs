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

//! Generated file from `PKPLAMKIINI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PKPLAMKIINI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PKPLAMKIINI {
    // message fields
    // @@protoc_insertion_point(field:PKPLAMKIINI.EGLINHEDAJA)
    pub EGLINHEDAJA: i64,
    // @@protoc_insertion_point(field:PKPLAMKIINI.BCELKCDMLHF)
    pub BCELKCDMLHF: u32,
    // @@protoc_insertion_point(field:PKPLAMKIINI.HMJBGDPIMCP)
    pub HMJBGDPIMCP: u32,
    // @@protoc_insertion_point(field:PKPLAMKIINI.OBJPEHEEMEE)
    pub OBJPEHEEMEE: ::std::string::String,
    // @@protoc_insertion_point(field:PKPLAMKIINI.CILOJBALHEF)
    pub CILOJBALHEF: u32,
    // @@protoc_insertion_point(field:PKPLAMKIINI.NMMJAMBLBFB)
    pub NMMJAMBLBFB: bool,
    // @@protoc_insertion_point(field:PKPLAMKIINI.NDBLEEBNCPG)
    pub NDBLEEBNCPG: i64,
    // @@protoc_insertion_point(field:PKPLAMKIINI.OBNMJMLKDIM)
    pub OBNMJMLKDIM: ::std::string::String,
    // @@protoc_insertion_point(field:PKPLAMKIINI.FHNBHGCAMLG)
    pub FHNBHGCAMLG: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:PKPLAMKIINI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PKPLAMKIINI {
    fn default() -> &'a PKPLAMKIINI {
        <PKPLAMKIINI as ::protobuf::Message>::default_instance()
    }
}

impl PKPLAMKIINI {
    pub fn new() -> PKPLAMKIINI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EGLINHEDAJA",
            |m: &PKPLAMKIINI| { &m.EGLINHEDAJA },
            |m: &mut PKPLAMKIINI| { &mut m.EGLINHEDAJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BCELKCDMLHF",
            |m: &PKPLAMKIINI| { &m.BCELKCDMLHF },
            |m: &mut PKPLAMKIINI| { &mut m.BCELKCDMLHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HMJBGDPIMCP",
            |m: &PKPLAMKIINI| { &m.HMJBGDPIMCP },
            |m: &mut PKPLAMKIINI| { &mut m.HMJBGDPIMCP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OBJPEHEEMEE",
            |m: &PKPLAMKIINI| { &m.OBJPEHEEMEE },
            |m: &mut PKPLAMKIINI| { &mut m.OBJPEHEEMEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CILOJBALHEF",
            |m: &PKPLAMKIINI| { &m.CILOJBALHEF },
            |m: &mut PKPLAMKIINI| { &mut m.CILOJBALHEF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMMJAMBLBFB",
            |m: &PKPLAMKIINI| { &m.NMMJAMBLBFB },
            |m: &mut PKPLAMKIINI| { &mut m.NMMJAMBLBFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NDBLEEBNCPG",
            |m: &PKPLAMKIINI| { &m.NDBLEEBNCPG },
            |m: &mut PKPLAMKIINI| { &mut m.NDBLEEBNCPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OBNMJMLKDIM",
            |m: &PKPLAMKIINI| { &m.OBNMJMLKDIM },
            |m: &mut PKPLAMKIINI| { &mut m.OBNMJMLKDIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHNBHGCAMLG",
            |m: &PKPLAMKIINI| { &m.FHNBHGCAMLG },
            |m: &mut PKPLAMKIINI| { &mut m.FHNBHGCAMLG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PKPLAMKIINI>(
            "PKPLAMKIINI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PKPLAMKIINI {
    const NAME: &'static str = "PKPLAMKIINI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.EGLINHEDAJA = is.read_int64()?;
                },
                56 => {
                    self.BCELKCDMLHF = is.read_uint32()?;
                },
                72 => {
                    self.HMJBGDPIMCP = is.read_uint32()?;
                },
                50 => {
                    self.OBJPEHEEMEE = is.read_string()?;
                },
                24 => {
                    self.CILOJBALHEF = is.read_uint32()?;
                },
                120 => {
                    self.NMMJAMBLBFB = is.read_bool()?;
                },
                40 => {
                    self.NDBLEEBNCPG = is.read_int64()?;
                },
                114 => {
                    self.OBNMJMLKDIM = is.read_string()?;
                },
                10 => {
                    self.FHNBHGCAMLG = is.read_string()?;
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
        if self.EGLINHEDAJA != 0 {
            my_size += ::protobuf::rt::int64_size(4, self.EGLINHEDAJA);
        }
        if self.BCELKCDMLHF != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.BCELKCDMLHF);
        }
        if self.HMJBGDPIMCP != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.HMJBGDPIMCP);
        }
        if !self.OBJPEHEEMEE.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.OBJPEHEEMEE);
        }
        if self.CILOJBALHEF != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.CILOJBALHEF);
        }
        if self.NMMJAMBLBFB != false {
            my_size += 1 + 1;
        }
        if self.NDBLEEBNCPG != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.NDBLEEBNCPG);
        }
        if !self.OBNMJMLKDIM.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.OBNMJMLKDIM);
        }
        if !self.FHNBHGCAMLG.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.FHNBHGCAMLG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EGLINHEDAJA != 0 {
            os.write_int64(4, self.EGLINHEDAJA)?;
        }
        if self.BCELKCDMLHF != 0 {
            os.write_uint32(7, self.BCELKCDMLHF)?;
        }
        if self.HMJBGDPIMCP != 0 {
            os.write_uint32(9, self.HMJBGDPIMCP)?;
        }
        if !self.OBJPEHEEMEE.is_empty() {
            os.write_string(6, &self.OBJPEHEEMEE)?;
        }
        if self.CILOJBALHEF != 0 {
            os.write_uint32(3, self.CILOJBALHEF)?;
        }
        if self.NMMJAMBLBFB != false {
            os.write_bool(15, self.NMMJAMBLBFB)?;
        }
        if self.NDBLEEBNCPG != 0 {
            os.write_int64(5, self.NDBLEEBNCPG)?;
        }
        if !self.OBNMJMLKDIM.is_empty() {
            os.write_string(14, &self.OBNMJMLKDIM)?;
        }
        if !self.FHNBHGCAMLG.is_empty() {
            os.write_string(1, &self.FHNBHGCAMLG)?;
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

    fn new() -> PKPLAMKIINI {
        PKPLAMKIINI::new()
    }

    fn clear(&mut self) {
        self.EGLINHEDAJA = 0;
        self.BCELKCDMLHF = 0;
        self.HMJBGDPIMCP = 0;
        self.OBJPEHEEMEE.clear();
        self.CILOJBALHEF = 0;
        self.NMMJAMBLBFB = false;
        self.NDBLEEBNCPG = 0;
        self.OBNMJMLKDIM.clear();
        self.FHNBHGCAMLG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PKPLAMKIINI {
        static instance: PKPLAMKIINI = PKPLAMKIINI {
            EGLINHEDAJA: 0,
            BCELKCDMLHF: 0,
            HMJBGDPIMCP: 0,
            OBJPEHEEMEE: ::std::string::String::new(),
            CILOJBALHEF: 0,
            NMMJAMBLBFB: false,
            NDBLEEBNCPG: 0,
            OBNMJMLKDIM: ::std::string::String::new(),
            FHNBHGCAMLG: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PKPLAMKIINI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PKPLAMKIINI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PKPLAMKIINI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PKPLAMKIINI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PKPLAMKIINI.proto\"\xbf\x02\n\x0bPKPLAMKIINI\x12\x20\n\x0bEGLINHED\
    AJA\x18\x04\x20\x01(\x03R\x0bEGLINHEDAJA\x12\x20\n\x0bBCELKCDMLHF\x18\
    \x07\x20\x01(\rR\x0bBCELKCDMLHF\x12\x20\n\x0bHMJBGDPIMCP\x18\t\x20\x01(\
    \rR\x0bHMJBGDPIMCP\x12\x20\n\x0bOBJPEHEEMEE\x18\x06\x20\x01(\tR\x0bOBJPE\
    HEEMEE\x12\x20\n\x0bCILOJBALHEF\x18\x03\x20\x01(\rR\x0bCILOJBALHEF\x12\
    \x20\n\x0bNMMJAMBLBFB\x18\x0f\x20\x01(\x08R\x0bNMMJAMBLBFB\x12\x20\n\x0b\
    NDBLEEBNCPG\x18\x05\x20\x01(\x03R\x0bNDBLEEBNCPG\x12\x20\n\x0bOBNMJMLKDI\
    M\x18\x0e\x20\x01(\tR\x0bOBNMJMLKDIM\x12\x20\n\x0bFHNBHGCAMLG\x18\x01\
    \x20\x01(\tR\x0bFHNBHGCAMLGb\x06proto3\
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
            messages.push(PKPLAMKIINI::generated_message_descriptor_data());
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
