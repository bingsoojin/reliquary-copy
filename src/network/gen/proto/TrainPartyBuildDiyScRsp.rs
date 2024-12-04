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

//! Generated file from `TrainPartyBuildDiyScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TrainPartyBuildDiyScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrainPartyBuildDiyScRsp {
    // message fields
    // @@protoc_insertion_point(field:TrainPartyBuildDiyScRsp.BLDHIEJJNJC)
    pub BLDHIEJJNJC: ::std::vec::Vec<super::FDJBLAFDLAC::FDJBLAFDLAC>,
    // @@protoc_insertion_point(field:TrainPartyBuildDiyScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:TrainPartyBuildDiyScRsp.OMBLHJDKEJA)
    pub OMBLHJDKEJA: u32,
    // @@protoc_insertion_point(field:TrainPartyBuildDiyScRsp.HEMODCFENHP)
    pub HEMODCFENHP: bool,
    // @@protoc_insertion_point(field:TrainPartyBuildDiyScRsp.HMNLCHNLEAP)
    pub HMNLCHNLEAP: ::std::vec::Vec<super::IGGCJANLKCL::IGGCJANLKCL>,
    // special fields
    // @@protoc_insertion_point(special_field:TrainPartyBuildDiyScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrainPartyBuildDiyScRsp {
    fn default() -> &'a TrainPartyBuildDiyScRsp {
        <TrainPartyBuildDiyScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TrainPartyBuildDiyScRsp {
    pub fn new() -> TrainPartyBuildDiyScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BLDHIEJJNJC",
            |m: &TrainPartyBuildDiyScRsp| { &m.BLDHIEJJNJC },
            |m: &mut TrainPartyBuildDiyScRsp| { &mut m.BLDHIEJJNJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &TrainPartyBuildDiyScRsp| { &m.ADADHIHDHJC },
            |m: &mut TrainPartyBuildDiyScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMBLHJDKEJA",
            |m: &TrainPartyBuildDiyScRsp| { &m.OMBLHJDKEJA },
            |m: &mut TrainPartyBuildDiyScRsp| { &mut m.OMBLHJDKEJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HEMODCFENHP",
            |m: &TrainPartyBuildDiyScRsp| { &m.HEMODCFENHP },
            |m: &mut TrainPartyBuildDiyScRsp| { &mut m.HEMODCFENHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HMNLCHNLEAP",
            |m: &TrainPartyBuildDiyScRsp| { &m.HMNLCHNLEAP },
            |m: &mut TrainPartyBuildDiyScRsp| { &mut m.HMNLCHNLEAP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrainPartyBuildDiyScRsp>(
            "TrainPartyBuildDiyScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrainPartyBuildDiyScRsp {
    const NAME: &'static str = "TrainPartyBuildDiyScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.BLDHIEJJNJC.push(is.read_message()?);
                },
                16 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                56 => {
                    self.OMBLHJDKEJA = is.read_uint32()?;
                },
                88 => {
                    self.HEMODCFENHP = is.read_bool()?;
                },
                74 => {
                    self.HMNLCHNLEAP.push(is.read_message()?);
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
        for value in &self.BLDHIEJJNJC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ADADHIHDHJC);
        }
        if self.OMBLHJDKEJA != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.OMBLHJDKEJA);
        }
        if self.HEMODCFENHP != false {
            my_size += 1 + 1;
        }
        for value in &self.HMNLCHNLEAP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.BLDHIEJJNJC {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(2, self.ADADHIHDHJC)?;
        }
        if self.OMBLHJDKEJA != 0 {
            os.write_uint32(7, self.OMBLHJDKEJA)?;
        }
        if self.HEMODCFENHP != false {
            os.write_bool(11, self.HEMODCFENHP)?;
        }
        for v in &self.HMNLCHNLEAP {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> TrainPartyBuildDiyScRsp {
        TrainPartyBuildDiyScRsp::new()
    }

    fn clear(&mut self) {
        self.BLDHIEJJNJC.clear();
        self.ADADHIHDHJC = 0;
        self.OMBLHJDKEJA = 0;
        self.HEMODCFENHP = false;
        self.HMNLCHNLEAP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrainPartyBuildDiyScRsp {
        static instance: TrainPartyBuildDiyScRsp = TrainPartyBuildDiyScRsp {
            BLDHIEJJNJC: ::std::vec::Vec::new(),
            ADADHIHDHJC: 0,
            OMBLHJDKEJA: 0,
            HEMODCFENHP: false,
            HMNLCHNLEAP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrainPartyBuildDiyScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrainPartyBuildDiyScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrainPartyBuildDiyScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrainPartyBuildDiyScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dTrainPartyBuildDiyScRsp.proto\x1a\x11FDJBLAFDLAC.proto\x1a\x11IGGC\
    JANLKCL.proto\"\xdf\x01\n\x17TrainPartyBuildDiyScRsp\x12.\n\x0bBLDHIEJJN\
    JC\x18\x08\x20\x03(\x0b2\x0c.FDJBLAFDLACR\x0bBLDHIEJJNJC\x12\x20\n\x0bAD\
    ADHIHDHJC\x18\x02\x20\x01(\rR\x0bADADHIHDHJC\x12\x20\n\x0bOMBLHJDKEJA\
    \x18\x07\x20\x01(\rR\x0bOMBLHJDKEJA\x12\x20\n\x0bHEMODCFENHP\x18\x0b\x20\
    \x01(\x08R\x0bHEMODCFENHP\x12.\n\x0bHMNLCHNLEAP\x18\t\x20\x03(\x0b2\x0c.\
    IGGCJANLKCLR\x0bHMNLCHNLEAPb\x06proto3\
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
            deps.push(super::FDJBLAFDLAC::file_descriptor().clone());
            deps.push(super::IGGCJANLKCL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TrainPartyBuildDiyScRsp::generated_message_descriptor_data());
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
