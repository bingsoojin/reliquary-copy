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

//! Generated file from `EPEDODHJOJF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EPEDODHJOJF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EPEDODHJOJF {
    // message fields
    // @@protoc_insertion_point(field:EPEDODHJOJF.BKHFBLBIOEG)
    pub BKHFBLBIOEG: ::protobuf::MessageField<super::BFIDNBACIPF::BFIDNBACIPF>,
    // @@protoc_insertion_point(field:EPEDODHJOJF.CDFOAECLKDP)
    pub CDFOAECLKDP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EPEDODHJOJF.LOOPEOMDNOH)
    pub LOOPEOMDNOH: ::protobuf::MessageField<super::CGMDBEIHMPK::CGMDBEIHMPK>,
    // @@protoc_insertion_point(field:EPEDODHJOJF.NCPHFIDNGHE)
    pub NCPHFIDNGHE: u32,
    // @@protoc_insertion_point(field:EPEDODHJOJF.KIKJKIACENH)
    pub KIKJKIACENH: ::protobuf::MessageField<super::MFKFODKMENJ::MFKFODKMENJ>,
    // @@protoc_insertion_point(field:EPEDODHJOJF.AMLCAEIJMEE)
    pub AMLCAEIJMEE: ::protobuf::MessageField<super::FAIFHIBBNAO::FAIFHIBBNAO>,
    // @@protoc_insertion_point(field:EPEDODHJOJF.KKIKGPPKHIP)
    pub KKIKGPPKHIP: ::protobuf::MessageField<super::LMHKCAHMNPE::LMHKCAHMNPE>,
    // @@protoc_insertion_point(field:EPEDODHJOJF.PAIAHLNACGC)
    pub PAIAHLNACGC: ::protobuf::MessageField<super::OOPKDEMPMFI::OOPKDEMPMFI>,
    // special fields
    // @@protoc_insertion_point(special_field:EPEDODHJOJF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EPEDODHJOJF {
    fn default() -> &'a EPEDODHJOJF {
        <EPEDODHJOJF as ::protobuf::Message>::default_instance()
    }
}

