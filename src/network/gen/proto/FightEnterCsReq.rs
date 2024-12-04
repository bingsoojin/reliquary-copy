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

//! Generated file from `FightEnterCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FightEnterCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FightEnterCsReq {
    // message fields
    // @@protoc_insertion_point(field:FightEnterCsReq.OMOGHACHNEJ)
    pub OMOGHACHNEJ: u32,
    // @@protoc_insertion_point(field:FightEnterCsReq.LFFMLLMDJNL)
    pub LFFMLLMDJNL: u32,
    // @@protoc_insertion_point(field:FightEnterCsReq.GJBHJBFLIHF)
    pub GJBHJBFLIHF: u32,
    // @@protoc_insertion_point(field:FightEnterCsReq.LNCNOFOEHAA)
    pub LNCNOFOEHAA: u32,
    // @@protoc_insertion_point(field:FightEnterCsReq.NBLPGHLNIHD)
    pub NBLPGHLNIHD: ::std::string::String,
    // @@protoc_insertion_point(field:FightEnterCsReq.ICGBEBADDBI)
    pub ICGBEBADDBI: u64,
    // @@protoc_insertion_point(field:FightEnterCsReq.IMKHANGGECP)
    pub IMKHANGGECP: u32,
    // @@protoc_insertion_point(field:FightEnterCsReq.IBMFPJBHJII)
    pub IBMFPJBHJII: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FightEnterCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FightEnterCsReq {
    fn default() -> &'a FightEnterCsReq {
        <FightEnterCsReq as ::protobuf::Message>::default_instance()
    }
}

