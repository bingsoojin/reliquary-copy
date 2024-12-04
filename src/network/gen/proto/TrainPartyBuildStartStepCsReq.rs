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

//! Generated file from `TrainPartyBuildStartStepCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TrainPartyBuildStartStepCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrainPartyBuildStartStepCsReq {
    // message fields
    // @@protoc_insertion_point(field:TrainPartyBuildStartStepCsReq.OMBLHJDKEJA)
    pub OMBLHJDKEJA: u32,
    // @@protoc_insertion_point(field:TrainPartyBuildStartStepCsReq.NPBHFEFLGJK)
    pub NPBHFEFLGJK: ::protobuf::MessageField<super::IGGCJANLKCL::IGGCJANLKCL>,
    // @@protoc_insertion_point(field:TrainPartyBuildStartStepCsReq.NAPNNKKHDLF)
    pub NAPNNKKHDLF: u32,
    // @@protoc_insertion_point(field:TrainPartyBuildStartStepCsReq.EAPOMIMHKEB)
    pub EAPOMIMHKEB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TrainPartyBuildStartStepCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrainPartyBuildStartStepCsReq {
    fn default() -> &'a TrainPartyBuildStartStepCsReq {
        <TrainPartyBuildStartStepCsReq as ::protobuf::Message>::default_instance()
    }
}

impl TrainPartyBuildStartStepCsReq {
    pub fn new() -> TrainPartyBuildStartStepCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMBLHJDKEJA",
            |m: &TrainPartyBuildStartStepCsReq| { &m.OMBLHJDKEJA },
            |m: &mut TrainPartyBuildStartStepCsReq| { &mut m.OMBLHJDKEJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IGGCJANLKCL::IGGCJANLKCL>(
            "NPBHFEFLGJK",
            |m: &TrainPartyBuildStartStepCsReq| { &m.NPBHFEFLGJK },
            |m: &mut TrainPartyBuildStartStepCsReq| { &mut m.NPBHFEFLGJK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NAPNNKKHDLF",
            |m: &TrainPartyBuildStartStepCsReq| { &m.NAPNNKKHDLF },
            |m: &mut TrainPartyBuildStartStepCsReq| { &mut m.NAPNNKKHDLF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EAPOMIMHKEB",
            |m: &TrainPartyBuildStartStepCsReq| { &m.EAPOMIMHKEB },
            |m: &mut TrainPartyBuildStartStepCsReq| { &mut m.EAPOMIMHKEB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrainPartyBuildStartStepCsReq>(
            "TrainPartyBuildStartStepCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrainPartyBuildStartStepCsReq {
    const NAME: &'static str = "TrainPartyBuildStartStepCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.OMBLHJDKEJA = is.read_uint32()?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NPBHFEFLGJK)?;
                },
                112 => {
                    self.NAPNNKKHDLF = is.read_uint32()?;
                },
                64 => {
                    self.EAPOMIMHKEB = is.read_uint32()?;
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
        if self.OMBLHJDKEJA != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.OMBLHJDKEJA);
        }
        if let Some(v) = self.NPBHFEFLGJK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NAPNNKKHDLF != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.NAPNNKKHDLF);
        }
        if self.EAPOMIMHKEB != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.EAPOMIMHKEB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OMBLHJDKEJA != 0 {
            os.write_uint32(10, self.OMBLHJDKEJA)?;
        }
        if let Some(v) = self.NPBHFEFLGJK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.NAPNNKKHDLF != 0 {
            os.write_uint32(14, self.NAPNNKKHDLF)?;
        }
        if self.EAPOMIMHKEB != 0 {
            os.write_uint32(8, self.EAPOMIMHKEB)?;
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

    fn new() -> TrainPartyBuildStartStepCsReq {
        TrainPartyBuildStartStepCsReq::new()
    }

    fn clear(&mut self) {
        self.OMBLHJDKEJA = 0;
        self.NPBHFEFLGJK.clear();
        self.NAPNNKKHDLF = 0;
        self.EAPOMIMHKEB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrainPartyBuildStartStepCsReq {
        static instance: TrainPartyBuildStartStepCsReq = TrainPartyBuildStartStepCsReq {
            OMBLHJDKEJA: 0,
            NPBHFEFLGJK: ::protobuf::MessageField::none(),
            NAPNNKKHDLF: 0,
            EAPOMIMHKEB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrainPartyBuildStartStepCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrainPartyBuildStartStepCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrainPartyBuildStartStepCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrainPartyBuildStartStepCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#TrainPartyBuildStartStepCsReq.proto\x1a\x11IGGCJANLKCL.proto\"\xb5\
    \x01\n\x1dTrainPartyBuildStartStepCsReq\x12\x20\n\x0bOMBLHJDKEJA\x18\n\
    \x20\x01(\rR\x0bOMBLHJDKEJA\x12.\n\x0bNPBHFEFLGJK\x18\x0c\x20\x01(\x0b2\
    \x0c.IGGCJANLKCLR\x0bNPBHFEFLGJK\x12\x20\n\x0bNAPNNKKHDLF\x18\x0e\x20\
    \x01(\rR\x0bNAPNNKKHDLF\x12\x20\n\x0bEAPOMIMHKEB\x18\x08\x20\x01(\rR\x0b\
    EAPOMIMHKEBb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::IGGCJANLKCL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TrainPartyBuildStartStepCsReq::generated_message_descriptor_data());
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
