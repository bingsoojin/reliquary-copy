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

//! Generated file from `ClockParkGetOngoingScriptInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ClockParkGetOngoingScriptInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClockParkGetOngoingScriptInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.FHFIFPMIJCH)
    pub FHFIFPMIJCH: ::protobuf::MessageField<super::JJIIANKENHO::JJIIANKENHO>,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.LDHEANPNKLN)
    pub LDHEANPNKLN: u32,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.GIGGHHFCEOK)
    pub GIGGHHFCEOK: ::std::string::String,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.HOIMHJHJCKE)
    pub HOIMHJHJCKE: u32,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.COJBEBINCML)
    pub COJBEBINCML: u32,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.GCPAFPNDPDA)
    pub GCPAFPNDPDA: ::protobuf::MessageField<super::EOIAPEFEDGE::EOIAPEFEDGE>,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.GOHGIEMLNOM)
    pub GOHGIEMLNOM: u32,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.OBNMBPEKKIN)
    pub OBNMBPEKKIN: ::protobuf::MessageField<super::PNKLPCLEOFC::PNKLPCLEOFC>,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.AMBCBCLHIHJ)
    pub AMBCBCLHIHJ: u32,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.FEHDAIOJLPN)
    pub FEHDAIOJLPN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ClockParkGetOngoingScriptInfoScRsp.NOHADGIMNIP)
    pub NOHADGIMNIP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ClockParkGetOngoingScriptInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClockParkGetOngoingScriptInfoScRsp {
    fn default() -> &'a ClockParkGetOngoingScriptInfoScRsp {
        <ClockParkGetOngoingScriptInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ClockParkGetOngoingScriptInfoScRsp {
    pub fn new() -> ClockParkGetOngoingScriptInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JJIIANKENHO::JJIIANKENHO>(
            "FHFIFPMIJCH",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.FHFIFPMIJCH },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.FHFIFPMIJCH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LDHEANPNKLN",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.LDHEANPNKLN },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.LDHEANPNKLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GIGGHHFCEOK",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.GIGGHHFCEOK },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.GIGGHHFCEOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HOIMHJHJCKE",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.HOIMHJHJCKE },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.HOIMHJHJCKE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COJBEBINCML",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.COJBEBINCML },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.COJBEBINCML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EOIAPEFEDGE::EOIAPEFEDGE>(
            "GCPAFPNDPDA",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.GCPAFPNDPDA },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.GCPAFPNDPDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GOHGIEMLNOM",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.GOHGIEMLNOM },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.GOHGIEMLNOM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PNKLPCLEOFC::PNKLPCLEOFC>(
            "OBNMBPEKKIN",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.OBNMBPEKKIN },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.OBNMBPEKKIN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AMBCBCLHIHJ",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.AMBCBCLHIHJ },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.AMBCBCLHIHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.retcode },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FEHDAIOJLPN",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.FEHDAIOJLPN },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.FEHDAIOJLPN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NOHADGIMNIP",
            |m: &ClockParkGetOngoingScriptInfoScRsp| { &m.NOHADGIMNIP },
            |m: &mut ClockParkGetOngoingScriptInfoScRsp| { &mut m.NOHADGIMNIP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClockParkGetOngoingScriptInfoScRsp>(
            "ClockParkGetOngoingScriptInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClockParkGetOngoingScriptInfoScRsp {
    const NAME: &'static str = "ClockParkGetOngoingScriptInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FHFIFPMIJCH)?;
                },
                40 => {
                    self.LDHEANPNKLN = is.read_uint32()?;
                },
                66 => {
                    self.GIGGHHFCEOK = is.read_string()?;
                },
                24 => {
                    self.HOIMHJHJCKE = is.read_uint32()?;
                },
                32 => {
                    self.COJBEBINCML = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GCPAFPNDPDA)?;
                },
                96 => {
                    self.GOHGIEMLNOM = is.read_uint32()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OBNMBPEKKIN)?;
                },
                8 => {
                    self.AMBCBCLHIHJ = is.read_uint32()?;
                },
                80 => {
                    self.retcode = is.read_uint32()?;
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.FEHDAIOJLPN)?;
                },
                112 => {
                    self.FEHDAIOJLPN.push(is.read_uint32()?);
                },
                72 => {
                    self.NOHADGIMNIP = is.read_uint32()?;
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
        if let Some(v) = self.FHFIFPMIJCH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LDHEANPNKLN != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.LDHEANPNKLN);
        }
        if !self.GIGGHHFCEOK.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.GIGGHHFCEOK);
        }
        if self.HOIMHJHJCKE != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.HOIMHJHJCKE);
        }
        if self.COJBEBINCML != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.COJBEBINCML);
        }
        if let Some(v) = self.GCPAFPNDPDA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.GOHGIEMLNOM != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.GOHGIEMLNOM);
        }
        if let Some(v) = self.OBNMBPEKKIN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.AMBCBCLHIHJ != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.AMBCBCLHIHJ);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.retcode);
        }
        for value in &self.FEHDAIOJLPN {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        if self.NOHADGIMNIP != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.NOHADGIMNIP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.FHFIFPMIJCH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.LDHEANPNKLN != 0 {
            os.write_uint32(5, self.LDHEANPNKLN)?;
        }
        if !self.GIGGHHFCEOK.is_empty() {
            os.write_string(8, &self.GIGGHHFCEOK)?;
        }
        if self.HOIMHJHJCKE != 0 {
            os.write_uint32(3, self.HOIMHJHJCKE)?;
        }
        if self.COJBEBINCML != 0 {
            os.write_uint32(4, self.COJBEBINCML)?;
        }
        if let Some(v) = self.GCPAFPNDPDA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.GOHGIEMLNOM != 0 {
            os.write_uint32(12, self.GOHGIEMLNOM)?;
        }
        if let Some(v) = self.OBNMBPEKKIN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if self.AMBCBCLHIHJ != 0 {
            os.write_uint32(1, self.AMBCBCLHIHJ)?;
        }
        if self.retcode != 0 {
            os.write_uint32(10, self.retcode)?;
        }
        for v in &self.FEHDAIOJLPN {
            os.write_uint32(14, *v)?;
        };
        if self.NOHADGIMNIP != 0 {
            os.write_uint32(9, self.NOHADGIMNIP)?;
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

    fn new() -> ClockParkGetOngoingScriptInfoScRsp {
        ClockParkGetOngoingScriptInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.FHFIFPMIJCH.clear();
        self.LDHEANPNKLN = 0;
        self.GIGGHHFCEOK.clear();
        self.HOIMHJHJCKE = 0;
        self.COJBEBINCML = 0;
        self.GCPAFPNDPDA.clear();
        self.GOHGIEMLNOM = 0;
        self.OBNMBPEKKIN.clear();
        self.AMBCBCLHIHJ = 0;
        self.retcode = 0;
        self.FEHDAIOJLPN.clear();
        self.NOHADGIMNIP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClockParkGetOngoingScriptInfoScRsp {
        static instance: ClockParkGetOngoingScriptInfoScRsp = ClockParkGetOngoingScriptInfoScRsp {
            FHFIFPMIJCH: ::protobuf::MessageField::none(),
            LDHEANPNKLN: 0,
            GIGGHHFCEOK: ::std::string::String::new(),
            HOIMHJHJCKE: 0,
            COJBEBINCML: 0,
            GCPAFPNDPDA: ::protobuf::MessageField::none(),
            GOHGIEMLNOM: 0,
            OBNMBPEKKIN: ::protobuf::MessageField::none(),
            AMBCBCLHIHJ: 0,
            retcode: 0,
            FEHDAIOJLPN: ::std::vec::Vec::new(),
            NOHADGIMNIP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClockParkGetOngoingScriptInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClockParkGetOngoingScriptInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClockParkGetOngoingScriptInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClockParkGetOngoingScriptInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(ClockParkGetOngoingScriptInfoScRsp.proto\x1a\x11EOIAPEFEDGE.proto\x1a\
    \x11JJIIANKENHO.proto\x1a\x11PNKLPCLEOFC.proto\"\xde\x03\n\"ClockParkGet\
    OngoingScriptInfoScRsp\x12.\n\x0bFHFIFPMIJCH\x18\r\x20\x01(\x0b2\x0c.JJI\
    IANKENHOR\x0bFHFIFPMIJCH\x12\x20\n\x0bLDHEANPNKLN\x18\x05\x20\x01(\rR\
    \x0bLDHEANPNKLN\x12\x20\n\x0bGIGGHHFCEOK\x18\x08\x20\x01(\tR\x0bGIGGHHFC\
    EOK\x12\x20\n\x0bHOIMHJHJCKE\x18\x03\x20\x01(\rR\x0bHOIMHJHJCKE\x12\x20\
    \n\x0bCOJBEBINCML\x18\x04\x20\x01(\rR\x0bCOJBEBINCML\x12.\n\x0bGCPAFPNDP\
    DA\x18\x07\x20\x01(\x0b2\x0c.EOIAPEFEDGER\x0bGCPAFPNDPDA\x12\x20\n\x0bGO\
    HGIEMLNOM\x18\x0c\x20\x01(\rR\x0bGOHGIEMLNOM\x12.\n\x0bOBNMBPEKKIN\x18\
    \x06\x20\x01(\x0b2\x0c.PNKLPCLEOFCR\x0bOBNMBPEKKIN\x12\x20\n\x0bAMBCBCLH\
    IHJ\x18\x01\x20\x01(\rR\x0bAMBCBCLHIHJ\x12\x18\n\x07retcode\x18\n\x20\
    \x01(\rR\x07retcode\x12\x20\n\x0bFEHDAIOJLPN\x18\x0e\x20\x03(\rR\x0bFEHD\
    AIOJLPN\x12\x20\n\x0bNOHADGIMNIP\x18\t\x20\x01(\rR\x0bNOHADGIMNIPb\x06pr\
    oto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::EOIAPEFEDGE::file_descriptor().clone());
            deps.push(super::JJIIANKENHO::file_descriptor().clone());
            deps.push(super::PNKLPCLEOFC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ClockParkGetOngoingScriptInfoScRsp::generated_message_descriptor_data());
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
