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

//! Generated file from `NOMDNDHHELA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NOMDNDHHELA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NOMDNDHHELA {
    // message fields
    // @@protoc_insertion_point(field:NOMDNDHHELA.JMELHEKBPBL)
    pub JMELHEKBPBL: ::protobuf::MessageField<super::GFMELEMCJNA::GFMELEMCJNA>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.EELBEPMNPGI)
    pub EELBEPMNPGI: ::protobuf::MessageField<super::MICALHHIDNN::MICALHHIDNN>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.MMOHLLKALAK)
    pub MMOHLLKALAK: ::protobuf::MessageField<super::NGNPFHGBPLD::NGNPFHGBPLD>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.CMOOKCLJDGL)
    pub CMOOKCLJDGL: ::protobuf::MessageField<super::NGEJECDNAEO::NGEJECDNAEO>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.LMPNOCEABNH)
    pub LMPNOCEABNH: ::protobuf::MessageField<super::LOJNAEKIENP::LOJNAEKIENP>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.IJOPANKCEKJ)
    pub IJOPANKCEKJ: ::protobuf::MessageField<super::HBADJJHCENK::HBADJJHCENK>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.LCFANOLIPIJ)
    pub LCFANOLIPIJ: ::protobuf::MessageField<super::DEFIBAENEFF::DEFIBAENEFF>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.JMNGDHGBOFL)
    pub JMNGDHGBOFL: ::protobuf::MessageField<super::IFIMEHHNAPL::IFIMEHHNAPL>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.DNCPGOIAJME)
    pub DNCPGOIAJME: ::protobuf::MessageField<super::OAGDODGDCJF::OAGDODGDCJF>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.HDADNBFLOCK)
    pub HDADNBFLOCK: ::protobuf::MessageField<super::PKPKDHENPMN::PKPKDHENPMN>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.OBNMBPEKKIN)
    pub OBNMBPEKKIN: ::protobuf::MessageField<super::EAAMMPJFKIB::EAAMMPJFKIB>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.HCOAPFAGBEI)
    pub HCOAPFAGBEI: ::std::vec::Vec<super::IDONDCHFKHG::IDONDCHFKHG>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.AGOHHBNKHIJ)
    pub AGOHHBNKHIJ: ::protobuf::MessageField<super::LLOBBCHJGDO::LLOBBCHJGDO>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.MJKPCDJJEML)
    pub MJKPCDJJEML: ::protobuf::MessageField<super::BLIMEDCMHMN::BLIMEDCMHMN>,
    // @@protoc_insertion_point(field:NOMDNDHHELA.NKPDMCKFPPP)
    pub NKPDMCKFPPP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NOMDNDHHELA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NOMDNDHHELA {
    fn default() -> &'a NOMDNDHHELA {
        <NOMDNDHHELA as ::protobuf::Message>::default_instance()
    }
}

