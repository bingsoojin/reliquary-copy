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

//! Generated file from `CNEGKPBIFCA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CNEGKPBIFCA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CNEGKPBIFCA {
    // message fields
    // @@protoc_insertion_point(field:CNEGKPBIFCA.HJHAIHFEFEN)
    pub HJHAIHFEFEN: u32,
    // @@protoc_insertion_point(field:CNEGKPBIFCA.OJBAILGKLBM)
    pub OJBAILGKLBM: ::protobuf::EnumOrUnknown<super::EPMEDGJKJOB::EPMEDGJKJOB>,
    // @@protoc_insertion_point(field:CNEGKPBIFCA.OHLACBFPACN)
    pub OHLACBFPACN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:CNEGKPBIFCA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CNEGKPBIFCA {
    fn default() -> &'a CNEGKPBIFCA {
        <CNEGKPBIFCA as ::protobuf::Message>::default_instance()
    }
}

impl CNEGKPBIFCA {
    pub fn new() -> CNEGKPBIFCA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJHAIHFEFEN",
            |m: &CNEGKPBIFCA| { &m.HJHAIHFEFEN },
            |m: &mut CNEGKPBIFCA| { &mut m.HJHAIHFEFEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJBAILGKLBM",
            |m: &CNEGKPBIFCA| { &m.OJBAILGKLBM },
            |m: &mut CNEGKPBIFCA| { &mut m.OJBAILGKLBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OHLACBFPACN",
            |m: &CNEGKPBIFCA| { &m.OHLACBFPACN },
            |m: &mut CNEGKPBIFCA| { &mut m.OHLACBFPACN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CNEGKPBIFCA>(
            "CNEGKPBIFCA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CNEGKPBIFCA {
    const NAME: &'static str = "CNEGKPBIFCA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.HJHAIHFEFEN = is.read_uint32()?;
                },
                64 => {
                    self.OJBAILGKLBM = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.OHLACBFPACN = is.read_uint32()?;
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
        if self.HJHAIHFEFEN != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.HJHAIHFEFEN);
        }
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::EPMEDGJKJOB::EPMEDGJKJOB::ROGUE_BOOTH_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.OJBAILGKLBM.value());
        }
        if self.OHLACBFPACN != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.OHLACBFPACN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HJHAIHFEFEN != 0 {
            os.write_uint32(11, self.HJHAIHFEFEN)?;
        }
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::EPMEDGJKJOB::EPMEDGJKJOB::ROGUE_BOOTH_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.OJBAILGKLBM))?;
        }
        if self.OHLACBFPACN != 0 {
            os.write_uint32(14, self.OHLACBFPACN)?;
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

    fn new() -> CNEGKPBIFCA {
        CNEGKPBIFCA::new()
    }

    fn clear(&mut self) {
        self.HJHAIHFEFEN = 0;
        self.OJBAILGKLBM = ::protobuf::EnumOrUnknown::new(super::EPMEDGJKJOB::EPMEDGJKJOB::ROGUE_BOOTH_NONE);
        self.OHLACBFPACN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CNEGKPBIFCA {
        static instance: CNEGKPBIFCA = CNEGKPBIFCA {
            HJHAIHFEFEN: 0,
            OJBAILGKLBM: ::protobuf::EnumOrUnknown::from_i32(0),
            OHLACBFPACN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CNEGKPBIFCA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CNEGKPBIFCA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CNEGKPBIFCA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNEGKPBIFCA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CNEGKPBIFCA.proto\x1a\x11EPMEDGJKJOB.proto\"\x81\x01\n\x0bCNEGKPBI\
    FCA\x12\x20\n\x0bHJHAIHFEFEN\x18\x0b\x20\x01(\rR\x0bHJHAIHFEFEN\x12.\n\
    \x0bOJBAILGKLBM\x18\x08\x20\x01(\x0e2\x0c.EPMEDGJKJOBR\x0bOJBAILGKLBM\
    \x12\x20\n\x0bOHLACBFPACN\x18\x0e\x20\x01(\rR\x0bOHLACBFPACNb\x06proto3\
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
            deps.push(super::EPMEDGJKJOB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CNEGKPBIFCA::generated_message_descriptor_data());
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
