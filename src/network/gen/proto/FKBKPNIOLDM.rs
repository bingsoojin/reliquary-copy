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

//! Generated file from `FKBKPNIOLDM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FKBKPNIOLDM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FKBKPNIOLDM {
    // message fields
    // @@protoc_insertion_point(field:FKBKPNIOLDM.BGLBHIBOKLN)
    pub BGLBHIBOKLN: u32,
    // @@protoc_insertion_point(field:FKBKPNIOLDM.PJBNFBFPNCH)
    pub PJBNFBFPNCH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FKBKPNIOLDM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FKBKPNIOLDM {
    fn default() -> &'a FKBKPNIOLDM {
        <FKBKPNIOLDM as ::protobuf::Message>::default_instance()
    }
}

impl FKBKPNIOLDM {
    pub fn new() -> FKBKPNIOLDM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BGLBHIBOKLN",
            |m: &FKBKPNIOLDM| { &m.BGLBHIBOKLN },
            |m: &mut FKBKPNIOLDM| { &mut m.BGLBHIBOKLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJBNFBFPNCH",
            |m: &FKBKPNIOLDM| { &m.PJBNFBFPNCH },
            |m: &mut FKBKPNIOLDM| { &mut m.PJBNFBFPNCH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FKBKPNIOLDM>(
            "FKBKPNIOLDM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FKBKPNIOLDM {
    const NAME: &'static str = "FKBKPNIOLDM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.BGLBHIBOKLN = is.read_uint32()?;
                },
                120 => {
                    self.PJBNFBFPNCH = is.read_uint32()?;
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
        if self.BGLBHIBOKLN != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.BGLBHIBOKLN);
        }
        if self.PJBNFBFPNCH != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.PJBNFBFPNCH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BGLBHIBOKLN != 0 {
            os.write_uint32(4, self.BGLBHIBOKLN)?;
        }
        if self.PJBNFBFPNCH != 0 {
            os.write_uint32(15, self.PJBNFBFPNCH)?;
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

    fn new() -> FKBKPNIOLDM {
        FKBKPNIOLDM::new()
    }

    fn clear(&mut self) {
        self.BGLBHIBOKLN = 0;
        self.PJBNFBFPNCH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FKBKPNIOLDM {
        static instance: FKBKPNIOLDM = FKBKPNIOLDM {
            BGLBHIBOKLN: 0,
            PJBNFBFPNCH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FKBKPNIOLDM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FKBKPNIOLDM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FKBKPNIOLDM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FKBKPNIOLDM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FKBKPNIOLDM.proto\"Q\n\x0bFKBKPNIOLDM\x12\x20\n\x0bBGLBHIBOKLN\x18\
    \x04\x20\x01(\rR\x0bBGLBHIBOKLN\x12\x20\n\x0bPJBNFBFPNCH\x18\x0f\x20\x01\
    (\rR\x0bPJBNFBFPNCHb\x06proto3\
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
            messages.push(FKBKPNIOLDM::generated_message_descriptor_data());
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
