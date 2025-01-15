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

//! Generated file from `FOMDHPEHADF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FOMDHPEHADF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FOMDHPEHADF {
    // message fields
    // @@protoc_insertion_point(field:FOMDHPEHADF.DEPEKPIEGJO)
    pub DEPEKPIEGJO: u32,
    // @@protoc_insertion_point(field:FOMDHPEHADF.level)
    pub level: u32,
    // @@protoc_insertion_point(field:FOMDHPEHADF.FFHMJJADIBG)
    pub FFHMJJADIBG: ::std::string::String,
    // @@protoc_insertion_point(field:FOMDHPEHADF.IALNGPAIKCH)
    pub IALNGPAIKCH: u32,
    // @@protoc_insertion_point(field:FOMDHPEHADF.MMMINKGDBAF)
    pub MMMINKGDBAF: ::protobuf::EnumOrUnknown<super::BMDNJEKCOAJ::BMDNJEKCOAJ>,
    // @@protoc_insertion_point(field:FOMDHPEHADF.PDCILADMJIJ)
    pub PDCILADMJIJ: ::std::string::String,
    // @@protoc_insertion_point(field:FOMDHPEHADF.IHEOMNBPMKM)
    pub IHEOMNBPMKM: ::std::string::String,
    // @@protoc_insertion_point(field:FOMDHPEHADF.PFKFGBKPBAJ)
    pub PFKFGBKPBAJ: u64,
    // special fields
    // @@protoc_insertion_point(special_field:FOMDHPEHADF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FOMDHPEHADF {
    fn default() -> &'a FOMDHPEHADF {
        <FOMDHPEHADF as ::protobuf::Message>::default_instance()
    }
}

