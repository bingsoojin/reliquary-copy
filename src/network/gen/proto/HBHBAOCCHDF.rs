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

//! Generated file from `HBHBAOCCHDF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HBHBAOCCHDF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HBHBAOCCHDF {
    // message fields
    // @@protoc_insertion_point(field:HBHBAOCCHDF.LBCCHMLPACD)
    pub LBCCHMLPACD: ::protobuf::MessageField<super::MLJBIFELFCN::MLJBIFELFCN>,
    // @@protoc_insertion_point(field:HBHBAOCCHDF.KIHFIIMHNIF)
    pub KIHFIIMHNIF: ::protobuf::MessageField<super::EHOCFBLOPKL::EHOCFBLOPKL>,
    // @@protoc_insertion_point(field:HBHBAOCCHDF.NEMGEOPCBAO)
    pub NEMGEOPCBAO: ::protobuf::MessageField<super::GDHMBEFBPHM::GDHMBEFBPHM>,
    // @@protoc_insertion_point(field:HBHBAOCCHDF.NMJLDCHMDPE)
    pub NMJLDCHMDPE: ::protobuf::MessageField<super::EAEBDMBNMCL::EAEBDMBNMCL>,
    // @@protoc_insertion_point(field:HBHBAOCCHDF.JCBGNKMLIKN)
    pub JCBGNKMLIKN: ::protobuf::MessageField<super::JHDCKDNIFID::JHDCKDNIFID>,
    // special fields
    // @@protoc_insertion_point(special_field:HBHBAOCCHDF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HBHBAOCCHDF {
    fn default() -> &'a HBHBAOCCHDF {
        <HBHBAOCCHDF as ::protobuf::Message>::default_instance()
    }
}

impl HBHBAOCCHDF {
    pub fn new() -> HBHBAOCCHDF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MLJBIFELFCN::MLJBIFELFCN>(
            "LBCCHMLPACD",
            |m: &HBHBAOCCHDF| { &m.LBCCHMLPACD },
            |m: &mut HBHBAOCCHDF| { &mut m.LBCCHMLPACD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EHOCFBLOPKL::EHOCFBLOPKL>(
            "KIHFIIMHNIF",
            |m: &HBHBAOCCHDF| { &m.KIHFIIMHNIF },
            |m: &mut HBHBAOCCHDF| { &mut m.KIHFIIMHNIF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GDHMBEFBPHM::GDHMBEFBPHM>(
            "NEMGEOPCBAO",
            |m: &HBHBAOCCHDF| { &m.NEMGEOPCBAO },
            |m: &mut HBHBAOCCHDF| { &mut m.NEMGEOPCBAO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EAEBDMBNMCL::EAEBDMBNMCL>(
            "NMJLDCHMDPE",
            |m: &HBHBAOCCHDF| { &m.NMJLDCHMDPE },
            |m: &mut HBHBAOCCHDF| { &mut m.NMJLDCHMDPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JHDCKDNIFID::JHDCKDNIFID>(
            "JCBGNKMLIKN",
            |m: &HBHBAOCCHDF| { &m.JCBGNKMLIKN },
            |m: &mut HBHBAOCCHDF| { &mut m.JCBGNKMLIKN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HBHBAOCCHDF>(
            "HBHBAOCCHDF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HBHBAOCCHDF {
    const NAME: &'static str = "HBHBAOCCHDF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LBCCHMLPACD)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KIHFIIMHNIF)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NEMGEOPCBAO)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NMJLDCHMDPE)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JCBGNKMLIKN)?;
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
        if let Some(v) = self.LBCCHMLPACD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KIHFIIMHNIF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.NEMGEOPCBAO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.NMJLDCHMDPE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.JCBGNKMLIKN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.LBCCHMLPACD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.KIHFIIMHNIF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.NEMGEOPCBAO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.NMJLDCHMDPE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.JCBGNKMLIKN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> HBHBAOCCHDF {
        HBHBAOCCHDF::new()
    }

    fn clear(&mut self) {
        self.LBCCHMLPACD.clear();
        self.KIHFIIMHNIF.clear();
        self.NEMGEOPCBAO.clear();
        self.NMJLDCHMDPE.clear();
        self.JCBGNKMLIKN.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HBHBAOCCHDF {
        static instance: HBHBAOCCHDF = HBHBAOCCHDF {
            LBCCHMLPACD: ::protobuf::MessageField::none(),
            KIHFIIMHNIF: ::protobuf::MessageField::none(),
            NEMGEOPCBAO: ::protobuf::MessageField::none(),
            NMJLDCHMDPE: ::protobuf::MessageField::none(),
            JCBGNKMLIKN: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HBHBAOCCHDF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HBHBAOCCHDF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HBHBAOCCHDF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HBHBAOCCHDF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HBHBAOCCHDF.proto\x1a\x11EAEBDMBNMCL.proto\x1a\x11EHOCFBLOPKL.prot\
    o\x1a\x11GDHMBEFBPHM.proto\x1a\x11JHDCKDNIFID.proto\x1a\x11MLJBIFELFCN.p\
    roto\"\xfd\x01\n\x0bHBHBAOCCHDF\x12.\n\x0bLBCCHMLPACD\x18\x01\x20\x01(\
    \x0b2\x0c.MLJBIFELFCNR\x0bLBCCHMLPACD\x12.\n\x0bKIHFIIMHNIF\x18\x0f\x20\
    \x01(\x0b2\x0c.EHOCFBLOPKLR\x0bKIHFIIMHNIF\x12.\n\x0bNEMGEOPCBAO\x18\n\
    \x20\x01(\x0b2\x0c.GDHMBEFBPHMR\x0bNEMGEOPCBAO\x12.\n\x0bNMJLDCHMDPE\x18\
    \x06\x20\x01(\x0b2\x0c.EAEBDMBNMCLR\x0bNMJLDCHMDPE\x12.\n\x0bJCBGNKMLIKN\
    \x18\t\x20\x01(\x0b2\x0c.JHDCKDNIFIDR\x0bJCBGNKMLIKNb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::EAEBDMBNMCL::file_descriptor().clone());
            deps.push(super::EHOCFBLOPKL::file_descriptor().clone());
            deps.push(super::GDHMBEFBPHM::file_descriptor().clone());
            deps.push(super::JHDCKDNIFID::file_descriptor().clone());
            deps.push(super::MLJBIFELFCN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HBHBAOCCHDF::generated_message_descriptor_data());
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
