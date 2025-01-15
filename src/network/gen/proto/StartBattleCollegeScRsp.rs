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

//! Generated file from `StartBattleCollegeScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartBattleCollegeScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartBattleCollegeScRsp {
    // message fields
    // @@protoc_insertion_point(field:StartBattleCollegeScRsp.FFKNMAONGIB)
    pub FFKNMAONGIB: u32,
    // @@protoc_insertion_point(field:StartBattleCollegeScRsp.LPPMFJOCKNO)
    pub LPPMFJOCKNO: ::protobuf::MessageField<super::DDCPCKIHMEF::DDCPCKIHMEF>,
    // @@protoc_insertion_point(field:StartBattleCollegeScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartBattleCollegeScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartBattleCollegeScRsp {
    fn default() -> &'a StartBattleCollegeScRsp {
        <StartBattleCollegeScRsp as ::protobuf::Message>::default_instance()
    }
}

impl StartBattleCollegeScRsp {
    pub fn new() -> StartBattleCollegeScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFKNMAONGIB",
            |m: &StartBattleCollegeScRsp| { &m.FFKNMAONGIB },
            |m: &mut StartBattleCollegeScRsp| { &mut m.FFKNMAONGIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DDCPCKIHMEF::DDCPCKIHMEF>(
            "LPPMFJOCKNO",
            |m: &StartBattleCollegeScRsp| { &m.LPPMFJOCKNO },
            |m: &mut StartBattleCollegeScRsp| { &mut m.LPPMFJOCKNO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &StartBattleCollegeScRsp| { &m.retcode },
            |m: &mut StartBattleCollegeScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartBattleCollegeScRsp>(
            "StartBattleCollegeScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartBattleCollegeScRsp {
    const NAME: &'static str = "StartBattleCollegeScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.FFKNMAONGIB = is.read_uint32()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LPPMFJOCKNO)?;
                },
                56 => {
                    self.retcode = is.read_uint32()?;
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
        if self.FFKNMAONGIB != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FFKNMAONGIB);
        }
        if let Some(v) = self.LPPMFJOCKNO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FFKNMAONGIB != 0 {
            os.write_uint32(1, self.FFKNMAONGIB)?;
        }
        if let Some(v) = self.LPPMFJOCKNO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(7, self.retcode)?;
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

    fn new() -> StartBattleCollegeScRsp {
        StartBattleCollegeScRsp::new()
    }

    fn clear(&mut self) {
        self.FFKNMAONGIB = 0;
        self.LPPMFJOCKNO.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartBattleCollegeScRsp {
        static instance: StartBattleCollegeScRsp = StartBattleCollegeScRsp {
            FFKNMAONGIB: 0,
            LPPMFJOCKNO: ::protobuf::MessageField::none(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartBattleCollegeScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartBattleCollegeScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartBattleCollegeScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartBattleCollegeScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dStartBattleCollegeScRsp.proto\x1a\x11DDCPCKIHMEF.proto\"\x85\x01\n\
    \x17StartBattleCollegeScRsp\x12\x20\n\x0bFFKNMAONGIB\x18\x01\x20\x01(\rR\
    \x0bFFKNMAONGIB\x12.\n\x0bLPPMFJOCKNO\x18\x0e\x20\x01(\x0b2\x0c.DDCPCKIH\
    MEFR\x0bLPPMFJOCKNO\x12\x18\n\x07retcode\x18\x07\x20\x01(\rR\x07retcodeb\
    \x06proto3\
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
            deps.push(super::DDCPCKIHMEF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartBattleCollegeScRsp::generated_message_descriptor_data());
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
