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

//! Generated file from `FONBBADFKBK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FONBBADFKBK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FONBBADFKBK {
    // message fields
    // @@protoc_insertion_point(field:FONBBADFKBK.ELGANMDPMID)
    pub ELGANMDPMID: u32,
    // @@protoc_insertion_point(field:FONBBADFKBK.JLHOGGDHMHG)
    pub JLHOGGDHMHG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:FONBBADFKBK.DKKLLMOHGFD)
    pub DKKLLMOHGFD: u32,
    // @@protoc_insertion_point(field:FONBBADFKBK.FDEJBIPGNDL)
    pub FDEJBIPGNDL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FONBBADFKBK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FONBBADFKBK {
    fn default() -> &'a FONBBADFKBK {
        <FONBBADFKBK as ::protobuf::Message>::default_instance()
    }
}

impl FONBBADFKBK {
    pub fn new() -> FONBBADFKBK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELGANMDPMID",
            |m: &FONBBADFKBK| { &m.ELGANMDPMID },
            |m: &mut FONBBADFKBK| { &mut m.ELGANMDPMID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JLHOGGDHMHG",
            |m: &FONBBADFKBK| { &m.JLHOGGDHMHG },
            |m: &mut FONBBADFKBK| { &mut m.JLHOGGDHMHG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKKLLMOHGFD",
            |m: &FONBBADFKBK| { &m.DKKLLMOHGFD },
            |m: &mut FONBBADFKBK| { &mut m.DKKLLMOHGFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDEJBIPGNDL",
            |m: &FONBBADFKBK| { &m.FDEJBIPGNDL },
            |m: &mut FONBBADFKBK| { &mut m.FDEJBIPGNDL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FONBBADFKBK>(
            "FONBBADFKBK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FONBBADFKBK {
    const NAME: &'static str = "FONBBADFKBK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.ELGANMDPMID = is.read_uint32()?;
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.JLHOGGDHMHG)?;
                },
                112 => {
                    self.JLHOGGDHMHG.push(is.read_uint32()?);
                },
                64 => {
                    self.DKKLLMOHGFD = is.read_uint32()?;
                },
                32 => {
                    self.FDEJBIPGNDL = is.read_uint32()?;
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
            my_size += ::protobuf::rt::uint32_size(2, self.ELGANMDPMID);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(14, &self.JLHOGGDHMHG);
        if self.DKKLLMOHGFD != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.DKKLLMOHGFD);
        }
        if self.FDEJBIPGNDL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FDEJBIPGNDL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ELGANMDPMID != 0 {
            os.write_uint32(2, self.ELGANMDPMID)?;
        }
        os.write_repeated_packed_uint32(14, &self.JLHOGGDHMHG)?;
        if self.DKKLLMOHGFD != 0 {
            os.write_uint32(8, self.DKKLLMOHGFD)?;
        }
        if self.FDEJBIPGNDL != 0 {
            os.write_uint32(4, self.FDEJBIPGNDL)?;
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

    fn new() -> FONBBADFKBK {
        FONBBADFKBK::new()
    }

    fn clear(&mut self) {
        self.ELGANMDPMID = 0;
        self.JLHOGGDHMHG.clear();
        self.DKKLLMOHGFD = 0;
        self.FDEJBIPGNDL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FONBBADFKBK {
        static instance: FONBBADFKBK = FONBBADFKBK {
            ELGANMDPMID: 0,
            JLHOGGDHMHG: ::std::vec::Vec::new(),
            DKKLLMOHGFD: 0,
            FDEJBIPGNDL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FONBBADFKBK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FONBBADFKBK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FONBBADFKBK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FONBBADFKBK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FONBBADFKBK.proto\"\x95\x01\n\x0bFONBBADFKBK\x12\x20\n\x0bELGANMDP\
    MID\x18\x02\x20\x01(\rR\x0bELGANMDPMID\x12\x20\n\x0bJLHOGGDHMHG\x18\x0e\
    \x20\x03(\rR\x0bJLHOGGDHMHG\x12\x20\n\x0bDKKLLMOHGFD\x18\x08\x20\x01(\rR\
    \x0bDKKLLMOHGFD\x12\x20\n\x0bFDEJBIPGNDL\x18\x04\x20\x01(\rR\x0bFDEJBIPG\
    NDLb\x06proto3\
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
            messages.push(FONBBADFKBK::generated_message_descriptor_data());
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
