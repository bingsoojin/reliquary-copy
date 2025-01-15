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

//! Generated file from `ELGAMGEDGOM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ELGAMGEDGOM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ELGAMGEDGOM {
    // message fields
    // @@protoc_insertion_point(field:ELGAMGEDGOM.KPHGMOIDFND)
    pub KPHGMOIDFND: bool,
    // @@protoc_insertion_point(field:ELGAMGEDGOM.AONPFEDMDBF)
    pub AONPFEDMDBF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ELGAMGEDGOM.NMBOMMJJMMB)
    pub NMBOMMJJMMB: ::protobuf::MessageField<super::LMMKDBFDOJF::LMMKDBFDOJF>,
    // @@protoc_insertion_point(field:ELGAMGEDGOM.KJKPMFFAMFK)
    pub KJKPMFFAMFK: u32,
    // @@protoc_insertion_point(field:ELGAMGEDGOM.LEJNOLOBOGE)
    pub LEJNOLOBOGE: u32,
    // @@protoc_insertion_point(field:ELGAMGEDGOM.DGKAJMOCFIA)
    pub DGKAJMOCFIA: ::protobuf::MessageField<super::NCDFMCGOKLH::NCDFMCGOKLH>,
    // @@protoc_insertion_point(field:ELGAMGEDGOM.NMGJIBAJGFG)
    pub NMGJIBAJGFG: bool,
    // @@protoc_insertion_point(field:ELGAMGEDGOM.HDMFPLOCPMK)
    pub HDMFPLOCPMK: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ELGAMGEDGOM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ELGAMGEDGOM {
    fn default() -> &'a ELGAMGEDGOM {
        <ELGAMGEDGOM as ::protobuf::Message>::default_instance()
    }
}

