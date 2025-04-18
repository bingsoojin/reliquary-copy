// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `AOIHEKLNDID.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AOIHEKLNDID)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AOIHEKLNDID {
    // message fields
    // @@protoc_insertion_point(field:AOIHEKLNDID.ALGAFOMNIIA)
    pub ALGAFOMNIIA: bool,
    // @@protoc_insertion_point(field:AOIHEKLNDID.AKCGHBFGBCC)
    pub AKCGHBFGBCC: bool,
    // special fields
    // @@protoc_insertion_point(special_field:AOIHEKLNDID.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AOIHEKLNDID {
    fn default() -> &'a AOIHEKLNDID {
        <AOIHEKLNDID as ::protobuf::Message>::default_instance()
    }
}

impl AOIHEKLNDID {
    pub fn new() -> AOIHEKLNDID {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ALGAFOMNIIA",
            |m: &AOIHEKLNDID| { &m.ALGAFOMNIIA },
            |m: &mut AOIHEKLNDID| { &mut m.ALGAFOMNIIA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKCGHBFGBCC",
            |m: &AOIHEKLNDID| { &m.AKCGHBFGBCC },
            |m: &mut AOIHEKLNDID| { &mut m.AKCGHBFGBCC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AOIHEKLNDID>(
            "AOIHEKLNDID",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AOIHEKLNDID {
    const NAME: &'static str = "AOIHEKLNDID";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.ALGAFOMNIIA = is.read_bool()?;
                },
                104 => {
                    self.AKCGHBFGBCC = is.read_bool()?;
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
        if self.ALGAFOMNIIA != false {
            my_size += 1 + 1;
        }
        if self.AKCGHBFGBCC != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ALGAFOMNIIA != false {
            os.write_bool(3, self.ALGAFOMNIIA)?;
        }
        if self.AKCGHBFGBCC != false {
            os.write_bool(13, self.AKCGHBFGBCC)?;
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

    fn new() -> AOIHEKLNDID {
        AOIHEKLNDID::new()
    }

    fn clear(&mut self) {
        self.ALGAFOMNIIA = false;
        self.AKCGHBFGBCC = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AOIHEKLNDID {
        static instance: AOIHEKLNDID = AOIHEKLNDID {
            ALGAFOMNIIA: false,
            AKCGHBFGBCC: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AOIHEKLNDID {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AOIHEKLNDID").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AOIHEKLNDID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AOIHEKLNDID {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AOIHEKLNDID.proto\"Q\n\x0bAOIHEKLNDID\x12\x20\n\x0bALGAFOMNIIA\x18\
    \x03\x20\x01(\x08R\x0bALGAFOMNIIA\x12\x20\n\x0bAKCGHBFGBCC\x18\r\x20\x01\
    (\x08R\x0bAKCGHBFGBCCb\x06proto3\
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
            messages.push(AOIHEKLNDID::generated_message_descriptor_data());
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