impl FightEnterCsReq {
    pub fn new() -> FightEnterCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMOGHACHNEJ",
            |m: &FightEnterCsReq| { &m.OMOGHACHNEJ },
            |m: &mut FightEnterCsReq| { &mut m.OMOGHACHNEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFFMLLMDJNL",
            |m: &FightEnterCsReq| { &m.LFFMLLMDJNL },
            |m: &mut FightEnterCsReq| { &mut m.LFFMLLMDJNL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GJBHJBFLIHF",
            |m: &FightEnterCsReq| { &m.GJBHJBFLIHF },
            |m: &mut FightEnterCsReq| { &mut m.GJBHJBFLIHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LNCNOFOEHAA",
            |m: &FightEnterCsReq| { &m.LNCNOFOEHAA },
            |m: &mut FightEnterCsReq| { &mut m.LNCNOFOEHAA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NBLPGHLNIHD",
            |m: &FightEnterCsReq| { &m.NBLPGHLNIHD },
            |m: &mut FightEnterCsReq| { &mut m.NBLPGHLNIHD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ICGBEBADDBI",
            |m: &FightEnterCsReq| { &m.ICGBEBADDBI },
            |m: &mut FightEnterCsReq| { &mut m.ICGBEBADDBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IMKHANGGECP",
            |m: &FightEnterCsReq| { &m.IMKHANGGECP },
            |m: &mut FightEnterCsReq| { &mut m.IMKHANGGECP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBMFPJBHJII",
            |m: &FightEnterCsReq| { &m.IBMFPJBHJII },
            |m: &mut FightEnterCsReq| { &mut m.IBMFPJBHJII },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FightEnterCsReq>(
            "FightEnterCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FightEnterCsReq {
    const NAME: &'static str = "FightEnterCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.OMOGHACHNEJ = is.read_uint32()?;
                },
                64 => {
                    self.LFFMLLMDJNL = is.read_uint32()?;
                },
                24 => {
                    self.GJBHJBFLIHF = is.read_uint32()?;
                },
                112 => {
                    self.LNCNOFOEHAA = is.read_uint32()?;
                },
                10 => {
                    self.NBLPGHLNIHD = is.read_string()?;
                },
                80 => {
                    self.ICGBEBADDBI = is.read_uint64()?;
                },
                56 => {
                    self.IMKHANGGECP = is.read_uint32()?;
                },
                16 => {
                    self.IBMFPJBHJII = is.read_uint32()?;
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
        if self.OMOGHACHNEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.OMOGHACHNEJ);
        }
        if self.LFFMLLMDJNL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.LFFMLLMDJNL);
        }
        if self.GJBHJBFLIHF != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.GJBHJBFLIHF);
        }
        if self.LNCNOFOEHAA != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.LNCNOFOEHAA);
        }
        if !self.NBLPGHLNIHD.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.NBLPGHLNIHD);
        }
        if self.ICGBEBADDBI != 0 {
            my_size += ::protobuf::rt::uint64_size(10, self.ICGBEBADDBI);
        }
        if self.IMKHANGGECP != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.IMKHANGGECP);
        }
        if self.IBMFPJBHJII != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.IBMFPJBHJII);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OMOGHACHNEJ != 0 {
            os.write_uint32(5, self.OMOGHACHNEJ)?;
        }
        if self.LFFMLLMDJNL != 0 {
            os.write_uint32(8, self.LFFMLLMDJNL)?;
        }
        if self.GJBHJBFLIHF != 0 {
            os.write_uint32(3, self.GJBHJBFLIHF)?;
        }
        if self.LNCNOFOEHAA != 0 {
            os.write_uint32(14, self.LNCNOFOEHAA)?;
        }
        if !self.NBLPGHLNIHD.is_empty() {
            os.write_string(1, &self.NBLPGHLNIHD)?;
        }
        if self.ICGBEBADDBI != 0 {
            os.write_uint64(10, self.ICGBEBADDBI)?;
        }
        if self.IMKHANGGECP != 0 {
            os.write_uint32(7, self.IMKHANGGECP)?;
        }
        if self.IBMFPJBHJII != 0 {
            os.write_uint32(2, self.IBMFPJBHJII)?;
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

    fn new() -> FightEnterCsReq {
        FightEnterCsReq::new()
    }

    fn clear(&mut self) {
        self.OMOGHACHNEJ = 0;
        self.LFFMLLMDJNL = 0;
        self.GJBHJBFLIHF = 0;
        self.LNCNOFOEHAA = 0;
        self.NBLPGHLNIHD.clear();
        self.ICGBEBADDBI = 0;
        self.IMKHANGGECP = 0;
        self.IBMFPJBHJII = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FightEnterCsReq {
        static instance: FightEnterCsReq = FightEnterCsReq {
            OMOGHACHNEJ: 0,
            LFFMLLMDJNL: 0,
            GJBHJBFLIHF: 0,
            LNCNOFOEHAA: 0,
            NBLPGHLNIHD: ::std::string::String::new(),
            ICGBEBADDBI: 0,
            IMKHANGGECP: 0,
            IBMFPJBHJII: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FightEnterCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FightEnterCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FightEnterCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FightEnterCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15FightEnterCsReq.proto\"\xa1\x02\n\x0fFightEnterCsReq\x12\x20\n\x0b\
    OMOGHACHNEJ\x18\x05\x20\x01(\rR\x0bOMOGHACHNEJ\x12\x20\n\x0bLFFMLLMDJNL\
    \x18\x08\x20\x01(\rR\x0bLFFMLLMDJNL\x12\x20\n\x0bGJBHJBFLIHF\x18\x03\x20\
    \x01(\rR\x0bGJBHJBFLIHF\x12\x20\n\x0bLNCNOFOEHAA\x18\x0e\x20\x01(\rR\x0b\
    LNCNOFOEHAA\x12\x20\n\x0bNBLPGHLNIHD\x18\x01\x20\x01(\tR\x0bNBLPGHLNIHD\
    \x12\x20\n\x0bICGBEBADDBI\x18\n\x20\x01(\x04R\x0bICGBEBADDBI\x12\x20\n\
    \x0bIMKHANGGECP\x18\x07\x20\x01(\rR\x0bIMKHANGGECP\x12\x20\n\x0bIBMFPJBH\
    JII\x18\x02\x20\x01(\rR\x0bIBMFPJBHJIIb\x06proto3\
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
            messages.push(FightEnterCsReq::generated_message_descriptor_data());
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