impl FOMDHPEHADF {
    pub fn new() -> FOMDHPEHADF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DEPEKPIEGJO",
            |m: &FOMDHPEHADF| { &m.DEPEKPIEGJO },
            |m: &mut FOMDHPEHADF| { &mut m.DEPEKPIEGJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &FOMDHPEHADF| { &m.level },
            |m: &mut FOMDHPEHADF| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFHMJJADIBG",
            |m: &FOMDHPEHADF| { &m.FFHMJJADIBG },
            |m: &mut FOMDHPEHADF| { &mut m.FFHMJJADIBG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IALNGPAIKCH",
            |m: &FOMDHPEHADF| { &m.IALNGPAIKCH },
            |m: &mut FOMDHPEHADF| { &mut m.IALNGPAIKCH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMMINKGDBAF",
            |m: &FOMDHPEHADF| { &m.MMMINKGDBAF },
            |m: &mut FOMDHPEHADF| { &mut m.MMMINKGDBAF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PDCILADMJIJ",
            |m: &FOMDHPEHADF| { &m.PDCILADMJIJ },
            |m: &mut FOMDHPEHADF| { &mut m.PDCILADMJIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHEOMNBPMKM",
            |m: &FOMDHPEHADF| { &m.IHEOMNBPMKM },
            |m: &mut FOMDHPEHADF| { &mut m.IHEOMNBPMKM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PFKFGBKPBAJ",
            |m: &FOMDHPEHADF| { &m.PFKFGBKPBAJ },
            |m: &mut FOMDHPEHADF| { &mut m.PFKFGBKPBAJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FOMDHPEHADF>(
            "FOMDHPEHADF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FOMDHPEHADF {
    const NAME: &'static str = "FOMDHPEHADF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.DEPEKPIEGJO = is.read_uint32()?;
                },
                16 => {
                    self.level = is.read_uint32()?;
                },
                26 => {
                    self.FFHMJJADIBG = is.read_string()?;
                },
                32 => {
                    self.IALNGPAIKCH = is.read_uint32()?;
                },
                40 => {
                    self.MMMINKGDBAF = is.read_enum_or_unknown()?;
                },
                50 => {
                    self.PDCILADMJIJ = is.read_string()?;
                },
                58 => {
                    self.IHEOMNBPMKM = is.read_string()?;
                },
                64 => {
                    self.PFKFGBKPBAJ = is.read_uint64()?;
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
        if self.DEPEKPIEGJO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.DEPEKPIEGJO);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.level);
        }
        if !self.FFHMJJADIBG.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.FFHMJJADIBG);
        }
        if self.IALNGPAIKCH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.IALNGPAIKCH);
        }
        if self.MMMINKGDBAF != ::protobuf::EnumOrUnknown::new(super::BMDNJEKCOAJ::BMDNJEKCOAJ::EDITOR) {
            my_size += ::protobuf::rt::int32_size(5, self.MMMINKGDBAF.value());
        }
        if !self.PDCILADMJIJ.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.PDCILADMJIJ);
        }
        if !self.IHEOMNBPMKM.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.IHEOMNBPMKM);
        }
        if self.PFKFGBKPBAJ != 0 {
            my_size += ::protobuf::rt::uint64_size(8, self.PFKFGBKPBAJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DEPEKPIEGJO != 0 {
            os.write_uint32(1, self.DEPEKPIEGJO)?;
        }
        if self.level != 0 {
            os.write_uint32(2, self.level)?;
        }
        if !self.FFHMJJADIBG.is_empty() {
            os.write_string(3, &self.FFHMJJADIBG)?;
        }
        if self.IALNGPAIKCH != 0 {
            os.write_uint32(4, self.IALNGPAIKCH)?;
        }
        if self.MMMINKGDBAF != ::protobuf::EnumOrUnknown::new(super::BMDNJEKCOAJ::BMDNJEKCOAJ::EDITOR) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.MMMINKGDBAF))?;
        }
        if !self.PDCILADMJIJ.is_empty() {
            os.write_string(6, &self.PDCILADMJIJ)?;
        }
        if !self.IHEOMNBPMKM.is_empty() {
            os.write_string(7, &self.IHEOMNBPMKM)?;
        }
        if self.PFKFGBKPBAJ != 0 {
            os.write_uint64(8, self.PFKFGBKPBAJ)?;
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

    fn new() -> FOMDHPEHADF {
        FOMDHPEHADF::new()
    }

    fn clear(&mut self) {
        self.DEPEKPIEGJO = 0;
        self.level = 0;
        self.FFHMJJADIBG.clear();
        self.IALNGPAIKCH = 0;
        self.MMMINKGDBAF = ::protobuf::EnumOrUnknown::new(super::BMDNJEKCOAJ::BMDNJEKCOAJ::EDITOR);
        self.PDCILADMJIJ.clear();
        self.IHEOMNBPMKM.clear();
        self.PFKFGBKPBAJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FOMDHPEHADF {
        static instance: FOMDHPEHADF = FOMDHPEHADF {
            DEPEKPIEGJO: 0,
            level: 0,
            FFHMJJADIBG: ::std::string::String::new(),
            IALNGPAIKCH: 0,
            MMMINKGDBAF: ::protobuf::EnumOrUnknown::from_i32(0),
            PDCILADMJIJ: ::std::string::String::new(),
            IHEOMNBPMKM: ::std::string::String::new(),
            PFKFGBKPBAJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FOMDHPEHADF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FOMDHPEHADF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FOMDHPEHADF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FOMDHPEHADF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FOMDHPEHADF.proto\x1a\x11BMDNJEKCOAJ.proto\"\x9f\x02\n\x0bFOMDHPEH\
    ADF\x12\x20\n\x0bDEPEKPIEGJO\x18\x01\x20\x01(\rR\x0bDEPEKPIEGJO\x12\x14\
    \n\x05level\x18\x02\x20\x01(\rR\x05level\x12\x20\n\x0bFFHMJJADIBG\x18\
    \x03\x20\x01(\tR\x0bFFHMJJADIBG\x12\x20\n\x0bIALNGPAIKCH\x18\x04\x20\x01\
    (\rR\x0bIALNGPAIKCH\x12.\n\x0bMMMINKGDBAF\x18\x05\x20\x01(\x0e2\x0c.BMDN\
    JEKCOAJR\x0bMMMINKGDBAF\x12\x20\n\x0bPDCILADMJIJ\x18\x06\x20\x01(\tR\x0b\
    PDCILADMJIJ\x12\x20\n\x0bIHEOMNBPMKM\x18\x07\x20\x01(\tR\x0bIHEOMNBPMKM\
    \x12\x20\n\x0bPFKFGBKPBAJ\x18\x08\x20\x01(\x04R\x0bPFKFGBKPBAJb\x06proto\
    3\
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
            deps.push(super::BMDNJEKCOAJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FOMDHPEHADF::generated_message_descriptor_data());
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
