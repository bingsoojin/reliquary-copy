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

//! Generated file from `PlayerGetTokenCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PlayerGetTokenCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerGetTokenCsReq {
    // message fields
    // @@protoc_insertion_point(field:PlayerGetTokenCsReq.LBNBDEKPPFN)
    pub LBNBDEKPPFN: u32,
    // @@protoc_insertion_point(field:PlayerGetTokenCsReq.FGOJLPAEJEC)
    pub FGOJLPAEJEC: u32,
    // @@protoc_insertion_point(field:PlayerGetTokenCsReq.CDGPKOMEPOK)
    pub CDGPKOMEPOK: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerGetTokenCsReq.MEMPBKCJJFJ)
    pub MEMPBKCJJFJ: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerGetTokenCsReq.ICMFPNPIJJF)
    pub ICMFPNPIJJF: u32,
    // @@protoc_insertion_point(field:PlayerGetTokenCsReq.BEILKPLGKMN)
    pub BEILKPLGKMN: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerGetTokenCsReq.HAEHHCPOAPP)
    pub HAEHHCPOAPP: u32,
    // @@protoc_insertion_point(field:PlayerGetTokenCsReq.KMONAGFELPG)
    pub KMONAGFELPG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerGetTokenCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerGetTokenCsReq {
    fn default() -> &'a PlayerGetTokenCsReq {
        <PlayerGetTokenCsReq as ::protobuf::Message>::default_instance()
    }
}

