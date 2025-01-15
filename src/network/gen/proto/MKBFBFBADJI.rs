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

//! Generated file from `MKBFBFBADJI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MKBFBFBADJI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MKBFBFBADJI {
    // message fields
    // @@protoc_insertion_point(field:MKBFBFBADJI.KJKPMFFAMFK)
    pub KJKPMFFAMFK: u32,
    // @@protoc_insertion_point(field:MKBFBFBADJI.HDMFPLOCPMK)
    pub HDMFPLOCPMK: bool,
    // @@protoc_insertion_point(field:MKBFBFBADJI.JILABLLNJCG)
    pub JILABLLNJCG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MKBFBFBADJI.NMGJIBAJGFG)
    pub NMGJIBAJGFG: bool,
    // @@protoc_insertion_point(field:MKBFBFBADJI.KPHGMOIDFND)
    pub KPHGMOIDFND: bool,
    // @@protoc_insertion_point(field:MKBFBFBADJI.LHOCPPKKMKH)
    pub LHOCPPKKMKH: bool,
    // special fields
    // @@protoc_insertion_point(special_field:MKBFBFBADJI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MKBFBFBADJI {
    fn default() -> &'a MKBFBFBADJI {
        <MKBFBFBADJI as ::protobuf::Message>::default_instance()
    }
}

impl MKBFBFBADJI {
    pub fn new() -> MKBFBFBADJI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KJKPMFFAMFK",
            |m: &MKBFBFBADJI| { &m.KJKPMFFAMFK },
            |m: &mut MKBFBFBADJI| { &mut m.KJKPMFFAMFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HDMFPLOCPMK",
            |m: &MKBFBFBADJI| { &m.HDMFPLOCPMK },
            |m: &mut MKBFBFBADJI| { &mut m.HDMFPLOCPMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JILABLLNJCG",
            |m: &MKBFBFBADJI| { &m.JILABLLNJCG },
            |m: &mut MKBFBFBADJI| { &mut m.JILABLLNJCG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMGJIBAJGFG",
            |m: &MKBFBFBADJI| { &m.NMGJIBAJGFG },
            |m: &mut MKBFBFBADJI| { &mut m.NMGJIBAJGFG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KPHGMOIDFND",
            |m: &MKBFBFBADJI| { &m.KPHGMOIDFND },
            |m: &mut MKBFBFBADJI| { &mut m.KPHGMOIDFND },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LHOCPPKKMKH",
            |m: &MKBFBFBADJI| { &m.LHOCPPKKMKH },
            |m: &mut MKBFBFBADJI| { &mut m.LHOCPPKKMKH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MKBFBFBADJI>(
            "MKBFBFBADJI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MKBFBFBADJI {
    const NAME: &'static str = "MKBFBFBADJI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.KJKPMFFAMFK = is.read_uint32()?;
                },
                72 => {
                    self.HDMFPLOCPMK = is.read_bool()?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.JILABLLNJCG)?;
                },
                32 => {
                    self.JILABLLNJCG.push(is.read_uint32()?);
                },
                40 => {
                    self.NMGJIBAJGFG = is.read_bool()?;
                },
                24 => {
                    self.KPHGMOIDFND = is.read_bool()?;
                },
                80 => {
                    self.LHOCPPKKMKH = is.read_bool()?;
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
        if self.KJKPMFFAMFK != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.KJKPMFFAMFK);
        }
        if self.HDMFPLOCPMK != false {
            my_size += 1 + 1;
        }
        for value in &self.JILABLLNJCG {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        if self.NMGJIBAJGFG != false {
            my_size += 1 + 1;
        }
        if self.KPHGMOIDFND != false {
            my_size += 1 + 1;
        }
        if self.LHOCPPKKMKH != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KJKPMFFAMFK != 0 {
            os.write_uint32(13, self.KJKPMFFAMFK)?;
        }
        if self.HDMFPLOCPMK != false {
            os.write_bool(9, self.HDMFPLOCPMK)?;
        }
        for v in &self.JILABLLNJCG {
            os.write_uint32(4, *v)?;
        };
        if self.NMGJIBAJGFG != false {
            os.write_bool(5, self.NMGJIBAJGFG)?;
        }
        if self.KPHGMOIDFND != false {
            os.write_bool(3, self.KPHGMOIDFND)?;
        }
        if self.LHOCPPKKMKH != false {
            os.write_bool(10, self.LHOCPPKKMKH)?;
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

    fn new() -> MKBFBFBADJI {
        MKBFBFBADJI::new()
    }

    fn clear(&mut self) {
        self.KJKPMFFAMFK = 0;
        self.HDMFPLOCPMK = false;
        self.JILABLLNJCG.clear();
        self.NMGJIBAJGFG = false;
        self.KPHGMOIDFND = false;
        self.LHOCPPKKMKH = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MKBFBFBADJI {
        static instance: MKBFBFBADJI = MKBFBFBADJI {
            KJKPMFFAMFK: 0,
            HDMFPLOCPMK: false,
            JILABLLNJCG: ::std::vec::Vec::new(),
            NMGJIBAJGFG: false,
            KPHGMOIDFND: false,
            LHOCPPKKMKH: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MKBFBFBADJI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MKBFBFBADJI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MKBFBFBADJI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MKBFBFBADJI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MKBFBFBADJI.proto\"\xd9\x01\n\x0bMKBFBFBADJI\x12\x20\n\x0bKJKPMFFA\
    MFK\x18\r\x20\x01(\rR\x0bKJKPMFFAMFK\x12\x20\n\x0bHDMFPLOCPMK\x18\t\x20\
    \x01(\x08R\x0bHDMFPLOCPMK\x12\x20\n\x0bJILABLLNJCG\x18\x04\x20\x03(\rR\
    \x0bJILABLLNJCG\x12\x20\n\x0bNMGJIBAJGFG\x18\x05\x20\x01(\x08R\x0bNMGJIB\
    AJGFG\x12\x20\n\x0bKPHGMOIDFND\x18\x03\x20\x01(\x08R\x0bKPHGMOIDFND\x12\
    \x20\n\x0bLHOCPPKKMKH\x18\n\x20\x01(\x08R\x0bLHOCPPKKMKHb\x06proto3\
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
            messages.push(MKBFBFBADJI::generated_message_descriptor_data());
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