impl EPEDODHJOJF {
    pub fn new() -> EPEDODHJOJF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BFIDNBACIPF::BFIDNBACIPF>(
            "BKHFBLBIOEG",
            |m: &EPEDODHJOJF| { &m.BKHFBLBIOEG },
            |m: &mut EPEDODHJOJF| { &mut m.BKHFBLBIOEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CDFOAECLKDP",
            |m: &EPEDODHJOJF| { &m.CDFOAECLKDP },
            |m: &mut EPEDODHJOJF| { &mut m.CDFOAECLKDP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CGMDBEIHMPK::CGMDBEIHMPK>(
            "LOOPEOMDNOH",
            |m: &EPEDODHJOJF| { &m.LOOPEOMDNOH },
            |m: &mut EPEDODHJOJF| { &mut m.LOOPEOMDNOH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCPHFIDNGHE",
            |m: &EPEDODHJOJF| { &m.NCPHFIDNGHE },
            |m: &mut EPEDODHJOJF| { &mut m.NCPHFIDNGHE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MFKFODKMENJ::MFKFODKMENJ>(
            "KIKJKIACENH",
            |m: &EPEDODHJOJF| { &m.KIKJKIACENH },
            |m: &mut EPEDODHJOJF| { &mut m.KIKJKIACENH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FAIFHIBBNAO::FAIFHIBBNAO>(
            "AMLCAEIJMEE",
            |m: &EPEDODHJOJF| { &m.AMLCAEIJMEE },
            |m: &mut EPEDODHJOJF| { &mut m.AMLCAEIJMEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LMHKCAHMNPE::LMHKCAHMNPE>(
            "KKIKGPPKHIP",
            |m: &EPEDODHJOJF| { &m.KKIKGPPKHIP },
            |m: &mut EPEDODHJOJF| { &mut m.KKIKGPPKHIP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OOPKDEMPMFI::OOPKDEMPMFI>(
            "PAIAHLNACGC",
            |m: &EPEDODHJOJF| { &m.PAIAHLNACGC },
            |m: &mut EPEDODHJOJF| { &mut m.PAIAHLNACGC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EPEDODHJOJF>(
            "EPEDODHJOJF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EPEDODHJOJF {
    const NAME: &'static str = "EPEDODHJOJF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BKHFBLBIOEG)?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.CDFOAECLKDP)?;
                },
                104 => {
                    self.CDFOAECLKDP.push(is.read_uint32()?);
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LOOPEOMDNOH)?;
                },
                112 => {
                    self.NCPHFIDNGHE = is.read_uint32()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KIKJKIACENH)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AMLCAEIJMEE)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KKIKGPPKHIP)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PAIAHLNACGC)?;
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
        if let Some(v) = self.BKHFBLBIOEG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.CDFOAECLKDP {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if let Some(v) = self.LOOPEOMDNOH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NCPHFIDNGHE != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.NCPHFIDNGHE);
        }
        if let Some(v) = self.KIKJKIACENH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.AMLCAEIJMEE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KKIKGPPKHIP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.PAIAHLNACGC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.BKHFBLBIOEG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        for v in &self.CDFOAECLKDP {
            os.write_uint32(13, *v)?;
        };
        if let Some(v) = self.LOOPEOMDNOH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.NCPHFIDNGHE != 0 {
            os.write_uint32(14, self.NCPHFIDNGHE)?;
        }
        if let Some(v) = self.KIKJKIACENH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.AMLCAEIJMEE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.KKIKGPPKHIP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.PAIAHLNACGC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> EPEDODHJOJF {
        EPEDODHJOJF::new()
    }

    fn clear(&mut self) {
        self.BKHFBLBIOEG.clear();
        self.CDFOAECLKDP.clear();
        self.LOOPEOMDNOH.clear();
        self.NCPHFIDNGHE = 0;
        self.KIKJKIACENH.clear();
        self.AMLCAEIJMEE.clear();
        self.KKIKGPPKHIP.clear();
        self.PAIAHLNACGC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EPEDODHJOJF {
        static instance: EPEDODHJOJF = EPEDODHJOJF {
            BKHFBLBIOEG: ::protobuf::MessageField::none(),
            CDFOAECLKDP: ::std::vec::Vec::new(),
            LOOPEOMDNOH: ::protobuf::MessageField::none(),
            NCPHFIDNGHE: 0,
            KIKJKIACENH: ::protobuf::MessageField::none(),
            AMLCAEIJMEE: ::protobuf::MessageField::none(),
            KKIKGPPKHIP: ::protobuf::MessageField::none(),
            PAIAHLNACGC: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EPEDODHJOJF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EPEDODHJOJF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EPEDODHJOJF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EPEDODHJOJF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EPEDODHJOJF.proto\x1a\x11BFIDNBACIPF.proto\x1a\x11CGMDBEIHMPK.prot\
    o\x1a\x11FAIFHIBBNAO.proto\x1a\x11LMHKCAHMNPE.proto\x1a\x11MFKFODKMENJ.p\
    roto\x1a\x11OOPKDEMPMFI.proto\"\xf1\x02\n\x0bEPEDODHJOJF\x12.\n\x0bBKHFB\
    LBIOEG\x18\x0f\x20\x01(\x0b2\x0c.BFIDNBACIPFR\x0bBKHFBLBIOEG\x12\x20\n\
    \x0bCDFOAECLKDP\x18\r\x20\x03(\rR\x0bCDFOAECLKDP\x12.\n\x0bLOOPEOMDNOH\
    \x18\x0c\x20\x01(\x0b2\x0c.CGMDBEIHMPKR\x0bLOOPEOMDNOH\x12\x20\n\x0bNCPH\
    FIDNGHE\x18\x0e\x20\x01(\rR\x0bNCPHFIDNGHE\x12.\n\x0bKIKJKIACENH\x18\x01\
    \x20\x01(\x0b2\x0c.MFKFODKMENJR\x0bKIKJKIACENH\x12.\n\x0bAMLCAEIJMEE\x18\
    \x08\x20\x01(\x0b2\x0c.FAIFHIBBNAOR\x0bAMLCAEIJMEE\x12.\n\x0bKKIKGPPKHIP\
    \x18\n\x20\x01(\x0b2\x0c.LMHKCAHMNPER\x0bKKIKGPPKHIP\x12.\n\x0bPAIAHLNAC\
    GC\x18\x05\x20\x01(\x0b2\x0c.OOPKDEMPMFIR\x0bPAIAHLNACGCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::BFIDNBACIPF::file_descriptor().clone());
            deps.push(super::CGMDBEIHMPK::file_descriptor().clone());
            deps.push(super::FAIFHIBBNAO::file_descriptor().clone());
            deps.push(super::LMHKCAHMNPE::file_descriptor().clone());
            deps.push(super::MFKFODKMENJ::file_descriptor().clone());
            deps.push(super::OOPKDEMPMFI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EPEDODHJOJF::generated_message_descriptor_data());
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
