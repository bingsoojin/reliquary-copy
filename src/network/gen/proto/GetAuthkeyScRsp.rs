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

//! Generated file from `GetAuthkeyScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetAuthkeyScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetAuthkeyScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetAuthkeyScRsp.GOIJEMCCCGL)
    pub GOIJEMCCCGL: u32,
    // @@protoc_insertion_point(field:GetAuthkeyScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetAuthkeyScRsp.MPGPGFKBMHG)
    pub MPGPGFKBMHG: u32,
    // @@protoc_insertion_point(field:GetAuthkeyScRsp.ACDJGNHJDAI)
    pub ACDJGNHJDAI: ::std::string::String,
    // @@protoc_insertion_point(field:GetAuthkeyScRsp.KBNIPIIICCN)
    pub KBNIPIIICCN: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:GetAuthkeyScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetAuthkeyScRsp {
    fn default() -> &'a GetAuthkeyScRsp {
        <GetAuthkeyScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetAuthkeyScRsp {
    pub fn new() -> GetAuthkeyScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GOIJEMCCCGL",
            |m: &GetAuthkeyScRsp| { &m.GOIJEMCCCGL },
            |m: &mut GetAuthkeyScRsp| { &mut m.GOIJEMCCCGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetAuthkeyScRsp| { &m.retcode },
            |m: &mut GetAuthkeyScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPGPGFKBMHG",
            |m: &GetAuthkeyScRsp| { &m.MPGPGFKBMHG },
            |m: &mut GetAuthkeyScRsp| { &mut m.MPGPGFKBMHG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACDJGNHJDAI",
            |m: &GetAuthkeyScRsp| { &m.ACDJGNHJDAI },
            |m: &mut GetAuthkeyScRsp| { &mut m.ACDJGNHJDAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KBNIPIIICCN",
            |m: &GetAuthkeyScRsp| { &m.KBNIPIIICCN },
            |m: &mut GetAuthkeyScRsp| { &mut m.KBNIPIIICCN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetAuthkeyScRsp>(
            "GetAuthkeyScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetAuthkeyScRsp {
    const NAME: &'static str = "GetAuthkeyScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.GOIJEMCCCGL = is.read_uint32()?;
                },
                16 => {
                    self.retcode = is.read_uint32()?;
                },
                96 => {
                    self.MPGPGFKBMHG = is.read_uint32()?;
                },
                106 => {
                    self.ACDJGNHJDAI = is.read_string()?;
                },
                122 => {
                    self.KBNIPIIICCN = is.read_string()?;
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
        if self.GOIJEMCCCGL != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.GOIJEMCCCGL);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        if self.MPGPGFKBMHG != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.MPGPGFKBMHG);
        }
        if !self.ACDJGNHJDAI.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.ACDJGNHJDAI);
        }
        if !self.KBNIPIIICCN.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.KBNIPIIICCN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GOIJEMCCCGL != 0 {
            os.write_uint32(6, self.GOIJEMCCCGL)?;
        }
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
        }
        if self.MPGPGFKBMHG != 0 {
            os.write_uint32(12, self.MPGPGFKBMHG)?;
        }
        if !self.ACDJGNHJDAI.is_empty() {
            os.write_string(13, &self.ACDJGNHJDAI)?;
        }
        if !self.KBNIPIIICCN.is_empty() {
            os.write_string(15, &self.KBNIPIIICCN)?;
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

    fn new() -> GetAuthkeyScRsp {
        GetAuthkeyScRsp::new()
    }

    fn clear(&mut self) {
        self.GOIJEMCCCGL = 0;
        self.retcode = 0;
        self.MPGPGFKBMHG = 0;
        self.ACDJGNHJDAI.clear();
        self.KBNIPIIICCN.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetAuthkeyScRsp {
        static instance: GetAuthkeyScRsp = GetAuthkeyScRsp {
            GOIJEMCCCGL: 0,
            retcode: 0,
            MPGPGFKBMHG: 0,
            ACDJGNHJDAI: ::std::string::String::new(),
            KBNIPIIICCN: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetAuthkeyScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetAuthkeyScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetAuthkeyScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAuthkeyScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15GetAuthkeyScRsp.proto\"\xb3\x01\n\x0fGetAuthkeyScRsp\x12\x20\n\x0b\
    GOIJEMCCCGL\x18\x06\x20\x01(\rR\x0bGOIJEMCCCGL\x12\x18\n\x07retcode\x18\
    \x02\x20\x01(\rR\x07retcode\x12\x20\n\x0bMPGPGFKBMHG\x18\x0c\x20\x01(\rR\
    \x0bMPGPGFKBMHG\x12\x20\n\x0bACDJGNHJDAI\x18\r\x20\x01(\tR\x0bACDJGNHJDA\
    I\x12\x20\n\x0bKBNIPIIICCN\x18\x0f\x20\x01(\tR\x0bKBNIPIIICCNb\x06proto3\
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
            messages.push(GetAuthkeyScRsp::generated_message_descriptor_data());
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
