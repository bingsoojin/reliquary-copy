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

//! Generated file from `GEDJODIELBB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GEDJODIELBB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GEDJODIELBB {
    // message fields
    // @@protoc_insertion_point(field:GEDJODIELBB.MIJHFMNPLIP)
    pub MIJHFMNPLIP: u32,
    // @@protoc_insertion_point(field:GEDJODIELBB.HOOICLKBEFF)
    pub HOOICLKBEFF: u32,
    // @@protoc_insertion_point(field:GEDJODIELBB.PHBPBGIJEKG)
    pub PHBPBGIJEKG: u32,
    // @@protoc_insertion_point(field:GEDJODIELBB.IOPPGEGDHGL)
    pub IOPPGEGDHGL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GEDJODIELBB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GEDJODIELBB {
    fn default() -> &'a GEDJODIELBB {
        <GEDJODIELBB as ::protobuf::Message>::default_instance()
    }
}

impl GEDJODIELBB {
    pub fn new() -> GEDJODIELBB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MIJHFMNPLIP",
            |m: &GEDJODIELBB| { &m.MIJHFMNPLIP },
            |m: &mut GEDJODIELBB| { &mut m.MIJHFMNPLIP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HOOICLKBEFF",
            |m: &GEDJODIELBB| { &m.HOOICLKBEFF },
            |m: &mut GEDJODIELBB| { &mut m.HOOICLKBEFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PHBPBGIJEKG",
            |m: &GEDJODIELBB| { &m.PHBPBGIJEKG },
            |m: &mut GEDJODIELBB| { &mut m.PHBPBGIJEKG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOPPGEGDHGL",
            |m: &GEDJODIELBB| { &m.IOPPGEGDHGL },
            |m: &mut GEDJODIELBB| { &mut m.IOPPGEGDHGL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GEDJODIELBB>(
            "GEDJODIELBB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GEDJODIELBB {
    const NAME: &'static str = "GEDJODIELBB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.MIJHFMNPLIP = is.read_uint32()?;
                },
                112 => {
                    self.HOOICLKBEFF = is.read_uint32()?;
                },
                72 => {
                    self.PHBPBGIJEKG = is.read_uint32()?;
                },
                104 => {
                    self.IOPPGEGDHGL = is.read_uint32()?;
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
        if self.MIJHFMNPLIP != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.MIJHFMNPLIP);
        }
        if self.HOOICLKBEFF != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.HOOICLKBEFF);
        }
        if self.PHBPBGIJEKG != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.PHBPBGIJEKG);
        }
        if self.IOPPGEGDHGL != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.IOPPGEGDHGL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MIJHFMNPLIP != 0 {
            os.write_uint32(15, self.MIJHFMNPLIP)?;
        }
        if self.HOOICLKBEFF != 0 {
            os.write_uint32(14, self.HOOICLKBEFF)?;
        }
        if self.PHBPBGIJEKG != 0 {
            os.write_uint32(9, self.PHBPBGIJEKG)?;
        }
        if self.IOPPGEGDHGL != 0 {
            os.write_uint32(13, self.IOPPGEGDHGL)?;
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

    fn new() -> GEDJODIELBB {
        GEDJODIELBB::new()
    }

    fn clear(&mut self) {
        self.MIJHFMNPLIP = 0;
        self.HOOICLKBEFF = 0;
        self.PHBPBGIJEKG = 0;
        self.IOPPGEGDHGL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GEDJODIELBB {
        static instance: GEDJODIELBB = GEDJODIELBB {
            MIJHFMNPLIP: 0,
            HOOICLKBEFF: 0,
            PHBPBGIJEKG: 0,
            IOPPGEGDHGL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GEDJODIELBB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GEDJODIELBB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GEDJODIELBB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GEDJODIELBB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GEDJODIELBB.proto\"\x95\x01\n\x0bGEDJODIELBB\x12\x20\n\x0bMIJHFMNP\
    LIP\x18\x0f\x20\x01(\rR\x0bMIJHFMNPLIP\x12\x20\n\x0bHOOICLKBEFF\x18\x0e\
    \x20\x01(\rR\x0bHOOICLKBEFF\x12\x20\n\x0bPHBPBGIJEKG\x18\t\x20\x01(\rR\
    \x0bPHBPBGIJEKG\x12\x20\n\x0bIOPPGEGDHGL\x18\r\x20\x01(\rR\x0bIOPPGEGDHG\
    Lb\x06proto3\
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
            messages.push(GEDJODIELBB::generated_message_descriptor_data());
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