impl PlayerGetTokenCsReq {
    pub fn new() -> PlayerGetTokenCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBNBDEKPPFN",
            |m: &PlayerGetTokenCsReq| { &m.LBNBDEKPPFN },
            |m: &mut PlayerGetTokenCsReq| { &mut m.LBNBDEKPPFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGOJLPAEJEC",
            |m: &PlayerGetTokenCsReq| { &m.FGOJLPAEJEC },
            |m: &mut PlayerGetTokenCsReq| { &mut m.FGOJLPAEJEC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CDGPKOMEPOK",
            |m: &PlayerGetTokenCsReq| { &m.CDGPKOMEPOK },
            |m: &mut PlayerGetTokenCsReq| { &mut m.CDGPKOMEPOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MEMPBKCJJFJ",
            |m: &PlayerGetTokenCsReq| { &m.MEMPBKCJJFJ },
            |m: &mut PlayerGetTokenCsReq| { &mut m.MEMPBKCJJFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ICMFPNPIJJF",
            |m: &PlayerGetTokenCsReq| { &m.ICMFPNPIJJF },
            |m: &mut PlayerGetTokenCsReq| { &mut m.ICMFPNPIJJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BEILKPLGKMN",
            |m: &PlayerGetTokenCsReq| { &m.BEILKPLGKMN },
            |m: &mut PlayerGetTokenCsReq| { &mut m.BEILKPLGKMN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HAEHHCPOAPP",
            |m: &PlayerGetTokenCsReq| { &m.HAEHHCPOAPP },
            |m: &mut PlayerGetTokenCsReq| { &mut m.HAEHHCPOAPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KMONAGFELPG",
            |m: &PlayerGetTokenCsReq| { &m.KMONAGFELPG },
            |m: &mut PlayerGetTokenCsReq| { &mut m.KMONAGFELPG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerGetTokenCsReq>(
            "PlayerGetTokenCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerGetTokenCsReq {
    const NAME: &'static str = "PlayerGetTokenCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.LBNBDEKPPFN = is.read_uint32()?;
                },
                48 => {
                    self.FGOJLPAEJEC = is.read_uint32()?;
                },
                74 => {
                    self.CDGPKOMEPOK = is.read_string()?;
                },
                18 => {
                    self.MEMPBKCJJFJ = is.read_string()?;
                },
                64 => {
                    self.ICMFPNPIJJF = is.read_uint32()?;
                },
                26 => {
                    self.BEILKPLGKMN = is.read_string()?;
                },
                88 => {
                    self.HAEHHCPOAPP = is.read_uint32()?;
                },
                32 => {
                    self.KMONAGFELPG = is.read_uint32()?;
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
        if self.LBNBDEKPPFN != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LBNBDEKPPFN);
        }
        if self.FGOJLPAEJEC != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FGOJLPAEJEC);
        }
        if !self.CDGPKOMEPOK.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.CDGPKOMEPOK);
        }
        if !self.MEMPBKCJJFJ.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.MEMPBKCJJFJ);
        }
        if self.ICMFPNPIJJF != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.ICMFPNPIJJF);
        }
        if !self.BEILKPLGKMN.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.BEILKPLGKMN);
        }
        if self.HAEHHCPOAPP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.HAEHHCPOAPP);
        }
        if self.KMONAGFELPG != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.KMONAGFELPG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LBNBDEKPPFN != 0 {
            os.write_uint32(7, self.LBNBDEKPPFN)?;
        }
        if self.FGOJLPAEJEC != 0 {
            os.write_uint32(6, self.FGOJLPAEJEC)?;
        }
        if !self.CDGPKOMEPOK.is_empty() {
            os.write_string(9, &self.CDGPKOMEPOK)?;
        }
        if !self.MEMPBKCJJFJ.is_empty() {
            os.write_string(2, &self.MEMPBKCJJFJ)?;
        }
        if self.ICMFPNPIJJF != 0 {
            os.write_uint32(8, self.ICMFPNPIJJF)?;
        }
        if !self.BEILKPLGKMN.is_empty() {
            os.write_string(3, &self.BEILKPLGKMN)?;
        }
        if self.HAEHHCPOAPP != 0 {
            os.write_uint32(11, self.HAEHHCPOAPP)?;
        }
        if self.KMONAGFELPG != 0 {
            os.write_uint32(4, self.KMONAGFELPG)?;
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

    fn new() -> PlayerGetTokenCsReq {
        PlayerGetTokenCsReq::new()
    }

    fn clear(&mut self) {
        self.LBNBDEKPPFN = 0;
        self.FGOJLPAEJEC = 0;
        self.CDGPKOMEPOK.clear();
        self.MEMPBKCJJFJ.clear();
        self.ICMFPNPIJJF = 0;
        self.BEILKPLGKMN.clear();
        self.HAEHHCPOAPP = 0;
        self.KMONAGFELPG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerGetTokenCsReq {
        static instance: PlayerGetTokenCsReq = PlayerGetTokenCsReq {
            LBNBDEKPPFN: 0,
            FGOJLPAEJEC: 0,
            CDGPKOMEPOK: ::std::string::String::new(),
            MEMPBKCJJFJ: ::std::string::String::new(),
            ICMFPNPIJJF: 0,
            BEILKPLGKMN: ::std::string::String::new(),
            HAEHHCPOAPP: 0,
            KMONAGFELPG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerGetTokenCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerGetTokenCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerGetTokenCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerGetTokenCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19PlayerGetTokenCsReq.proto\"\xa5\x02\n\x13PlayerGetTokenCsReq\x12\
    \x20\n\x0bLBNBDEKPPFN\x18\x07\x20\x01(\rR\x0bLBNBDEKPPFN\x12\x20\n\x0bFG\
    OJLPAEJEC\x18\x06\x20\x01(\rR\x0bFGOJLPAEJEC\x12\x20\n\x0bCDGPKOMEPOK\
    \x18\t\x20\x01(\tR\x0bCDGPKOMEPOK\x12\x20\n\x0bMEMPBKCJJFJ\x18\x02\x20\
    \x01(\tR\x0bMEMPBKCJJFJ\x12\x20\n\x0bICMFPNPIJJF\x18\x08\x20\x01(\rR\x0b\
    ICMFPNPIJJF\x12\x20\n\x0bBEILKPLGKMN\x18\x03\x20\x01(\tR\x0bBEILKPLGKMN\
    \x12\x20\n\x0bHAEHHCPOAPP\x18\x0b\x20\x01(\rR\x0bHAEHHCPOAPP\x12\x20\n\
    \x0bKMONAGFELPG\x18\x04\x20\x01(\rR\x0bKMONAGFELPGb\x06proto3\
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
            messages.push(PlayerGetTokenCsReq::generated_message_descriptor_data());
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
