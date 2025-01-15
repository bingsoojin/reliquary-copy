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

//! Generated file from `GMFJGMBPAFF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GMFJGMBPAFF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GMFJGMBPAFF {
    // message fields
    // @@protoc_insertion_point(field:GMFJGMBPAFF.DAFLHMLLKDK)
    pub DAFLHMLLKDK: u32,
    // @@protoc_insertion_point(field:GMFJGMBPAFF.FPGDLFIPAEN)
    pub FPGDLFIPAEN: bool,
    // @@protoc_insertion_point(field:GMFJGMBPAFF.KBEKACAMFOO)
    pub KBEKACAMFOO: u32,
    // @@protoc_insertion_point(field:GMFJGMBPAFF.NDBLEEBNCPG)
    pub NDBLEEBNCPG: i64,
    // special fields
    // @@protoc_insertion_point(special_field:GMFJGMBPAFF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GMFJGMBPAFF {
    fn default() -> &'a GMFJGMBPAFF {
        <GMFJGMBPAFF as ::protobuf::Message>::default_instance()
    }
}

impl GMFJGMBPAFF {
    pub fn new() -> GMFJGMBPAFF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAFLHMLLKDK",
            |m: &GMFJGMBPAFF| { &m.DAFLHMLLKDK },
            |m: &mut GMFJGMBPAFF| { &mut m.DAFLHMLLKDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FPGDLFIPAEN",
            |m: &GMFJGMBPAFF| { &m.FPGDLFIPAEN },
            |m: &mut GMFJGMBPAFF| { &mut m.FPGDLFIPAEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KBEKACAMFOO",
            |m: &GMFJGMBPAFF| { &m.KBEKACAMFOO },
            |m: &mut GMFJGMBPAFF| { &mut m.KBEKACAMFOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NDBLEEBNCPG",
            |m: &GMFJGMBPAFF| { &m.NDBLEEBNCPG },
            |m: &mut GMFJGMBPAFF| { &mut m.NDBLEEBNCPG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GMFJGMBPAFF>(
            "GMFJGMBPAFF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GMFJGMBPAFF {
    const NAME: &'static str = "GMFJGMBPAFF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.DAFLHMLLKDK = is.read_uint32()?;
                },
                24 => {
                    self.FPGDLFIPAEN = is.read_bool()?;
                },
                32 => {
                    self.KBEKACAMFOO = is.read_uint32()?;
                },
                104 => {
                    self.NDBLEEBNCPG = is.read_int64()?;
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
        if self.DAFLHMLLKDK != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.DAFLHMLLKDK);
        }
        if self.FPGDLFIPAEN != false {
            my_size += 1 + 1;
        }
        if self.KBEKACAMFOO != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.KBEKACAMFOO);
        }
        if self.NDBLEEBNCPG != 0 {
            my_size += ::protobuf::rt::int64_size(13, self.NDBLEEBNCPG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DAFLHMLLKDK != 0 {
            os.write_uint32(15, self.DAFLHMLLKDK)?;
        }
        if self.FPGDLFIPAEN != false {
            os.write_bool(3, self.FPGDLFIPAEN)?;
        }
        if self.KBEKACAMFOO != 0 {
            os.write_uint32(4, self.KBEKACAMFOO)?;
        }
        if self.NDBLEEBNCPG != 0 {
            os.write_int64(13, self.NDBLEEBNCPG)?;
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

    fn new() -> GMFJGMBPAFF {
        GMFJGMBPAFF::new()
    }

    fn clear(&mut self) {
        self.DAFLHMLLKDK = 0;
        self.FPGDLFIPAEN = false;
        self.KBEKACAMFOO = 0;
        self.NDBLEEBNCPG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GMFJGMBPAFF {
        static instance: GMFJGMBPAFF = GMFJGMBPAFF {
            DAFLHMLLKDK: 0,
            FPGDLFIPAEN: false,
            KBEKACAMFOO: 0,
            NDBLEEBNCPG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GMFJGMBPAFF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GMFJGMBPAFF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GMFJGMBPAFF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GMFJGMBPAFF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GMFJGMBPAFF.proto\"\x95\x01\n\x0bGMFJGMBPAFF\x12\x20\n\x0bDAFLHMLL\
    KDK\x18\x0f\x20\x01(\rR\x0bDAFLHMLLKDK\x12\x20\n\x0bFPGDLFIPAEN\x18\x03\
    \x20\x01(\x08R\x0bFPGDLFIPAEN\x12\x20\n\x0bKBEKACAMFOO\x18\x04\x20\x01(\
    \rR\x0bKBEKACAMFOO\x12\x20\n\x0bNDBLEEBNCPG\x18\r\x20\x01(\x03R\x0bNDBLE\
    EBNCPGb\x06proto3\
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
            messages.push(GMFJGMBPAFF::generated_message_descriptor_data());
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