impl ELGAMGEDGOM {
    pub fn new() -> ELGAMGEDGOM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KPHGMOIDFND",
            |m: &ELGAMGEDGOM| { &m.KPHGMOIDFND },
            |m: &mut ELGAMGEDGOM| { &mut m.KPHGMOIDFND },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AONPFEDMDBF",
            |m: &ELGAMGEDGOM| { &m.AONPFEDMDBF },
            |m: &mut ELGAMGEDGOM| { &mut m.AONPFEDMDBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LMMKDBFDOJF::LMMKDBFDOJF>(
            "NMBOMMJJMMB",
            |m: &ELGAMGEDGOM| { &m.NMBOMMJJMMB },
            |m: &mut ELGAMGEDGOM| { &mut m.NMBOMMJJMMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KJKPMFFAMFK",
            |m: &ELGAMGEDGOM| { &m.KJKPMFFAMFK },
            |m: &mut ELGAMGEDGOM| { &mut m.KJKPMFFAMFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEJNOLOBOGE",
            |m: &ELGAMGEDGOM| { &m.LEJNOLOBOGE },
            |m: &mut ELGAMGEDGOM| { &mut m.LEJNOLOBOGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NCDFMCGOKLH::NCDFMCGOKLH>(
            "DGKAJMOCFIA",
            |m: &ELGAMGEDGOM| { &m.DGKAJMOCFIA },
            |m: &mut ELGAMGEDGOM| { &mut m.DGKAJMOCFIA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMGJIBAJGFG",
            |m: &ELGAMGEDGOM| { &m.NMGJIBAJGFG },
            |m: &mut ELGAMGEDGOM| { &mut m.NMGJIBAJGFG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HDMFPLOCPMK",
            |m: &ELGAMGEDGOM| { &m.HDMFPLOCPMK },
            |m: &mut ELGAMGEDGOM| { &mut m.HDMFPLOCPMK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ELGAMGEDGOM>(
            "ELGAMGEDGOM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ELGAMGEDGOM {
    const NAME: &'static str = "ELGAMGEDGOM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.KPHGMOIDFND = is.read_bool()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.AONPFEDMDBF)?;
                },
                104 => {
                    self.AONPFEDMDBF.push(is.read_uint32()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NMBOMMJJMMB)?;
                },
                32 => {
                    self.KJKPMFFAMFK = is.read_uint32()?;
                },
                96 => {
                    self.LEJNOLOBOGE = is.read_uint32()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DGKAJMOCFIA)?;
                },
                112 => {
                    self.NMGJIBAJGFG = is.read_bool()?;
                },
                48 => {
                    self.HDMFPLOCPMK = is.read_bool()?;
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
        if self.KPHGMOIDFND != false {
            my_size += 1 + 1;
        }
        for value in &self.AONPFEDMDBF {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if let Some(v) = self.NMBOMMJJMMB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.KJKPMFFAMFK != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.KJKPMFFAMFK);
        }
        if self.LEJNOLOBOGE != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.LEJNOLOBOGE);
        }
        if let Some(v) = self.DGKAJMOCFIA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NMGJIBAJGFG != false {
            my_size += 1 + 1;
        }
        if self.HDMFPLOCPMK != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KPHGMOIDFND != false {
            os.write_bool(8, self.KPHGMOIDFND)?;
        }
        for v in &self.AONPFEDMDBF {
            os.write_uint32(13, *v)?;
        };
        if let Some(v) = self.NMBOMMJJMMB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.KJKPMFFAMFK != 0 {
            os.write_uint32(4, self.KJKPMFFAMFK)?;
        }
        if self.LEJNOLOBOGE != 0 {
            os.write_uint32(12, self.LEJNOLOBOGE)?;
        }
        if let Some(v) = self.DGKAJMOCFIA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.NMGJIBAJGFG != false {
            os.write_bool(14, self.NMGJIBAJGFG)?;
        }
        if self.HDMFPLOCPMK != false {
            os.write_bool(6, self.HDMFPLOCPMK)?;
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

    fn new() -> ELGAMGEDGOM {
        ELGAMGEDGOM::new()
    }

    fn clear(&mut self) {
        self.KPHGMOIDFND = false;
        self.AONPFEDMDBF.clear();
        self.NMBOMMJJMMB.clear();
        self.KJKPMFFAMFK = 0;
        self.LEJNOLOBOGE = 0;
        self.DGKAJMOCFIA.clear();
        self.NMGJIBAJGFG = false;
        self.HDMFPLOCPMK = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ELGAMGEDGOM {
        static instance: ELGAMGEDGOM = ELGAMGEDGOM {
            KPHGMOIDFND: false,
            AONPFEDMDBF: ::std::vec::Vec::new(),
            NMBOMMJJMMB: ::protobuf::MessageField::none(),
            KJKPMFFAMFK: 0,
            LEJNOLOBOGE: 0,
            DGKAJMOCFIA: ::protobuf::MessageField::none(),
            NMGJIBAJGFG: false,
            HDMFPLOCPMK: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ELGAMGEDGOM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ELGAMGEDGOM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ELGAMGEDGOM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ELGAMGEDGOM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ELGAMGEDGOM.proto\x1a\x11LMMKDBFDOJF.proto\x1a\x11NCDFMCGOKLH.prot\
    o\"\xb9\x02\n\x0bELGAMGEDGOM\x12\x20\n\x0bKPHGMOIDFND\x18\x08\x20\x01(\
    \x08R\x0bKPHGMOIDFND\x12\x20\n\x0bAONPFEDMDBF\x18\r\x20\x03(\rR\x0bAONPF\
    EDMDBF\x12.\n\x0bNMBOMMJJMMB\x18\x02\x20\x01(\x0b2\x0c.LMMKDBFDOJFR\x0bN\
    MBOMMJJMMB\x12\x20\n\x0bKJKPMFFAMFK\x18\x04\x20\x01(\rR\x0bKJKPMFFAMFK\
    \x12\x20\n\x0bLEJNOLOBOGE\x18\x0c\x20\x01(\rR\x0bLEJNOLOBOGE\x12.\n\x0bD\
    GKAJMOCFIA\x18\x03\x20\x01(\x0b2\x0c.NCDFMCGOKLHR\x0bDGKAJMOCFIA\x12\x20\
    \n\x0bNMGJIBAJGFG\x18\x0e\x20\x01(\x08R\x0bNMGJIBAJGFG\x12\x20\n\x0bHDMF\
    PLOCPMK\x18\x06\x20\x01(\x08R\x0bHDMFPLOCPMKb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::LMMKDBFDOJF::file_descriptor().clone());
            deps.push(super::NCDFMCGOKLH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ELGAMGEDGOM::generated_message_descriptor_data());
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
