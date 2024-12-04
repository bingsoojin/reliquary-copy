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

//! Generated file from `AlleyPlacingGameScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AlleyPlacingGameScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AlleyPlacingGameScRsp {
    // message fields
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.FKMOJLILEDA)
    pub FKMOJLILEDA: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.PNNFEJILDKD)
    pub PNNFEJILDKD: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.LDJOPBIMDDM)
    pub LDJOPBIMDDM: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.GAABGDGLDAB)
    pub GAABGDGLDAB: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.PCBPAMGCIGG)
    pub PCBPAMGCIGG: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.IAMJMDNOFCO)
    pub IAMJMDNOFCO: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.DDNLDMOBGPC)
    pub DDNLDMOBGPC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AlleyPlacingGameScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AlleyPlacingGameScRsp {
    fn default() -> &'a AlleyPlacingGameScRsp {
        <AlleyPlacingGameScRsp as ::protobuf::Message>::default_instance()
    }
}

impl AlleyPlacingGameScRsp {
    pub fn new() -> AlleyPlacingGameScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKMOJLILEDA",
            |m: &AlleyPlacingGameScRsp| { &m.FKMOJLILEDA },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.FKMOJLILEDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &AlleyPlacingGameScRsp| { &m.ADADHIHDHJC },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PNNFEJILDKD",
            |m: &AlleyPlacingGameScRsp| { &m.PNNFEJILDKD },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.PNNFEJILDKD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LDJOPBIMDDM",
            |m: &AlleyPlacingGameScRsp| { &m.LDJOPBIMDDM },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.LDJOPBIMDDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GAABGDGLDAB",
            |m: &AlleyPlacingGameScRsp| { &m.GAABGDGLDAB },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.GAABGDGLDAB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PCBPAMGCIGG",
            |m: &AlleyPlacingGameScRsp| { &m.PCBPAMGCIGG },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.PCBPAMGCIGG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IAMJMDNOFCO",
            |m: &AlleyPlacingGameScRsp| { &m.IAMJMDNOFCO },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.IAMJMDNOFCO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDNLDMOBGPC",
            |m: &AlleyPlacingGameScRsp| { &m.DDNLDMOBGPC },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.DDNLDMOBGPC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AlleyPlacingGameScRsp>(
            "AlleyPlacingGameScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AlleyPlacingGameScRsp {
    const NAME: &'static str = "AlleyPlacingGameScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.FKMOJLILEDA = is.read_uint32()?;
                },
                64 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                40 => {
                    self.PNNFEJILDKD = is.read_uint32()?;
                },
                56 => {
                    self.LDJOPBIMDDM = is.read_uint32()?;
                },
                104 => {
                    self.GAABGDGLDAB = is.read_uint32()?;
                },
                112 => {
                    self.PCBPAMGCIGG = is.read_uint32()?;
                },
                32 => {
                    self.IAMJMDNOFCO = is.read_uint32()?;
                },
                16 => {
                    self.DDNLDMOBGPC = is.read_uint32()?;
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
        if self.FKMOJLILEDA != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.FKMOJLILEDA);
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.ADADHIHDHJC);
        }
        if self.PNNFEJILDKD != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.PNNFEJILDKD);
        }
        if self.LDJOPBIMDDM != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LDJOPBIMDDM);
        }
        if self.GAABGDGLDAB != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.GAABGDGLDAB);
        }
        if self.PCBPAMGCIGG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PCBPAMGCIGG);
        }
        if self.IAMJMDNOFCO != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.IAMJMDNOFCO);
        }
        if self.DDNLDMOBGPC != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.DDNLDMOBGPC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FKMOJLILEDA != 0 {
            os.write_uint32(3, self.FKMOJLILEDA)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(8, self.ADADHIHDHJC)?;
        }
        if self.PNNFEJILDKD != 0 {
            os.write_uint32(5, self.PNNFEJILDKD)?;
        }
        if self.LDJOPBIMDDM != 0 {
            os.write_uint32(7, self.LDJOPBIMDDM)?;
        }
        if self.GAABGDGLDAB != 0 {
            os.write_uint32(13, self.GAABGDGLDAB)?;
        }
        if self.PCBPAMGCIGG != 0 {
            os.write_uint32(14, self.PCBPAMGCIGG)?;
        }
        if self.IAMJMDNOFCO != 0 {
            os.write_uint32(4, self.IAMJMDNOFCO)?;
        }
        if self.DDNLDMOBGPC != 0 {
            os.write_uint32(2, self.DDNLDMOBGPC)?;
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

    fn new() -> AlleyPlacingGameScRsp {
        AlleyPlacingGameScRsp::new()
    }

    fn clear(&mut self) {
        self.FKMOJLILEDA = 0;
        self.ADADHIHDHJC = 0;
        self.PNNFEJILDKD = 0;
        self.LDJOPBIMDDM = 0;
        self.GAABGDGLDAB = 0;
        self.PCBPAMGCIGG = 0;
        self.IAMJMDNOFCO = 0;
        self.DDNLDMOBGPC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AlleyPlacingGameScRsp {
        static instance: AlleyPlacingGameScRsp = AlleyPlacingGameScRsp {
            FKMOJLILEDA: 0,
            ADADHIHDHJC: 0,
            PNNFEJILDKD: 0,
            LDJOPBIMDDM: 0,
            GAABGDGLDAB: 0,
            PCBPAMGCIGG: 0,
            IAMJMDNOFCO: 0,
            DDNLDMOBGPC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AlleyPlacingGameScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AlleyPlacingGameScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AlleyPlacingGameScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlleyPlacingGameScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bAlleyPlacingGameScRsp.proto\"\xa7\x02\n\x15AlleyPlacingGameScRsp\
    \x12\x20\n\x0bFKMOJLILEDA\x18\x03\x20\x01(\rR\x0bFKMOJLILEDA\x12\x20\n\
    \x0bADADHIHDHJC\x18\x08\x20\x01(\rR\x0bADADHIHDHJC\x12\x20\n\x0bPNNFEJIL\
    DKD\x18\x05\x20\x01(\rR\x0bPNNFEJILDKD\x12\x20\n\x0bLDJOPBIMDDM\x18\x07\
    \x20\x01(\rR\x0bLDJOPBIMDDM\x12\x20\n\x0bGAABGDGLDAB\x18\r\x20\x01(\rR\
    \x0bGAABGDGLDAB\x12\x20\n\x0bPCBPAMGCIGG\x18\x0e\x20\x01(\rR\x0bPCBPAMGC\
    IGG\x12\x20\n\x0bIAMJMDNOFCO\x18\x04\x20\x01(\rR\x0bIAMJMDNOFCO\x12\x20\
    \n\x0bDDNLDMOBGPC\x18\x02\x20\x01(\rR\x0bDDNLDMOBGPCb\x06proto3\
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
            messages.push(AlleyPlacingGameScRsp::generated_message_descriptor_data());
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
