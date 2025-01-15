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

//! Generated file from `BKNHOHPNCLG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BKNHOHPNCLG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BKNHOHPNCLG {
    // message fields
    // @@protoc_insertion_point(field:BKNHOHPNCLG.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:BKNHOHPNCLG.HOCIBKAEDLL)
    pub HOCIBKAEDLL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BKNHOHPNCLG.IOOGGEHHMCG)
    pub IOOGGEHHMCG: u32,
    // @@protoc_insertion_point(field:BKNHOHPNCLG.BCNLJNJKOBN)
    pub BCNLJNJKOBN: f64,
    // @@protoc_insertion_point(field:BKNHOHPNCLG.EHJHNCJNMDO)
    pub EHJHNCJNMDO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BKNHOHPNCLG.FONPDAELOFA)
    pub FONPDAELOFA: i32,
    // @@protoc_insertion_point(field:BKNHOHPNCLG.JHOFCLMCHJM)
    pub JHOFCLMCHJM: f64,
    // @@protoc_insertion_point(field:BKNHOHPNCLG.GDHMPKIAEGO)
    pub GDHMPKIAEGO: u32,
    // @@protoc_insertion_point(field:BKNHOHPNCLG.LFGKKOHHIGE)
    pub LFGKKOHHIGE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BKNHOHPNCLG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BKNHOHPNCLG {
    fn default() -> &'a BKNHOHPNCLG {
        <BKNHOHPNCLG as ::protobuf::Message>::default_instance()
    }
}