impl NOMDNDHHELA {
    pub fn new() -> NOMDNDHHELA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(15);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GFMELEMCJNA::GFMELEMCJNA>(
            "JMELHEKBPBL",
            |m: &NOMDNDHHELA| { &m.JMELHEKBPBL },
            |m: &mut NOMDNDHHELA| { &mut m.JMELHEKBPBL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MICALHHIDNN::MICALHHIDNN>(
            "EELBEPMNPGI",
            |m: &NOMDNDHHELA| { &m.EELBEPMNPGI },
            |m: &mut NOMDNDHHELA| { &mut m.EELBEPMNPGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NGNPFHGBPLD::NGNPFHGBPLD>(
            "MMOHLLKALAK",
            |m: &NOMDNDHHELA| { &m.MMOHLLKALAK },
            |m: &mut NOMDNDHHELA| { &mut m.MMOHLLKALAK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NGEJECDNAEO::NGEJECDNAEO>(
            "CMOOKCLJDGL",
            |m: &NOMDNDHHELA| { &m.CMOOKCLJDGL },
            |m: &mut NOMDNDHHELA| { &mut m.CMOOKCLJDGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LOJNAEKIENP::LOJNAEKIENP>(
            "LMPNOCEABNH",
            |m: &NOMDNDHHELA| { &m.LMPNOCEABNH },
            |m: &mut NOMDNDHHELA| { &mut m.LMPNOCEABNH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HBADJJHCENK::HBADJJHCENK>(
            "IJOPANKCEKJ",
            |m: &NOMDNDHHELA| { &m.IJOPANKCEKJ },
            |m: &mut NOMDNDHHELA| { &mut m.IJOPANKCEKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DEFIBAENEFF::DEFIBAENEFF>(
            "LCFANOLIPIJ",
            |m: &NOMDNDHHELA| { &m.LCFANOLIPIJ },
            |m: &mut NOMDNDHHELA| { &mut m.LCFANOLIPIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IFIMEHHNAPL::IFIMEHHNAPL>(
            "JMNGDHGBOFL",
            |m: &NOMDNDHHELA| { &m.JMNGDHGBOFL },
            |m: &mut NOMDNDHHELA| { &mut m.JMNGDHGBOFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OAGDODGDCJF::OAGDODGDCJF>(
            "DNCPGOIAJME",
            |m: &NOMDNDHHELA| { &m.DNCPGOIAJME },
            |m: &mut NOMDNDHHELA| { &mut m.DNCPGOIAJME },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PKPKDHENPMN::PKPKDHENPMN>(
            "HDADNBFLOCK",
            |m: &NOMDNDHHELA| { &m.HDADNBFLOCK },
            |m: &mut NOMDNDHHELA| { &mut m.HDADNBFLOCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EAAMMPJFKIB::EAAMMPJFKIB>(
            "OBNMBPEKKIN",
            |m: &NOMDNDHHELA| { &m.OBNMBPEKKIN },
            |m: &mut NOMDNDHHELA| { &mut m.OBNMBPEKKIN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HCOAPFAGBEI",
            |m: &NOMDNDHHELA| { &m.HCOAPFAGBEI },
            |m: &mut NOMDNDHHELA| { &mut m.HCOAPFAGBEI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LLOBBCHJGDO::LLOBBCHJGDO>(
            "AGOHHBNKHIJ",
            |m: &NOMDNDHHELA| { &m.AGOHHBNKHIJ },
            |m: &mut NOMDNDHHELA| { &mut m.AGOHHBNKHIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BLIMEDCMHMN::BLIMEDCMHMN>(
            "MJKPCDJJEML",
            |m: &NOMDNDHHELA| { &m.MJKPCDJJEML },
            |m: &mut NOMDNDHHELA| { &mut m.MJKPCDJJEML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NKPDMCKFPPP",
            |m: &NOMDNDHHELA| { &m.NKPDMCKFPPP },
            |m: &mut NOMDNDHHELA| { &mut m.NKPDMCKFPPP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NOMDNDHHELA>(
            "NOMDNDHHELA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NOMDNDHHELA {
    const NAME: &'static str = "NOMDNDHHELA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JMELHEKBPBL)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EELBEPMNPGI)?;
                },
                1418 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MMOHLLKALAK)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CMOOKCLJDGL)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LMPNOCEABNH)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IJOPANKCEKJ)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LCFANOLIPIJ)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JMNGDHGBOFL)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DNCPGOIAJME)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HDADNBFLOCK)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OBNMBPEKKIN)?;
                },
                98 => {
                    self.HCOAPFAGBEI.push(is.read_message()?);
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AGOHHBNKHIJ)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MJKPCDJJEML)?;
                },
                120 => {
                    self.NKPDMCKFPPP = is.read_uint32()?;
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
        if let Some(v) = self.JMELHEKBPBL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EELBEPMNPGI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.MMOHLLKALAK.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.CMOOKCLJDGL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.LMPNOCEABNH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.IJOPANKCEKJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.LCFANOLIPIJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.JMNGDHGBOFL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.DNCPGOIAJME.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.HDADNBFLOCK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.OBNMBPEKKIN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.HCOAPFAGBEI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.AGOHHBNKHIJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.MJKPCDJJEML.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NKPDMCKFPPP != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.NKPDMCKFPPP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.JMELHEKBPBL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.EELBEPMNPGI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.MMOHLLKALAK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(177, v, os)?;
        }
        if let Some(v) = self.CMOOKCLJDGL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.LMPNOCEABNH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.IJOPANKCEKJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.LCFANOLIPIJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.JMNGDHGBOFL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.DNCPGOIAJME.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.HDADNBFLOCK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.OBNMBPEKKIN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        for v in &self.HCOAPFAGBEI {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if let Some(v) = self.AGOHHBNKHIJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.MJKPCDJJEML.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.NKPDMCKFPPP != 0 {
            os.write_uint32(15, self.NKPDMCKFPPP)?;
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

    fn new() -> NOMDNDHHELA {
        NOMDNDHHELA::new()
    }

    fn clear(&mut self) {
        self.JMELHEKBPBL.clear();
        self.EELBEPMNPGI.clear();
        self.MMOHLLKALAK.clear();
        self.CMOOKCLJDGL.clear();
        self.LMPNOCEABNH.clear();
        self.IJOPANKCEKJ.clear();
        self.LCFANOLIPIJ.clear();
        self.JMNGDHGBOFL.clear();
        self.DNCPGOIAJME.clear();
        self.HDADNBFLOCK.clear();
        self.OBNMBPEKKIN.clear();
        self.HCOAPFAGBEI.clear();
        self.AGOHHBNKHIJ.clear();
        self.MJKPCDJJEML.clear();
        self.NKPDMCKFPPP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NOMDNDHHELA {
        static instance: NOMDNDHHELA = NOMDNDHHELA {
            JMELHEKBPBL: ::protobuf::MessageField::none(),
            EELBEPMNPGI: ::protobuf::MessageField::none(),
            MMOHLLKALAK: ::protobuf::MessageField::none(),
            CMOOKCLJDGL: ::protobuf::MessageField::none(),
            LMPNOCEABNH: ::protobuf::MessageField::none(),
            IJOPANKCEKJ: ::protobuf::MessageField::none(),
            LCFANOLIPIJ: ::protobuf::MessageField::none(),
            JMNGDHGBOFL: ::protobuf::MessageField::none(),
            DNCPGOIAJME: ::protobuf::MessageField::none(),
            HDADNBFLOCK: ::protobuf::MessageField::none(),
            OBNMBPEKKIN: ::protobuf::MessageField::none(),
            HCOAPFAGBEI: ::std::vec::Vec::new(),
            AGOHHBNKHIJ: ::protobuf::MessageField::none(),
            MJKPCDJJEML: ::protobuf::MessageField::none(),
            NKPDMCKFPPP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NOMDNDHHELA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NOMDNDHHELA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NOMDNDHHELA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NOMDNDHHELA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NOMDNDHHELA.proto\x1a\x11BLIMEDCMHMN.proto\x1a\x11DEFIBAENEFF.prot\
    o\x1a\x11EAAMMPJFKIB.proto\x1a\x11GFMELEMCJNA.proto\x1a\x11HBADJJHCENK.p\
    roto\x1a\x11IDONDCHFKHG.proto\x1a\x11IFIMEHHNAPL.proto\x1a\x11LLOBBCHJGD\
    O.proto\x1a\x11LOJNAEKIENP.proto\x1a\x11MICALHHIDNN.proto\x1a\x11NGEJECD\
    NAEO.proto\x1a\x11NGNPFHGBPLD.proto\x1a\x11OAGDODGDCJF.proto\x1a\x11PKPK\
    DHENPMN.proto\"\xd0\x05\n\x0bNOMDNDHHELA\x12.\n\x0bJMELHEKBPBL\x18\x01\
    \x20\x01(\x0b2\x0c.GFMELEMCJNAR\x0bJMELHEKBPBL\x12.\n\x0bEELBEPMNPGI\x18\
    \x05\x20\x01(\x0b2\x0c.MICALHHIDNNR\x0bEELBEPMNPGI\x12/\n\x0bMMOHLLKALAK\
    \x18\xb1\x01\x20\x01(\x0b2\x0c.NGNPFHGBPLDR\x0bMMOHLLKALAK\x12.\n\x0bCMO\
    OKCLJDGL\x18\t\x20\x01(\x0b2\x0c.NGEJECDNAEOR\x0bCMOOKCLJDGL\x12.\n\x0bL\
    MPNOCEABNH\x18\x02\x20\x01(\x0b2\x0c.LOJNAEKIENPR\x0bLMPNOCEABNH\x12.\n\
    \x0bIJOPANKCEKJ\x18\n\x20\x01(\x0b2\x0c.HBADJJHCENKR\x0bIJOPANKCEKJ\x12.\
    \n\x0bLCFANOLIPIJ\x18\x06\x20\x01(\x0b2\x0c.DEFIBAENEFFR\x0bLCFANOLIPIJ\
    \x12.\n\x0bJMNGDHGBOFL\x18\x07\x20\x01(\x0b2\x0c.IFIMEHHNAPLR\x0bJMNGDHG\
    BOFL\x12.\n\x0bDNCPGOIAJME\x18\x0b\x20\x01(\x0b2\x0c.OAGDODGDCJFR\x0bDNC\
    PGOIAJME\x12.\n\x0bHDADNBFLOCK\x18\x04\x20\x01(\x0b2\x0c.PKPKDHENPMNR\
    \x0bHDADNBFLOCK\x12.\n\x0bOBNMBPEKKIN\x18\x08\x20\x01(\x0b2\x0c.EAAMMPJF\
    KIBR\x0bOBNMBPEKKIN\x12.\n\x0bHCOAPFAGBEI\x18\x0c\x20\x03(\x0b2\x0c.IDON\
    DCHFKHGR\x0bHCOAPFAGBEI\x12.\n\x0bAGOHHBNKHIJ\x18\x03\x20\x01(\x0b2\x0c.\
    LLOBBCHJGDOR\x0bAGOHHBNKHIJ\x12.\n\x0bMJKPCDJJEML\x18\r\x20\x01(\x0b2\
    \x0c.BLIMEDCMHMNR\x0bMJKPCDJJEML\x12\x20\n\x0bNKPDMCKFPPP\x18\x0f\x20\
    \x01(\rR\x0bNKPDMCKFPPPb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(14);
            deps.push(super::BLIMEDCMHMN::file_descriptor().clone());
            deps.push(super::DEFIBAENEFF::file_descriptor().clone());
            deps.push(super::EAAMMPJFKIB::file_descriptor().clone());
            deps.push(super::GFMELEMCJNA::file_descriptor().clone());
            deps.push(super::HBADJJHCENK::file_descriptor().clone());
            deps.push(super::IDONDCHFKHG::file_descriptor().clone());
            deps.push(super::IFIMEHHNAPL::file_descriptor().clone());
            deps.push(super::LLOBBCHJGDO::file_descriptor().clone());
            deps.push(super::LOJNAEKIENP::file_descriptor().clone());
            deps.push(super::MICALHHIDNN::file_descriptor().clone());
            deps.push(super::NGEJECDNAEO::file_descriptor().clone());
            deps.push(super::NGNPFHGBPLD::file_descriptor().clone());
            deps.push(super::OAGDODGDCJF::file_descriptor().clone());
            deps.push(super::PKPKDHENPMN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NOMDNDHHELA::generated_message_descriptor_data());
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
