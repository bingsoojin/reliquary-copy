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

//! Generated file from `HeartDialTraceScriptScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HeartDialTraceScriptScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HeartDialTraceScriptScRsp {
    // message fields
    // @@protoc_insertion_point(field:HeartDialTraceScriptScRsp.PENKIFIKIAC)
    pub PENKIFIKIAC: ::protobuf::MessageField<super::HNGOJHBPFEJ::HNGOJHBPFEJ>,
    // @@protoc_insertion_point(field:HeartDialTraceScriptScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HeartDialTraceScriptScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HeartDialTraceScriptScRsp {
    fn default() -> &'a HeartDialTraceScriptScRsp {
        <HeartDialTraceScriptScRsp as ::protobuf::Message>::default_instance()
    }
}

impl HeartDialTraceScriptScRsp {
    pub fn new() -> HeartDialTraceScriptScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HNGOJHBPFEJ::HNGOJHBPFEJ>(
            "PENKIFIKIAC",
            |m: &HeartDialTraceScriptScRsp| { &m.PENKIFIKIAC },
            |m: &mut HeartDialTraceScriptScRsp| { &mut m.PENKIFIKIAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &HeartDialTraceScriptScRsp| { &m.ADADHIHDHJC },
            |m: &mut HeartDialTraceScriptScRsp| { &mut m.ADADHIHDHJC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HeartDialTraceScriptScRsp>(
            "HeartDialTraceScriptScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HeartDialTraceScriptScRsp {
    const NAME: &'static str = "HeartDialTraceScriptScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PENKIFIKIAC)?;
                },
                88 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
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
        if let Some(v) = self.PENKIFIKIAC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.ADADHIHDHJC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.PENKIFIKIAC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(11, self.ADADHIHDHJC)?;
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

    fn new() -> HeartDialTraceScriptScRsp {
        HeartDialTraceScriptScRsp::new()
    }

    fn clear(&mut self) {
        self.PENKIFIKIAC.clear();
        self.ADADHIHDHJC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HeartDialTraceScriptScRsp {
        static instance: HeartDialTraceScriptScRsp = HeartDialTraceScriptScRsp {
            PENKIFIKIAC: ::protobuf::MessageField::none(),
            ADADHIHDHJC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HeartDialTraceScriptScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HeartDialTraceScriptScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HeartDialTraceScriptScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeartDialTraceScriptScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fHeartDialTraceScriptScRsp.proto\x1a\x11HNGOJHBPFEJ.proto\"m\n\x19H\
    eartDialTraceScriptScRsp\x12.\n\x0bPENKIFIKIAC\x18\x0c\x20\x01(\x0b2\x0c\
    .HNGOJHBPFEJR\x0bPENKIFIKIAC\x12\x20\n\x0bADADHIHDHJC\x18\x0b\x20\x01(\r\
    R\x0bADADHIHDHJCb\x06proto3\
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
            deps.push(super::HNGOJHBPFEJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HeartDialTraceScriptScRsp::generated_message_descriptor_data());
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
