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

//! Generated file from `FKFEBAHFGCD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FKFEBAHFGCD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FKFEBAHFGCD {
    // message fields
    // @@protoc_insertion_point(field:FKFEBAHFGCD.IPNHCCODNDI)
    pub IPNHCCODNDI: u32,
    // @@protoc_insertion_point(field:FKFEBAHFGCD.FINLPBFNLHP)
    pub FINLPBFNLHP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FKFEBAHFGCD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FKFEBAHFGCD {
    fn default() -> &'a FKFEBAHFGCD {
        <FKFEBAHFGCD as ::protobuf::Message>::default_instance()
    }
}

impl FKFEBAHFGCD {
    pub fn new() -> FKFEBAHFGCD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPNHCCODNDI",
            |m: &FKFEBAHFGCD| { &m.IPNHCCODNDI },
            |m: &mut FKFEBAHFGCD| { &mut m.IPNHCCODNDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FINLPBFNLHP",
            |m: &FKFEBAHFGCD| { &m.FINLPBFNLHP },
            |m: &mut FKFEBAHFGCD| { &mut m.FINLPBFNLHP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FKFEBAHFGCD>(
            "FKFEBAHFGCD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FKFEBAHFGCD {
    const NAME: &'static str = "FKFEBAHFGCD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.IPNHCCODNDI = is.read_uint32()?;
                },
                88 => {
                    self.FINLPBFNLHP = is.read_uint32()?;
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
        if self.IPNHCCODNDI != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.IPNHCCODNDI);
        }
        if self.FINLPBFNLHP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.FINLPBFNLHP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IPNHCCODNDI != 0 {
            os.write_uint32(12, self.IPNHCCODNDI)?;
        }
        if self.FINLPBFNLHP != 0 {
            os.write_uint32(11, self.FINLPBFNLHP)?;
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

    fn new() -> FKFEBAHFGCD {
        FKFEBAHFGCD::new()
    }

    fn clear(&mut self) {
        self.IPNHCCODNDI = 0;
        self.FINLPBFNLHP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FKFEBAHFGCD {
        static instance: FKFEBAHFGCD = FKFEBAHFGCD {
            IPNHCCODNDI: 0,
            FINLPBFNLHP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FKFEBAHFGCD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FKFEBAHFGCD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FKFEBAHFGCD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FKFEBAHFGCD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FKFEBAHFGCD.proto\"Q\n\x0bFKFEBAHFGCD\x12\x20\n\x0bIPNHCCODNDI\x18\
    \x0c\x20\x01(\rR\x0bIPNHCCODNDI\x12\x20\n\x0bFINLPBFNLHP\x18\x0b\x20\x01\
    (\rR\x0bFINLPBFNLHPb\x06proto3\
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
            messages.push(FKFEBAHFGCD::generated_message_descriptor_data());
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
