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

//! Generated file from `RogueTournStartCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RogueTournStartCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueTournStartCsReq {
    // message fields
    // @@protoc_insertion_point(field:RogueTournStartCsReq.EJOIJGCLCJO)
    pub EJOIJGCLCJO: bool,
    // @@protoc_insertion_point(field:RogueTournStartCsReq.NBCGLEFOKDM)
    pub NBCGLEFOKDM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:RogueTournStartCsReq.DBAHFEFGLMD)
    pub DBAHFEFGLMD: u32,
    // @@protoc_insertion_point(field:RogueTournStartCsReq.LGBOHDICFPK)
    pub LGBOHDICFPK: bool,
    // special fields
    // @@protoc_insertion_point(special_field:RogueTournStartCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueTournStartCsReq {
    fn default() -> &'a RogueTournStartCsReq {
        <RogueTournStartCsReq as ::protobuf::Message>::default_instance()
    }
}

impl RogueTournStartCsReq {
    pub fn new() -> RogueTournStartCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EJOIJGCLCJO",
            |m: &RogueTournStartCsReq| { &m.EJOIJGCLCJO },
            |m: &mut RogueTournStartCsReq| { &mut m.EJOIJGCLCJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NBCGLEFOKDM",
            |m: &RogueTournStartCsReq| { &m.NBCGLEFOKDM },
            |m: &mut RogueTournStartCsReq| { &mut m.NBCGLEFOKDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBAHFEFGLMD",
            |m: &RogueTournStartCsReq| { &m.DBAHFEFGLMD },
            |m: &mut RogueTournStartCsReq| { &mut m.DBAHFEFGLMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LGBOHDICFPK",
            |m: &RogueTournStartCsReq| { &m.LGBOHDICFPK },
            |m: &mut RogueTournStartCsReq| { &mut m.LGBOHDICFPK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueTournStartCsReq>(
            "RogueTournStartCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueTournStartCsReq {
    const NAME: &'static str = "RogueTournStartCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.EJOIJGCLCJO = is.read_bool()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.NBCGLEFOKDM)?;
                },
                8 => {
                    self.NBCGLEFOKDM.push(is.read_uint32()?);
                },
                40 => {
                    self.DBAHFEFGLMD = is.read_uint32()?;
                },
                72 => {
                    self.LGBOHDICFPK = is.read_bool()?;
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
        if self.EJOIJGCLCJO != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.NBCGLEFOKDM);
        if self.DBAHFEFGLMD != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.DBAHFEFGLMD);
        }
        if self.LGBOHDICFPK != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EJOIJGCLCJO != false {
            os.write_bool(13, self.EJOIJGCLCJO)?;
        }
        os.write_repeated_packed_uint32(1, &self.NBCGLEFOKDM)?;
        if self.DBAHFEFGLMD != 0 {
            os.write_uint32(5, self.DBAHFEFGLMD)?;
        }
        if self.LGBOHDICFPK != false {
            os.write_bool(9, self.LGBOHDICFPK)?;
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

    fn new() -> RogueTournStartCsReq {
        RogueTournStartCsReq::new()
    }

    fn clear(&mut self) {
        self.EJOIJGCLCJO = false;
        self.NBCGLEFOKDM.clear();
        self.DBAHFEFGLMD = 0;
        self.LGBOHDICFPK = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueTournStartCsReq {
        static instance: RogueTournStartCsReq = RogueTournStartCsReq {
            EJOIJGCLCJO: false,
            NBCGLEFOKDM: ::std::vec::Vec::new(),
            DBAHFEFGLMD: 0,
            LGBOHDICFPK: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueTournStartCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueTournStartCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueTournStartCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueTournStartCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aRogueTournStartCsReq.proto\"\x9e\x01\n\x14RogueTournStartCsReq\x12\
    \x20\n\x0bEJOIJGCLCJO\x18\r\x20\x01(\x08R\x0bEJOIJGCLCJO\x12\x20\n\x0bNB\
    CGLEFOKDM\x18\x01\x20\x03(\rR\x0bNBCGLEFOKDM\x12\x20\n\x0bDBAHFEFGLMD\
    \x18\x05\x20\x01(\rR\x0bDBAHFEFGLMD\x12\x20\n\x0bLGBOHDICFPK\x18\t\x20\
    \x01(\x08R\x0bLGBOHDICFPKb\x06proto3\
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
            messages.push(RogueTournStartCsReq::generated_message_descriptor_data());
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
