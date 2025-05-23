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

//! Generated file from `ChessRogueStartCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ChessRogueStartCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueStartCsReq {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueStartCsReq.DHNDAMPBHPP)
    pub DHNDAMPBHPP: u32,
    // @@protoc_insertion_point(field:ChessRogueStartCsReq.LGHCAHBBCAM)
    pub LGHCAHBBCAM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ChessRogueStartCsReq.ELGANMDPMID)
    pub ELGANMDPMID: u32,
    // @@protoc_insertion_point(field:ChessRogueStartCsReq.ELAPPCMELOA)
    pub ELAPPCMELOA: u32,
    // @@protoc_insertion_point(field:ChessRogueStartCsReq.NBCGLEFOKDM)
    pub NBCGLEFOKDM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ChessRogueStartCsReq.HJGNDHLMMIB)
    pub HJGNDHLMMIB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ChessRogueStartCsReq.AHIIBHKDFJB)
    pub AHIIBHKDFJB: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueStartCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueStartCsReq {
    fn default() -> &'a ChessRogueStartCsReq {
        <ChessRogueStartCsReq as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueStartCsReq {
    pub fn new() -> ChessRogueStartCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DHNDAMPBHPP",
            |m: &ChessRogueStartCsReq| { &m.DHNDAMPBHPP },
            |m: &mut ChessRogueStartCsReq| { &mut m.DHNDAMPBHPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LGHCAHBBCAM",
            |m: &ChessRogueStartCsReq| { &m.LGHCAHBBCAM },
            |m: &mut ChessRogueStartCsReq| { &mut m.LGHCAHBBCAM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELGANMDPMID",
            |m: &ChessRogueStartCsReq| { &m.ELGANMDPMID },
            |m: &mut ChessRogueStartCsReq| { &mut m.ELGANMDPMID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELAPPCMELOA",
            |m: &ChessRogueStartCsReq| { &m.ELAPPCMELOA },
            |m: &mut ChessRogueStartCsReq| { &mut m.ELAPPCMELOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NBCGLEFOKDM",
            |m: &ChessRogueStartCsReq| { &m.NBCGLEFOKDM },
            |m: &mut ChessRogueStartCsReq| { &mut m.NBCGLEFOKDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HJGNDHLMMIB",
            |m: &ChessRogueStartCsReq| { &m.HJGNDHLMMIB },
            |m: &mut ChessRogueStartCsReq| { &mut m.HJGNDHLMMIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AHIIBHKDFJB",
            |m: &ChessRogueStartCsReq| { &m.AHIIBHKDFJB },
            |m: &mut ChessRogueStartCsReq| { &mut m.AHIIBHKDFJB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueStartCsReq>(
            "ChessRogueStartCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueStartCsReq {
    const NAME: &'static str = "ChessRogueStartCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.DHNDAMPBHPP = is.read_uint32()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.LGHCAHBBCAM)?;
                },
                24 => {
                    self.LGHCAHBBCAM.push(is.read_uint32()?);
                },
                120 => {
                    self.ELGANMDPMID = is.read_uint32()?;
                },
                64 => {
                    self.ELAPPCMELOA = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.NBCGLEFOKDM)?;
                },
                8 => {
                    self.NBCGLEFOKDM.push(is.read_uint32()?);
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.HJGNDHLMMIB)?;
                },
                56 => {
                    self.HJGNDHLMMIB.push(is.read_uint32()?);
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.AHIIBHKDFJB)?;
                },
                16 => {
                    self.AHIIBHKDFJB.push(is.read_uint32()?);
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
        if self.DHNDAMPBHPP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.DHNDAMPBHPP);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(3, &self.LGHCAHBBCAM);
        if self.ELGANMDPMID != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.ELGANMDPMID);
        }
        if self.ELAPPCMELOA != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.ELAPPCMELOA);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.NBCGLEFOKDM);
        my_size += ::protobuf::rt::vec_packed_uint32_size(7, &self.HJGNDHLMMIB);
        my_size += ::protobuf::rt::vec_packed_uint32_size(2, &self.AHIIBHKDFJB);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DHNDAMPBHPP != 0 {
            os.write_uint32(11, self.DHNDAMPBHPP)?;
        }
        os.write_repeated_packed_uint32(3, &self.LGHCAHBBCAM)?;
        if self.ELGANMDPMID != 0 {
            os.write_uint32(15, self.ELGANMDPMID)?;
        }
        if self.ELAPPCMELOA != 0 {
            os.write_uint32(8, self.ELAPPCMELOA)?;
        }
        os.write_repeated_packed_uint32(1, &self.NBCGLEFOKDM)?;
        os.write_repeated_packed_uint32(7, &self.HJGNDHLMMIB)?;
        os.write_repeated_packed_uint32(2, &self.AHIIBHKDFJB)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ChessRogueStartCsReq {
        ChessRogueStartCsReq::new()
    }

    fn clear(&mut self) {
        self.DHNDAMPBHPP = 0;
        self.LGHCAHBBCAM.clear();
        self.ELGANMDPMID = 0;
        self.ELAPPCMELOA = 0;
        self.NBCGLEFOKDM.clear();
        self.HJGNDHLMMIB.clear();
        self.AHIIBHKDFJB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueStartCsReq {
        static instance: ChessRogueStartCsReq = ChessRogueStartCsReq {
            DHNDAMPBHPP: 0,
            LGHCAHBBCAM: ::std::vec::Vec::new(),
            ELGANMDPMID: 0,
            ELAPPCMELOA: 0,
            NBCGLEFOKDM: ::std::vec::Vec::new(),
            HJGNDHLMMIB: ::std::vec::Vec::new(),
            AHIIBHKDFJB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueStartCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueStartCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueStartCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueStartCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aChessRogueStartCsReq.proto\"\x84\x02\n\x14ChessRogueStartCsReq\x12\
    \x20\n\x0bDHNDAMPBHPP\x18\x0b\x20\x01(\rR\x0bDHNDAMPBHPP\x12\x20\n\x0bLG\
    HCAHBBCAM\x18\x03\x20\x03(\rR\x0bLGHCAHBBCAM\x12\x20\n\x0bELGANMDPMID\
    \x18\x0f\x20\x01(\rR\x0bELGANMDPMID\x12\x20\n\x0bELAPPCMELOA\x18\x08\x20\
    \x01(\rR\x0bELAPPCMELOA\x12\x20\n\x0bNBCGLEFOKDM\x18\x01\x20\x03(\rR\x0b\
    NBCGLEFOKDM\x12\x20\n\x0bHJGNDHLMMIB\x18\x07\x20\x03(\rR\x0bHJGNDHLMMIB\
    \x12\x20\n\x0bAHIIBHKDFJB\x18\x02\x20\x03(\rR\x0bAHIIBHKDFJBb\x06proto3\
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
            messages.push(ChessRogueStartCsReq::generated_message_descriptor_data());
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