impl BKNHOHPNCLG {
    pub fn new() -> BKNHOHPNCLG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &BKNHOHPNCLG| { &m.avatar_id },
            |m: &mut BKNHOHPNCLG| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HOCIBKAEDLL",
            |m: &BKNHOHPNCLG| { &m.HOCIBKAEDLL },
            |m: &mut BKNHOHPNCLG| { &mut m.HOCIBKAEDLL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOOGGEHHMCG",
            |m: &BKNHOHPNCLG| { &m.IOOGGEHHMCG },
            |m: &mut BKNHOHPNCLG| { &mut m.IOOGGEHHMCG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BCNLJNJKOBN",
            |m: &BKNHOHPNCLG| { &m.BCNLJNJKOBN },
            |m: &mut BKNHOHPNCLG| { &mut m.BCNLJNJKOBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EHJHNCJNMDO",
            |m: &BKNHOHPNCLG| { &m.EHJHNCJNMDO },
            |m: &mut BKNHOHPNCLG| { &mut m.EHJHNCJNMDO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FONPDAELOFA",
            |m: &BKNHOHPNCLG| { &m.FONPDAELOFA },
            |m: &mut BKNHOHPNCLG| { &mut m.FONPDAELOFA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JHOFCLMCHJM",
            |m: &BKNHOHPNCLG| { &m.JHOFCLMCHJM },
            |m: &mut BKNHOHPNCLG| { &mut m.JHOFCLMCHJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GDHMPKIAEGO",
            |m: &BKNHOHPNCLG| { &m.GDHMPKIAEGO },
            |m: &mut BKNHOHPNCLG| { &mut m.GDHMPKIAEGO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFGKKOHHIGE",
            |m: &BKNHOHPNCLG| { &m.LFGKKOHHIGE },
            |m: &mut BKNHOHPNCLG| { &mut m.LFGKKOHHIGE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BKNHOHPNCLG>(
            "BKNHOHPNCLG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BKNHOHPNCLG {
    const NAME: &'static str = "BKNHOHPNCLG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.avatar_id = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.HOCIBKAEDLL)?;
                },
                16 => {
                    self.HOCIBKAEDLL.push(is.read_uint32()?);
                },
                24 => {
                    self.IOOGGEHHMCG = is.read_uint32()?;
                },
                33 => {
                    self.BCNLJNJKOBN = is.read_double()?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.EHJHNCJNMDO)?;
                },
                40 => {
                    self.EHJHNCJNMDO.push(is.read_uint32()?);
                },
                48 => {
                    self.FONPDAELOFA = is.read_int32()?;
                },
                57 => {
                    self.JHOFCLMCHJM = is.read_double()?;
                },
                64 => {
                    self.GDHMPKIAEGO = is.read_uint32()?;
                },
                72 => {
                    self.LFGKKOHHIGE = is.read_uint32()?;
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
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.avatar_id);
        }
        for value in &self.HOCIBKAEDLL {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.IOOGGEHHMCG != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.IOOGGEHHMCG);
        }
        if self.BCNLJNJKOBN != 0. {
            my_size += 1 + 8;
        }
        for value in &self.EHJHNCJNMDO {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if self.FONPDAELOFA != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.FONPDAELOFA);
        }
        if self.JHOFCLMCHJM != 0. {
            my_size += 1 + 8;
        }
        if self.GDHMPKIAEGO != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.GDHMPKIAEGO);
        }
        if self.LFGKKOHHIGE != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.LFGKKOHHIGE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(1, self.avatar_id)?;
        }
        for v in &self.HOCIBKAEDLL {
            os.write_uint32(2, *v)?;
        };
        if self.IOOGGEHHMCG != 0 {
            os.write_uint32(3, self.IOOGGEHHMCG)?;
        }
        if self.BCNLJNJKOBN != 0. {
            os.write_double(4, self.BCNLJNJKOBN)?;
        }
        for v in &self.EHJHNCJNMDO {
            os.write_uint32(5, *v)?;
        };
        if self.FONPDAELOFA != 0 {
            os.write_int32(6, self.FONPDAELOFA)?;
        }
        if self.JHOFCLMCHJM != 0. {
            os.write_double(7, self.JHOFCLMCHJM)?;
        }
        if self.GDHMPKIAEGO != 0 {
            os.write_uint32(8, self.GDHMPKIAEGO)?;
        }
        if self.LFGKKOHHIGE != 0 {
            os.write_uint32(9, self.LFGKKOHHIGE)?;
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

    fn new() -> BKNHOHPNCLG {
        BKNHOHPNCLG::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.HOCIBKAEDLL.clear();
        self.IOOGGEHHMCG = 0;
        self.BCNLJNJKOBN = 0.;
        self.EHJHNCJNMDO.clear();
        self.FONPDAELOFA = 0;
        self.JHOFCLMCHJM = 0.;
        self.GDHMPKIAEGO = 0;
        self.LFGKKOHHIGE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BKNHOHPNCLG {
        static instance: BKNHOHPNCLG = BKNHOHPNCLG {
            avatar_id: 0,
            HOCIBKAEDLL: ::std::vec::Vec::new(),
            IOOGGEHHMCG: 0,
            BCNLJNJKOBN: 0.,
            EHJHNCJNMDO: ::std::vec::Vec::new(),
            FONPDAELOFA: 0,
            JHOFCLMCHJM: 0.,
            GDHMPKIAEGO: 0,
            LFGKKOHHIGE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BKNHOHPNCLG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BKNHOHPNCLG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BKNHOHPNCLG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BKNHOHPNCLG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BKNHOHPNCLG.proto\"\xba\x02\n\x0bBKNHOHPNCLG\x12\x1b\n\tavatar_id\
    \x18\x01\x20\x01(\rR\x08avatarId\x12\x20\n\x0bHOCIBKAEDLL\x18\x02\x20\
    \x03(\rR\x0bHOCIBKAEDLL\x12\x20\n\x0bIOOGGEHHMCG\x18\x03\x20\x01(\rR\x0b\
    IOOGGEHHMCG\x12\x20\n\x0bBCNLJNJKOBN\x18\x04\x20\x01(\x01R\x0bBCNLJNJKOB\
    N\x12\x20\n\x0bEHJHNCJNMDO\x18\x05\x20\x03(\rR\x0bEHJHNCJNMDO\x12\x20\n\
    \x0bFONPDAELOFA\x18\x06\x20\x01(\x05R\x0bFONPDAELOFA\x12\x20\n\x0bJHOFCL\
    MCHJM\x18\x07\x20\x01(\x01R\x0bJHOFCLMCHJM\x12\x20\n\x0bGDHMPKIAEGO\x18\
    \x08\x20\x01(\rR\x0bGDHMPKIAEGO\x12\x20\n\x0bLFGKKOHHIGE\x18\t\x20\x01(\
    \rR\x0bLFGKKOHHIGEb\x06proto3\
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
            messages.push(BKNHOHPNCLG::generated_message_descriptor_data());
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
